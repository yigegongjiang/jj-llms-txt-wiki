> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Renderização em tela cheia

> Ative um modo de renderização mais suave e sem cintilação com suporte a mouse e uso de memória estável em conversas longas.

<Note>
  A renderização em tela cheia é uma [visualização de pesquisa](#research-preview). Se você [inicia em tela cheia ou no renderizador clássico](#fullscreen-by-default) depende da sua configuração. Execute `/tui fullscreen` ou `/tui default` para alternar em sua conversa atual. O comportamento pode mudar com base no feedback.
</Note>

A renderização em tela cheia é um caminho de renderização alternativo para o Claude Code CLI que elimina cintilação, mantém o uso de memória constante em conversas longas e adiciona suporte a mouse. Ela desenha a interface no buffer de tela alternativa do terminal, como `vim` ou `htop`, e renderiza apenas as mensagens que estão visíveis no momento. Isso reduz a quantidade de dados enviados para seu terminal em cada atualização.

A diferença é mais notável em emuladores de terminal onde a taxa de transferência de renderização é o gargalo, como o terminal integrado do VS Code, tmux e iTerm2. Se a posição de rolagem do seu terminal pular para o topo enquanto Claude está trabalhando, ou a tela piscar conforme a saída da ferramenta é transmitida, este modo resolve esses problemas.

<Note>
  O termo tela cheia descreve como Claude Code assume a superfície de desenho do terminal, da mesma forma que `vim` faz. Não tem nada a ver com maximizar a janela do seu terminal e funciona em qualquer tamanho de janela.
</Note>

<h2 id="enable-fullscreen-rendering">
  Ativar renderização em tela cheia
</h2>

Execute `/tui fullscreen` dentro de qualquer conversa do Claude Code. O CLI salva a configuração [`tui`](/docs/pt/settings-reference#tui) e reinicia em tela cheia com sua conversa intacta, para que você possa alternar no meio da sessão sem perder contexto. Execute `/tui default` para voltar ao renderizador clássico, ou `/tui` sem argumentos para imprimir qual renderizador está ativo.

No [modo leitor de tela](/docs/pt/accessibility), Claude Code sempre usa o renderizador clássico, exceto em [sessões em segundo plano](/docs/pt/agent-view) anexadas, que ainda renderizam em tela cheia. Se você executar `/tui fullscreen` em qualquer outra sessão, Claude Code imprime uma explicação em vez de alternar e não altera a configuração `tui` salva.

Claude Code carrega estes itens para a sessão reiniciada:

* A conversa como aparece na tela. Após um [`/rewind`](/docs/pt/checkpointing#rewind-and-summarize), isso significa:
  * Se você reverteu anteriormente na sessão, Claude Code reinicia a partir do ponto revertido, não do transcript mais longo salvo no disco. Por exemplo, se você reverteu além de suas últimas três mensagens, a sessão reiniciada abre sem elas
  * Se você reverteu para antes de sua primeira mensagem, Claude Code reinicia com uma conversa vazia
* Seu [modo de permissão](/docs/pt/permission-modes) e [nível de esforço](/docs/pt/model-config#adjust-effort-level)
* O modelo que você selecionou pela última vez com [`/model`](/docs/pt/model-config#setting-your-model)
* Regras que você passou com [`--allowed-tools` ou `--disallowed-tools`](/docs/pt/cli-reference#cli-flags), e seus sinalizadores `--agent`, `--agents`, `--append-system-prompt` e `--system-prompt-snapshot`

Claude Code recusa reiniciar se a sessão tiver uma restrição que não possa passar para o processo reiniciado. As restrições que não pode passar incluem:

* Sinalizadores de inicialização, como uma substituição de [`--system-prompt`](/docs/pt/cli-reference#cli-flags), uma lista de permissões [`--tools`](/docs/pt/cli-reference#cli-flags) ou [`--setting-sources`](/docs/pt/cli-reference#cli-flags)
* Regras de negação ou pergunta que uma [atualização de permissão de hook ou SDK](/docs/pt/hooks#permission-update-entries) adicionou apenas para esta sessão

Nesse caso, Claude Code imprime [`Cannot switch renderers in this session`](/docs/pt/errors#cannot-switch-renderers-in-this-session) com os motivos. Ele não alterna nem salva nada.

Você também pode definir a variável de ambiente `CLAUDE_CODE_NO_FLICKER` antes de iniciar Claude Code:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

Para saber como a configuração [`tui`](/docs/pt/settings-reference#tui) e a variável se combinam quando ambas estão definidas, consulte a entrada da configuração. Após uma [inicialização de tela cheia falhada](#fullscreen-renderer-didnt-finish-starting), Claude Code ainda honra a variável, mas não a configuração. O comando `/tui` limpa `CLAUDE_CODE_NO_FLICKER` do processo reiniciado para que a configuração que ele escreve tenha efeito.

<h3 id="fullscreen-by-default">
  Tela cheia por padrão
</h3>

[Sessões em segundo plano](/docs/pt/agent-view) anexadas renderizam em tela cheia, e outras sessões no [modo leitor de tela](/docs/pt/accessibility) usam o renderizador clássico. Caso contrário, Claude Code inicia você no renderizador da primeira linha desta tabela que corresponde à sua configuração:

| Sua situação                                                                                                                                                                                  | Renderizador em que você inicia          |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- |
| Você definiu [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`](/docs/pt/env-vars) ou `CLAUDE_CODE_NO_FLICKER=0`                                                                                           | Clássico                                 |
| Você definiu `CLAUDE_CODE_NO_FLICKER=1`                                                                                                                                                       | Tela cheia                               |
| Claude Code [desativou tela cheia após uma inicialização de tela cheia falhada](#fullscreen-renderer-didnt-finish-starting) nesta máquina                                                     | Clássico                                 |
| Você está no modo de integração [`tmux -CC`](#use-with-tmux) do iTerm2, ou está conectado via SSH ao Claude Code em execução no Windows                                                       | Clássico                                 |
| Você salvou uma configuração [`tui`](/docs/pt/settings-reference#tui)                                                                                                                              | O renderizador que a configuração nomeia |
| Sua sessão não [busca sinalizadores de recurso da Anthropic](/docs/pt/env-vars#features-that-need-feature-flag-fetching), e Claude Code parou de oferecer o diálogo de inicialização nesta máquina | Clássico                                 |
| Sua sessão não busca sinalizadores de recurso da Anthropic, e o primeiro lançamento do Claude Code desta máquina executou v2.1.239 ou posterior                                               | Tela cheia                               |
| Sua sessão busca sinalizadores de recurso da Anthropic, e você usou Claude Code pela primeira vez em ou após 6 de maio de 2026                                                                | Tela cheia                               |
| Qualquer outra coisa                                                                                                                                                                          | Clássico                                 |

Sessões que não buscam sinalizadores de recurso incluem aquelas através do [Amazon Bedrock](/docs/pt/amazon-bedrock), [Agent Platform do Google Cloud](/docs/pt/google-vertex-ai) ou [Microsoft Foundry](/docs/pt/microsoft-foundry), e aquelas com telemetria desativada.

Se você iniciar no renderizador clássico e não tiver salvo uma configuração `tui`, Claude Code pode abrir um diálogo na inicialização oferecendo a alternância:

* Se você aceitar, Claude Code reinicia da mesma forma que `/tui fullscreen` faz, carregando o mesmo estado da sessão, e salva a configuração assim que a sessão reiniciada tiver [iniciado com sucesso](#fullscreen-renderer-didnt-finish-starting).
* Se você escolher **Não agora**, Claude Code não oferece novamente nesta máquina.
* Claude Code para de oferecer após ter mostrado o diálogo em três inicializações, respondidas ou não.

<h2 id="what-changes">
  O que muda
</h2>

A renderização em tela cheia altera como o CLI desenha no seu terminal. A caixa de entrada permanece fixa na parte inferior da tela em vez de se mover conforme a saída é transmitida. Se a entrada permanecer no lugar enquanto Claude está trabalhando, a renderização em tela cheia está ativa. Apenas as mensagens visíveis são mantidas na árvore de renderização, portanto a memória permanece constante independentemente do comprimento da conversa.

Como a conversa vive no buffer de tela alternativa em vez do scrollback do seu terminal, algumas coisas funcionam de forma diferente:

| Antes                                                        | Agora                                                                                        | Detalhes                                                           |
| :----------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------- |
| `Cmd+f` ou busca tmux para encontrar texto                   | `Ctrl+o` para modo de transcrição, depois `/` para buscar ou `[` para escrever no scrollback | [Buscar e revisar a conversa](#search-and-review-the-conversation) |
| Clique e arraste nativo do terminal para selecionar e copiar | Seleção no aplicativo, copia automaticamente ao soltar o mouse                               | [Usar o mouse](#use-the-mouse)                                     |
| `Cmd`-clique para abrir uma URL                              | `Cmd`-clique no macOS, `Ctrl`-clique em outro lugar                                          | [Usar o mouse](#use-the-mouse)                                     |

Se a captura de mouse interferir no seu fluxo de trabalho, você pode [desativá-la](#keep-native-text-selection) mantendo a renderização sem cintilação.

<h2 id="use-the-mouse">
  Use the mouse
</h2>

Fullscreen rendering captures mouse events and handles them inside Claude Code:

* **Click in the prompt input** to position your cursor anywhere in the text you're typing.
* **Click a suggestion in the `/` command or `@` file list** to accept it. Hovering highlights the row under your cursor.
* **Click an option in a select menu** to choose it. This covers permission prompts, `/model`, `/config`, and other dialogs that show a list of options. Hovering shows a pointer on the row under your cursor.
* **Click an option in a multi-select menu** to toggle it, and click the submit button to confirm your choices. Clicking a free-text row, such as the `Other` row in a multiple-choice question, focuses its input field so you can type an answer. Requires Claude Code v2.1.208 or later.
* **Click a setting's value in the `/config` panel** to change it, and scroll the settings list with the mouse wheel. Requires Claude Code v2.1.271 or later.
* **Scroll a select or multi-select menu with the mouse wheel** when it has more options than it shows at once, such as the `/model` list in a short terminal window. The wheel scrolls the list while the pointer is over its options. Requires Claude Code v2.1.280 or later.
* **Click a collapsed tool result** to expand it and see the full output. Click again to collapse. The tool call and its result expand together. Only messages that have more to show are clickable.
  * Clicking also expands the output of a `!` shell command, whether an older truncated result or the live progress row while the command runs. Requires Claude Code v2.1.257 or later.
* **Hold `Cmd` on macOS, or `Ctrl` on Linux and Windows, and click a URL or file path** to open it. Plain `http://` and `https://` URLs open in your browser, and file paths in tool output, like the ones printed after an Edit or Write, open in your default application. A plain click without the modifier doesn't open links, matching native terminal behavior.
  * Claude Code renders a network (UNC) path, such as `\\server\share\file.ts`, as plain text with no link, because opening a network path can send your Windows credentials to the host it names.
  * Some macOS terminals forward `Cmd`+click to the running app instead of opening the link themselves, and the terminal mouse protocol has no way to encode the `Cmd` key, so Claude Code receives a plain click. In Ghostty, and in Warp on macOS, Claude Code detects this and lets a plain click on a link open it, and holding `Cmd` still works.
  * In the VS Code integrated terminal and similar xterm.js-based terminals, Claude Code defers to the terminal's own link handler, which uses the same gesture.
* **Click and drag** to select text anywhere in the conversation. Double-click selects a word, matching iTerm2's word boundaries so a file path selects as one unit. Double-clicking a URL selects the whole URL, including the scheme. Triple-click selects the line.
* **Scroll with the mouse wheel** to move through the conversation.

Selected text copies to your clipboard automatically on mouse release. To turn this off, toggle Copy on select in `/config`.

With Copy on select off, press `Ctrl+Shift+c` to copy manually. On terminals that support the kitty keyboard protocol, such as kitty, WezTerm, Ghostty, and iTerm2, `Cmd+c` also works. If you have a selection active, `Ctrl+c` copies instead of cancelling.

With a selection active, hold `Shift` and press the arrow keys to extend it from the keyboard. `Shift+↑` and `Shift+↓` scroll the viewport when the selection reaches the top or bottom edge. `Shift+Home` and `Shift+End` extend to the start or end of the current line.

In the normal prompt view, what happens to an active selection depends on the key you press:

* **`Esc`**: Claude Code performs the key's usual action, such as interrupting the running response or dismissing an open dialog, and the selection stays highlighted.
* **`PgUp`, `PgDn`, `Ctrl+Home`, `Ctrl+End`, or `Shift`, `Alt` or `Option`, or `Cmd`, `Win`, or `Super` with an arrow, `Home`, or `End` key**: the selection stays.
* **Any other key, including plain arrow keys, `Enter`, and typed characters**: Claude Code clears the selection.
* **A key bound to [`selection:clear`](/docs/pt/keybindings#scroll-actions)**: Claude Code clears the selection, even when the key is `Esc` or another key that otherwise keeps it. The action has no default binding.

In [transcript mode](#search-and-review-the-conversation), the navigation and search keys listed there also keep the selection.

<h2 id="scroll-the-conversation">
  Rolar a conversa
</h2>

A renderização em tela cheia lida com a rolagem dentro do aplicativo. Use estes atalhos para navegar:

| Atalho          | Ação                                                     |
| :-------------- | :------------------------------------------------------- |
| `PgUp` / `PgDn` | Rolar para cima ou para baixo meia tela                  |
| `Ctrl+Home`     | Ir para o início da conversa                             |
| `Ctrl+End`      | Ir para a mensagem mais recente e reativar o auto-follow |
| Roda do mouse   | Rolar algumas linhas por vez                             |

Você pode rolar de volta para o início da sessão mesmo após [compactação](/docs/pt/context-window#what-survives-compaction). Claude continua trabalhando a partir do resumo de compactação, mas Claude Code mantém todas as mensagens anteriores no scrollback em tela cheia em compactações repetidas.

Em teclados sem teclas dedicadas `PgUp`, `PgDn`, `Home` ou `End`, como teclados de MacBook, mantenha `Fn` pressionado com as teclas de seta: `Fn+↑` envia `PgUp`, `Fn+↓` envia `PgDn`, `Fn+←` envia `Home` e `Fn+→` envia `End`. `Ctrl+Fn+→` não alcança Claude Code no macOS, portanto um teclado de MacBook não tem um atalho de salto para o final funcionando por padrão. Em vez disso, use uma destas opções:

* Clique no [botão de salto para o final](#auto-follow).
* Role até o final com a roda do mouse para retomar o acompanhamento.
* Rebinde `scroll:bottom` para um atalho que seu teclado possa enviar.

Essas ações são rebindáveis. Consulte [Ações de rolagem](/docs/pt/keybindings#scroll-actions) para a lista completa de nomes de ações, incluindo variantes de meia página e página inteira que não têm vinculação padrão.

Enquanto você está rolado para cima, uma linha de cabeçalho atenuada no topo da conversa mostra o prompt mais recente que rolou acima da visualização. Clique na linha para ir para esse prompt.

<h3 id="auto-follow">
  Auto-follow
</h3>

Rolar para cima pausa o auto-follow para que a nova saída não o puxe de volta para o final. Um botão `Jump to bottom` flutua sobre a borda inferior da transcrição enquanto você está rolado para cima, e mostra uma contagem como `3 new messages` quando nova saída chega. Clique nele, pressione `Ctrl+End` ou role até o final para retomar o acompanhamento.

Enquanto o auto-follow está pausado, a visualização também permanece onde você a rolou quando uma resposta termina de ser transmitida.

A dica de teclado do botão reflete o que seu teclado pode enviar. No macOS, ele sugere clicar ou `Fn+↓` para rolar, porque `Ctrl+End` não alcança Claude Code a partir de um teclado Mac. Rebinde [`scroll:bottom`](/docs/pt/keybindings#scroll-actions) e o botão mostra seu atalho em todas as plataformas.

Em um terminal muito estreito para o rótulo completo, o botão encurta a dica em vez de quebrar para a linha de transcrição abaixo.

Para desativar completamente o auto-follow para que a visualização permaneça onde você a deixou, abra `/config` e defina Auto-scroll como desativado. Com auto-scroll desativado, a visualização nunca salta para o final por conta própria. Prompts de permissão e outros diálogos que precisam de uma resposta ainda rolam para a visualização independentemente dessa configuração.

<h3 id="mouse-wheel-scrolling">
  Rolagem com roda do mouse
</h3>

A rolagem com roda do mouse requer que seu terminal encaminhe eventos do mouse para Claude Code. A maioria dos terminais faz isso sempre que um aplicativo solicita. iTerm2 torna isso uma configuração por perfil: se a roda não faz nada mas `PgUp` e `PgDn` funcionam, abra Settings → Profiles → Terminal e ative Enable mouse reporting. A mesma configuração também é necessária para que o clique para expandir e a seleção de texto funcionem.

Se a rolagem com roda do mouse parecer lenta, seu terminal pode estar enviando um evento de rolagem por entalhe físico sem multiplicador. Alguns terminais, como Ghostty e iTerm2 com rolagem mais rápida ativada, já amplificam eventos de roda. Outros, incluindo o terminal integrado do VS Code, enviam exatamente um evento por entalhe. Claude Code não consegue detectar qual.

Defina `CLAUDE_CODE_SCROLL_SPEED` para multiplicar a distância de rolagem base:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Um valor de `3` corresponde ao padrão em `vim` e aplicativos similares. A configuração aceita qualquer valor positivo até 20, incluindo valores fracionários abaixo de 1, como `0.25` para desacelerar a rolagem de trackpad e roda do mouse acelerada em terminais que já amplificam eventos de roda.

Para ajustar a velocidade de rolagem interativamente, execute `/scroll-speed`. O diálogo mostra uma régua que você pode rolar enquanto está aberto para que você possa sentir a mudança imediatamente. Pressione `←` e `→` para ajustar a velocidade, `r` para redefinir para o padrão detectado automaticamente e `Enter` para salvar. O diálogo avança em números inteiros até 10, e em terminais que suportam controle mais fino, também oferece passos de um quarto até 0,25.

O comando escreve o mesmo valor que a variável de ambiente `CLAUDE_CODE_SCROLL_SPEED` define, persistido em `~/.claude/settings.json`. O máximo do diálogo é 10: se você definir um valor mais alto através da variável de ambiente, o diálogo mostra 10, e salvar a partir do diálogo persiste 10. O comando não está disponível no terminal IDE JetBrains.

Separadamente da velocidade base, Claude Code acelera a taxa de rolagem quando você gira a roda rapidamente, portanto um giro rápido cobre mais distância do que o mesmo número de entalhes lentos. Para desativar a aceleração e manter uma taxa constante por entalhe, defina `wheelScrollAccelerationEnabled` como `false` em [`settings.json`](/docs/pt/settings-reference#all-settings). Esta configuração requer Claude Code v2.1.174 ou posterior.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Rolar no terminal IDE JetBrains
</h3>

No terminal IDE JetBrains, Claude Code aplica seu próprio tratamento de rolagem e ignora `CLAUDE_CODE_SCROLL_SPEED`. O terminal envia eventos de rolagem em uma taxa muito mais alta do que outros emuladores, portanto um multiplicador ajustado em outro lugar ultrapassa aqui.

Em 2025.2, o terminal também tem bugs de rolagem de roda que produzem teclas de seta espúrias e eventos de direção errada. Claude Code detecta isso em tempo de execução e mitiga automaticamente, portanto a rolagem de trackpad e roda do mouse funcionam sem configuração. Para a melhor experiência de rolagem, atualize para 2025.3 ou posterior. Claude Code mostra uma dica na primeira vez que você rola se detectar o bug.

<h2 id="search-and-review-the-conversation">
  Pesquisar e revisar a conversa
</h2>

`Ctrl+o` alterna entre o modo de prompt normal e o modo de transcrição.

Para uma visualização mais silenciosa que mostra apenas seu último prompt, um resumo de uma linha de chamadas de ferramentas com estatísticas de edição de diff, e a resposta final, execute `/focus`. A configuração persiste entre sessões. Execute `/focus` novamente para desativá-la.

O modo de transcrição ganha navegação e pesquisa no estilo `less`:

| Tecla                                | Ação                                                                                                                                  |
| :----------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `/`                                  | Abre a pesquisa. Digite para encontrar correspondências, `Enter` para aceitar, `Esc` para cancelar e restaurar sua posição de rolagem |
| `n` / `N`                            | Pula para a próxima ou anterior correspondência. Funciona depois que você fechou a barra de pesquisa                                  |
| `j` / `k` ou `↑` / `↓`               | Rola uma linha                                                                                                                        |
| `g` / `G` ou `Home` / `End`          | Pula para o topo ou para o final                                                                                                      |
| `{` / `}`                            | Pula para o prompt anterior ou próximo                                                                                                |
| `Ctrl+u` / `Ctrl+d`                  | Rola meia página                                                                                                                      |
| `Ctrl+b` / `Ctrl+f` ou `Space` / `b` | Rola uma página inteira                                                                                                               |
| `Ctrl+o`, `Esc`, ou `q`              | Sai do modo de transcrição e retorna ao prompt                                                                                        |

A `Cmd+f` do seu terminal e a pesquisa do tmux não veem a conversa porque ela vive no buffer de tela alternativa, não no scrollback nativo. Para devolver o conteúdo ao seu terminal, pressione `Ctrl+o` para entrar no modo de transcrição primeiro, depois:

* **`[`**: escreve a conversa completa no buffer de scrollback nativo do seu terminal, com toda a saída de ferramentas expandida. A conversa agora é texto comum no seu terminal, então `Cmd+f`, modo de cópia do tmux e qualquer outra ferramenta nativa podem pesquisar ou selecioná-la. Sessões longas podem pausar por um momento enquanto isso acontece. Isso dura até você sair do modo de transcrição com `Esc` ou `q`, que o retorna à renderização em tela cheia. O próximo `Ctrl+o` começa do zero.
* **`v`**: escreve a conversa em um arquivo temporário e o abre em `$VISUAL` ou `$EDITOR`.

<h2 id="watch-your-changes-in-the-diff-panel">
  Observe suas alterações no painel de diff
</h2>

Na renderização em tela cheia, [`/diff`](/docs/pt/interactive-mode#review-changes-with-%2Fdiff) abre um painel ao lado da conversa em vez de um visualizador que você precisa fechar, para que você possa observar as alterações se acumularem enquanto Claude trabalha. Em um terminal largo, o painel também pode abrir por conta própria assim que Claude começar a editar arquivos. [Painel de diff](/docs/pt/interactive-mode#diff-panel) aborda o que ele mostra, como mantê-lo fechado e como alterar com o que ele se compara.

<h2 id="clear-the-conversation">
  Limpar a conversa
</h2>

Execute `/clear` para iniciar uma nova conversa.

Se a exibição parecer distorcida ou parcialmente em branco, pressione `Ctrl+L` para redesenhar a tela. O redesenho mantém a conversa e sua entrada no lugar.

`Cmd+K` faz o mesmo que `Ctrl+L` quando seu terminal o passa para Claude Code. iTerm2 e Terminal.app lidam com `Cmd+K` por conta própria e limpam sua própria tela, e Claude Code detecta a tela limpa e repinta a conversa. Antes da v2.1.280, começando com v2.1.260, pressionar `Ctrl+L` ou `Cmd+K` onde chega a Claude Code limpava a tela na renderização em tela cheia. Antes da v2.1.238, pressionar `Ctrl+L` duas vezes em dois segundos executava `/clear`.

<h2 id="use-with-tmux">
  Usar com tmux
</h2>

A renderização em tela cheia funciona dentro do tmux, com três ressalvas.

A rolagem da roda do mouse requer o modo de mouse do tmux. Se seu `~/.tmux.conf` ainda não o ativa, adicione esta linha e recarregue sua configuração:

```bash theme={null}
set -g mouse on
```

Sem o modo de mouse, os eventos da roda vão para o tmux em vez de Claude Code. A rolagem por teclado com `PgUp` e `PgDn` funciona de qualquer forma. Claude Code imprime uma dica única na inicialização se detectar tmux com o modo de mouse desativado.

A renderização em tela cheia é incompatível com o modo de integração tmux do iTerm2, que é o modo que você entra com `tmux -CC`. No modo de integração, o iTerm2 renderiza cada painel tmux como uma divisão nativa em vez de permitir que o tmux desenhe no terminal. O buffer de tela alternativa e o rastreamento de mouse não funcionam corretamente lá: a roda do mouse não faz nada e o clique duplo pode corromper o estado do terminal. Não ative a renderização em tela cheia em sessões `tmux -CC`. O tmux regular dentro do iTerm2, sem `-CC`, funciona bem.

As versões do tmux até a série 3.6 não implementam saída sincronizada, portanto, sob essas versões, você pode ver mais cintilação durante redesenhos do que ao executar Claude Code diretamente em seu terminal. Claude Code investiga o terminal para suporte de saída sincronizada na inicialização e a usa quando o terminal a relata. Se você vir cintilação sob tmux, atualize para o tmux mais recente ou execute Claude Code em sua própria aba de terminal fora do tmux.

<h2 id="keep-native-text-selection">
  Manter a seleção de texto nativa
</h2>

A captura de mouse é o ponto de atrito mais comum, especialmente sobre SSH ou dentro do tmux. Quando Claude Code captura eventos de mouse, a cópia nativa ao selecionar do seu terminal deixa de funcionar. A seleção que você faz com clique e arrasto existe dentro do Claude Code, não no buffer de seleção do seu terminal, portanto o modo de cópia do tmux, dicas do Kitty e ferramentas similares não a veem.

Claude Code escreve a seleção para a área de transferência do seu sistema, e o caminho que usa depende da sua configuração. Em uma sessão local, ele executa uma ferramenta de área de transferência nativa:

* **macOS**: `pbcopy`
* **Linux**: `wl-copy` no Wayland, ou `xclip` ou `xsel` no X11, o que estiver instalado. Claude Code escreve tanto a área de transferência quanto a seleção PRIMARY, portanto a colagem com clique do meio funciona.
* **Windows e WSL**: PowerShell `Set-Clipboard`

Dentro do tmux, ele também escreve no buffer de colagem do tmux. Sobre SSH, ele volta para sequências de escape OSC 52. Dentro do GNU screen, Claude Code copia seleções longas para a área de transferência também. Antes da v2.1.219, se você copiasse uma seleção mais longa que aproximadamente 570 caracteres, o GNU screen imprimia texto em base64 na janela. Claude Code imprime um aviso após cada cópia informando qual caminho foi usado.

Alguns terminais bloqueiam OSC 52 por padrão. O iTerm2 bloqueia até que você ative Configurações → Geral → Seleção → Aplicativos no terminal podem acessar a área de transferência; executar [`/terminal-setup`](/docs/pt/terminal-config) no iTerm2 ativa isso para você.

Para uma seleção nativa única, a tecla a usar depende do seu terminal:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor e Devin Desktop**: `Shift`, ou `Option` no macOS com a configuração `terminal.integrated.macOptionClickForcesSelection` ativada
* **A maioria dos outros terminais**: `Shift`

Mantenha essa tecla pressionada enquanto clica e arrasta. Seu terminal lida com a seleção em si, em vez de passá-la para Claude Code, portanto atalhos de cópia como `Cmd+C` funcionam no que você seleciona. Claude Code também mostra a tecla correta em sua dica na tela.

Sobre SSH ou dentro do tmux, Claude Code nem sempre consegue detectar o terminal do qual você está se conectando, portanto a dica lista as teclas candidatas.

Se você depender de seleção nativa o tempo todo, defina `CLAUDE_CODE_DISABLE_MOUSE=1` para desativar a captura de mouse mantendo a renderização sem cintilação e memória plana:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Com a captura de mouse desativada, a rolagem por teclado com `PgUp`, `PgDn`, `Ctrl+Home` e `Ctrl+End` ainda funciona, e seu terminal lida com a seleção nativamente. Você perde clique para posicionar o cursor, clique para expandir a saída da ferramenta, clique em URL e rolagem de roda dentro do Claude Code.

Para manter a rolagem de roda mas desativar o clique, arrasto e manipulação de hover, defina `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1`. Requer Claude Code v2.1.195 ou posterior. `CLAUDE_CODE_DISABLE_MOUSE` tem precedência quando ambas as variáveis estão definidas.

Com cliques desativados, Claude Code ainda captura o mouse, portanto a roda e o touchpad rolam a conversa, mas cliques esquerdos não fazem nada dentro do Claude Code. Você ainda precisa manter a tecla do seu terminal para seleção nativa de clique e arrasto. Clique direito e colagem com clique do meio continuam funcionando em terminais que os suportam.

<h2 id="troubleshooting">
  Solução de Problemas
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  Texto obsoleto ou deslocado na tela
</h3>

A renderização em tela cheia envia apenas as células que mudaram entre quadros. Alguns terminais, mais comumente Windows Terminal e outros hosts baseados em ConPTY, coalescem essas escritas posicionadas incorretamente e deixam fragmentos de saída anterior na tela até você redimensionar a janela.

Defina [`CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1`](/docs/pt/env-vars) para repintar cada célula em cada quadro em vez de enviar atualizações incrementais.

No Windows PowerShell:

```powershell theme={null}
$env:CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT = "1"
claude
```

No macOS ou Linux:

```bash theme={null}
CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1 claude
```

No Windows, Claude Code já ativa o repaint completo automaticamente para sessões em segundo plano e [visualização de agente](/docs/pt/agent-view), portanto você só precisa definir a variável para uma sessão interativa em tela cheia que você iniciou diretamente.

<h3 id="fullscreen-renderer-didnt-finish-starting">
  `Claude Code's fullscreen renderer didn't finish starting last time` aparece na inicialização
</h3>

Se uma sessão em tela cheia nesta máquina falhar antes de ter iniciado com sucesso, Claude Code inicia sua próxima sessão no renderizador clássico e imprime uma de duas linhas. Uma sessão foi iniciada com sucesso uma vez que desenhou seu primeiro quadro e então permaneceu ativa por 10 segundos ou você a encerrou com `/exit`, Ctrl+C ou Ctrl+D. A linha que você vê informa o que Claude Code faz após esta sessão:

* Após uma falha na inicialização, você vê `Claude Code's fullscreen renderer didn't finish starting last time on this machine`. Claude Code tenta renderização em tela cheia novamente na próxima sessão que você inicia
* Após duas falhas na inicialização, você vê `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`. Claude Code continua usando o renderizador clássico até você atualizar Claude Code ou executar `/tui fullscreen`, e não imprime nada nessas sessões posteriores

Para confirmar que uma falha na inicialização é o motivo de você estar no renderizador clássico, execute `/tui` sem argumento. Enquanto uma falha na inicialização for o motivo, a linha `Current renderer` diz isso.

Para manter o renderizador clássico, execute `/tui default`, que salva a configuração `tui` sem reiniciar. Para tentar renderização em tela cheia novamente, execute `/tui fullscreen`. Se essa sessão também não terminar de iniciar, [relate o problema](#research-preview).

Antes da v2.1.236, Claude Code continuava iniciando sessões em renderização em tela cheia após uma falha na inicialização.

<h4 id="how-claude-code-counts-failed-starts">
  Como Claude Code conta falhas na inicialização
</h4>

* Sessões que contam: apenas sessões que iniciaram em renderização em tela cheia porque sua configuração `tui` diz isso, porque você aceitou o [diálogo de inicialização](#fullscreen-by-default), ou porque Claude Code inicia você em tela cheia por padrão
* `CLAUDE_CODE_NO_FLICKER=1`: se você defini-lo, Claude Code renderiza essa sessão em tela cheia mesmo após uma falha na inicialização, e não a conta
* Contagem redefinida: Claude Code conta falhas na inicialização por versão do Claude Code, e um início bem-sucedido em tela cheia redefine a contagem
* Diálogo de inicialização: se você aceitou o diálogo e a sessão reiniciada falhou, Claude Code não imprime nenhuma linha e não mostra o diálogo novamente nesta versão do Claude Code

<h2 id="research-preview">
  Visualização de pesquisa
</h2>

A renderização em tela cheia é um recurso de visualização de pesquisa. Ela foi testada em emuladores de terminal comuns, mas você pode encontrar problemas de renderização em terminais menos comuns ou configurações incomuns.

Se você encontrar um problema, execute `/feedback` dentro do Claude Code para reportá-lo, ou abra uma issue no [repositório GitHub do claude-code](https://github.com/anthropics/claude-code/issues). Inclua o nome e a versão do seu emulador de terminal.

Para desativar a renderização em tela cheia, execute `/tui default`, ou desative `CLAUDE_CODE_NO_FLICKER` se você a ativou dessa forma. Quando você voltar com `/tui default`, o Claude Code pode primeiro mostrar um prompt de feedback opcional perguntando o que o fez mudar. Digite um motivo e pressione `Enter` para enviá-lo, ou pressione `Esc` para pular. A CLI é relançada no renderizador clássico de qualquer forma. Para forçar o renderizador clássico independentemente da configuração `tui` salva, defina `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. O renderizador clássico mantém a conversa no scrollback nativo do seu terminal, portanto `Cmd+f` e o modo de cópia do tmux funcionam como de costume.

As sessões em segundo plano abertas a partir da [visualização de agente](/docs/pt/agent-view) ou `claude attach` sempre usam renderização em tela cheia. O terminal anexado entra no buffer de tela alternativa para mostrar a sessão, e o renderizador clássico não tem scrollback ou manipulação de mouse lá, portanto a configuração `tui` e `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` não se aplicam a elas.
