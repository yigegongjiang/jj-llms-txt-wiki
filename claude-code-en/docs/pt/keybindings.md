> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personalizar atalhos de teclado

> Personalize atalhos de teclado no Claude Code com um arquivo de configuração de keybindings.

Claude Code suporta atalhos de teclado personalizáveis. Execute `/keybindings` para criar ou abrir seu arquivo de configuração em `~/.claude/keybindings.json`.

<h2 id="configuration-file">
  Arquivo de configuração
</h2>

O arquivo de configuração de atalhos de teclado é um objeto com um array `bindings`. Cada bloco especifica um contexto e um mapa de sequências de teclas para ações.

<Note>As alterações no arquivo de atalhos de teclado são detectadas automaticamente e aplicadas sem reiniciar Claude Code.</Note>

| Campo      | Descrição                                                |
| :--------- | :------------------------------------------------------- |
| `$schema`  | URL opcional do JSON Schema para autocompletar do editor |
| `$docs`    | URL opcional de documentação                             |
| `bindings` | Array de blocos de vinculação por contexto               |

Este exemplo vincula `Ctrl+E` para abrir um editor externo no contexto de chat e desvincula `Ctrl+U`:

```json theme={null}
{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/pt/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+u": null
      }
    }
  ]
}
```

<h2 id="contexts">
  Contextos
</h2>

Cada bloco de vinculação especifica um **contexto** onde as vinculações se aplicam:

