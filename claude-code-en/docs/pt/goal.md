> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Manter Claude trabalhando em direção a um objetivo

> Defina uma condição de conclusão com /goal e Claude continua trabalhando entre turnos até que a condição seja atendida.

<Note>
  `/goal` requer Claude Code v2.1.139 ou posterior.
</Note>

O comando `/goal` define uma condição de conclusão e Claude continua trabalhando em direção a ela sem que você solicite cada etapa. Após cada turno, um modelo pequeno e rápido verifica se a condição é atendida. Se não for, Claude inicia outro turno em vez de devolver o controle a você. O objetivo é limpo automaticamente assim que a condição é atendida.

Use um objetivo para trabalho substancial com um estado final verificável:

* Migrar um módulo para uma nova API até que cada site de chamada compile e os testes passem
* Implementar um documento de design até que todos os critérios de aceitação sejam atendidos
* Dividir um arquivo grande em módulos focados até que cada um esteja dentro de um orçamento de tamanho
* Trabalhar através de uma fila de problemas rotulados até que a fila esteja vazia

<h2 id="compare-ways-to-keep-a-session-running">
  Comparar formas de manter uma sessão em execução
</h2>

Três abordagens mantêm a sessão atual em execução entre prompts. Escolha com base no que deve iniciar o próximo turno:

| Abordagem                                                           | Próximo turno começa quando   | Para quando                                                       |
| :------------------------------------------------------------------ | :---------------------------- | :---------------------------------------------------------------- |
| `/goal`                                                             | O turno anterior termina      | Um modelo confirma que a condição é atendida                      |
| [`/loop`](/docs/pt/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Um intervalo de tempo decorre | Você o interrompe, ou Claude decide que o trabalho está concluído |
| [Stop hook](/docs/pt/hooks-guide#prompt-based-hooks)                     | O turno anterior termina      | Seu próprio script ou prompt decide                               |

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

Um objetivo não altera permissões. No modo de permissão padrão, Claude ainda solicita antes de chamadas de ferramentas que suas configurações ainda não permitem, como o comando de teste acima. Para permitir que turnos de objetivo sejam executados sem supervisão, combine `/goal` com [modo automático](/docs/pt/auto-mode-config).

Após cada turno, o avaliador retorna uma breve razão explicando por que a condição é ou não atendida. A razão mais recente aparece na visualização de status e na transcrição para que você possa ver para o que Claude está trabalhando a seguir.

<Note>
  Um objetivo continua em execução até que a condição seja atendida ou você execute `/goal clear`. Execute `/goal` sem argumento para ver os turnos e tokens gastos até agora.
</Note>

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

Execute `/goal clear` para remover um objetivo ativo antes que sua condição seja atendida.

```text theme={null}
/goal clear
```

Claude imprime `Goal cleared:` seguido pela condição para confirmar, ou `No goal set` se nada estava ativo.

`stop`, `off`, `reset`, `none` e `cancel` são aceitos como aliases para `clear`. Executar `/clear` para iniciar uma nova conversa também remove qualquer objetivo ativo.

<h3 id="resume-with-an-active-goal">
  Retomar com um objetivo ativo
</h3>

Um objetivo que ainda estava ativo quando uma sessão terminou é restaurado quando você retoma essa sessão com `--resume` ou `--continue`. A condição é mantida, mas a contagem de turnos, o cronômetro e a linha de base de gasto de token são redefinidos ao retomar. Um objetivo que já foi alcançado ou limpo não é restaurado.

<h3 id="run-non-interactively">
  Executar de forma não interativa
</h3>

`/goal` funciona em [modo não interativo](/docs/pt/headless), no [aplicativo desktop](/docs/pt/desktop) e através de [Remote Control](/docs/pt/remote-control). Definir um objetivo com `-p` executa o loop até a conclusão em uma única invocação:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Com a saída de texto padrão, nada é impresso até que a condição seja atendida, portanto um objetivo que executa muitos turnos pode parecer travado. Adicione `--output-format stream-json --verbose` para emitir cada mensagem conforme o loop é executado.

Interrompa o processo com Ctrl+C para parar um objetivo não interativo antes que a condição seja atendida.

<h2 id="how-evaluation-works">
  Como a avaliação funciona
</h2>

`/goal` é um wrapper em torno de um [Stop hook baseado em prompt](/docs/pt/hooks#prompt-based-hooks) com escopo de sessão. Cada vez que Claude termina um turno, a condição e a conversa até agora são enviadas para seu [modelo pequeno e rápido](/docs/pt/model-config) configurado, que é padronizado para Haiku. O modelo retorna uma decisão sim ou não e uma breve razão. Um "não" diz a Claude para continuar trabalhando e inclui a razão como orientação para o próximo turno. Um "sim" limpa o objetivo e registra uma entrada alcançada na transcrição.

O avaliador é executado no provedor para o qual sua sessão está configurada. Ele não chama ferramentas, portanto só pode julgar o que Claude já apresentou na conversa.

<Note>
  Os tokens de avaliação são cobrados no modelo pequeno e rápido configurado para seu provedor e são tipicamente negligenciáveis em comparação com o gasto do turno principal.
</Note>

<h2 id="requirements">
  Requisitos
</h2>

`/goal` é executado apenas em espaços de trabalho onde você aceitou a caixa de diálogo de confiança, porque o avaliador faz parte do sistema de hooks. `/goal` também não está disponível quando [`disableAllHooks`](/docs/pt/hooks#disable-or-remove-hooks) está definido em qualquer nível de configurações ou quando [`allowManagedHooksOnly`](/docs/pt/settings#hook-configuration) está definido nas configurações gerenciadas. Em cada caso, o comando informa por que em vez de fazer nada silenciosamente.

<h2 id="see-also">
  Veja também
</h2>

* [Executar um prompt repetidamente com `/loop`](/docs/pt/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): re-executar em um intervalo de tempo em vez de até que uma condição seja atendida
* [Hooks baseados em prompt](/docs/pt/hooks-guide#prompt-based-hooks): escreva seu próprio Stop hook quando precisar de lógica de avaliação personalizada
* [Modo automático](/docs/pt/auto-mode-config): aprove chamadas de ferramentas automaticamente para que cada turno de objetivo seja executado sem supervisão
* [Comparação de agendamento](/docs/pt/scheduled-tasks#compare-scheduling-options): execute trabalho em um cronograma independente de qualquer sessão aberta
