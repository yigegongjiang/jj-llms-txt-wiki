> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Use Claude Code com um leitor de tela

> Configure Claude Code para leitores de tela como VoiceOver e NVDA, além de configurações para ampliadores de tela, movimento reduzido e temas amigáveis para daltônicos.

Claude Code possui um modo leitor de tela que substitui sua interface de terminal visual por texto simples e linear. Em vez de caixas, animações de progresso e redesenhos no local, Claude Code imprime linhas rotuladas que um leitor de tela como VoiceOver ou NVDA lê em ordem. Você pode manter uma conversa completa, aprovar permissões de ferramentas e revisar a saída de ponta a ponta.

O modo leitor de tela é opcional. Se você usar um ampliador de tela, movimento reduzido ou um tema amigável para daltônicos em vez de um leitor de tela, defina `CLAUDE_CODE_ACCESSIBILITY`, `prefersReducedMotion` ou `theme` a partir da tabela [Configurações de acessibilidade](#accessibility-settings). O modo leitor de tela adapta apenas a interface do terminal, portanto você não precisa dele no painel de chat da extensão VS Code. No Claude Code v2.1.236 ou posterior, a extensão [anuncia atividade de conversa para seu leitor de tela](/docs/pt/vs-code#use-a-screen-reader) lá sem nenhuma configuração.

<h2 id="turn-on-screen-reader-mode">
  Ativar o modo leitor de tela
</h2>

Escolha o método que corresponde à frequência com que você usa um leitor de tela:

* Para uma sessão: execute `claude --ax-screen-reader`.
* Para sessões iniciadas a partir de um shell: defina a variável de ambiente `CLAUDE_AX_SCREEN_READER` como `1`. Em Bash ou Zsh, execute `export CLAUDE_AX_SCREEN_READER=1`. Em PowerShell, execute `$env:CLAUDE_AX_SCREEN_READER = "1"`. Adicione essa linha ao seu perfil de shell para mantê-la para shells futuros.
* Para cada sessão na máquina: adicione `"axScreenReader": true` ao seu [arquivo de configurações](/docs/pt/settings). A configuração se aplica em qualquer terminal, incluindo o terminal integrado do VS Code.

Se você combinar métodos, Claude Code aplica a flag [`--ax-screen-reader`](/docs/pt/cli-reference#cli-flags) sobre a variável de ambiente [`CLAUDE_AX_SCREEN_READER`](/docs/pt/env-vars#variables), e a variável sobre a configuração [`axScreenReader`](/docs/pt/settings-reference#axscreenreader).

Se você usar Claude Code via SSH, defina a variável de ambiente ou configuração na máquina remota onde Claude Code é executado.

A primeira linha que Claude Code imprime confirma o modo: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]`, ou `[Screen Reader Mode: on via settings]`.

<h2 id="turn-off-screen-reader-mode">
  Desativar o modo leitor de tela
</h2>

Reverta o método que ativou o modo: inicie sem a flag, desdefina a variável de ambiente ou defina `axScreenReader` como `false`. Se você definir `CLAUDE_AX_SCREEN_READER` como `0`, Claude Code mantém o modo desativado mesmo quando a configuração é `true`.

<h2 id="accessibility-settings">
  Configurações de acessibilidade
</h2>

A tabela lista cada opção de acessibilidade, se você a define como um sinalizador, uma variável de ambiente ou uma configuração, e o que ela altera.

| Opção                                                                   | Tipo                 | O que altera                                                                                                                                                                                                                                                     |
| :---------------------------------------------------------------------- | :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`--ax-screen-reader`](/docs/pt/cli-reference#cli-flags)                     | Sinalizador          | Modo leitor de tela para uma sessão.                                                                                                                                                                                                                             |
| [`CLAUDE_AX_SCREEN_READER`](/docs/pt/env-vars#variables)                     | Variável de ambiente | Modo leitor de tela para sessões iniciadas a partir do shell onde você a define.                                                                                                                                                                                 |
| [`axScreenReader`](/docs/pt/settings-reference#axscreenreader)               | Configuração         | Modo leitor de tela para cada sessão quando `true`.                                                                                                                                                                                                              |
| [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/pt/env-vars#variables)                  | Variável de ambiente | Quanto tempo Claude Code aguarda após a linha de confirmação antes de desenhar o primeiro prompt no modo leitor de tela. Requer Claude Code v2.1.217 ou posterior.                                                                                               |
| [`CLAUDE_AX_PREPARK_MS`](/docs/pt/env-vars#variables)                        | Variável de ambiente | Quanto tempo Claude Code aguarda, com o cursor no início da linha, antes de escrever uma linha nova ou alterada no modo leitor de tela. Requer Claude Code v2.1.233 ou posterior.                                                                                |
| [`CLAUDE_CODE_ACCESSIBILITY`](/docs/pt/env-vars#variables)                   | Variável de ambiente | Um cursor de terminal que permanece visível para ampliadores de tela como macOS Zoom quando você o define como `1`. O cursor segue o cursor de entrada e, no Claude Code v2.1.218 ou posterior, a linha destacada em menus e painéis como `/config` e `/plugin`. |
| [`prefersReducedMotion`](/docs/pt/settings-reference#prefersreducedmotion)   | Configuração         | Spinners reduzidos ou sem spinners, shimmer e outras animações quando `true`.                                                                                                                                                                                    |
| [`theme`](/docs/pt/settings-reference#theme)                                 | Configuração         | As cores da interface, incluindo os temas amigáveis para daltônicos `dark-daltonized` e `light-daltonized`. Você também pode escolher um com [`/theme`](/docs/pt/commands#all-commands).                                                                              |
| [`preferredNotifChannel`](/docs/pt/settings-reference#preferrednotifchannel) | Configuração         | Com o valor `"terminal_bell"`, um sino de terminal fora do modo leitor de tela quando Claude está aguardando você.                                                                                                                                               |

<h2 id="what-your-screen-reader-hears">
  O que seu leitor de tela ouve
</h2>

No modo leitor de tela, Claude Code escreve texto simples:

* Sem caracteres de desenho de caixa para a interface
* Sem pistas apenas de cor
* Sem redesenhos de conteúdo que não mudou. Spinners de progresso são renderizados como texto estático
* Tabelas nas respostas do Claude são lidas como sentenças `Header: value` em vez de uma grade com caracteres de caixa

Claude Code deixa tudo que imprime no scrollback do seu terminal, para que você possa reler turnos anteriores com os comandos de revisão do seu leitor de tela ou a busca do seu terminal. Claude Code ignora a configuração [`tui`](/docs/pt/settings-reference#tui) no modo leitor de tela. Além das sessões em background anexadas listadas em [Limitações conhecidas](#known-limitations), ele imprime texto rolável em vez de [renderização em tela cheia](/docs/pt/fullscreen).

Claude Code também aguarda em dois pontos para que seu leitor de tela possa acompanhar:

* Depois que Claude Code imprime a linha de confirmação, ele aguarda 3 segundos antes de desenhar o prompt, para que seu leitor de tela possa terminar a linha. Pressione qualquer tecla para encerrar a espera. Para alterar o comprimento da espera, defina [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/pt/env-vars#variables).
* Antes de Claude Code escrever uma linha nova ou alterada, como uma dica ou mais da resposta do Claude, ele move o cursor para o início da linha e aguarda 50 milissegundos. Seu leitor de tela então lê a linha a partir de seu primeiro caractere. Caracteres que você digita ou deleta no final da linha de entrada aparecem imediatamente. Para alterar o comprimento da espera, defina [`CLAUDE_AX_PREPARK_MS`](/docs/pt/env-vars#variables).

Cada mensagem na transcrição começa com um rótulo que seu leitor de tela anuncia, nomeando o que é: suas mensagens, respostas e pensamentos do Claude, atividade de ferramentas, erros e avisos, e prompts. Os rótulos também são pesquisáveis, para que você possa pular entre seções da transcrição pesquisando o scrollback do seu terminal:

| Rótulo                 | Significado                                                                                 |
| :--------------------- | :------------------------------------------------------------------------------------------ |
| `you:`                 | Suas mensagens                                                                              |
| `claude:`              | Respostas do Claude                                                                         |
| `thinking:`            | Pensamento do Claude                                                                        |
| `tool:`                | Atividade de ferramentas, como uma edição de arquivo ou um comando executado                |
| `tool error:`          | Uma ferramenta que falhou                                                                   |
| `error:`               | Um erro na conversa, como uma solicitação de API falhada                                    |
| `warning:`             | Um aviso do Claude Code, como uma mudança para um modelo alternativo                        |
| `Permission Required:` | Um prompt de permissão aguardando sua resposta                                              |
| `Cost:`                | O resumo de custo da sessão quando Claude Code sai, se sua conta [mostra custos](/docs/pt/costs) |

Claude Code mantém o cursor do terminal no acento de entrada, para que o comando de leitura de linha atual do seu leitor de tela leia o prompt que você está editando.

Conforme você digita no final da linha de entrada, ou pressiona `Backspace` lá, Claude Code escreve apenas os caracteres que mudam. Seu leitor de tela ecoa apenas esses caracteres.

Quando você deleta uma palavra ou uma linha com um dos [atalhos de edição de texto](/docs/pt/interactive-mode#text-editing), Claude Code anuncia o texto deletado:

* Deletando palavras com `Ctrl+W` ou `Alt+D`, ou com `Option+Delete` no macOS ou `Ctrl+Backspace` no Windows
* Deletando até o início da linha com `Ctrl+U` ou `Cmd+Backspace`
* Deletando até o final da linha com `Ctrl+K`

Quando você alterna [modos de permissão](/docs/pt/permission-modes) com `Shift+Tab`, Claude Code anuncia o modo de permissão em que você chega, como `[plan mode on]` ou `[accept edits on]`. Claude Code imprime o anúncio uma vez e não o repete em redesenhos posteriores.

<h3 id="jump-between-turns">
  Pular entre turnos
</h3>

Claude Code emite marcadores de integração de shell OSC 133 nos limites de turno, para que a tecla de pulo para prompt anterior do seu terminal se mova entre turnos sem ler toda a transcrição:

* iTerm2: Cmd+Shift+Up
* Terminal VS Code: Ctrl+Up no Windows, Cmd+Up no macOS
* Windows Terminal: nenhuma tecla por padrão; vincule a ação `scrollToMark` em suas configurações
* Kitty e Ghostty: verifique a documentação do terminal para sua tecla de pulo para prompt

macOS Terminal não age nos marcadores, e Claude Code não os emite no WezTerm. Nesses terminais, pesquise o scrollback pelo rótulo `you:` em vez disso.

<h2 id="answer-menus-and-prompts">
  Responda a menus e prompts
</h2>

No modo leitor de tela, menus que você normalmente navegaria com as teclas de seta, incluindo prompts de permissão, tornam-se listas numeradas. Claude Code anuncia cada opção como uma linha numerada, seguida por um prompt `Enter selection` que nomeia o intervalo válido. Digite o número da opção que deseja e pressione Enter.

* Pressione Escape para cancelar um menu cujo prompt termina com `or Escape to cancel`.
* Se você digitar um número que não está na lista, Claude Code anuncia o intervalo válido e permite que você tente novamente.

O seletor [`/effort`](/docs/pt/model-config#adjust-effort-level), que é um controle deslizante fora do modo leitor de tela, torna-se o mesmo tipo de lista numerada.

Os prompts sim-ou-não pedem uma resposta digitada em vez de um menu de duas opções. Responda `y` ou `n` e pressione Enter. `yes` e `no` também funcionam.

<h2 id="hear-when-claude-code-needs-you">
  Ouça quando Claude Code precisa de você
</h2>

No modo leitor de tela, Claude Code toca o sino do terminal quando precisa de sua atenção, para que você não tenha que ficar verificando a transcrição. O sino toca quando:

* Claude termina uma resposta
* Um prompt ou diálogo precisa de sua resposta, como um prompt de permissão
* Uma ferramenta que foi executada por mais de 5 segundos termina

O sino é o alerta padrão do seu terminal. Para silenciá-lo, altere a configuração de sino no seu aplicativo de terminal. Fora do modo leitor de tela, defina [`preferredNotifChannel`](/docs/pt/settings-reference#preferrednotifchannel) como `"terminal_bell"` para obter um [sino semelhante](/docs/pt/terminal-config#get-a-terminal-bell-or-notification) quando Claude estiver aguardando você.

<h2 id="known-limitations">
  Limitações conhecidas
</h2>

Alguns comportamentos não são adaptados para o modo leitor de tela:

* O modo leitor de tela não é ativado automaticamente quando um leitor de tela está em execução.
* Claude Code não anuncia uma mudança de modo de permissão feita de qualquer forma diferente de ciclar com `Shift+Tab`, como entrar em [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode) a partir de um comando.
* Anexar a uma [sessão de fundo](/docs/pt/agent-view) com `claude attach` ou da visualização de agente entra na tela alternativa do terminal, que não tem scrollback nativo. Este é o [mesmo comportamento que outras sessões anexadas](/docs/pt/fullscreen). Para sair, pressione Left Arrow em um prompt vazio, ou Ctrl+Z se um diálogo tiver foco.
* Claude Code anuncia custos no resumo que imprime na saída, não por turno.
* O modo leitor de tela não altera [modo não interativo](/docs/pt/headless) com a flag `-p`. O modo não interativo já escreve texto simples e permanece uma alternativa para scripts.

<h2 id="report-an-issue">
  Relatar um problema
</h2>

Se algo não funcionar com seu leitor de tela, ampliador ou terminal, abra um problema no [rastreador de problemas do Claude Code](https://github.com/anthropics/claude-code/issues) e mencione sua tecnologia assistiva no título. Inclua seu sistema operacional, aplicativo de terminal e nome e versão da tecnologia assistiva no relatório.
