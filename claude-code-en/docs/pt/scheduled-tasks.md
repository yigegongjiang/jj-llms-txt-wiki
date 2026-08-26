> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Executar prompts em um cronograma

> Use /loop e as ferramentas de agendamento cron para executar prompts repetidamente, pesquisar status ou definir lembretes únicos em uma sessão do Claude Code.

Tarefas agendadas permitem que Claude execute novamente um prompt automaticamente em um intervalo. Use-as para pesquisar uma implantação, cuidar de um PR, verificar uma compilação de longa duração ou lembrar-se de fazer algo mais tarde na sessão. Para reagir a eventos conforme eles acontecem em vez de pesquisar, consulte [Channels](/docs/pt/channels): seu CI pode enviar a falha para a sessão diretamente. Para manter a sessão funcionando turno após turno até que uma condição seja atendida em vez de em um intervalo, consulte [`/goal`](/docs/pt/goal).

As tarefas têm escopo de sessão: elas vivem na conversa atual e param quando você inicia uma nova. Retomar com `--resume` ou `--continue` traz de volta qualquer tarefa que não tenha [expirado](#seven-day-expiry): uma tarefa recorrente criada nos últimos 7 dias, ou uma única cujo tempo agendado ainda não passou. Para agendamento que sobreviva independentemente de qualquer sessão, use [Routines](/docs/pt/routines) para criar uma rotina na infraestrutura gerenciada pela Anthropic, configure uma [tarefa agendada do Desktop](/docs/pt/desktop-scheduled-tasks) ou use [GitHub Actions](/docs/pt/github-actions).

<h2 id="compare-scheduling-options">
  Comparar opções de agendamento
</h2>

Claude Code offers three ways to schedule recurring or one-off work:

|                            | [Cloud](/docs/en/routines)               | [Desktop](/docs/en/desktop-scheduled-tasks) | [`/loop`](/docs/en/scheduled-tasks)      |
| :------------------------- | :---------------------------------- | :------------------------------------- | :---------------------------------- |
| Runs on                    | Cloud, Anthropic-managed by default | Your machine                           | Your machine                        |
| Requires machine on        | No                                  | Yes                                    | Yes                                 |
| Requires open session      | No                                  | No                                     | Yes                                 |
| Persistent across restarts | Yes                                 | Yes                                    | Restored on `--resume` if unexpired |
| Access to local files      | No (fresh clone)                    | Yes                                    | Yes                                 |
| MCP servers                | Connectors configured per task      | [Config files](/docs/en/mcp) and connectors | Inherits from session               |
| Permission prompts         | No (runs autonomously)              | Configurable per task                  | Inherits from session               |
| Customizable schedule      | Via `/schedule` in the CLI          | Yes                                    | Yes                                 |
| Minimum interval           | 1 hour                              | 1 minute                               | 1 minute                            |

<Tip>
  Use **cloud tasks** for work that should run reliably without your machine. Use **Desktop tasks** when you need access to local files and tools. Use **`/loop`** for quick polling during a session.
</Tip>

<h2 id="run-a-prompt-repeatedly-with-/loop">
  Execute um prompt repetidamente com /loop
</h2>

A skill agrupada `/loop` [bundled skill](/docs/pt/commands) é a maneira mais rápida de executar um prompt repetidamente enquanto a sessão permanece aberta. Tanto o intervalo quanto o prompt são opcionais, e o que você fornece determina como o loop se comporta.

| O que você fornece        | Exemplo                     | O que acontece                                                                                                    |
| :------------------------ | :-------------------------- | :---------------------------------------------------------------------------------------------------------------- |
| Intervalo e prompt        | `/loop 5m check the deploy` | Seu prompt é executado em um [cronograma fixo](#run-on-a-fixed-interval)                                          |
| Apenas prompt             | `/loop check the deploy`    | Seu prompt é executado em um [intervalo que Claude escolhe](#let-claude-choose-the-interval) a cada iteração      |
| Apenas intervalo, ou nada | `/loop`                     | O [prompt de manutenção integrado](#run-the-built-in-maintenance-prompt) é executado, ou seu `loop.md` se existir |

Você também pode passar uma skill como o prompt, por exemplo `/loop 20m /review-pr 1234`, para re-executar essa skill a cada iteração. A partir da v2.1.196, um disparo agendado executa apenas skills que Claude [tem permissão para invocar por conta própria](/docs/pt/skills#control-who-invokes-a-skill). Os seguintes chegam a Claude como texto simples em vez de executar:

* comandos integrados como `/permissions`, `/model`, ou `/clear`
* skills marcadas [`disable-model-invocation: true`](/docs/pt/skills#frontmatter-reference)
* skills retidas de Claude por uma configuração [`skillOverrides`](/docs/pt/skills#override-skill-visibility-from-settings) ou uma regra de [negação](/docs/pt/skills#restrict-claude’s-skill-access) de `Skill`
* [prompts MCP](/docs/pt/mcp#use-mcp-prompts-as-commands) como `/mcp__github__list_prs`; skills que um servidor MCP expõe ainda são executadas

<h3 id="run-on-a-fixed-interval">
  Execute em um intervalo fixo
</h3>

Quando você fornece um intervalo, Claude o converte em uma expressão cron, agenda o trabalho e confirma a cadência e o ID do trabalho.

```text theme={null}
/loop 5m check if the deployment finished and tell me what happened
```

O intervalo pode preceder o prompt como um token simples como `30m`, ou seguir como uma cláusula como `every 2 hours`. As unidades suportadas são `s` para segundos, `m` para minutos, `h` para horas e `d` para dias.

Os segundos são arredondados para o minuto mais próximo, pois o cron tem granularidade de um minuto. Os intervalos que não mapeiam para um passo cron limpo, como `7m` ou `90m`, são arredondados para o intervalo mais próximo que funciona e Claude informa qual foi escolhido.

<h3 id="let-claude-choose-the-interval">
  Deixe Claude escolher o intervalo
</h3>

Quando você omite o intervalo, Claude escolhe um dinamicamente em vez de executar em um cronograma cron fixo. Após cada iteração, ele escolhe um atraso entre um minuto e uma hora com base no que observou: esperas curtas enquanto uma compilação está terminando ou um PR está ativo, esperas mais longas quando nada está pendente. O atraso escolhido e o motivo dele são impressos no final de cada iteração.

O exemplo abaixo verifica CI e comentários de revisão, com Claude esperando mais tempo entre iterações uma vez que o PR fica silencioso:

```text theme={null}
/loop check whether CI passed and address any review comments
```

Quando você pede um cronograma `/loop` dinâmico, Claude pode usar a [ferramenta Monitor](/docs/pt/tools-reference#monitor-tool) diretamente. Monitor executa um script em segundo plano e transmite cada linha de saída de volta, o que evita pesquisa completamente e geralmente é mais eficiente em tokens e responsivo do que re-executar um prompt em um intervalo.

Um loop agendado dinamicamente aparece em sua [lista de tarefas agendadas](#manage-scheduled-tasks) como qualquer outra tarefa, portanto você pode listá-lo ou cancelá-lo da mesma forma. As [regras de jitter](#jitter) não se aplicam a ele, mas a [expiração de sete dias](#seven-day-expiry) se aplica: o loop termina automaticamente sete dias após você iniciá-lo.

<Note>
  Na Amazon Bedrock, na Plataforma Claude no AWS, na Plataforma de Agentes do Google Cloud e no Microsoft Foundry, um prompt sem intervalo é executado em um cronograma fixo de 10 minutos.
</Note>

<h3 id="run-the-built-in-maintenance-prompt">
  Execute o prompt de manutenção integrado
</h3>

Quando você omite o prompt, Claude usa um prompt de manutenção integrado em vez de um que você fornece. A cada iteração, ele trabalha através do seguinte, em ordem:

* continuar qualquer trabalho inacabado da conversa
* cuidar do pull request do branch atual: comentários de revisão, execuções de CI falhadas, conflitos de mesclagem
* executar passes de limpeza, como caças a bugs ou simplificação quando nada mais está pendente

Claude não inicia novas iniciativas fora desse escopo, e ações irreversíveis como envio ou exclusão apenas prosseguem quando continuam algo que a transcrição já autorizou.

```text theme={null}
/loop
```

Um `/loop` simples executa este prompt em um [intervalo escolhido dinamicamente](#let-claude-choose-the-interval). Adicione um intervalo, por exemplo `/loop 15m`, para executá-lo em um cronograma fixo. Para substituir o prompt integrado pelo seu próprio padrão, consulte [Personalize o prompt padrão com loop.md](#customize-the-default-prompt-with-loop-md).

<Note>
  Na Amazon Bedrock, na Plataforma Claude no AWS, na Plataforma de Agentes do Google Cloud e no Microsoft Foundry, `/loop` sem prompt imprime a mensagem de uso em vez de executar o prompt de manutenção integrado.
</Note>

<h3 id="customize-the-default-prompt-with-loop-md">
  Personalize o prompt padrão com loop.md
</h3>

Um arquivo `loop.md` substitui o prompt de manutenção integrado pelas suas próprias instruções. Ele define um único prompt padrão para `/loop` simples, não uma lista de tarefas agendadas separadas, e é ignorado sempre que você fornece um prompt na linha de comando. Para agendar prompts adicionais junto com ele, use `/loop <prompt>` ou [peça a Claude diretamente](#manage-scheduled-tasks).

Claude procura o arquivo em dois locais e usa o primeiro que encontra.

| Caminho             | Escopo                                                                        |
| :------------------ | :---------------------------------------------------------------------------- |
| `.claude/loop.md`   | Nível do projeto. Tem precedência quando ambos os arquivos existem.           |
| `~/.claude/loop.md` | Nível do usuário. Aplica-se em qualquer projeto que não defina o seu próprio. |

O arquivo é Markdown simples sem estrutura obrigatória. Escreva como se estivesse digitando o prompt `/loop` diretamente. O exemplo a seguir mantém um branch de lançamento saudável:

```markdown title=".claude/loop.md" theme={null}
Check the `release/next` PR. If CI is red, pull the failing job log,
diagnose, and push a minimal fix. If new review comments have arrived,
address each one and resolve the thread. If everything is green and
quiet, say so in one line.
```

Edições em `loop.md` entram em vigor na próxima iteração, portanto você pode refinar as instruções enquanto um loop está em execução. Quando nenhum `loop.md` existe em nenhum local, o loop volta ao prompt de manutenção integrado. Mantenha o arquivo conciso: conteúdo além de 25.000 bytes é truncado.

<Note>
  Na Amazon Bedrock, na Plataforma Claude no AWS, na Plataforma de Agentes do Google Cloud e no Microsoft Foundry, `loop.md` não é lido e `/loop` sem prompt imprime a mensagem de uso em vez disso.
</Note>

<h3 id="stop-a-loop">
  Pare um loop
</h3>

Para parar um `/loop` enquanto ele está aguardando a próxima iteração, pressione `Esc`. Isso limpa o despertar pendente para que o loop não dispare novamente. As tarefas que você agendou [pedindo a Claude diretamente](#manage-scheduled-tasks) não são afetadas por `Esc` e permanecem no lugar até que você as delete.

Em [modo de ritmo próprio](#let-claude-choose-the-interval), Claude também pode encerrar o loop por conta própria uma vez que a tarefa está completa. Claude chama a [ferramenta `ScheduleWakeup`](/docs/pt/tools-reference) com `stop: true`, o que cancela o despertar pendente imediatamente. Se uma iteração terminar sem reagendar ou parar, Claude Code agenda um despertar de fallback cerca de 20 minutos depois e encerra o loop quando essa iteração também não reagendar. Antes da v2.1.202, não reagendar era a única maneira de Claude poder encerrar um loop por conta própria.

Loops em um intervalo fixo continuam em execução até que você os pare ou [sete dias se passem](#seven-day-expiry).

<h2 id="set-a-one-time-reminder">
  Defina um lembrete único
</h2>

Para lembretes únicos, descreva o que você deseja em linguagem natural em vez de usar `/loop`. Claude agenda uma tarefa de disparo único que se deleta após ser executada.

```text theme={null}
remind me at 3pm to push the release branch
```

```text theme={null}
in 45 minutes, check whether the integration tests passed
```

Claude fixa o tempo de disparo em um minuto e hora específicos usando uma expressão cron e confirma quando será acionado.

<h2 id="manage-scheduled-tasks">
  Gerencie tarefas agendadas
</h2>

Peça a Claude em linguagem natural para listar ou cancelar tarefas, ou referencie as ferramentas subjacentes diretamente.

```text theme={null}
what scheduled tasks do I have?
```

```text theme={null}
cancel the deploy check job
```

Nos bastidores, Claude usa estas ferramentas:

| Ferramenta   | Propósito                                                                                                                 |
| :----------- | :------------------------------------------------------------------------------------------------------------------------ |
| `CronCreate` | Agendar uma nova tarefa. Aceita uma expressão cron de 5 campos, o prompt a ser executado e se recorre ou dispara uma vez. |
| `CronList`   | Listar todas as tarefas agendadas com seus IDs, cronogramas e prompts.                                                    |
| `CronDelete` | Cancelar uma tarefa por ID.                                                                                               |

Cada tarefa agendada tem um ID de 8 caracteres que você pode passar para `CronDelete`. Uma sessão pode conter até 50 tarefas agendadas por vez.

<h2 id="how-scheduled-tasks-run">
  Como as tarefas agendadas são executadas
</h2>

O agendador verifica a cada segundo se há tarefas vencidas e as enfileira com baixa prioridade. Um prompt agendado é acionado entre seus turnos, não enquanto Claude está no meio de uma resposta. Se Claude estiver ocupado quando uma tarefa vencer, o prompt aguarda até que o turno atual termine.

Todos os horários são interpretados em seu fuso horário local. Uma expressão cron como `0 9 * * *` significa 9h onde você está executando Claude Code, não UTC.

<h3 id="jitter">
  Jitter
</h3>

Para evitar que cada sessão atinja a API no mesmo momento de tempo real, o agendador adiciona um deslocamento determinístico aos tempos de disparo:

* Tarefas recorrentes disparam até 30 minutos após o horário agendado (ou até metade do intervalo, para tarefas que são executadas com mais frequência que por hora). Um trabalho por hora agendado para `:00` pode disparar em qualquer lugar até `:30`.
* Tarefas únicas agendadas para o topo ou fundo da hora disparam até 90 segundos mais cedo.

O deslocamento é derivado do ID da tarefa, portanto a mesma tarefa sempre obtém o mesmo deslocamento. Se o tempo exato for importante, escolha um minuto que não seja `:00` ou `:30`, por exemplo `3 9 * * *` em vez de `0 9 * * *`, e o jitter único não será aplicado.

<h3 id="seven-day-expiry">
  Expiração de sete dias
</h3>

Tarefas recorrentes expiram automaticamente 7 dias após a criação. A tarefa é acionada uma última vez e depois se deleta. Isso limita quanto tempo um loop esquecido pode ser executado. Se você precisar que uma tarefa recorrente dure mais tempo, cancele e recrie-a antes de expirar, ou use [Routines](/docs/pt/routines) ou [tarefas agendadas do Desktop](/docs/pt/desktop-scheduled-tasks) para agendamento durável.

<h2 id="cron-expression-reference">
  Referência de expressão cron
</h2>

`CronCreate` aceita expressões cron padrão de 5 campos: `minute hour day-of-month month day-of-week`. Todos os campos suportam curingas (`*`), valores únicos (`5`), passos (`*/15`), intervalos (`1-5`) e listas separadas por vírgula (`1,15,30`).

| Exemplo        | Significado                        |
| :------------- | :--------------------------------- |
| `*/5 * * * *`  | A cada 5 minutos                   |
| `0 * * * *`    | A cada hora na hora                |
| `7 * * * *`    | A cada hora aos 7 minutos passados |
| `0 9 * * *`    | Todos os dias às 9h local          |
| `0 9 * * 1-5`  | Dias da semana às 9h local         |
| `30 14 15 3 *` | 15 de março às 14h30 local         |

O dia da semana usa `0` ou `7` para domingo até `6` para sábado. A sintaxe estendida como `L`, `W`, `?` e aliases de nome como `MON` ou `JAN` não é suportada.

Quando tanto o dia do mês quanto o dia da semana são restritos, uma data corresponde se qualquer campo corresponder. Isso segue a semântica padrão do vixie-cron.

<h2 id="disable-scheduled-tasks">
  Desabilite tarefas agendadas
</h2>

Defina `CLAUDE_CODE_DISABLE_CRON=1` em seu ambiente para desabilitar o agendador completamente. As ferramentas cron e `/loop` ficam indisponíveis e qualquer tarefa já agendada para de ser acionada. Consulte [Variáveis de ambiente](/docs/pt/env-vars) para a lista completa de sinalizadores de desabilitação.

<h2 id="limitations">
  Limitações
</h2>

O agendamento com escopo de sessão tem limitações inerentes:

* As tarefas só são acionadas enquanto Claude Code está em execução e ocioso. Fechar o terminal ou deixar a sessão sair para tudo. [Colocar a sessão em segundo plano](/docs/pt/agent-view#from-inside-a-session) leva tarefas `/loop` para uma sessão em segundo plano, que continua em execução sem um terminal.
* Sem recuperação para disparos perdidos. Se o tempo agendado de uma tarefa passar enquanto Claude está ocupado em uma solicitação de longa duração, ela dispara uma vez quando Claude fica ocioso, não uma vez por intervalo perdido.
* Iniciar uma conversa nova limpa todas as tarefas com escopo de sessão. Retomar com `claude --resume` ou `claude --continue` restaura tarefas que não expiraram: tarefas recorrentes dentro de sete dias de criação, e tarefas únicas cujo tempo agendado ainda não passou. Tarefas de Bash em segundo plano e tarefas de monitor nunca são restauradas ao retomar.

Para automação orientada por cron que precisa ser executada sem supervisão:

* [Routines](/docs/pt/routines): executadas na infraestrutura gerenciada pela Anthropic em um cronograma, via chamada de API ou em eventos do GitHub
* [GitHub Actions](/docs/pt/github-actions): use um gatilho `schedule` em CI
* [Tarefas agendadas do Desktop](/docs/pt/desktop-scheduled-tasks): executadas localmente em sua máquina
