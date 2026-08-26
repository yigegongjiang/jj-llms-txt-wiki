> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agendar tarefas recorrentes no Claude Code Desktop

> Configure tarefas agendadas no Claude Code Desktop para executar Claude automaticamente em uma base recorrente para análises de código diárias, auditorias de dependências ou briefings matinais.

As tarefas agendadas iniciam uma nova sessão automaticamente em um horário e frequência que você escolhe. Use-as para trabalho recorrente como análises de código diárias, verificações de atualizações de dependências ou briefings matinais que extraem informações do seu calendário e caixa de entrada.

A página **Routines** do aplicativo Desktop permite criar tanto tarefas agendadas locais quanto [routines](/docs/pt/routines) remotas. Uma tarefa local é executada em sua máquina com acesso direto aos seus arquivos e ferramentas, mas só funciona enquanto o aplicativo está aberto e seu computador está acordado. Uma routine remota é executada na infraestrutura em nuvem gerenciada pela Anthropic, mesmo quando seu computador está desligado, e também pode ser acionada por chamadas de API ou eventos do GitHub. Esta página aborda tarefas agendadas locais; para routines remotas e suas opções de acionamento, consulte [Routines](/docs/pt/routines).

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

<Note>
  Por padrão, as tarefas agendadas são executadas contra qualquer estado em que seu diretório de trabalho esteja, incluindo alterações não confirmadas. Ative o toggle de worktree ao criar a tarefa para dar a cada execução seu próprio worktree Git isolado, da mesma forma que [sessões paralelas](/docs/pt/desktop#work-in-parallel-with-sessions) funcionam.
</Note>

<h2 id="create-a-scheduled-task">
  Criar uma tarefa agendada
</h2>

Clique em **Routines** na barra lateral e, em seguida, clique em **New routine** e escolha **Local**. Configure estes campos:

| Campo        | Descrição                                                                                                                                                                                                                                                                                                     |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Name         | Identificador da tarefa. Convertido para kebab-case em minúsculas e usado como nome da pasta no disco. Deve ser único entre suas tarefas.                                                                                                                                                                     |
| Description  | Resumo breve mostrado na lista de tarefas.                                                                                                                                                                                                                                                                    |
| Instructions | O que Claude deve fazer quando a tarefa é executada. Escreva isso da mesma forma que você escreveria qualquer mensagem na caixa de prompt. A entrada de instruções inclui seletores para o modo de permissão e modelo, e abaixo você seleciona a pasta de trabalho e se deve executar em um worktree isolado. |
| Schedule     | Com que frequência a tarefa é executada. Consulte [opções de agendamento](#schedule-options) abaixo.                                                                                                                                                                                                          |

Uma pasta é necessária antes de você poder salvar a tarefa. Se você ainda não confiou nessa pasta, o Desktop solicita que você confie nela antes de salvar.

Você também pode criar uma tarefa descrevendo o que deseja em qualquer sessão. Por exemplo, "configurar uma análise de código diária que é executada todos os dias de manhã às 9h" cria uma tarefa recorrente, e "me lembre às 15h amanhã para verificar o deploy" cria uma tarefa única que se desativa após ser executada.

<h2 id="schedule-options">
  Opções de agendamento
</h2>

Escolha um preset do controle Schedule:

* **Manual**: sem agendamento, só é executado quando você clica em **Run now**. Útil para salvar um prompt que você aciona sob demanda
* **Hourly**: é executado a cada hora
* **Daily**: mostra um seletor de hora, padrão 9:00 AM hora local
* **Weekdays**: igual a Daily, mas pula sábado e domingo
* **Weekly**: mostra um seletor de hora e um seletor de dia

Para intervalos que o seletor não oferece, como a cada 15 minutos, no primeiro de cada mês ou uma única execução em um horário futuro específico, peça ao Claude em qualquer sessão do Desktop para definir o agendamento. Use linguagem simples; por exemplo, "agende uma tarefa para executar todos os testes a cada 6 horas."

<h2 id="how-scheduled-tasks-run">
  Como as tarefas agendadas são executadas
</h2>

As tarefas agendadas são executadas em sua máquina. O Desktop verifica o agendamento a cada minuto enquanto o aplicativo está aberto e inicia uma sessão nova quando uma tarefa está vencida, independentemente de qualquer sessão manual que você tenha aberta. Cada tarefa recebe um pequeno atraso de alguns minutos após o horário agendado para escalonar o tráfego de API. O atraso é determinístico: a mesma tarefa sempre inicia no mesmo deslocamento.

Quando uma tarefa é acionada, você recebe uma notificação da área de trabalho e uma nova sessão aparece em uma seção **Scheduled** na barra lateral. Abra-a para ver o que Claude fez, revisar alterações ou responder a prompts de permissão. A sessão funciona como qualquer outra: Claude pode editar arquivos, executar comandos, criar commits e abrir pull requests.

As tarefas só são executadas enquanto o aplicativo desktop está em execução e seu computador está acordado. Se seu computador dormir durante um horário agendado, a execução é ignorada. Para evitar suspensão por inatividade, ative **Keep computer awake** em Settings em **Desktop app → General**. Fechar a tampa do laptop ainda o coloca em suspensão. Para tarefas que precisam ser executadas mesmo quando seu computador está desligado, ou que devem ser acionadas por uma chamada de API ou evento do GitHub, crie uma [routine](/docs/pt/routines) remota.

<h2 id="missed-runs">
  Execuções perdidas
</h2>

Quando o aplicativo inicia ou seu computador acorda, o Desktop verifica se cada tarefa perdeu alguma execução nos últimos sete dias. Se perdeu, o Desktop inicia exatamente uma execução de recuperação para o horário mais recentemente perdido e descarta qualquer coisa mais antiga. Uma tarefa diária que perdeu seis dias é executada uma vez ao acordar. O Desktop mostra uma notificação quando uma execução de recuperação inicia.

Tenha isso em mente ao escrever prompts. Uma tarefa agendada para 9h pode ser executada às 23h se seu computador esteve dormindo o dia todo. Se o tempo importa, adicione proteções ao próprio prompt, por exemplo: "Revise apenas os commits de hoje. Se for depois das 17h, pule a revisão e apenas poste um resumo do que foi perdido."

<h2 id="permissions-for-scheduled-tasks">
  Permissões para tarefas agendadas
</h2>

Cada tarefa tem seu próprio modo de permissão, que você define ao criar ou editar a tarefa. As regras de permissão de `~/.claude/settings.json` também se aplicam a sessões de tarefas agendadas. Se uma tarefa é executada em modo Ask e precisa executar uma ferramenta para a qual não tem permissão, a execução fica travada até que você a aprove. A sessão permanece aberta na barra lateral para que você possa responder mais tarde.

Para evitar travamentos, clique em **Run now** após criar uma tarefa, observe prompts de permissão e selecione "always allow" para cada um. Execuções futuras dessa tarefa aprovam automaticamente as mesmas ferramentas sem solicitar. Você pode revisar e revogar essas aprovações na página de detalhes da tarefa.

As ferramentas do Connector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e as ferramentas MCP marcadas como [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool) solicitam confirmação a cada chamada e não oferecem uma opção de sempre permitir. Execuções que chamam essas ferramentas ficam travadas cada vez.

<h2 id="manage-scheduled-tasks">
  Gerenciar tarefas agendadas
</h2>

Clique em uma tarefa na lista **Routines** para abrir sua página de detalhes. A partir daqui você pode:

* **Run now**: inicie a tarefa imediatamente sem esperar pelo próximo horário agendado
* **Status**: alterne entre Active e Paused para pausar ou retomar execuções agendadas sem deletar a tarefa
* **Edit**: altere as instruções, agendamento, pasta ou outras configurações
* **Review history**: veja cada execução anterior, incluindo execuções ignoradas. Passe o mouse sobre uma entrada ignorada para ver por quê: seu computador estava dormindo, a execução anterior ainda estava em andamento ou outras tarefas agendadas já estavam em execução. Clique em **Show more** para carregar entradas mais antigas.
* **Review allowed permissions**: veja e revogue aprovações de ferramentas salvas para esta tarefa no painel **Always allowed**
* **Delete**: remova a tarefa e arquive todas as sessões que ela criou. Uma caixa de seleção **Also delete files on disk** aparece no diálogo de confirmação; marque-a para também remover o arquivo `SKILL.md` da tarefa e dados associados de `~/.claude/scheduled-tasks/`.

Você também pode listar, criar, editar e pausar tarefas pedindo ao Claude em qualquer sessão do Desktop. Por exemplo, "pause my dependency-audit task" ou "show me my scheduled tasks." Para deletar uma tarefa, use o botão **Delete** em sua página de detalhes.

Uma tarefa agendada também pode modificar seu próprio agendamento ou prompt dentro de uma sessão em execução usando a ferramenta MCP `update_scheduled_task`. Isso permite que uma tarefa se reagende com base no que encontra, por exemplo, reagendando uma análise de código para ser executada mais cedo quando detecta que um branch de release foi criado.

Para editar o prompt de uma tarefa no disco, abra `~/.claude/scheduled-tasks/<task-name>/SKILL.md` (ou em [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars) se definido). O arquivo usa frontmatter YAML para `name` e `description`, com o prompt como corpo. As alterações entram em vigor na próxima execução. Schedule, pasta, modelo e estado habilitado não estão neste arquivo: altere-os através do formulário Edit ou peça ao Claude.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Routines](/docs/pt/routines): execute tarefas na infraestrutura gerenciada pela Anthropic em um agendamento, via chamada de API ou em resposta a eventos do GitHub, mesmo quando seu computador está desligado
* [Run prompts on a schedule](/docs/pt/scheduled-tasks): agendamento com escopo de sessão com `/loop` na CLI
* [Claude Code GitHub Actions](/docs/pt/github-actions): execute Claude em um agendamento em CI em vez de em sua máquina
* [Use Claude Code Desktop](/docs/pt/desktop): o guia completo do aplicativo Desktop
