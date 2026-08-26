> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Renderização em tela cheia

> Ative um modo de renderização mais suave e sem cintilação com suporte a mouse e uso de memória estável em conversas longas.

<Note>
  A renderização em tela cheia é uma [visualização de pesquisa](#research-preview) opcional. Execute `/tui fullscreen` para alternar em sua conversa atual. O comportamento pode mudar com base no feedback.
</Note>

A renderização em tela cheia é um caminho de renderização alternativo para o Claude Code CLI que elimina cintilação, mantém o uso de memória constante em conversas longas e adiciona suporte a mouse. Ela desenha a interface no buffer de tela alternativa do terminal, como `vim` ou `htop`, e renderiza apenas as mensagens que estão visíveis no momento. Isso reduz a quantidade de dados enviados para seu terminal em cada atualização.

A diferença é mais notável em emuladores de terminal onde a taxa de transferência de renderização é o gargalo, como o terminal integrado do VS Code, tmux e iTerm2. Se a posição de rolagem do seu terminal pular para o topo enquanto Claude está trabalhando, ou a tela piscar conforme a saída da ferramenta é transmitida, este modo resolve esses problemas.

<Note>
  O termo tela cheia descreve como Claude Code assume a superfície de desenho do terminal, da mesma forma que `vim` faz. Não tem nada a ver com maximizar a janela do seu terminal e funciona em qualquer tamanho de janela.
</Note>

<h2 id="enable-fullscreen-rendering">
  Ativar renderização em tela cheia
</h2>

Execute `/tui fullscreen` dentro de qualquer conversa do Claude Code. O CLI salva a configuração [`tui`](/docs/pt/settings#available-settings) e reinicia em tela cheia com sua conversa intacta, para que você possa alternar no meio da sessão sem perder contexto. Execute `/tui default` para voltar ao renderizador clássico, ou `/tui` sem argumentos para imprimir qual renderizador está ativo.

A sessão reiniciada mantém a conversa como aparece na tela. Se você executou [`/rewind`](/docs/pt/checkpointing#rewind-and-summarize) anteriormente na sessão, a reinicialização retoma do ponto revertido em vez do transcript mais longo salvo no disco. Antes da v2.1.207, alternar renderizadores após um rewind restaurava a conversa que o rewind havia removido.

Você também pode definir a variável de ambiente `CLAUDE_CODE_NO_FLICKER` antes de iniciar Claude Code:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

A configuração `tui` e a variável de ambiente são equivalentes. O comando `/tui` limpa `CLAUDE_CODE_NO_FLICKER` do processo reiniciado para que a configuração que ele escreve tenha efeito.

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
  Usar o mouse
</h2>

A renderização em tela cheia captura eventos de mouse e os manipula dentro do Claude Code:

* **Clique na entrada do prompt** para posicionar seu cursor em qualquer lugar no texto que você está digitando.
* **Clique em uma sugestão no comando `/` ou na lista de arquivo `@`** para aceitá-la. Passar o mouse destaca a linha sob seu cursor.
* **Clique em uma opção em um menu de seleção** para escolhê-la. Isso cobre prompts de permissão, `/model`, `/config` e outros diálogos que mostram uma lista de opções. Passar o mouse mostra um ponteiro na linha sob seu cursor. Requer Claude Code v2.1.187 ou posterior.
* **Clique em uma opção em um menu de seleção múltipla** para alternná-la, e clique no botão enviar para confirmar suas escolhas. Clicar em uma linha de texto livre, como a linha `Other` em uma pergunta de múltipla escolha, foca seu campo de entrada para que você possa digitar uma resposta. Requer Claude Code v2.1.208 ou posterior.
* **Clique em um resultado de ferramenta recolhido** para expandi-lo e ver a saída completa. Clique novamente para recolher. A chamada de ferramenta e seu resultado se expandem juntos. Apenas mensagens que têm mais a mostrar são clicáveis.
* **Mantenha `Cmd` no macOS, ou `Ctrl` no Linux e Windows, e clique em uma URL ou caminho de arquivo** para abri-lo. Caminhos de arquivo na saída da ferramenta, como os impressos após um Edit ou Write, abrem no seu aplicativo padrão. URLs simples `http://` e `https://` abrem no seu navegador. A partir da v2.1.181, um clique simples sem manter `Cmd` ou `Ctrl` não abre mais links, correspondendo ao comportamento do terminal nativo. Alguns terminais macOS encaminham `Cmd`+clique para o aplicativo em execução em vez de abrir o link eles mesmos, e o protocolo de mouse do terminal não tem como codificar a tecla `Cmd`, então Claude Code a recebe como um clique simples. No Ghostty, e a partir da v2.1.198 no Warp no macOS, Claude Code detecta isso e permite que um clique simples em um link o abra, e manter `Cmd` ainda funciona. No terminal integrado do VS Code e terminais semelhantes baseados em xterm.js, Claude Code defere para o próprio manipulador de links do terminal, que usa o mesmo gesto.
* **Clique e arraste** para selecionar texto em qualquer lugar da conversa. Clique duplo seleciona uma palavra, correspondendo aos limites de palavra do iTerm2 para que um caminho de arquivo seja selecionado como uma unidade. A partir da v2.1.198, clicar duas vezes em uma URL seleciona a URL inteira, incluindo o esquema. Clique triplo seleciona a linha.
* **Role com a roda do mouse** para se mover pela conversa.

O texto selecionado é copiado para sua área de transferência automaticamente ao soltar o mouse. Para desativar isso, alterne Copiar ao selecionar em `/config`.

Com Copiar ao selecionar desativado, pressione `Ctrl+Shift+c` para copiar manualmente. Em terminais que suportam o protocolo de teclado kitty, como kitty, WezTerm, Ghostty e iTerm2, `Cmd+c` também funciona. Se você tiver uma seleção ativa, `Ctrl+c` copia em vez de cancelar.

Com uma seleção ativa, mantenha `Shift` pressionado e pressione as teclas de seta para estendê-la a partir do teclado. `Shift+↑` e `Shift+↓` rolam a janela de visualização quando a seleção atinge a borda superior ou inferior. `Shift+Home` e `Shift+End` estendem para o início ou fim da linha atual.

<h2 id="scroll-the-conversation">
  Rolar a conversa
</h2>

A renderização em tela cheia manipula a rolagem dentro do aplicativo. Use estes atalhos para navegar:

| Atalho          | Ação                                                      |
| :-------------- | :-------------------------------------------------------- |
| `PgUp` / `PgDn` | Role para cima ou para baixo por meia tela                |
| `Ctrl+Home`     | Pule para o início da conversa                            |
| `Ctrl+End`      | Pule para a mensagem mais recente e reative o auto-follow |
| Roda do mouse   | Role algumas linhas por vez                               |

Em teclados sem teclas dedicadas `PgUp`, `PgDn`, `Home` ou `End`, como teclados MacBook, mantenha `Fn` pressionado com as teclas de seta: `Fn+↑` envia `PgUp`, `Fn+↓` envia `PgDn`, `Fn+←` envia `Home` e `Fn+→` envia `End`. `Ctrl+Fn+→` não alcança Claude Code no macOS, portanto um teclado MacBook não tem um atalho de pulo para o final funcionando por padrão. Em vez disso, use uma destas opções:

* Clique no [botão de pulo para o final](#auto-follow).
* Role para o final com a roda do mouse para retomar o seguimento.
* Rebinde `scroll:bottom` para um atalho que seu teclado possa enviar.

Essas ações são rebindáveis. Veja [Ações de rolagem](/docs/pt/keybindings#scroll-actions) para a lista completa de nomes de ações, incluindo variantes de meia página e página completa que não têm vinculação padrão.

<h3 id="auto-follow">
  Auto-follow
</h3>

Rolar para cima pausa o auto-follow para que a nova saída não o puxe de volta para o final. Um botão `Pular para o final` flutua sobre a borda inferior da transcrição enquanto você está rolado para cima, e mostra uma contagem como `3 novas mensagens` quando nova saída chega. Clique nele, pressione `Ctrl+End` ou role para o final para retomar o seguimento.

Enquanto o auto-follow está pausado, a visualização também permanece onde você a rolou quando uma resposta termina de transmitir. Antes da v2.1.207, a visualização poderia pular acima do início da resposta quando uma resposta longa terminava de transmitir.

A dica de teclado do botão reflete o que seu teclado pode enviar. No macOS, ele sugere clicar ou `Fn+↓` para rolar, porque `Ctrl+End` não alcança Claude Code de um teclado Mac. Rebinde [`scroll:bottom`](/docs/pt/keybindings#scroll-actions) e o botão mostra seu atalho em todas as plataformas. Antes da v2.1.206, o botão sugeria `Ctrl+End` no macOS.

Em um terminal muito estreito para o rótulo completo, o botão encurta a dica em vez de quebrar para a linha de transcrição abaixo. Antes da v2.1.206, um rótulo longo poderia quebrar sobre a transcrição.

Para desativar o auto-follow completamente para que a visualização permaneça onde você a deixar, abra `/config` e defina Auto-scroll como desativado. Com auto-scroll desativado, a visualização nunca pula para o final por conta própria. Prompts de permissão e outros diálogos que precisam de uma resposta ainda rolam para a visualização independentemente dessa configuração.

<h3 id="mouse-wheel-scrolling">
  Rolagem da roda do mouse
</h3>

A rolagem da roda do mouse requer que seu terminal encaminhe eventos de mouse para Claude Code. A maioria dos terminais faz isso sempre que um aplicativo solicita. O iTerm2 torna isso uma configuração por perfil: se a roda não fizer nada, mas `PgUp` e `PgDn` funcionarem, abra Configurações → Perfis → Terminal e ative Ativar relatório de mouse. A mesma configuração também é necessária para clique para expandir e seleção de texto funcionarem.

Se a rolagem da roda do mouse parecer lenta, seu terminal pode estar enviando um evento de rolagem por entalhe físico sem multiplicador. Alguns terminais, como Ghostty e iTerm2 com rolagem mais rápida ativada, já amplificam eventos de roda. Outros, incluindo o terminal integrado do VS Code, enviam exatamente um evento por entalhe. Claude Code não consegue detectar qual.

Defina `CLAUDE_CODE_SCROLL_SPEED` para multiplicar a distância de rolagem base:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Um valor de `3` corresponde ao padrão em `vim` e aplicativos semelhantes. A configuração aceita valores de 1 a 20, e valores fracionários abaixo de 1, como `0.5`, para desacelerar a rolagem acelerada do trackpad e da roda em terminais que já amplificam eventos de roda.

Para ajustar a velocidade de rolagem interativamente, execute `/scroll-speed`. O diálogo mostra uma régua que você pode rolar enquanto está aberto para que você possa sentir a mudança imediatamente. Pressione `←` e `→` para ajustar, `r` para redefinir para o padrão detectado automaticamente e `Enter` para salvar.

O comando escreve o mesmo valor que a variável de ambiente `CLAUDE_CODE_SCROLL_SPEED` define, persistido em `~/.claude/settings.json`. O comando não está disponível no terminal do IDE JetBrains.

Separadamente da velocidade base, Claude Code acelera a taxa de rolagem quando você gira a roda rapidamente, portanto uma rotação rápida cobre mais distância do que o mesmo número de entalhes lentos. Para desativar a aceleração e manter uma taxa constante por entalhe, defina `wheelScrollAccelerationEnabled` como `false` em [`settings.json`](/docs/pt/settings#available-settings). Esta configuração requer Claude Code v2.1.174 ou posterior.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Rolagem no terminal do IDE JetBrains
</h3>

No terminal do IDE JetBrains, Claude Code aplica sua própria manipulação de rolagem e ignora `CLAUDE_CODE_SCROLL_SPEED`. O terminal envia eventos de rolagem em uma taxa muito mais alta do que outros emuladores, portanto um multiplicador ajustado em outro lugar ultrapassa aqui.

Em 2025.2, o terminal também tem bugs de rolagem de roda que produzem teclas de seta espúrias e eventos de direção incorreta. Claude Code detecta esses em tempo de execução e os mitiga automaticamente, portanto a rolagem do trackpad e da roda do mouse funcionam sem configuração. Para a melhor experiência de rolagem, atualize para 2025.3 ou posterior. Claude Code mostra uma dica na primeira vez que você rola se detectar o bug.

<h2 id="search-and-review-the-conversation">
  Buscar e revisar a conversa
</h2>

`Ctrl+o` alterna entre o prompt normal e o modo de transcrição.

Para uma visualização mais silenciosa que mostra apenas seu último prompt, um resumo de uma linha de chamadas de ferramenta com estatísticas de diff de edição e a resposta final, execute `/focus`. A configuração persiste entre sessões. Execute `/focus` novamente para desativá-la.

O modo de transcrição ganha navegação e busca no estilo `less`:

| Tecla                                | Ação                                                                                                                              |
| :----------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| `/`                                  | Abrir busca. Digite para encontrar correspondências, `Enter` para aceitar, `Esc` para cancelar e restaurar sua posição de rolagem |
| `n` / `N`                            | Pule para a próxima ou anterior correspondência. Funciona depois que você fechou a barra de busca                                 |
| `j` / `k` ou `↑` / `↓`               | Role uma linha                                                                                                                    |
| `g` / `G` ou `Home` / `End`          | Pule para o topo ou final                                                                                                         |
| `Ctrl+u` / `Ctrl+d`                  | Role meia página                                                                                                                  |
| `Ctrl+b` / `Ctrl+f` ou `Space` / `b` | Role uma página completa                                                                                                          |
| `Ctrl+o`, `Esc`, ou `q`              | Sair do modo de transcrição e retornar ao prompt                                                                                  |

O `Cmd+f` do seu terminal e a busca tmux não veem a conversa porque ela vive no buffer de tela alternativa, não no scrollback nativo. Para devolver o conteúdo ao seu terminal, pressione `Ctrl+o` para entrar no modo de transcrição primeiro, depois:

* **`[`**: escreve a conversa completa no buffer de scrollback nativo do seu terminal, com toda a saída da ferramenta expandida. A conversa agora é texto comum no seu terminal, portanto `Cmd+f`, modo de cópia tmux e qualquer outra ferramenta nativa pode buscá-la ou selecioná-la. Sessões longas podem pausar por um momento enquanto isso acontece. Isso dura até você sair do modo de transcrição com `Esc` ou `q`, que o retorna à renderização em tela cheia. O próximo `Ctrl+o` começa do zero.
* **`v`**: escreve a conversa em um arquivo temporário e a abre em `$VISUAL` ou `$EDITOR`.

Pressione `Esc` ou `q` para retornar ao prompt.

<h2 id="clear-the-conversation">
  Limpar a conversa
</h2>

Pressione `Ctrl+L` duas vezes em dois segundos para executar `/clear` e iniciar uma nova conversa. O primeiro pressionamento redesenha a tela e mostra uma dica; o segundo pressionamento limpa a conversa. No macOS, pressionar duas vezes `Cmd+K` também executa `/clear`.

<h2 id="use-with-tmux">
  Usar com tmux
</h2>

A renderização em tela cheia funciona dentro do tmux, com três ressalvas.

A rolagem da roda do mouse requer o modo de mouse do tmux. Se seu `~/.tmux.conf` ainda não o ativa, adicione esta linha e recarregue sua configuração:

```bash theme={null}
set -g mouse on
```

Sem o modo de mouse, os eventos de roda vão para tmux em vez de Claude Code. A rolagem do teclado com `PgUp` e `PgDn` funciona de qualquer forma. Claude Code imprime uma dica única na inicialização se detectar tmux com o modo de mouse desativado.

A renderização em tela cheia é incompatível com o modo de integração tmux do iTerm2, que é o modo que você entra com `tmux -CC`. No modo de integração, o iTerm2 renderiza cada painel tmux como uma divisão nativa em vez de deixar o tmux desenhar no terminal. O buffer de tela alternativa e o rastreamento de mouse não funcionam corretamente lá: a roda do mouse não faz nada e o clique duplo pode corromper o estado do terminal. Não ative a renderização em tela cheia em sessões `tmux -CC`. O tmux regular dentro do iTerm2, sem `-CC`, funciona bem.

Nem toda versão do tmux aplica saída sincronizada de aplicações, portanto você pode ver mais cintilação durante redesenhos sob tmux do que ao executar Claude Code diretamente em seu terminal. Se a cintilação for perceptível, especialmente via SSH, atualize para o tmux mais recente ou execute Claude Code em sua própria aba de terminal fora do tmux. Verifique sua versão do tmux com `tmux -V`.

Claude Code ativa a saída sincronizada automaticamente quando detecta tmux 3.4 ou posterior a partir da variável `TERM_PROGRAM_VERSION`, e volta a consultar o terminal diretamente para suporte de saída sincronizada quando a versão não pode ser determinada. Se os redesenhos realmente se tornam atômicos depende de sua versão do tmux honrar a saída sincronizada; se você ainda vir cintilação sob tmux 3.4 ou posterior, atualize para o tmux mais recente. Esta detecção requer Claude Code v2.1.200 ou posterior.

<h2 id="keep-native-text-selection">
  Manter seleção de texto nativa
</h2>

A captura de mouse é o ponto de atrito mais comum, especialmente sobre SSH ou dentro do tmux. Quando Claude Code captura eventos de mouse, a cópia nativa ao selecionar do seu terminal para de funcionar. A seleção que você faz com clique e arraste existe dentro do Claude Code, não no buffer de seleção do seu terminal, portanto o modo de cópia tmux, dicas do Kitty e ferramentas semelhantes não a veem.

Claude Code escreve a seleção na sua área de transferência do sistema, e o caminho que usa depende da sua configuração. Em uma sessão local, ele executa uma ferramenta de área de transferência nativa:

* **macOS**: `pbcopy`
* **Linux**: `wl-copy` no Wayland, ou `xclip` ou `xsel` no X11, o que estiver instalado. Claude Code escreve tanto a área de transferência quanto a seleção PRIMARY, portanto a colagem com clique do meio funciona.
* **Windows e WSL**: PowerShell `Set-Clipboard`

Dentro do tmux, também escreve no buffer de colagem do tmux. Sobre SSH, volta para sequências de escape OSC 52. Claude Code imprime um toast após cada cópia informando qual caminho foi usado.

Alguns terminais bloqueiam OSC 52 por padrão. O iTerm2 bloqueia até que você ative Configurações → Geral → Seleção → Aplicativos no terminal podem acessar a área de transferência; executar [`/terminal-setup`](/docs/pt/terminal-config) no iTerm2 ativa isso para você.

Para uma seleção nativa única, a tecla a usar depende do seu terminal:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor e Devin Desktop**: `Shift`, ou `Option` no macOS com a configuração `terminal.integrated.macOptionClickForcesSelection` ativada
* **Maioria dos outros terminais**: `Shift`

Mantenha essa tecla pressionada enquanto clica e arrasta. Seu terminal manipula a seleção em si em vez de passá-la para Claude Code, portanto atalhos de cópia como `Cmd+C` funcionam no que você seleciona. Claude Code também mostra a tecla correta em sua dica na tela.

Sobre SSH ou dentro do tmux, Claude Code nem sempre consegue detectar o terminal do qual você está se conectando, portanto a dica lista as teclas candidatas em vez disso.

Se você depender da seleção nativa o tempo todo, defina `CLAUDE_CODE_DISABLE_MOUSE=1` para optar por não participar da captura de mouse mantendo a renderização sem cintilação e memória plana:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Com a captura de mouse desativada, a rolagem do teclado com `PgUp`, `PgDn`, `Ctrl+Home` e `Ctrl+End` ainda funciona, e seu terminal manipula a seleção nativamente. Você perde clique para posicionar cursor, clique para expandir saída de ferramenta, clique em URL e rolagem de roda dentro do Claude Code.

Para manter a rolagem de roda mas desativar clique, arraste e manipulação de hover, defina `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1` em vez disso. Requer Claude Code v2.1.195 ou posterior. `CLAUDE_CODE_DISABLE_MOUSE` tem precedência quando ambas as variáveis são definidas.

Com cliques desativados, Claude Code ainda captura o mouse, portanto a roda e o touchpad rolam a conversa, mas cliques esquerdos não fazem nada dentro do Claude Code. Você ainda precisa manter a tecla do seu terminal pressionada para seleção nativa de clique e arraste. Clique direito e colagem com clique do meio continuam funcionando em terminais que os suportam.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  Texto obsoleto ou deslocado na tela
</h3>

A renderização em tela cheia envia apenas as células que mudaram entre quadros. Alguns terminais, mais comumente Windows Terminal e outros hosts baseados em ConPTY, coalescem essas escritas posicionadas incorretamente e deixam fragmentos de saída anterior na tela até que você redimensione a janela.

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

<h2 id="research-preview">
  Visualização de pesquisa
</h2>

A renderização em tela cheia é um recurso de visualização de pesquisa. Ela foi testada em emuladores de terminal comuns, mas você pode encontrar problemas de renderização em terminais menos comuns ou configurações incomuns.

Se encontrar um problema, execute `/feedback` dentro do Claude Code para relatá-lo, ou abra uma issue no [repositório GitHub claude-code](https://github.com/anthropics/claude-code/issues). Inclua o nome e a versão do seu emulador de terminal.

Para desativar a renderização em tela cheia, execute `/tui default`, ou desdefina `CLAUDE_CODE_NO_FLICKER` se você a ativou dessa forma. Para forçar o renderizador clássico independentemente da configuração `tui` salva, defina `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. O renderizador clássico mantém a conversa no scrollback nativo do seu terminal, portanto `Cmd+f` e o modo de cópia do tmux funcionam como de costume.

As sessões em segundo plano abertas a partir da [visualização de agente](/docs/pt/agent-view) ou `claude attach` sempre usam renderização em tela cheia. O terminal anexado entra no buffer de tela alternativa para mostrar a sessão, e o renderizador clássico não tem scrollback ou manipulação de mouse lá, portanto a configuração `tui` e `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` não se aplicam a elas.