| Contexto          | Descrição                                                            |
| :---------------- | :------------------------------------------------------------------- |
| `Global`          | Aplica-se em qualquer lugar do aplicativo                            |
| `Chat`            | Área principal de entrada de chat                                    |
| `Autocomplete`    | Menu de autocompletar está aberto                                    |
| `Settings`        | Menu de configurações                                                |
| `Confirmation`    | Diálogos de permissão e confirmação                                  |
| `Tabs`            | Componentes de navegação de abas                                     |
| `Help`            | Menu de ajuda está visível                                           |
| `Transcript`      | Visualizador de transcrição                                          |
| `HistorySearch`   | Modo de busca de histórico (Ctrl+R)                                  |
| `Task`            | Tarefa em segundo plano está em execução                             |
| `ThemePicker`     | Diálogo do seletor de tema                                           |
| `Attachments`     | Navegação de anexo de imagem em diálogos de seleção                  |
| `Footer`          | Navegação do indicador de rodapé (tarefas, equipes, diff, artefatos) |
| `MessageSelector` | Seleção de mensagem do diálogo de retrocesso e resumo                |
| `DiffDialog`      | Navegação do visualizador de diff                                    |
| `DiffPanel`       | O [painel de diff](/docs/pt/interactive-mode#diff-panel) está aberto      |
| `ModelPicker`     | Nível de esforço do seletor de modelo                                |
| `EffortSlider`    | Controle deslizante de esforço aberto por `/effort`                  |
| `Select`          | Componentes genéricos de seleção/lista                               |
| `Plugin`          | Diálogo de plugin (procurar, descobrir, gerenciar)                   |
| `Agents`          | [Visualização de agente](/docs/pt/agent-view) (`claude agents`)           |
| `Scroll`          | Rolagem de conversa e seleção de texto em modo tela cheia            |

Antes da v2.1.205, um contexto `Doctor` e uma ação `doctor:fix` existiam para a tela de diagnósticos `/doctor`.

<h2 id="available-actions">
  Ações disponíveis
</h2>

As ações seguem um formato `namespace:action`, como `chat:submit` para enviar uma mensagem ou `app:toggleTodos` para mostrar a lista de tarefas. Cada contexto tem ações específicas disponíveis.

<h3 id="app-actions">
  Ações do aplicativo
</h3>

Ações disponíveis no contexto `Global`:

| Ação                   | Padrão         | Descrição                                                                                                                          |
| :--------------------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `app:interrupt`        | Ctrl+C         | Cancelar operação atual                                                                                                            |
| `app:exit`             | Ctrl+D         | Sair do Claude Code. Pressione duas vezes em 800ms para confirmar                                                                  |
| `app:redraw`           | (desvinculado) | Forçar redesenho do terminal                                                                                                       |
| `app:toggleTodos`      | Ctrl+T         | Alternar visibilidade da lista de tarefas do Claude. Esta não é a visualização de tarefa em segundo plano [`/tasks`](/docs/pt/commands) |
| `app:toggleTranscript` | Ctrl+O         | Alternar transcrição detalhada                                                                                                     |

<h3 id="history-actions">
  Ações de histórico
</h3>

Ações para navegar no histórico de comandos:

| Ação               | Padrão | Descrição                  |
| :----------------- | :----- | :------------------------- |
| `history:search`   | Ctrl+R | Abrir busca de histórico   |
| `history:previous` | Up     | Item de histórico anterior |
| `history:next`     | Down   | Próximo item de histórico  |

<h3 id="chat-actions">
  Ações de chat
</h3>

Ações disponíveis no contexto `Chat`:

| Ação                  | Padrão                          | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :-------------------- | :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chat:cancel`         | Escape                          | Cancelar entrada atual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `chat:clearInput`     | Ctrl+L                          | Forçar um redesenho de tela cheia, preservando a entrada e a conversa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `chat:clearScreen`    | Cmd+K                           | Mesmo que `chat:clearInput`. Veja [Limpar a conversa](/docs/pt/fullscreen#clear-the-conversation) para saber como Cmd+K se comporta no iTerm2 e Terminal.app                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `chat:killAgents`     | Ctrl+X Ctrl+K                   | Encerrar todos os [subagentes em segundo plano](/docs/pt/sub-agents#run-subagents-in-foreground-or-background) nesta sessão e desativar [respostas automáticas de artefatos](/docs/pt/artifacts#let-claude-reply-to-comments-on-its-own) para o resto dela                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `chat:cycleMode`      | Shift+Tab\*                     | Ciclar modos de permissão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `chat:modelPicker`    | Meta+P                          | Abrir seletor de modelo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `chat:fastMode`       | Meta+O                          | Alternar modo rápido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `chat:thinkingToggle` | Meta+T                          | Alternar pensamento estendido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `chat:submit`         | Enter                           | Enviar mensagem                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `chat:queueSubmit`    | Ctrl+X Enter                    | Enviar a mensagem, marcada para aguardar sua vez: enquanto Claude está trabalhando, Claude Code [a coloca na fila](/docs/pt/interactive-mode#queue-messages-while-claude-works) e nunca interrompe a vez. Ao contrário de `chat:submit`, ela envia o rascunho mesmo enquanto as sugestões de autocompletar estão abertas. Requer v2.1.247 ou posterior                                                                                                                                                                                                                                                                                                                       |
| `chat:sendNow`        | Ctrl+Enter, Ctrl+X Ctrl+S       | Enviar suas [mensagens enfileiradas](/docs/pt/interactive-mode#queue-messages-while-claude-works) e seu rascunho com elas imediatamente. [Quando Claude Code envia o que você enfileirou](/docs/pt/interactive-mode#when-claude-code-sends-what-you-queued) cobre o que acontece com a vez em que Claude está trabalhando. Quando nada está em execução, a tecla envia o rascunho, e no [modo shell](/docs/pt/interactive-mode#shell-mode-with-prefix) ela apenas enfileira o comando. Terminais que não relatam chaves estendidas entregam `Ctrl+Enter` como `Enter` simples, portanto `Ctrl+X Ctrl+S` é a vinculação que funciona em qualquer terminal. Requer v2.1.275 ou posterior |
| `chat:newline`        | Ctrl+J                          | Inserir uma nova linha sem enviar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `chat:undo`           | Ctrl+\_, Ctrl+Shift+-           | Desfazer última ação                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E           | Abrir em editor externo. A [entrada de despacho da visualização do agente](/docs/pt/agent-view#keyboard-shortcuts) também segue os atalhos de teclado de ligação única desta ação                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `chat:stash`          | Ctrl+S                          | Guardar prompt atual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `chat:imagePaste`     | Ctrl+V (Alt+V no Windows e WSL) | Colar imagem da área de transferência. No WSL, ambos os atalhos estão vinculados por padrão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

\*No Windows sem modo VT (Node \<24.2.0/\<22.17.0, Bun \<1.2.23), o padrão é Meta+M.

<h3 id="autocomplete-actions">
  Ações de autocompletar
</h3>

Ações disponíveis no contexto `Autocomplete`:

| Ação                    | Padrão | Descrição         |
| :---------------------- | :----- | :---------------- |
| `autocomplete:accept`   | Tab    | Aceitar sugestão  |
| `autocomplete:dismiss`  | Escape | Descartar menu    |
| `autocomplete:previous` | Up     | Sugestão anterior |
| `autocomplete:next`     | Down   | Próxima sugestão  |

<h3 id="confirmation-actions">
  Ações de confirmação
</h3>

Ações disponíveis no contexto `Confirmation`:

| Ação                    | Padrão         | Descrição                                                                                                                                                                                                                                                                                           |
| :---------------------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `confirm:yes`           | Enter          | Confirmar ação                                                                                                                                                                                                                                                                                      |
| `confirm:no`            | Escape         | Recusar ação                                                                                                                                                                                                                                                                                        |
| `confirm:previous`      | Up             | Opção anterior                                                                                                                                                                                                                                                                                      |
| `confirm:next`          | Down           | Próxima opção                                                                                                                                                                                                                                                                                       |
| `confirm:nextField`     | Tab            | Próximo campo                                                                                                                                                                                                                                                                                       |
| `confirm:previousField` | (desvinculado) | Campo anterior                                                                                                                                                                                                                                                                                      |
| `confirm:toggle`        | Space          | Alternar seleção                                                                                                                                                                                                                                                                                    |
| `confirm:cycleMode`     | Shift+Tab\*    | Ciclar modos de permissão. Em um prompt de permissão de arquivo, fecha um [campo de comentário](/docs/pt/permissions#add-a-comment-when-you-answer-a-permission-prompt) aberto; sem nenhum campo aberto, seleciona a opção que permite a ação para o resto da sessão, quando o prompt oferece essa opção |

\*No Windows sem modo VT (Node \<24.2.0/\<22.17.0, Bun \<1.2.23), o padrão é Meta+M.

Antes da v2.1.257, uma ação `confirm:toggleExplanation`, vinculada a `Ctrl+E` por padrão, mostrava uma explicação gerada por modelo do comando em prompts de permissão Bash e PowerShell.

Os diálogos usam `confirm:yes` e `confirm:no` para aceitar e cancelar mesmo quando não fazem uma pergunta sim-ou-não. Se você vincular uma letra simples como `y` ou `n` neste contexto, a letra também atua em diálogos que nunca a mostram como uma chave. Um diálogo que mostra `y` e `n` como suas chaves lê essas letras em si e não precisa de vinculação.

Este exemplo vincula `y` a `confirm:yes` e `n` a `confirm:no`:

```json theme={null}
{
  "bindings": [
    {
      "context": "Confirmation",
      "bindings": {
        "y": "confirm:yes",
        "n": "confirm:no"
      }
    }
  ]
}
```

Com essas vinculações, `y` e `n` ainda digitam como letras enquanto um [campo de texto](#text-fields) tem foco.

Antes da v2.1.280, `y` também estava vinculado a `confirm:yes` e `n` a `confirm:no` por padrão. Se você criou seu `keybindings.json` com `/keybindings` antes da v2.1.280, o arquivo lista ambas as vinculações e elas permanecem em vigor até que você delete essas duas linhas.

<h3 id="permission-actions">
  Ações de permissão
</h3>

Ações disponíveis no contexto `Confirmation` para diálogos de permissão:

| Ação                     | Padrão         | Descrição                                                                                                                        |
| :----------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `permission:toggleDebug` | (desvinculado) | Alternar informações de depuração de permissão. O padrão anterior de Ctrl+D foi removido na v2.1.146 porque sombreava `app:exit` |

<h3 id="transcript-actions">
  Ações de transcrição
</h3>

Ações disponíveis no contexto `Transcript`:

| Ação                       | Padrão            | Descrição                           |
| :------------------------- | :---------------- | :---------------------------------- |
| `transcript:toggleShowAll` | Ctrl+E            | Alternar mostrar todo o conteúdo    |
| `transcript:exit`          | q, Ctrl+C, Escape | Sair da visualização de transcrição |

`transcript:toggleShowAll` se aplica apenas no renderizador clássico; na [renderização em tela cheia](/docs/pt/fullscreen), o visualizador de transcrição não oferece um toggle de mostrar tudo.

<h3 id="history-search-actions">
  Ações de busca de histórico
</h3>

Ações disponíveis no contexto `HistorySearch`:

| Ação                       | Padrão      | Descrição                                         |
| :------------------------- | :---------- | :------------------------------------------------ |
| `historySearch:next`       | Ctrl+R      | Próxima correspondência                           |
| `historySearch:accept`     | Escape, Tab | Aceitar seleção                                   |
| `historySearch:cancel`     | Ctrl+C      | Cancelar busca                                    |
| `historySearch:execute`    | Enter       | Executar comando selecionado                      |
| `historySearch:cycleScope` | Ctrl+S      | Ciclar escopo: sessão, projeto, em qualquer lugar |

Os padrões `historySearch:next`, `historySearch:accept`, `historySearch:cancel` e `historySearch:execute` se aplicam à busca de histórico inline no renderizador clássico, que sempre busca prompts de todos os projetos. `historySearch:cycleScope` entra em vigor apenas na [renderização em tela cheia](/docs/pt/fullscreen), onde `Ctrl+R` abre um diálogo de busca em vez disso e `Ctrl+S` cicla seu escopo. As outras teclas do diálogo são fixas e não podem ser rebindadas: `Enter` ou `Tab` coloca a correspondência destacada na entrada do prompt e `Esc` cancela.

<h3 id="task-actions">
  Ações de tarefa
</h3>

Ações disponíveis no contexto `Task`:

| Ação              | Padrão                | Descrição                                                                                         |
| :---------------- | :-------------------- | :------------------------------------------------------------------------------------------------ |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | Colocar tarefa atual em segundo plano. O acorde Ctrl+X Ctrl+B evita o conflito de prefixo do tmux |

<h3 id="theme-actions">
  Ações de tema
</h3>

Ações disponíveis no contexto `ThemePicker`:

| Ação                             | Padrão | Descrição                    |
| :------------------------------- | :----- | :--------------------------- |
| `theme:toggleSyntaxHighlighting` | Ctrl+T | Alternar destaque de sintaxe |

<h3 id="help-actions">
  Ações de ajuda
</h3>

Ações disponíveis no contexto `Help`:

| Ação           | Padrão | Descrição            |
| :------------- | :----- | :------------------- |
| `help:dismiss` | Escape | Fechar menu de ajuda |

<h3 id="tabs-actions">
  Ações de abas
</h3>

Ações disponíveis no contexto `Tabs`:

| Ação            | Padrão          | Descrição    |
| :-------------- | :-------------- | :----------- |
| `tabs:next`     | Tab, Right      | Próxima aba  |
| `tabs:previous` | Shift+Tab, Left | Aba anterior |

<h3 id="attachments-actions">
  Ações de anexos
</h3>

Ações disponíveis no contexto `Attachments`:

| Ação                   | Padrão            | Descrição                   |
| :--------------------- | :---------------- | :-------------------------- |
| `attachments:next`     | Right             | Próximo anexo               |
| `attachments:previous` | Left              | Anexo anterior              |
| `attachments:remove`   | Backspace, Delete | Remover anexo selecionado   |
| `attachments:exit`     | Down, Escape      | Sair da navegação de anexos |

<h3 id="footer-actions">
  Ações de rodapé
</h3>

Ações disponíveis no contexto `Footer`:

| Ação                    | Padrão            | Descrição                                                                                                                                                                                            |
| :---------------------- | :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `footer:next`           | Right             | Próximo item do rodapé                                                                                                                                                                               |
| `footer:previous`       | Left              | Item anterior do rodapé                                                                                                                                                                              |
| `footer:up`             | Up                | Navegar para cima no rodapé (desseleciona no topo)                                                                                                                                                   |
| `footer:down`           | Down              | Navegar para baixo no rodapé                                                                                                                                                                         |
| `footer:openSelected`   | Enter             | Abrir item do rodapé selecionado                                                                                                                                                                     |
| `footer:clearSelection` | Escape            | Limpar seleção do rodapé                                                                                                                                                                             |
| `footer:dismiss`        | Backspace, Delete | Descartar o link de [artefato](/docs/pt/artifacts) selecionado do rodapé; o artefato publicado em si não é afetado. Em outras linhas do rodapé, essas teclas não têm efeito. Requer v2.1.217 ou posterior |

Enquanto um item do rodapé está selecionado, como uma linha no painel do agente abaixo do prompt, `Enter` o abre mesmo quando você rebinda `Enter` no contexto `Chat` para `chat:queueSubmit` ou `chat:newline`.

As vinculações `Chat` em teclas que o contexto `Footer` não vincula, como `Shift+Tab` para `chat:cycleMode`, continuam funcionando enquanto um item está selecionado.

<h3 id="message-selector-actions">
  Ações do seletor de mensagem
</h3>

Ações disponíveis no contexto `MessageSelector`:

| Ação                     | Padrão                                    | Descrição                 |
| :----------------------- | :---------------------------------------- | :------------------------ |
| `messageSelector:up`     | Up, K, Ctrl+P                             | Mover para cima na lista  |
| `messageSelector:down`   | Down, J, Ctrl+N                           | Mover para baixo na lista |
| `messageSelector:top`    | Ctrl+Up, Shift+Up, Meta+Up, Shift+K       | Pular para o topo         |
| `messageSelector:bottom` | Ctrl+Down, Shift+Down, Meta+Down, Shift+J | Pular para o final        |
| `messageSelector:select` | Enter                                     | Selecionar mensagem       |

<h3 id="diff-actions">
  Ações de diff
</h3>

Ações disponíveis no contexto `DiffDialog`:

| Ação                  | Padrão         | Descrição                                                                                                                                                          |
| :-------------------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `diff:dismiss`        | Escape         | Fechar visualizador de diff; da visualização de detalhes, retorna à lista de arquivos em vez disso                                                                 |
| `diff:previousSource` | Left           | Fonte de diff anterior                                                                                                                                             |
| `diff:nextSource`     | Right          | Próxima fonte de diff                                                                                                                                              |
| `diff:previousFile`   | Up, K          | Arquivo anterior na lista de arquivos; rolar para cima uma linha na visualização de detalhes                                                                       |
| `diff:nextFile`       | Down, J        | Próximo arquivo na lista de arquivos; rolar para baixo uma linha na visualização de detalhes                                                                       |
| `diff:viewDetails`    | Enter          | Visualizar detalhes do diff                                                                                                                                        |
| `diff:back`           | (desvinculado) | Voltar no visualizador de diff. Escape executa a ação de voltar via `diff:dismiss`. O padrão anterior de Left na visualização de detalhes foi removido na v2.1.203 |

A visualização de detalhes do diff também vincula atalhos de teclado no estilo pager às [ações de rolagem](#scroll-actions) padrão. Essas vinculações fazem parte do contexto `DiffDialog` e se aplicam apenas na visualização de detalhes; os padrões do contexto `Scroll` listados em [Ações de rolagem](#scroll-actions) permanecem inalterados.

| Ação                  | Padrão         | Descrição                                            |
| :-------------------- | :------------- | :--------------------------------------------------- |
| `scroll:pageUp`       | PageUp         | Rolar para cima metade da janela de visualização     |
| `scroll:pageDown`     | PageDown       | Rolar para baixo metade da janela de visualização    |
| `scroll:fullPageUp`   | Shift+Space, B | Rolar para cima uma janela de visualização completa  |
| `scroll:fullPageDown` | Space          | Rolar para baixo uma janela de visualização completa |
| `scroll:top`          | G, Home        | Pular para o topo                                    |
| `scroll:bottom`       | Shift+G, End   | Pular para o final                                   |

<h3 id="diff-panel-actions">
  Ações do painel de diff
</h3>

Ações para o [painel de diff](/docs/pt/interactive-mode#diff-panel) que `/diff` abre na renderização em tela cheia. `app:cycleDiffBase` está no contexto `DiffPanel`, que está ativo enquanto o painel está aberto; os outros estão em `Global`. O painel requer Claude Code v2.1.260 ou posterior.

| Ação                        | Padrão               | Descrição                                                                         |
| :-------------------------- | :------------------- | :-------------------------------------------------------------------------------- |
| `app:toggleReplTab`         | (desvinculado)       | Abrir ou fechar o painel de diff, o mesmo que executar `/diff`                    |
| `app:cycleDiffBase`         | Ctrl+X B             | Ciclar a base de comparação do painel: esta sessão, não confirmado, depois branch |
| `app:diffFileListUp`        | Ctrl+Up, Meta+Up     | Rolar a lista de arquivos do painel para cima quando ela transborda               |
| `app:diffFileListDown`      | Ctrl+Down, Meta+Down | Rolar a lista de arquivos do painel para baixo quando ela transborda              |
| `app:toggleDiffNoiseFilter` | (desvinculado)       | Mostrar ou ocultar arquivos de teste e gerados no painel                          |
| `app:toggleDiffPreSession`  | (desvinculado)       | Expandir ou recolher as alterações de antes desta sessão                          |

<h3 id="model-picker-actions">
  Ações do seletor de modelo
</h3>

Ações disponíveis no contexto `ModelPicker`:

| Ação                          | Padrão | Descrição                                     |
| :---------------------------- | :----- | :-------------------------------------------- |
| `modelPicker:decreaseEffort`  | Left   | Diminuir nível de esforço                     |
| `modelPicker:increaseEffort`  | Right  | Aumentar nível de esforço                     |
| `modelPicker:thisSessionOnly` | s      | Aplicar modelo destacado apenas a esta sessão |

<h3 id="effort-slider-actions">
  Ações do controle deslizante de esforço
</h3>

Ações disponíveis no contexto `EffortSlider`, o controle deslizante que abre quando você executa `/effort` sem argumentos. As teclas Left, Right, Enter e Escape do controle deslizante não podem ser rebindadas.

| Ação                           | Padrão | Descrição                                                                                                                    |
| :----------------------------- | :----- | :--------------------------------------------------------------------------------------------------------------------------- |
| `effortSlider:thisSessionOnly` | s      | Aplicar o [nível de esforço](/docs/pt/model-config#adjust-effort-level) focado apenas a esta sessão. Requer v2.1.257 ou posterior |

<h3 id="select-actions">
  Ações de seleção
</h3>

Ações disponíveis no contexto `Select`:

| Ação              | Padrão          | Descrição                             |
| :---------------- | :-------------- | :------------------------------------ |
| `select:next`     | Down, J, Ctrl+N | Próxima opção                         |
| `select:previous` | Up, K, Ctrl+P   | Opção anterior                        |
| `select:pageUp`   | PageUp          | Mover para cima uma página de opções  |
| `select:pageDown` | PageDown        | Mover para baixo uma página de opções |
| `select:first`    | Home            | Primeira opção                        |
| `select:last`     | End             | Última opção                          |
| `select:accept`   | Enter           | Aceitar seleção                       |
| `select:cancel`   | Escape          | Cancelar seleção                      |

Claude Code aplica suas vinculações `select:pageUp`, `select:pageDown`, `select:first` e `select:last` no menu `/skills`. Na maioria das outras listas, como o seletor `/model`, suas vinculações `select:first` e `select:last` se aplicam. PageUp e PageDown pagina através das opções nessas listas independentemente de suas vinculações.

Antes da v2.1.280, essas outras listas ignoravam Home, End e suas vinculações `select:first` e `select:last`.

<h3 id="plugin-actions">
  Ações de plugin
</h3>

Ações disponíveis no contexto `Plugin`:

| Ação              | Padrão | Descrição                                                                                           |
| :---------------- | :----- | :-------------------------------------------------------------------------------------------------- |
| `plugin:toggle`   | Space  | Alternar seleção de plugin                                                                          |
| `plugin:install`  | I      | Instalar plugins selecionados                                                                       |
| `plugin:favorite` | F      | Marcar o plugin selecionado como favorito para que seja classificado perto do topo da aba Instalado |

<h3 id="settings-actions">
  Ações de configurações
</h3>

Ações disponíveis no contexto `Settings`. As ações `select:accept` e `confirm:no` são reutilizadas dos contextos [Select](#select-actions) e [Confirmation](#confirmation-actions) com comportamento específico de Configurações: as alterações se aplicam a cada configuração assim que você a altera, portanto Escape fecha o painel com suas alterações salvas em vez de recusar.

| Ação              | Padrão       | Descrição                                               |
| :---------------- | :----------- | :------------------------------------------------------ |
| `settings:search` | /            | Entrar no modo de busca                                 |
| `settings:retry`  | R            | Tentar novamente carregar dados de uso em caso de erro  |
| `select:accept`   | Enter, Space | Alterar a configuração selecionada ou abrir seu submenu |
| `confirm:no`      | Escape       | Fechar o painel. As alterações já foram salvas          |

<h3 id="agents-actions">
  Ações de agentes
</h3>

Ações disponíveis no contexto `Agents`, que se aplica na [visualização do agente](/docs/pt/agent-view), aberta com `claude agents`. Requer v2.1.257 ou posterior.

| Ação                | Padrão | Descrição                                                                                   |
| :------------------ | :----- | :------------------------------------------------------------------------------------------ |
| `agents:switchView` | Ctrl+S | Alternar [agrupamento de sessão](/docs/pt/agent-view#organize-the-list) entre estado e diretório |
| `agents:togglePin`  | Ctrl+T | [Fixar ou desafixar](/docs/pt/agent-view#organize-the-list) a sessão selecionada                 |

Enquanto a visualização do agente está aberta, Claude Code usa a vinculação `Agents` para qualquer tecla que o contexto `Agents` vincula, e ignora uma vinculação `Chat` ou `Global` na mesma tecla. Por exemplo, pressionar Ctrl+S na visualização do agente alterna o agrupamento de sessão em vez de disparar o padrão `chat:stash`.

O atalho de editor externo da entrada de despacho não é uma ação `Agents`. A visualização do agente segue a vinculação `chat:externalEditor` do contexto `Chat`, Ctrl+G por padrão.

As vinculações disparam em pressionamentos de tecla únicos na visualização do agente, portanto o acorde Ctrl+X Ctrl+E vinculado a `chat:externalEditor` não abre o editor lá.

<h3 id="voice-actions">
  Ações de voz
</h3>

Ações disponíveis no contexto `Chat` quando a [ditação por voz](/docs/pt/voice-dictation) está ativada:

| Ação               | Padrão | Descrição                                                                  |
| :----------------- | :----- | :------------------------------------------------------------------------- |
| `voice:pushToTalk` | Space  | Ditar um prompt. Mantenha pressionado ou toque dependendo do modo `/voice` |

<h3 id="scroll-actions">
  Ações de rolagem
</h3>

Ações disponíveis no contexto `Scroll` quando a [renderização em tela cheia](/docs/pt/fullscreen) está ativada:

| Ação                        | Padrão               | Descrição                                                                                                                                   |
| :-------------------------- | :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `scroll:lineUp`             | `wheelup`            | Rolar para cima uma linha. A rolagem da roda do mouse dispara esta ação                                                                     |
| `scroll:lineDown`           | `wheeldown`          | Rolar para baixo uma linha. A rolagem da roda do mouse dispara esta ação                                                                    |
| `scroll:pageUp`             | PageUp               | Rolar para cima metade da altura da janela de visualização                                                                                  |
| `scroll:pageDown`           | PageDown             | Rolar para baixo metade da altura da janela de visualização                                                                                 |
| `scroll:top`                | Ctrl+Home            | Pular para o início da conversa                                                                                                             |
| `scroll:bottom`             | Ctrl+End             | Pular para a mensagem mais recente e reativar o auto-follow                                                                                 |
| `scroll:halfPageUp`         | (desvinculado)       | Rolar para cima metade da altura da janela de visualização. Mesmo comportamento que `scroll:pageUp`, fornecido para rebinds no estilo vi    |
| `scroll:halfPageDown`       | (desvinculado)       | Rolar para baixo metade da altura da janela de visualização. Mesmo comportamento que `scroll:pageDown`, fornecido para rebinds no estilo vi |
| `scroll:fullPageUp`         | (desvinculado)       | Rolar para cima a altura completa da janela de visualização                                                                                 |
| `scroll:fullPageDown`       | (desvinculado)       | Rolar para baixo a altura completa da janela de visualização                                                                                |
| `selection:copy`            | Ctrl+Shift+C / Cmd+C | Copiar o texto selecionado para a área de transferência                                                                                     |
| `selection:clear`           | (desvinculado)       | Limpar a seleção de texto ativa. Requer v2.1.234 ou posterior                                                                               |
| `selection:extendLeft`      | Shift+Left           | Estender a seleção ativa uma coluna para a esquerda                                                                                         |
| `selection:extendRight`     | Shift+Right          | Estender a seleção ativa uma coluna para a direita                                                                                          |
| `selection:extendUp`        | Shift+Up             | Estender a seleção ativa uma linha para cima. Rola a janela de visualização quando a seleção atinge a borda superior                        |
| `selection:extendDown`      | Shift+Down           | Estender a seleção ativa uma linha para baixo. Rola a janela de visualização quando a seleção atinge a borda inferior                       |
| `selection:extendLineStart` | Shift+Home           | Estender a seleção ativa para o início da linha                                                                                             |
| `selection:extendLineEnd`   | Shift+End            | Estender a seleção ativa para o final da linha                                                                                              |

<h2 id="keystroke-syntax">
  Sintaxe de sequência de teclas
</h2>

<h3 id="modifiers">
  Modificadores
</h3>

Use teclas modificadoras com o separador `+`:

* `ctrl` ou `control` - Tecla Control
* `shift` - Tecla Shift
* `alt`, `opt`, `option`, ou `meta` - Tecla Alt no Windows e Linux, tecla Option no macOS
* `cmd`, `command`, `super`, ou `win` - Tecla Command no macOS, tecla Windows no Windows, tecla Super no Linux

O grupo `cmd` é detectado apenas em terminais que relatam o modificador Super, como aqueles que suportam o protocolo de teclado Kitty ou o modo `modifyOtherKeys` do xterm. A maioria dos terminais não o envia, portanto use `ctrl` ou `meta` para atalhos de teclado que você deseja que funcionem em qualquer lugar.

Por exemplo:

```text theme={null}
ctrl+k          Ctrl + K
shift+tab       Shift + Tab
meta+p          Option + P no macOS, Alt + P em outros lugares
ctrl+shift+c    Múltiplos modificadores
```

<h3 id="uppercase-letters">
  Letras maiúsculas
</h3>

Claude Code analisa nomes de teclas sem distinção entre maiúsculas e minúsculas, portanto `K` é o mesmo atalho de teclado que `k` e `ctrl+K` é o mesmo que `ctrl+k`. Para vincular Shift e uma letra, escreva `shift+k`.

<h3 id="non-us-keyboard-layouts">
  Layouts de teclado não-US
</h3>

Escreva os nomes das teclas de atalhos Ctrl como caracteres latinos mesmo quando seu layout de teclado ativo digita outros caracteres.

A forma como Claude Code corresponde à tecla que você pressiona a um atalho de teclado depende do tipo de layout:

* Sob um layout não-latino, como Cirílico, Claude Code corresponde aos atalhos de teclado Ctrl pela posição da tecla no layout US quando o terminal usa o protocolo de teclado Kitty e relata essa posição. Em tal terminal, com um layout russo ativo, pressionar Ctrl e a tecla W física dispara `ctrl+w`. Em um terminal que não relata a posição, Claude Code corresponde ao que o terminal envia para o pressionamento de tecla: um código de controle ASCII dispara o atalho de teclado latino, e um pressionamento de tecla que chega como o caractere cirílico não corresponde a nenhum atalho de teclado
* Sob layouts que reorganizam letras latinas, como AZERTY, Claude Code corresponde à letra que a tecla digita, portanto pressionar Ctrl e a tecla rotulada A dispara `ctrl+a`

Antes da v2.1.247, pressionar um atalho de teclado Ctrl sob um layout não-latino não disparava seu atalho de teclado em terminais que usam o protocolo de teclado Kitty, como Ghostty, Kitty, WezTerm e iTerm2.

<h3 id="chords">
  Acordes
</h3>

Acordes são sequências de sequências de teclas separadas por espaços:

```text theme={null}
ctrl+k ctrl+s   Pressione Ctrl+K, solte, depois Ctrl+S
```

Pressione cada sequência de teclas dentro de 3 segundos da anterior. Se você esperar mais tempo, Claude Code cancela o acorde e mostra um breve aviso dizendo isso.

<h3 id="special-keys">
  Teclas especiais
</h3>

* `escape` ou `esc` - Tecla Escape
* `enter` ou `return` - Tecla Enter
* `tab` - Tecla Tab
* `space` - Barra de espaço
* `up`, `down`, `left`, `right` - Teclas de seta
* `pageup`, `pagedown` - Teclas Page Up e Page Down
* `home`, `end` - Teclas Home e End
* `backspace`, `delete` - Teclas de exclusão
* `wheelup`, `wheeldown` - Eventos de rolagem da roda do mouse

<h2 id="unbind-default-shortcuts">
  Desassociar atalhos de teclado padrão
</h2>

Defina uma ação como `null` para desassociar um atalho de teclado padrão:

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+s": null
      }
    }
  ]
}
```

Isso também funciona para atalhos de teclado de acordes. Desassociar cada acorde que compartilha um prefixo libera esse prefixo para uso como um atalho de teclado de uma única tecla. Um acorde em qualquer contexto ativo mantém seu prefixo reservado, portanto você deve desassociar cada acorde no contexto que o define.

Claude Code vincula esses acordes padrão no prefixo `ctrl+x`: `ctrl+x ctrl+k`, `ctrl+x ctrl+e`, `ctrl+x enter`, `ctrl+x ctrl+a`, `ctrl+x ctrl+s` e `ctrl+x tab` em `Chat`, `ctrl+x ctrl+b` em `Task` e `ctrl+x b` em `DiffPanel`. O acorde `ctrl+x enter` requer v2.1.247 ou posterior, `ctrl+x b`, `ctrl+x ctrl+a` e `ctrl+x tab` requerem v2.1.260 ou posterior, e `ctrl+x ctrl+s` requer v2.1.275 ou posterior.

Para recuperar `ctrl+x` em si como um atalho de teclado de uma única tecla, desassocie todos eles:

```json theme={null}
{
  "bindings": [
    {
      "context": "Task",
      "bindings": {
        "ctrl+x ctrl+b": null
      }
    },
    {
      "context": "DiffPanel",
      "bindings": {
        "ctrl+x b": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x enter": null,
        "ctrl+x ctrl+a": null,
        "ctrl+x ctrl+s": null,
        "ctrl+x tab": null,
        "ctrl+x": "chat:newline"
      }
    }
  ]
}
```

Se você desassociar alguns, mas não todos os acordes em um prefixo, pressionar o prefixo ainda entra no modo de espera de acorde para as associações restantes.

<h2 id="reserved-shortcuts">
  Atalhos reservados
</h2>

Estes atalhos não podem ser revinculados:

| Atalho    | Motivo                                                                                                                                                                                                                                       |
| :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ctrl+C    | Interrupção/cancelamento codificado                                                                                                                                                                                                          |
| Ctrl+D    | Saída codificada                                                                                                                                                                                                                             |
| Ctrl+M    | Claude Code sempre o recebe como Enter                                                                                                                                                                                                       |
| Ctrl+\[   | Claude Code sempre o recebe como Escape. Em terminais que usam o protocolo de teclado Kitty, isso requer v2.1.242 ou posterior                                                                                                               |
| Ctrl+I    | Claude Code sempre o recebe como Tab                                                                                                                                                                                                         |
| Ctrl+H    | Envia o byte ASCII de backspace. [Como Claude Code o lê no Windows](/docs/pt/terminal-config#fix-backspace-deleting-a-whole-word-on-windows) depende do seu terminal e da variável de ambiente [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE`](/docs/pt/env-vars) |
| Caps Lock | Não entregue a aplicações de terminal                                                                                                                                                                                                        |

<h2 id="terminal-conflicts">
  Conflitos de terminal
</h2>

Alguns atalhos podem entrar em conflito com multiplexadores de terminal:

| Atalho | Conflito                                        |
| :----- | :---------------------------------------------- |
| Ctrl+B | Prefixo tmux (pressione duas vezes para enviar) |
| Ctrl+A | Prefixo GNU screen                              |
| Ctrl+Z | Suspensão de processo Unix (SIGTSTP)            |

<h2 id="text-fields">
  Campos de texto
</h2>

Se você vincular uma letra simples, dígito ou Espaço, ainda poderá digitar esse caractere em um campo de texto dentro de um diálogo ou painel. Um desses campos é a resposta `Other` para uma pergunta que Claude faz. Enquanto o campo tem foco, uma tecla imprimível que você pressiona sem Ctrl, Alt ou Cmd vai para o campo, e Claude Code não a compara com seus vínculos.

Essas teclas ainda executam seus vínculos enquanto o campo tem foco:

* Teclas que não digitam um caractere, como Enter, Escape, Tab e as teclas de seta
* Qualquer tecla pressionada com Ctrl, Alt ou Cmd
* O segundo pressionamento de uma [chord](#chords) já em progresso

No prompt principal, Claude Code compara cada tecla contra os contextos ativos, como `Chat`, e digita a tecla apenas quando nenhum vínculo a utiliza.

<h2 id="vim-mode-interaction">
  Interação com modo vim
</h2>

Quando o modo vim está ativado via `/config` → Editor mode, atalhos de teclado e modo vim operam independentemente:

* **Modo vim** manipula entrada no nível de entrada de texto (movimento do cursor, modos, motions)
* **Atalhos de teclado** manipulam ações no nível de componente (alternar tarefas, enviar, etc.)
* A tecla Escape no modo vim muda INSERT para NORMAL; ela não dispara `chat:cancel`
* A maioria dos atalhos Ctrl+key passam pelo modo vim para o sistema de atalhos de teclado
* As chaves vim não são remapeáveis através do arquivo de atalhos de teclado. Para mapear uma sequência de dois-key no modo INSERT como `jj` para Escape, use a configuração [`vimInsertModeRemaps`](/docs/pt/interactive-mode#remap-insert-mode-key-sequences)
* No modo NORMAL do vim, `?` mostra o menu de ajuda (comportamento vim)
* No modo NORMAL do vim, `/` abre a busca de histórico, o mesmo que Ctrl+R no modo padrão

<h2 id="validation">
  Validação
</h2>

Claude Code valida seus atalhos de teclado e mostra avisos para:

* Erros de análise (JSON inválido ou estrutura)
* Nomes de contexto inválidos
* Valores de ação inválidos, como uma ação que não é uma string ou `null`
* Nomes de ação desconhecidos, como um erro de digitação de uma ação registrada. Claude Code pula a vinculação e mantém qualquer vinculação padrão para essa tecla em vigor. Antes da v2.1.246, uma vinculação com um nome de ação desconhecido desativava silenciosamente essa tecla
* Conflitos de atalho reservado
* Vinculações duplicadas no mesmo contexto

Claude Code relata avisos quando o arquivo é carregado e escreve cada um no log de depuração. Inicie Claude Code com [`--debug`](/docs/pt/cli-reference#cli-flags) para ver os detalhes.
