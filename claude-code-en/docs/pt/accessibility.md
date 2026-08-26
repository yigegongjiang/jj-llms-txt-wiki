> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Use Claude Code com um leitor de tela

> Configure Claude Code para leitores de tela como VoiceOver e NVDA, além de configurações para ampliadores de tela, movimento reduzido e temas amigáveis para daltônicos.

Claude Code possui um modo leitor de tela que substitui sua interface de terminal visual por texto simples e linear. Em vez de caixas, animações de progresso e redesenhos no local, o modo imprime linhas rotuladas que um leitor de tela como VoiceOver ou NVDA lê em ordem, para que você possa manter uma conversa completa, aprovar permissões de ferramentas e revisar a saída de ponta a ponta.

O modo leitor de tela é opcional. Se você usar um ampliador de tela, movimento reduzido ou um tema amigável para daltônicos em vez de um leitor de tela, consulte [Configurações de acessibilidade além do modo leitor de tela](#accessibility-settings-beyond-screen-reader-mode).

<Note>
  O modo leitor de tela requer Claude Code v2.1.181 ou posterior. Versões anteriores rejeitam a flag `--ax-screen-reader` com `error: unknown option '--ax-screen-reader'`.
</Note>

<h2 id="turn-on-screen-reader-mode">
  Ativar o modo leitor de tela
</h2>

Escolha o método que corresponde à frequência com que você usa um leitor de tela:

* Para uma sessão: execute `claude --ax-screen-reader`.
* Para sessões iniciadas a partir de um shell: defina a variável de ambiente `CLAUDE_AX_SCREEN_READER` como `1`. Em Bash ou Zsh, execute `export CLAUDE_AX_SCREEN_READER=1`; em PowerShell, execute `$env:CLAUDE_AX_SCREEN_READER = "1"`. Adicione a linha ao seu perfil de shell para cobrir todos os shells.
* Para cada sessão na máquina: adicione `"axScreenReader": true` ao seu [arquivo de configurações](/docs/pt/settings). Isso cobre qualquer terminal, incluindo o terminal integrado do VS Code.

<Note>
  Os métodos são listados em ordem de precedência: a flag [`--ax-screen-reader`](/docs/pt/cli-reference#cli-flags) substitui a variável de ambiente [`CLAUDE_AX_SCREEN_READER`](/docs/pt/env-vars), que substitui a configuração [`axScreenReader`](/docs/pt/settings#available-settings).
</Note>

Se você usar Claude Code via SSH, defina a variável de ambiente ou configuração na máquina remota onde Claude Code é executado.

Quando o modo está ativado, a primeira coisa que Claude Code imprime é uma linha de confirmação nomeando o método que o ativou: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]`, ou `[Screen Reader Mode: on via settings]`. O formato de nomeação de método requer Claude Code v2.1.206 ou posterior. Quando Claude Code se reinicia, por exemplo para terminar de instalar uma atualização, o novo processo herda o modo através da variável de ambiente `CLAUDE_AX_SCREEN_READER`, então sua linha de confirmação lê `[Screen Reader Mode: on via env]` independentemente de qual método você usou.
Versões anteriores imprimem `[Accessible screen reader mode: on]`.

<h2 id="turn-off-screen-reader-mode">
  Desativar o modo leitor de tela
</h2>

Reverta o método que ativou o modo: inicie sem a flag, desdefina a variável de ambiente ou defina `axScreenReader` como `false`. Definir `CLAUDE_AX_SCREEN_READER=0` mantém o modo desativado mesmo quando a configuração é `true`.

<h2 id="what-your-screen-reader-hears">
  O que seu leitor de tela ouve
</h2>

No modo leitor de tela, Claude Code escreve texto simples:

* sem caracteres de desenho de caixa para o chrome da interface
* sem pistas apenas de cor
* sem redesenhos de conteúdo que não mudou; spinners de progresso são renderizados como texto estático
* tabelas nas respostas de Claude são lidas como sentenças `Header: value` em vez de uma grade de caracteres de caixa. Requer Claude Code v2.1.198 ou posterior; versões anteriores desenham tabelas como grades mesmo no modo leitor de tela.

A saída se acumula no scrollback do seu terminal, para que você possa reler turnos anteriores com os comandos de revisão do seu leitor de tela ou a busca do seu terminal.

O modo leitor de tela é renderizado como texto simples em rolagem, mesmo se você ativou [renderização em tela cheia](/docs/pt/fullscreen) com a [configuração `tui`](/docs/pt/settings#available-settings); a configuração não tem efeito enquanto o modo está ativo. Sessões de fundo anexadas ainda são renderizadas em tela cheia; consulte [Limitações conhecidas](#known-limitations).

Cada mensagem na transcrição começa com um rótulo que seu leitor de tela anuncia, nomeando o que é: suas mensagens, respostas de Claude, atividade de ferramentas, erros e prompts. Os rótulos também são pesquisáveis, para que você possa pular entre seções da transcrição pesquisando o scrollback do seu terminal:

| Rótulo                 | Significado                                                                                 |
| :--------------------- | :------------------------------------------------------------------------------------------ |
| `you:`                 | Suas mensagens                                                                              |
| `claude:`              | Respostas de Claude                                                                         |
| `tool:`                | Atividade de ferramentas, como uma edição de arquivo ou um comando executado                |
| `tool error:`          | Uma ferramenta que falhou                                                                   |
| `error:`               | Um erro na conversa, como uma solicitação de API com falha                                  |
| `Permission Required:` | Um prompt de permissão aguardando sua resposta                                              |
| `Cost:`                | O resumo de custo da sessão quando Claude Code sai, se sua conta [mostra custos](/docs/pt/costs) |

O cursor do terminal segue o cursor de entrada, para que o comando de leitura de linha atual do seu leitor de tela responda "onde estou" com o prompt que você está editando.

<h3 id="jump-between-turns">
  Pular entre turnos
</h3>

Claude Code emite marcadores de integração de shell OSC 133 nos limites de turno, para que a tecla de pulo para prompt anterior do seu terminal se mova entre turnos sem ler toda a transcrição:

* iTerm2: Cmd+Shift+Up
* Terminal VS Code: Ctrl+Up no Windows, Cmd+Up no macOS
* Windows Terminal: nenhuma tecla por padrão; vincule a ação `scrollToMark` em suas configurações
* Kitty e Ghostty: consulte a documentação do terminal para sua tecla de pulo para prompt

macOS Terminal não atua nos marcadores, e Claude Code não os emite em WezTerm. Nesses terminais, pesquise o scrollback pelo rótulo `you:` em vez disso.

<h2 id="answer-menus-and-prompts">
  Responder menus e prompts
</h2>

No modo leitor de tela, menus que você normalmente navegaria com as teclas de seta, incluindo prompts de permissão, se tornam listas numeradas. Cada opção é anunciada como uma linha numerada, seguida por um prompt `Enter selection` que nomeia o intervalo válido. Digite o número da opção que deseja e pressione Enter.

* Para cancelar um menu dispensável: pressione Escape. Seu prompt termina com `or Escape to cancel`.
* Se você digitar um número que não está na lista: Claude Code anuncia o intervalo válido e permite que você tente novamente.

Prompts sim-ou-não pedem uma resposta digitada em vez de um menu de duas opções. Responda `y` ou `n` e pressione Enter. `yes` e `no` também funcionam.

<h2 id="hear-when-claude-code-needs-you">
  Ouça quando Claude Code precisa de você
</h2>

No modo leitor de tela, Claude Code toca o sino do terminal quando precisa de sua atenção, para que você não tenha que ficar verificando a transcrição. O sino toca quando:

* Claude termina uma resposta
* um prompt de permissão aparece
* uma ferramenta que foi executada por mais de 5 segundos termina

O sino é o alerta padrão do seu terminal. Para silenciá-lo, altere a configuração de sino no seu aplicativo de terminal. O sino não requer modo leitor de tela: fora do modo, defina [`preferredNotifChannel`](/docs/pt/settings#available-settings) como `"terminal_bell"` para alertas semelhantes quando Claude está esperando por você. Consulte [Obter um sino de terminal ou notificação](/docs/pt/terminal-config#get-a-terminal-bell-or-notification).

<h2 id="accessibility-settings-beyond-screen-reader-mode">
  Configurações de acessibilidade além do modo leitor de tela
</h2>

Essas opções abordam necessidades de acessibilidade fora do modo leitor de tela. Todas elas funcionam junto com ele.

* A [variável de ambiente](/docs/pt/env-vars) `CLAUDE_CODE_ACCESSIBILITY` é para ampliadores de tela. Defina `CLAUDE_CODE_ACCESSIBILITY=1` para manter o cursor de terminal nativo visível para que ampliadores, como Zoom do macOS, possam rastrear a posição do cursor.
* A [configuração](/docs/pt/settings#available-settings) `prefersReducedMotion` reduz ou desabilita spinners, shimmer e outras animações sem alterar o resto da interface.
* A [configuração](/docs/pt/settings#available-settings) `theme` seleciona as cores da interface, incluindo os temas amigáveis para daltônicos `dark-daltonized` e `light-daltonized`.

<h2 id="known-limitations">
  Limitações conhecidas
</h2>

Alguns comportamentos não são adaptados para o modo leitor de tela:

* O modo leitor de tela não é ativado automaticamente quando um leitor de tela está em execução.
* Mudanças de modo, como entrar em [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode), ainda não são anunciadas.
* Anexar a uma [sessão de fundo](/docs/pt/agent-view) com `claude attach` ou da visualização de agente entra na tela alternativa do terminal, que não tem scrollback nativo. Este é o [mesmo comportamento que outras sessões anexadas](/docs/pt/fullscreen). Para sair, pressione Left Arrow em um prompt vazio, ou Ctrl+Z se um diálogo tiver foco.
* Claude Code anuncia custos no resumo que imprime na saída, não por turno.
* O modo leitor de tela não altera [modo não interativo](/docs/pt/headless) com a flag `-p`. O modo não interativo já escreve texto simples e permanece uma alternativa para scripts.

<h2 id="report-an-issue">
  Relatar um problema
</h2>

Se algo não funcionar com seu leitor de tela, ampliador ou terminal, abra um problema no [rastreador de problemas do Claude Code](https://github.com/anthropics/claude-code/issues) e mencione sua tecnologia assistiva no título. Inclua seu sistema operacional, aplicativo de terminal e nome e versão da tecnologia assistiva no relatório.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Essas páginas contêm as entradas de referência completa e a configuração relacionada para o que esta página cobre:

* [Settings](/docs/pt/settings#available-settings): as entradas `axScreenReader`, `prefersReducedMotion`, `theme` e `preferredNotifChannel`
* [Environment variables](/docs/pt/env-vars): as entradas `CLAUDE_AX_SCREEN_READER` e `CLAUDE_CODE_ACCESSIBILITY`
* [CLI reference](/docs/pt/cli-reference#cli-flags): a flag `--ax-screen-reader`
* [Terminal configuration](/docs/pt/terminal-config): sinos, notificações e temas fora do modo leitor de tela
* [Non-interactive mode](/docs/pt/headless): execuções de `claude -p` com script, que escrevem texto simples sem modo leitor de tela
