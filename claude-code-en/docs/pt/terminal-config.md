> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configure seu terminal para Claude Code

> Corrija Shift+Enter para novas linhas, obtenha um sinal sonoro do terminal quando Claude terminar, configure tmux, corresponda ao tema de cores e ative o modo Vim na CLI do Claude Code.

Claude Code funciona em qualquer terminal sem configuração. Esta página é para quando algo específico não está se comportando da forma que você espera. Encontre seu sintoma abaixo. Se tudo já se sentir certo, você não precisa desta página.

* [Shift+Enter envia em vez de inserir uma nova linha](#enter-multiline-prompts)
* [Atalhos de tecla Option não funcionam no macOS](#enable-option-key-shortcuts-on-macos)
* [Sem som ou alerta quando Claude termina](#get-a-terminal-bell-or-notification)
* [Você executa Claude Code dentro do tmux](#configure-tmux)
* [Backspace deleta uma palavra inteira no Windows](#fix-backspace-deleting-a-whole-word-on-windows)
* [A exibição cintila ou o scrollback salta](#switch-to-fullscreen-rendering)
* [Você quer teclas Vim no prompt](#edit-prompts-with-vim-keybindings)

Esta página é sobre fazer seu terminal enviar os sinais corretos para Claude Code. Para alterar quais teclas Claude Code responde, consulte [atalhos de teclado](/docs/pt/keybindings) em vez disso.

<h2 id="enter-multiline-prompts">
  Inserir prompts multilinhas
</h2>

Pressionar Enter envia sua mensagem. Para adicionar uma quebra de linha sem enviar, pressione Ctrl+J, ou digite `\` e depois pressione Enter. Ambos funcionam em todo terminal sem necessidade de configuração.

Na maioria dos terminais você também pode pressionar Shift+Enter, mas o suporte varia de acordo com o emulador de terminal:

| Terminal                                                                                            | Shift+Enter para nova linha                                         |
| :-------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------ |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal                             | Funciona sem configuração                                           |
| Outros terminais que suportam o protocolo de teclado kitty, como foot e Alacritty 0.16 ou posterior | Funciona sem configuração. Requer Claude Code v2.1.269 ou posterior |
| VS Code, Cursor, Devin Desktop, Alacritty anterior a 0.16, Zed                                      | Execute `/terminal-setup` uma vez                                   |
| gnome-terminal, JetBrains IDEs como PyCharm e Android Studio                                        | Não disponível; use Ctrl+J ou `\` depois Enter                      |

Para VS Code, Cursor, Devin Desktop, Alacritty anterior a 0.16 e Zed, `/terminal-setup` escreve um atalho de teclado Shift+Enter no arquivo de configuração do terminal. Na primeira execução você vê uma confirmação como `Installed VSCode terminal Shift+Enter key binding`. Os bindings existentes são mantidos no lugar; se você vir uma mensagem como `VSCode terminal Shift+Enter key binding already configured`, nenhuma alteração foi feita. Execute `/terminal-setup` diretamente no terminal do host em vez de dentro de tmux ou screen, pois ele precisa escrever na configuração do terminal do host.

No VS Code, Cursor e Devin Desktop, `/terminal-setup` também atualiza duas configurações do editor: define `terminal.integrated.gpuAcceleration` para `"off"` para evitar texto distorcido no terminal integrado, e define `terminal.integrated.mouseWheelScrollSensitivity` para rolagem mais suave no [modo tela cheia](/docs/pt/fullscreen). Para desfazer a alteração de aceleração de GPU, defina-a de volta para `"auto"` e recarregue a janela do editor.

No Zed, `/terminal-setup` atualiza seu `keymap.json` no local:

* Se o keymap já tiver bindings e nenhum deles for um Terminal `shift-enter`, Claude Code primeiro faz um backup para uma cópia no mesmo diretório, como `keymap.json.1a2b3c4d.bak`, depois mescla o binding Shift+Enter em seu keymap, mantendo seus outros atalhos de teclado e comentários
* Se Claude Code não conseguir ler ou analisar o keymap, não conseguir fazer backup, ou não conseguir verificar o resultado mesclado, ele [deixa o arquivo inalterado e imprime o bloco de binding para você adicionar você mesmo](/docs/pt/errors#terminal-setup-left-your-zed-keymap-unchanged)

Se você está executando dentro de tmux, Shift+Enter também requer a [configuração de tmux abaixo](#configure-tmux) mesmo quando o terminal externo a suporta.

Para vincular nova linha a uma tecla diferente, ou para trocar o comportamento para que Enter insira uma nova linha e Shift+Enter envie, mapeie as ações `chat:newline` e `chat:submit` em seu [arquivo de atalhos de teclado](/docs/pt/keybindings).

<h2 id="enable-option-key-shortcuts-on-macos">
  Ativar atalhos de teclado Option no macOS
</h2>

Alguns atalhos de teclado do Claude Code usam a tecla Option, como Option+Enter para uma nova linha ou Option+P para alternar modelos. No macOS, a maioria dos terminais não envia Option como um modificador por padrão, portanto esses atalhos não funcionam até que você o ative. A configuração do terminal para isso geralmente é rotulada como "Use Option as Meta Key"; Meta é o nome histórico do Unix para a tecla agora rotulada como Option ou Alt.

<Tabs>
  <Tab title="Apple Terminal">
    Abra Configurações → Perfis → Teclado e marque "Use Option as Meta Key".

    Se você aceitou o prompt de configuração do terminal na primeira execução do Claude Code, isso já foi feito. Esse prompt executa `/terminal-setup` para você, que ativa Option como Meta e desativa o sino audível no seu perfil do Apple Terminal.

    No [modo leitor de tela](/docs/pt/accessibility), `/terminal-setup` deixa a configuração do sino inalterada para que o sino do terminal permaneça audível. Antes da v2.1.211, `/terminal-setup` desativava o sino mesmo no modo leitor de tela. Se uma execução anterior desativou o sino, ative-o novamente em Configurações → Perfis → Avançado → "Audible bell".
  </Tab>

  <Tab title="iTerm2">
    Abra Configurações → Perfis → Teclas → Geral e defina a tecla Option esquerda e a tecla Option direita como "Esc+".

    Executar `/terminal-setup` no iTerm2 ativa "Applications in terminal may access clipboard" em Configurações → Geral → Seleção para que o comando `/copy` possa escrever na sua área de transferência do sistema. O comando detecta iTerm2 mesmo quando executado dentro do tmux. Reinicie o iTerm2 para que a alteração tenha efeito.
  </Tab>

  <Tab title="VS Code">
    Adicione `"terminal.integrated.macOptionIsMeta": true` às suas configurações do VS Code.
  </Tab>
</Tabs>

Para Ghostty, Kitty e outros terminais, procure por uma configuração Option-as-Alt ou Option-as-Meta no arquivo de configuração do terminal.

<h2 id="get-a-terminal-bell-or-notification">
  Obtenha um sinal sonoro do terminal ou notificação
</h2>

Quando Claude termina uma tarefa ou pausa para um prompt de permissão, e você parece estar longe do terminal, ele dispara um evento de notificação. Veja [quando cada tipo de notificação é disparado](/docs/pt/hooks#notification) para o tempo exato. Exibir isso como um sinal sonoro do terminal ou notificação de desktop permite que você mude para outro trabalho enquanto uma tarefa longa é executada.

Por padrão, Claude Code envia uma notificação de desktop apenas em Ghostty, Kitty e iTerm2. Em outros terminais, defina [`preferredNotifChannel`](/docs/pt/settings-reference#preferrednotifchannel) como `"terminal_bell"` para tocar o sinal sonoro do terminal, ou configure um [hook de Notificação](#play-a-sound-with-a-notification-hook) para um som personalizado ou comando. A seguinte entrada de configurações ativa o sinal sonoro do terminal:

```json ~/.claude/settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

A notificação de desktop chega à sua máquina local via SSH, portanto uma sessão remota ainda pode alertá-lo. Ghostty e Kitty a encaminham para seu centro de notificações do SO sem configuração adicional. iTerm2 requer que você ative o encaminhamento:

<Steps>
  <Step title="Abra as configurações de notificação do iTerm2">
    Vá para Configurações → Perfis → Terminal.
  </Step>

  <Step title="Ative alertas">
    Marque "Notification Center Alerts", depois clique em "Filter Alerts" e ative "Send escape sequence-generated alerts".
  </Step>
</Steps>

Se as notificações ainda não aparecerem, confirme que seu aplicativo de terminal tem permissão de notificação nas configurações do SO, e se você está executando dentro do tmux, [ative passthrough](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Reproduza um som com um hook de Notification
</h3>

Em qualquer terminal, você pode configurar um [hook de Notification](/docs/pt/hooks-guide#get-notified-when-claude-needs-input) para reproduzir um som ou executar um comando personalizado quando Claude precisar de sua atenção. Hooks são executados junto com a notificação integrada em vez de substituí-la, portanto terminais que não recebem uma notificação de desktop, como Warp ou o terminal integrado do VS Code, podem usar um hook ou definir `preferredNotifChannel` como `"terminal_bell"` em vez disso.

O exemplo abaixo reproduz um som do sistema no macOS. O guia vinculado tem comandos de notificação de desktop para macOS, Linux e Windows.

```json ~/.claude/settings.json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "hooks": [{ "type": "command", "command": "afplay /System/Library/Sounds/Glass.aiff" }]
      }
    ]
  }
}
```

<h2 id="configure-tmux">
  Configurar tmux
</h2>

Quando Claude Code é executado dentro do tmux, por padrão Shift+Enter envia em vez de inserir uma nova linha, e notificações de desktop e a [barra de progresso](/docs/pt/settings-reference#terminalprogressbarenabled) nunca chegam ao terminal externo. Adicione estas linhas a `~/.tmux.conf`, depois execute `tmux source-file ~/.tmux.conf` para aplicá-las ao servidor em execução:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

A linha `allow-passthrough` permite que notificações e atualizações de progresso cheguem ao terminal externo em vez de serem engolidas pelo tmux. As linhas `extended-keys` permitem que o tmux distinga Shift+Enter de Enter simples para que o atalho de nova linha funcione.

<h2 id="fix-backspace-deleting-a-whole-word-on-windows">
  Corrigir Backspace deletando uma palavra inteira no Windows
</h2>

No Windows, Claude Code lê um Backspace que chega como `^H` como Ctrl+Backspace, que [deleta a palavra anterior](/docs/pt/interactive-mode#text-editing), exceto quando `TERM_PROGRAM` é `mintty` ou `TERM` é `cygwin`. No macOS e Linux, Claude Code o lê como Backspace simples.

Se cada pressão de Backspace deleta uma palavra inteira, seu terminal envia `^H` para Backspace simples. Defina [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE=0`](/docs/pt/env-vars). Backspace e Ctrl+H então apagam um caractere cada. Se Ctrl+Backspace apaga apenas um caractere no macOS ou Linux porque seu terminal envia `^H` para ele, defina a variável para `1` em vez disso.

<h2 id="match-the-color-theme">
  Corresponder ao tema de cores
</h2>

Use o comando `/theme`, ou o seletor de tema em `/config`, para escolher um tema Claude Code que corresponda ao seu terminal. Selecionar a opção automática detecta o fundo claro ou escuro do seu terminal, para que o tema siga as mudanças de aparência do SO sempre que seu terminal fizer. Claude Code não controla o esquema de cores do próprio terminal, que é definido pela aplicação do terminal.

Para personalizar o que aparece na parte inferior da interface, configure uma [linha de status personalizada](/docs/pt/statusline) que mostra o modelo atual, diretório de trabalho, branch do git ou outro contexto.

<h3 id="create-a-custom-theme">
  Criar um tema personalizado
</h3>

Além dos presets integrados, `/theme` lista todos os temas personalizados que você definiu e quaisquer temas contribuídos pelos [plugins](/docs/pt/plugins/components#themes-and-output-styles) instalados. Selecione **Novo tema personalizado…** no final da lista para criar um interativamente: você nomeia o tema e depois escolhe tokens de cores individuais para substituir. Pressione `Ctrl+E` enquanto um tema personalizado está destacado para editá-lo.

Cada tema personalizado é um arquivo JSON em `~/.claude/themes/`. O nome do arquivo sem a extensão `.json` é o slug do tema, e selecionar o tema armazena `custom:<slug>` como sua preferência de tema. O arquivo tem três campos opcionais:

| Campo       | Tipo   | Descrição                                                                                                                                     |
| :---------- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`      | string | Rótulo de exibição mostrado em `/theme`. Padrão é o slug do nome do arquivo                                                                   |
| `base`      | string | Preset integrado do qual o tema começa: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi`, ou `light-ansi`. Padrão é `dark` |
| `overrides` | object | Mapa de nomes de tokens de cores para valores de cores. Tokens não listados aqui caem através do preset base                                  |

Valores de cores aceitam `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, ou `ansi:<name>` onde `<name>` é um dos 16 nomes de cores ANSI padrão, como `red` ou `cyanBright`. Tokens desconhecidos e valores de cores inválidos são ignorados, portanto um erro de digitação não pode quebrar a renderização.

O exemplo a seguir define um tema que mantém o preset escuro, mas recolore o acento do prompt, o texto de erro e o texto de sucesso:

```json ~/.claude/themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Claude Code observa `~/.claude/themes/` e recarrega quando um arquivo é adicionado ou alterado, portanto as edições feitas no seu editor se aplicam a uma sessão em execução sem uma reinicialização. Se a pasta `~/.claude/themes/` em si não existisse quando Claude Code foi iniciado, reinicie uma vez após criar seu primeiro arquivo de tema. Depois disso, as mudanças se aplicam sem uma reinicialização.

A referência abaixo cobre os tokens que você pode definir em `overrides`. O editor interativo em `/theme` mostra os mesmos tokens com uma visualização ao vivo, além de alguns acentos de propósito único, como cores de tela de integração, que são omitidas aqui.

<Accordion title="Referência de token de cor">
  O exemplo a seguir combina tokens de vários dos grupos abaixo: o acento da marca, a borda do modo de plano, os fundos de diff e o fundo da mensagem.

  ```json ~/.claude/themes/midnight.json theme={null}
  {
    "name": "Midnight",
    "base": "dark",
    "overrides": {
      "claude": "#a78bfa",
      "planMode": "#38bdf8",
      "diffAdded": "#14532d",
      "diffRemoved": "#7f1d1d",
      "userMessageBackground": "#1e1b4b"
    }
  }
  ```

  <h4 id="text-and-accent-colors">
    Cores de texto e acento
  </h4>

  Controle o acento da marca primária e as tonalidades de texto em primeiro plano usadas em toda a interface.

  | Token         | Controla                                                                 |
  | :------------ | :----------------------------------------------------------------------- |
  | `claude`      | Acento da marca primária, usado para o spinner e rótulo do assistente    |
  | `text`        | Texto em primeiro plano padrão                                           |
  | `inverseText` | Texto desenhado sobre um fundo colorido, como badges de status           |
  | `inactive`    | Texto secundário, como dicas, timestamps e itens desabilitados           |
  | `subtle`      | Bordas fracas e texto secundário de-enfatizado                           |
  | `suggestion`  | Sugestões de preenchimento automático e destaque de seleção em seletores |
  | `permission`  | Bordas de diálogo, incluindo prompts de permissão e seletores            |
  | `remember`    | Indicadores de memória e `CLAUDE.md`                                     |

  <h4 id="status-colors">
    Cores de status
  </h4>

  Sinalize estados de sucesso, falha e aviso em mensagens e indicadores.

  | Token     | Controla                                                  |
  | :-------- | :-------------------------------------------------------- |
  | `success` | Mensagens de sucesso e verificações aprovadas             |
  | `error`   | Mensagens de erro e falhas                                |
  | `warning` | Avisos, mensagens de cautela e a borda do modo automático |
  | `merged`  | Status de solicitação de pull mesclada                    |

  <h4 id="input-box-and-mode-indicators">
    Caixa de entrada e indicadores de modo
  </h4>

  Defina a cor da borda da caixa de entrada e o acento mostrado enquanto um modo de permissão ou indicador está ativo.

  | Token          | Controla                                                                                                                                                                                               |
  | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
  | `promptBorder` | Borda da caixa de entrada                                                                                                                                                                              |
  | `planMode`     | Acento do modo de plano, mensagens de plano e diálogos do modo de plano                                                                                                                                |
  | `autoAccept`   | Acento do modo aceitar-edições                                                                                                                                                                         |
  | `bashBorder`   | Borda da caixa de entrada ao inserir um comando shell `!`                                                                                                                                              |
  | `ide`          | Indicador de conexão IDE                                                                                                                                                                               |
  | `fastMode`     | Indicador de modo rápido                                                                                                                                                                               |
  | `effortUltra`  | A tag `ultracode` na borda da caixa de entrada enquanto [ultracode](/docs/pt/model-config#adjust-effort-level) está ativado. Sua substituição dessa cor entra em vigor no Claude Code v2.1.239 ou posterior |

  <h4 id="diff-rendering">
    Renderização de diff
  </h4>

  Colora código adicionado e removido em edições e revisões de arquivo.

  | Token               | Controla                                                                            |
  | :------------------ | :---------------------------------------------------------------------------------- |
  | `diffAdded`         | Fundo de linhas adicionadas                                                         |
  | `diffRemoved`       | Fundo de linhas removidas                                                           |
  | `diffAddedDimmed`   | Fundo de linhas adicionadas no diff atenuado mostrado após você rejeitar uma edição |
  | `diffRemovedDimmed` | Fundo de linhas removidas no diff atenuado mostrado após você rejeitar uma edição   |
  | `diffAddedWord`     | Destaque em nível de palavra dentro de uma linha adicionada                         |
  | `diffRemovedWord`   | Destaque em nível de palavra dentro de uma linha removida                           |

  <h4 id="fullscreen-mode">
    Modo tela cheia
  </h4>

  Claude Code pinta `userMessageBackground`, `bashMessageBackgroundColor` e `memoryBackgroundColor` nos renderizadores padrão e tela cheia. Ele usa `userMessageBackgroundHover` e `selectionBg` apenas no [modo de renderização tela cheia](/docs/pt/fullscreen).

  | Token                        | Controla                                                    |
  | :--------------------------- | :---------------------------------------------------------- |
  | `userMessageBackground`      | Fundo atrás de suas mensagens na transcrição                |
  | `userMessageBackgroundHover` | Fundo atrás de uma mensagem enquanto pairada ou expandida   |
  | `bashMessageBackgroundColor` | Fundo atrás de entradas de comando shell `!` na transcrição |
  | `memoryBackgroundColor`      | Fundo atrás de entradas de memória `#` na transcrição       |
  | `selectionBg`                | Fundo do texto selecionado com o mouse                      |

  <h4 id="usage-meter-and-speaker-labels">
    Medidor de uso e rótulos de alto-falante
  </h4>

  Ajuste a barra mostrada na visualização `/usage` e os rótulos que distinguem suas mensagens das de Claude.

  | Token              | Controla                                          |
  | :----------------- | :------------------------------------------------ |
  | `rate_limit_fill`  | Porção preenchida do medidor de uso               |
  | `rate_limit_empty` | Porção não preenchida do medidor de uso           |
  | `briefLabelYou`    | Cor do rótulo `You` em suas mensagens             |
  | `briefLabelClaude` | Cor do rótulo `Claude` em mensagens do assistente |

  <h4 id="shimmer-variants-and-subagent-colors">
    Variantes de shimmer e cores de subagente
  </h4>

  Vários tokens têm uma variante de shimmer emparelhada que fornece a cor mais clara usada no gradiente animado do spinner. Substitua o shimmer junto com seu token base se a animação parecer incompatível.

  * `claude` e `claudeShimmer`
  * `warning` e `warningShimmer`
  * `permission` e `permissionShimmer`
  * `promptBorder` e `promptBorderShimmer`
  * `inactive` e `inactiveShimmer`
  * `fastMode` e `fastModeShimmer`

  Cada [subagente](/docs/pt/sub-agents) e tarefa paralela é mostrado em uma das oito cores nomeadas para que você possa diferenciá-los na transcrição. Os nomes dos tokens seguem o padrão `<color>_FOR_SUBAGENTS_ONLY`, onde `<color>` é `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` ou `cyan`. Substitua estes para alterar a aparência de cada cor nomeada. Por exemplo, um subagente com `color: blue` em sua definição é desenhado usando o valor `blue_FOR_SUBAGENTS_ONLY`.

  Claude Code renderiza a palavra-chave [`ultrathink`](/docs/pt/model-config#use-ultrathink-for-one-off-deep-reasoning) na entrada do prompt com um gradiente arco-íris de sete cores. Os nomes dos tokens seguem o padrão `rainbow_<color>` e `rainbow_<color>_shimmer`, onde `<color>` é `red`, `orange`, `yellow`, `green`, `blue`, `indigo` ou `violet`.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Mudar para renderização em tela cheia
</h2>

No [modo leitor de tela](/docs/pt/accessibility), esta seção não se aplica. Claude Code sempre renderiza como texto simples com rolagem, exceto em [sessões em segundo plano](/docs/pt/agent-view) anexadas, e se você executar `/tui fullscreen` em qualquer outra sessão, Claude Code imprime uma explicação em vez de mudar.

Se a exibição piscar ou a posição de rolagem pular enquanto Claude está trabalhando, mude para o [modo de renderização em tela cheia](/docs/pt/fullscreen). Neste modo você rola com o mouse ou PageUp dentro de Claude Code em vez de usar a rolagem nativa do seu terminal; consulte a [página de tela cheia](/docs/pt/fullscreen#search-and-review-the-conversation) para saber como pesquisar e copiar.

Se o piscar for o único problema e seu terminal suportar saída sincronizada, mas não for detectado automaticamente, como o Emacs `eat`, defina [`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/pt/env-vars) para parar o piscar sem alterar renderizadores.

Execute `/tui fullscreen` para mudar e salvar a preferência. Sua conversa é relançada intacta e futuras sessões começam em tela cheia, a menos que uma [falha de início em tela cheia](/docs/pt/fullscreen#fullscreen-renderer-didnt-finish-starting) ocorra. Você também pode definir a variável de ambiente `CLAUDE_CODE_NO_FLICKER` antes de iniciar Claude Code:

<CodeGroup>
  ```bash Bash and Zsh theme={null}
  CLAUDE_CODE_NO_FLICKER=1 claude
  ```

  ```powershell PowerShell theme={null}
  $env:CLAUDE_CODE_NO_FLICKER = "1"; claude
  ```

  ```json ~/.claude/settings.json theme={null}
  {
    "env": {
      "CLAUDE_CODE_NO_FLICKER": "1"
    }
  }
  ```
</CodeGroup>

<h2 id="paste-large-content">
  Colar conteúdo grande
</h2>

Quando você cola mais de 800 caracteres ou mais de três linhas no prompt, Claude Code recolhe a entrada para um espaço reservado como `[Pasted text #1 +120 lines]` para que a caixa de entrada permaneça utilizável, e ainda envia o conteúdo completo quando você submete. Para entradas muito grandes, como arquivos inteiros ou logs longos, escreva o conteúdo em um arquivo e peça ao Claude para lê-lo em vez de colar. A transcrição da conversa permanece legível e Claude pode referenciar o arquivo por caminho em turnos posteriores. O terminal integrado do VS Code também pode descartar caracteres de colas muito grandes antes de chegarem ao Claude Code, então use um arquivo lá.

Se a colagem contiver [caracteres Unicode invisíveis](/docs/pt/interactive-mode#invisible-characters-in-prompts), Claude Code os remove quando você pressiona Enter e coloca o prompt limpo de volta na caixa de entrada para você enviar com outro Enter.

<h3 id="how-claude-treats-pasted-text">
  Como Claude trata texto colado
</h3>

Quando você submete, Claude vê o conteúdo atrás de cada espaço reservado `[Pasted text #N]` marcado como texto que você colou de outro lugar em vez de digitar. Claude é informado que uma colagem pode conter instruções que você não escreveu, e deve seguir instruções dentro dela apenas onde a mensagem que você digitou pede. Em sessões que não [buscam sinalizadores de recurso](/docs/pt/env-vars#features-that-need-feature-flag-fetching), colas não são marcadas.

<h3 id="delete-and-restore-a-collapsed-paste">
  Deletar e restaurar uma colagem recolhida
</h3>

Quando você deleta com um atalho de palavra ou linha como `Ctrl+W` ou `Ctrl+K`, ou com uma exclusão vim através de um movimento `f`/`t` como `df]`, e o intervalo deletado atinge dentro de um espaço reservado `[Pasted text #N]`, Claude Code remove o espaço reservado inteiro. Para restaurá-lo, cole a exclusão de volta com [`Ctrl+Y`](/docs/pt/interactive-mode#text-editing) após um atalho de palavra ou linha, ou com [`p` em NORMAL mode](/docs/pt/interactive-mode#editing-normal-mode) após uma exclusão vim.

<h3 id="recall-a-prompt-that-had-pasted-text">
  Recuperar um prompt que tinha texto colado
</h3>

Claude Code mantém o conteúdo atrás de cada espaço reservado `[Pasted text #N]` em `~/.claude/paste-cache/`, então quando você recupera um prompt do [histórico de comandos](/docs/pt/interactive-mode#command-history) e o resubmete, o conteúdo colado completo é enviado novamente, inclusive em uma sessão posterior.

Arquivos de cache com mais de [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays) de idade são deletados sob as [regras de varredura de retenção](/docs/pt/claude-directory#cleaned-up-automatically), então um prompt recuperado pode referenciar texto colado que não existe mais. Quando você submete tal prompt, Claude Code nunca envia a string literal `[Pasted text #N]`, e mostra uma notificação nomeando a colagem ausente:

* Em um prompt simples com texto restante, Claude Code remove o espaço reservado e envia o texto restante.
* Em um comando [shell mode](/docs/pt/interactive-mode#shell-mode-with-prefix) ou um comando `/`, onde a remoção alteraria o que é executado, e em qualquer prompt cuja remoção deixa vazio, Claude Code cancela a submissão e mantém o texto original na entrada, com o espaço reservado ainda nele. Delete o espaço reservado ou edite o comando, depois resubmeta.

<h2 id="edit-prompts-with-vim-keybindings">
  Editar prompts com atalhos de teclado Vim
</h2>

Claude Code inclui um modo de edição estilo Vim para a entrada de prompt. Ative-o através de `/config` → Editor mode, ou definindo [`editorMode`](/docs/pt/settings-reference#editormode) como `"vim"` em `~/.claude/settings.json`. Defina Editor mode de volta para `normal` para desativá-lo.

O modo Vim suporta um subconjunto de motions e operadores dos modos NORMAL e VISUAL, como navegação `hjkl`, seleção `v`/`V`, e `d`/`c`/`y` com objetos de texto. Consulte a [referência do modo editor Vim](/docs/pt/interactive-mode#vim-editor-mode) para a tabela de teclas completa.

Os motions do Vim não são remapeáveis através do arquivo de atalhos de teclado. Para mapear uma sequência de dois caracteres no modo INSERT, como `jj` para Escape, defina [`vimInsertModeRemaps`](/docs/pt/interactive-mode#remap-insert-mode-key-sequences) nas suas configurações de usuário.

Pressionar Enter ainda envia seu prompt no modo INSERT, diferentemente do Vim padrão. Use `o` ou `O` no modo NORMAL, ou Ctrl+J, para inserir uma nova linha.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Interactive mode](/docs/pt/interactive-mode): referência completa de atalhos de teclado e a tabela de teclas Vim
* [Keybindings](/docs/pt/keybindings): remapeie qualquer atalho do Claude Code, incluindo Enter e Shift+Enter
* [Fullscreen rendering](/docs/pt/fullscreen): detalhes sobre rolagem, pesquisa e cópia no modo tela cheia
* [Hooks guide](/docs/pt/hooks-guide): mais exemplos de hook de Notificação para Linux e Windows
* [Troubleshooting](/docs/pt/troubleshooting): correções para problemas fora da configuração do terminal
