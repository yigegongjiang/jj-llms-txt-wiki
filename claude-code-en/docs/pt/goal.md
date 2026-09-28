> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Manter Claude trabalhando em direção a um objetivo

> Defina uma condição de conclusão com /goal e Claude continua trabalhando até que seja atendida, um modelo a julgue impossível ou um erro que você tenha que corrigir limpe o objetivo.

O comando `/goal` define uma condição de conclusão e Claude continua trabalhando em direção a ela sem que você solicite cada etapa. Após cada turno, um modelo pequeno e rápido verifica se a condição é atendida. Se o modelo julgar que ainda não foi atendida, Claude inicia outro turno em vez de devolver o controle a você. O objetivo é limpo automaticamente assim que a condição é atendida, se o modelo julgar a condição impossível de satisfazer, ou se um turno falhar em [um erro que você tenha que corrigir](#errors-you-have-to-fix-clear-the-goal).

Use um objetivo para trabalho substancial com um estado final verificável:

* Migrar um módulo para uma nova API até que cada site de chamada compile e os testes passem
* Implementar um documento de design até que todos os critérios de aceitação sejam atendidos
* Dividir um arquivo grande em módulos focados até que cada um esteja dentro de um orçamento de tamanho
* Trabalhar através de uma fila de problemas rotulados até que a fila esteja vazia

<h2 id="compare-ways-to-keep-a-session-running">
  Comparar formas de manter uma sessão em execução
</h2>

Três abordagens mantêm a sessão atual em execução entre prompts. Escolha com base no que deve iniciar o próximo turno:

| Abordagem                                                           | Próximo turno começa quando                                                                                                                                                                             | Para quando                                                                                                                                                                                                        |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/goal`                                                             | O turno anterior termina, ou, em uma sessão interativa, uma [verificação de inatividade](#background-work-defers-evaluation) ou uma [tentativa automática](#other-errors-retry-or-pause-the-goal) vence | Um modelo confirma que a condição é atendida ou julga impossível, ou um turno falha em [um erro que você tem que corrigir](#errors-you-have-to-fix-clear-the-goal), ou você executa [`/goal clear`](#clear-a-goal) |
| [`/loop`](/docs/pt/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Um intervalo de tempo decorre                                                                                                                                                                           | Você o interrompe, ou Claude decide que o trabalho está concluído                                                                                                                                                  |
| [Stop hook](/docs/pt/hooks-guide#prompt-based-hooks)                     | O turno anterior termina                                                                                                                                                                                | Seu próprio script ou prompt decide                                                                                                                                                                                |

`/goal` e um Stop hook são acionados após cada turno. `/goal` é um atalho com escopo de sessão: você digita uma condição e ela fica ativa apenas para a sessão atual. Um Stop hook reside em seu arquivo de configurações, se aplica a cada sessão em seu escopo e pode executar um script para verificações determinísticas ou um prompt para avaliações baseadas em modelo.

[Modo automático](/docs/pt/auto-mode-config) por si só aprova chamadas de ferramentas dentro de um único turno, mas não inicia um novo. Claude para quando julga o trabalho concluído. `/goal` adiciona um avaliador separado que verifica sua condição após cada turno, portanto a conclusão é decidida por um modelo novo em vez daquele que está fazendo o trabalho. Os dois são complementares: o modo automático remove prompts por ferramenta e `/goal` remove prompts por turno.

<Tip>
  As abordagens acima mantêm a sessão atual em execução. Você também pode agendar trabalho que seja executado independentemente de qualquer sessão aberta, como testes noturnos ou triagem matinal. Consulte [opções de agendamento](/docs/pt/scheduled-tasks#compare-scheduling-options) para rotinas em nuvem e tarefas agendadas de desktop.
</Tip>

<h2 id="use-/goal">
  Usar `/goal`
</h2>

Um objetivo pode estar ativo por sessão. O mesmo comando define, verifica e limpa dependendo do argumento.

<h3 id="set-a-goal">
  Definir um objetivo
</h3>

Execute `/goal` seguido pela condição que você deseja satisfazer. Se um objetivo já estiver ativo, o novo o substitui.

```text theme={null}
/goal all tests in test/auth pass and the lint step is clean
```

Definir um objetivo inicia um turno imediatamente, com a própria condição como diretiva. Você não precisa enviar um prompt separado. Enquanto o objetivo está ativo, um indicador `◎ /goal active` mostra há quanto tempo o objetivo está em execução.

Um objetivo não altera seu modo de permissão. Para permitir que turnos de objetivo sejam executados sem supervisão, execute `/goal` em [modo automático](/docs/pt/auto-mode-config). Em [modo Manual](/docs/pt/permission-modes), Claude ainda solicita antes de chamadas de ferramentas que suas configurações ainda não permitem, como o comando de teste acima.

Enquanto o objetivo está ativo, a transcrição mostra cada veredicto que o avaliador retorna, e você pode pressionar Ctrl+O para ver o motivo por trás dele. A visualização de status também mostra o motivo mais recente, para que você possa ver para o que Claude está trabalhando a seguir.

<h3 id="write-an-effective-condition">
  Escrever uma condição eficaz
</h3>

O [avaliador](#how-evaluation-works) julga sua condição em relação ao que Claude apresentou na conversa. Ele não executa comandos ou lê arquivos independentemente, portanto escreva a condição como algo que a própria saída de Claude possa demonstrar. "Todos os testes em `test/auth` passam" funciona porque Claude executa os testes e o resultado aparece na transcrição para o avaliador ler.

Uma condição que se mantém em muitos turnos geralmente tem:

* **Um estado final mensurável**: um resultado de teste, um código de saída de compilação, uma contagem de arquivos, uma fila vazia
* **Uma verificação declarada**: como Claude deve provar isso, como "`npm test` sai com 0" ou "`git status` está limpo"
* **Restrições que importam**: qualquer coisa que não deve mudar no caminho, como "nenhum outro arquivo de teste é modificado"

A condição pode ter até 4.000 caracteres.

Para limitar quanto tempo um objetivo é executado, inclua uma cláusula de turno ou tempo na condição, como `or stop after 20 turns`. Claude relata progresso em relação a essa cláusula a cada turno e o avaliador a julga a partir da conversa.

<h3 id="check-status">
  Verificar status
</h3>

Execute `/goal` sem argumentos para ver o estado atual.

```text theme={null}
/goal
```

Se um objetivo está ativo, o status mostra:

* A condição
* Há quanto tempo está em execução
* Quantos turnos foram avaliados
* O gasto de token atual
* A razão mais recente do avaliador

O número de turnos e a razão mais recente aparecem após a primeira avaliação ter sido executada.

Se nenhum objetivo está ativo, mas um foi alcançado anteriormente na sessão, o status mostra a condição alcançada junto com sua duração, contagem de turnos e gasto de token.

<h3 id="clear-a-goal">
  Limpar um objetivo
</h3>

Execute `/goal clear` para remover um objetivo ativo antes que ele seja resolvido.

```text theme={null}
/goal clear
```

Claude imprime `Goal cleared:` seguido pela condição para confirmar, ou `No goal set` se nada estava ativo.

`stop`, `off`, `reset`, `none` e `cancel` são aceitos como aliases para `clear`. Executar `/clear` para iniciar uma nova conversa também remove qualquer objetivo ativo.

<h3 id="resume-with-an-active-goal">
  Retomar com um objetivo ativo
</h3>

Quando você retoma uma sessão, Claude Code restaura um objetivo que ainda estava ativo quando a sessão terminou. Claude Code o restaura em cada rota de retomada: `--continue`, `--resume` com um ID de sessão, nome ou [caminho do arquivo de transcrição](/docs/pt/sessions#resume-a-session), e o [seletor de sessão](/docs/pt/sessions#use-the-session-picker). Antes da v2.1.239, Claude Code restaurava o objetivo em cada rota, exceto no seletor `claude --resume`.

Claude Code mantém a condição, mas redefine a contagem de turnos, cronômetro e linha de base de gasto de token. Ele não restaura um objetivo que já foi alcançado ou limpo.

<h3 id="run-non-interactively">
  Executar de forma não interativa
</h3>

`/goal` funciona em [modo não interativo](/docs/pt/headless), no [aplicativo desktop](/docs/pt/desktop) e através de [Remote Control](/docs/pt/remote-control). Definir um objetivo com `-p` executa o loop até a conclusão em uma única invocação:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Com a saída de texto padrão, nada é impresso até que a execução termine, portanto um objetivo que executa muitos turnos pode parecer travado. Adicione `--output-format stream-json --verbose` para emitir cada mensagem conforme o loop é executado.

Interrompa o processo com Ctrl+C para parar um objetivo não interativo antes que ele seja resolvido.

<h2 id="how-evaluation-works">
  Como a avaliação funciona
</h2>

`/goal` é um wrapper em torno de um [Stop hook baseado em prompt](/docs/pt/hooks#prompt-based-hooks) com escopo de sessão. Cada vez que Claude termina um turno, Claude Code envia a condição e a conversa até agora para seu [modelo pequeno e rápido](/docs/pt/model-config) configurado, que é padronizado para Haiku na Claude API; em um provedor de terceiros, verifique sua [página do provedor](/docs/pt/third-party-integrations) para o padrão da plataforma. O modelo retorna um de três vereditos, cada um com uma breve razão:

* **Ainda não atendido**: Claude continua trabalhando e toma a razão como orientação para o próximo turno.
* **Atendido**: Claude Code limpa o objetivo e registra uma entrada alcançada na transcrição.
* **Impossível**: o avaliador julgou que a condição nunca pode ser satisfeita. Claude Code limpa o objetivo e registra uma entrada falhada na transcrição junto com a razão. Você não precisa limpá-lo você mesmo.

Se Claude continuar respondendo ao avaliador sem fazer progresso (nenhum uso de ferramenta por vários turnos seguidos), Claude Code para o loop, imprime um aviso e retorna o controle para você com o objetivo ainda definido. A avaliação retoma após seu próximo prompt. O [guia de hooks](/docs/pt/hooks-guide#stop-hook-hits-the-block-cap) explica o mecanismo subjacente.

<h3 id="when-a-turn-fails">
  Quando um turno falha
</h3>

Quando um turno falha, Claude Code limpa o objetivo se o erro for um que você precisa corrigir. Após qualquer outro erro, o objetivo permanece definido.

<h4 id="errors-you-have-to-fix-clear-the-goal">
  Erros que você precisa corrigir limpam o objetivo
</h4>

Se um turno falhar em um erro que não será limpo até você corrigi-lo, Claude Code limpa o objetivo e imprime um aviso nomeando a causa. O aviso começa com `Goal cleared after an unrecoverable error` e termina com `Run /goal again to continue`. Corrija a causa, então [defina o objetivo novamente](#set-a-goal) com `/goal <condition>`. Quatro tipos de falha limpam o objetivo:

* Uma falha de autenticação, quando Claude Code gerencia suas próprias credenciais. Quando um host as gerencia para você, como o aplicativo desktop, a extensão VS Code ou uma [sessão na nuvem](/docs/pt/claude-code-on-the-web), Claude Code deixa o objetivo ativo porque o host restaura o acesso por conta própria.
* Um saldo de crédito esgotado
* Um overflow de contexto que [auto-compactação](/docs/pt/model-config#set-the-auto-compact-window) não conseguiu limpar
* Um modelo que não está disponível

<h4 id="other-errors-retry-or-pause-the-goal">
  Outros erros tentam novamente ou pausam o objetivo
</h4>

Após qualquer outra falha, o objetivo permanece definido. Em uma sessão interativa no Claude Code v2.1.269 ou posterior, Claude Code também imprime uma linha nomeando a causa e tenta novamente por conta própria ou aguarda você:

* **Tentar novamente**: após uma falha que tende a se resolver por conta própria, como um servidor sobrecarregado ou uma conexão perdida, um aviso começando com `Goal still active` mostra a espera antes da próxima tentativa. Após três tentativas automáticas, o objetivo pausa em vez disso.
* **Pausar**: após uma falha que uma tentativa apenas repetiria, como um limite de taxa de API, um [limite de uso](/docs/pt/errors#youve-hit-your-session-limit) do claude.ai ou um hook que encerrou o turno, um aviso começando com `Goal paused` nomeia a causa. Se a sessão está [aguardando para continuar automaticamente quando um limite de uso é redefinido](/docs/pt/interactive-mode#wait-for-a-usage-limit-to-reset), Claude retoma o trabalho em direção ao objetivo então.

Envie uma mensagem a qualquer momento para iniciar o próximo turno imediatamente. Para desativar tentativas automáticas, defina [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/pt/env-vars) como `0`, o que também desativa [verificações](#background-work-defers-evaluation).

<h3 id="background-work-defers-evaluation">
  Trabalho em segundo plano adia a avaliação
</h3>

Se um subagente ou um comando shell em segundo plano ainda estiver em execução quando um turno termina, Claude Code pula a avaliação para esse turno. Ele avalia no final do próximo turno que termina sem nenhum trabalho em segundo plano em execução. Quando o trabalho em segundo plano termina, Claude Code entrega o resultado para Claude como um novo turno, então você não precisa fazer um prompt.

Uma vez que o trabalho em segundo plano mantém o objetivo esperando por 30 minutos, uma verificação é devida. Na verificação, Claude Code lista as tarefas em execução e pede a Claude para ler sua saída, continuar esperando se estiverem progredindo e corrigir ou parar qualquer uma que esteja travada. Após a primeira verificação, Claude Code espera o dobro do tempo antes de cada verificação posterior, até quatro vezes o primeiro intervalo: com o padrão, 1 hora após a primeira verificação, depois a cada 2 horas. Claude Code entrega uma verificação devida, a primeira incluída, de uma de duas maneiras:

* **Quando um turno termina**: Claude Code entrega a verificação no final do próximo turno que termina com o trabalho ainda em execução. Em uma sessão não interativa, como uma iniciada com `-p`, esta é a única maneira que Claude Code entrega verificações.
* **Enquanto a sessão está ociosa**: em uma sessão interativa, Claude Code também inicia um turno por conta própria para entregar a verificação em vez de esperar pelo seu próximo prompt. Se o trabalho em segundo plano parou sem relatar um resultado, Claude Code pede a Claude para continuar em direção ao objetivo. Claude Code inicia no máximo três verificações ociosas por objetivo entre seus prompts. Na terceira verificação ociosa, Claude Code diz que as verificações ociosas estão pausadas até você enviar outro prompt. Antes da v2.1.246, as verificações ociosas eram ilimitadas. As verificações ociosas requerem Claude Code v2.1.236 ou posterior.

Antes da v2.1.239, apenas as verificações ociosas recuavam dessa forma; uma verificação entregue no final de um turno recorria no primeiro intervalo.

Para alterar o primeiro intervalo, defina [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/pt/env-vars). Claude Code usa seu valor no lugar do intervalo de 30 minutos e dimensiona os intervalos posteriores com ele. Defina como `0` para desativar as verificações e [tentativas automáticas](#other-errors-retry-or-pause-the-goal).

As verificações requerem Claude Code v2.1.234 ou posterior.

<h3 id="evaluation-model-and-cost">
  Modelo de avaliação e custo
</h3>

Para avaliar em um modelo diferente, defina [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/pt/model-config#environment-variables).

<Warning>
  Claude Code lê `ANTHROPIC_DEFAULT_HAIKU_MODEL` em todos os lugares onde usa o modelo pequeno e rápido, não apenas para avaliação de `/goal`. Quando você o define, Claude Code também resolve o [alias `haiku`](/docs/pt/model-config#model-aliases) para esse modelo e executa [funcionalidade em segundo plano](/docs/pt/costs#background-token-usage), como resumo de conversa, nele.
</Warning>

O avaliador é executado no provedor para o qual sua sessão está configurada. Ele não chama ferramentas, portanto só pode julgar o que Claude já apresentou na conversa.

<Note>
  Os tokens de avaliação são cobrados no modelo pequeno e rápido configurado para seu provedor e são tipicamente negligenciáveis em comparação com o gasto do turno principal.
</Note>

<h2 id="requirements">
  Requisitos
</h2>

Claude Code disponibiliza `/goal` sob a mesma [regra de confiança do espaço de trabalho que hooks em arquivos de configurações](/docs/pt/permissions#what-runs-before-you-trust-a-folder), porque o avaliador faz parte do sistema de hooks. `/goal` também não está disponível quando [`disableAllHooks`](/docs/pt/hooks#disable-or-remove-hooks) é `true` após a precedência de configurações ser aplicada, ou quando [`allowManagedHooksOnly`](/docs/pt/settings-reference#allowmanagedhooksonly) está definido nas configurações gerenciadas. Em cada caso, o comando informa por que em vez de fazer nada silenciosamente.

<h2 id="see-also">
  Veja também
</h2>

* [Executar um prompt repetidamente com `/loop`](/docs/pt/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): re-executar em um intervalo de tempo em vez de até que uma condição seja atendida
* [Hooks baseados em prompt](/docs/pt/hooks-guide#prompt-based-hooks): escreva seu próprio Stop hook quando precisar de lógica de avaliação personalizada
* [Modo automático](/docs/pt/auto-mode-config): aprove chamadas de ferramentas automaticamente para que cada turno de objetivo seja executado sem supervisão
* [Comparação de agendamento](/docs/pt/scheduled-tasks#compare-scheduling-options): execute trabalho em um cronograma independente de qualquer sessão aberta
