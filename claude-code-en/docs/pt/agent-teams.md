> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orquestre equipes de sessões Claude Code

> Coordene múltiplas instâncias Claude Code trabalhando juntas como uma equipe, com tarefas compartilhadas, mensagens entre agentes e gerenciamento centralizado.

<Warning>
  Equipes de agentes são experimentais e desabilitadas por padrão. Ative-as definindo `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` no seu [settings.json](/docs/pt/settings) ou ambiente. Sem essa variável, nenhuma equipe é configurada no início da sessão, nenhum diretório de equipe é escrito, e Claude não gera ou propõe companheiros de equipe. Equipes de agentes têm [limitações conhecidas](#limitations) em torno de retomada de sessão, coordenação de tarefas e comportamento de encerramento.
</Warning>

Equipes de agentes permitem que você coordene múltiplas instâncias Claude Code trabalhando juntas. Uma sessão atua como o líder da equipe, coordenando o trabalho, atribuindo tarefas e sintetizando resultados. Os companheiros de equipe trabalham independentemente, cada um em sua própria context window, e se comunicam diretamente uns com os outros. Você também pode conversar com qualquer companheiro de equipe diretamente sem passar pelo líder.

Antes de configurar uma equipe, verifique se uma opção mais leve faz o trabalho. [Subagents](/docs/pt/sub-agents) funcionam dentro de uma única sessão, e com [mensagens entre sessões](/docs/pt/cross-session-messaging) Claude pode passar descobertas entre as sessões que você executa por conta própria.

<h2 id="when-to-use-agent-teams">
  Quando usar equipes de agentes
</h2>

Equipes de agentes são mais eficazes para tarefas onde a exploração paralela adiciona valor real. Veja [exemplos de casos de uso](#use-case-examples) para cenários completos. Os casos de uso mais fortes são:

* **Pesquisa e revisão**: múltiplos companheiros de equipe podem investigar diferentes aspectos de um problema simultaneamente, depois compartilhar e desafiar as descobertas uns dos outros
* **Novos módulos ou recursos**: companheiros de equipe podem possuir cada um uma peça separada sem se atrapalharem
* **Depuração com hipóteses concorrentes**: companheiros de equipe testam diferentes teorias em paralelo e convergem para a resposta mais rapidamente
* **Coordenação entre camadas**: mudanças que abrangem frontend, backend e testes, cada uma de propriedade de um companheiro de equipe diferente

Equipes de agentes adicionam sobrecarga de coordenação e usam significativamente mais tokens do que uma única sessão. Funcionam melhor quando os companheiros de equipe podem operar independentemente. Para tarefas sequenciais, edições no mesmo arquivo ou trabalho com muitas dependências, uma única sessão ou [subagents](/docs/pt/sub-agents) são mais eficazes.

<h3 id="compare-with-subagents">
  Comparar com subagents
</h3>

Tanto equipes de agentes quanto [subagents](/docs/pt/sub-agents) permitem que você paralelizar o trabalho, mas operam de forma diferente. Para sessões separadas que passam mensagens uma para a outra sem uma equipe, veja [mensagens entre sessões](/docs/pt/cross-session-messaging).

<Frame caption="Subagents relatam resultados de volta ao agente principal. Em equipes de agentes, os companheiros de equipe compartilham uma lista de tarefas, reivindicam trabalho e se comunicam diretamente uns com os outros.">
  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-light.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=2f8db9b4f3705dd3ab931fbe2d96e42a" className="dark:hidden" alt="Diagrama comparando arquiteturas de subagent e equipe de agentes. Subagents são gerados pelo agente principal, fazem trabalho e relatam resultados de volta. Equipes de agentes coordenam através de uma lista de tarefas compartilhada, com companheiros de equipe se comunicando diretamente uns com os outros." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-light.png" />

  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-dark.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=d573a037540f2ada6a9ae7d8285b46fd" className="hidden dark:block" alt="Diagrama comparando arquiteturas de subagent e equipe de agentes. Subagents são gerados pelo agente principal, fazem trabalho e relatam resultados de volta. Equipes de agentes coordenam através de uma lista de tarefas compartilhada, com companheiros de equipe se comunicando diretamente uns com os outros." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-dark.png" />
</Frame>

|                   | Subagents                                                                                                                                                          | Agent teams                                                                                                                                                           |
| :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Context**       | Context window própria; resultados retornam ao chamador                                                                                                            | Context window própria; totalmente independente                                                                                                                       |
| **Communication** | Retornam um resultado ao chamador. Subagents que Claude nomeou quando os gerou também podem [se mensagear uns com os outros](/docs/pt/sub-agents#what-loads-at-startup) | Companheiros de equipe se mensageiam diretamente                                                                                                                      |
| **Coordination**  | Agente principal gerencia todo o trabalho                                                                                                                          | Auto-coordenação através de mensagens, mais uma lista de tarefas compartilhada para [agentes que têm as ferramentas Task](/docs/pt/tools-reference#task-tool-availability) |
| **Best for**      | Tarefas focadas onde apenas o resultado importa                                                                                                                    | Trabalho complexo que requer discussão e colaboração                                                                                                                  |
| **Token cost**    | Menor: resultados resumidos de volta ao contexto principal                                                                                                         | Maior: cada companheiro de equipe é uma instância Claude separada                                                                                                     |

Use subagents quando você precisa de trabalhadores rápidos e focados que relatem de volta. Use equipes de agentes quando os companheiros de equipe precisam compartilhar descobertas, desafiar uns aos outros e coordenar por conta própria.

<h2 id="enable-agent-teams">
  Ativar equipes de agentes
</h2>

Equipes de agentes são desabilitadas por padrão. Ative-as definindo a variável de ambiente `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` como `1`, seja no seu ambiente de shell ou através de [settings.json](/docs/pt/settings):

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
  }
}
```

Ativar equipes de agentes também altera a delegação ordinária. Claude pode [nomear um subagente](/docs/pt/sub-agents#subagent-names) por conta própria, e enquanto equipes de agentes estão ativadas, um subagente que Claude nomeia é iniciado como um colega de equipe, então equipes podem se formar mesmo quando você não pediu por uma. Para mais informações, consulte [Como Claude inicia equipes de agentes](#how-claude-starts-agent-teams); para desativar o comportamento, consulte [Claude gera colegas de equipe em vez de subagentes](#claude-spawns-teammates-instead-of-subagents).

Gerar colegas de equipe também requer uma sessão interativa. Em [modo não interativo](/docs/pt/headless) com a flag `-p`, incluindo sessões do Agent SDK, Claude não gera colegas de equipe, e um subagente que Claude nomeia é executado como um [subagente](/docs/pt/sub-agents) ordinário mesmo com equipes de agentes ativadas.

<h2 id="start-your-first-agent-team">
  Inicie sua primeira equipe de agentes
</h2>

Após ativar equipes de agentes, descreva a tarefa e os companheiros de equipe que você deseja em linguagem natural. Claude os cria e coordena o trabalho com base no seu prompt.

Este exemplo funciona bem porque os três papéis são independentes e podem explorar o problema sem esperar um pelo outro:

```text wrap theme={null}
I'm designing a CLI tool that helps developers track TODO comments across
their codebase. Spawn three teammates to explore this from different angles:
one on UX, one on technical architecture, one playing devil's advocate.
```

A partir daí, Claude popula uma [lista de tarefas compartilhada](/docs/pt/interactive-mode#task-list) em uma [sessão que possui as ferramentas Task](/docs/pt/tools-reference#task-tool-availability), cria companheiros de equipe para cada perspectiva, faz com que explorem o problema e sintetiza descobertas quando terminar.

Claude às vezes pode usar [subagentes](/docs/pt/sub-agents) em vez de criar uma equipe. Subagentes aparecem no mesmo painel de agente que os companheiros de equipe, portanto o painel sozinho não confirma que uma equipe foi formada. Se Claude criou subagentes em vez disso, peça novamente e solicite explicitamente uma equipe de agentes.

O terminal do líder lista companheiros de equipe no painel do agente abaixo da entrada do prompt. A partir do painel:

* **Setas para cima e para baixo**: selecione um companheiro de equipe
* **Enter**: abra a transcrição do companheiro de equipe selecionado e envie uma mensagem diretamente
* **Escape**: limpe a seleção. Enquanto você está visualizando a transcrição de um companheiro de equipe, Escape interrompe o turno atual desse companheiro de equipe

A partir da v2.1.199, a linha de um companheiro de equipe ocioso permanece no painel enquanto qualquer companheiro de equipe ou subagente ainda estiver trabalhando, para que você possa selecioná-lo para revisar sua transcrição ou enviar-lhe mais trabalho. Quando todos os agentes no painel estão ociosos, as linhas ociosas se ocultam após 30 segundos e reaparecem no próximo turno do companheiro de equipe; o companheiro de equipe continua em execução e endereçável enquanto oculto. Na v2.1.181 até v2.1.198, uma linha ociosa se ocultava 30 segundos após seu próprio turno terminar, mesmo enquanto outros companheiros de equipe ainda estavam trabalhando; linhas ociosas não são ocultadas em versões anteriores à v2.1.181.

Quando mais de três companheiros de equipe estão ociosos ao mesmo tempo, as linhas além das três primeiras se recolhem em uma única linha que conta os companheiros de equipe recolhidos, como `2 idle agents` quando cinco estão ociosos. Selecione-a e pressione Enter para expandir as linhas recolhidas, ou pressione Esc para recolhê-las novamente. Companheiros de equipe trabalhando, companheiros de equipe que falharam e o companheiro de equipe que você está visualizando sempre mantêm suas próprias linhas.

Se você quiser cada companheiro de equipe em seu próprio painel dividido, veja [Escolha um modo de exibição](#choose-a-display-mode).

<h2 id="control-your-agent-team">
  Controle sua equipe de agentes
</h2>

Diga ao líder o que você quer em linguagem natural. Ele lida com coordenação de equipe, atribuição de tarefas e delegação com base em suas instruções.

<h3 id="choose-a-display-mode">
  Escolha um modo de exibição
</h3>

Equipes de agentes suportam dois modos de exibição:

* **In-process**: todos os companheiros de equipe são executados dentro do seu terminal principal. Use as teclas de seta para cima e para baixo no painel de agentes para selecionar um companheiro de equipe, depois pressione Enter para visualizá-lo e digite para enviar mensagens para ele diretamente. Funciona em qualquer terminal, nenhuma configuração extra necessária.
* **Split panes**: cada companheiro de equipe recebe seu próprio painel. Você pode ver a saída de todos de uma vez e clicar em um painel para interagir diretamente. Requer tmux ou iTerm2.

<Note>
  `tmux` tem limitações conhecidas em certos sistemas operacionais e tradicionalmente funciona melhor no macOS. Usar `tmux -CC` no iTerm2 é o ponto de entrada sugerido para `tmux`.
</Note>

O padrão é `"in-process"`. Defina `"auto"` para ativar split panes quando você já estiver executando dentro de uma sessão tmux, ou quando seu terminal for iTerm2 com o CLI `it2` instalado, voltando para in-process caso contrário. A configuração `"tmux"` ativa o modo split-pane e detecta automaticamente se deve usar tmux ou iTerm2 com base no seu terminal.

Defina `"iterm2"` para usar explicitamente split panes nativos do iTerm2. Este modo requer o [CLI `it2`](https://github.com/mkusaka/it2) e mostra um erro com o comando de instalação se `it2` estiver faltando. O prompt de configuração que oferece instalar `it2` ou mudar para tmux aparece em `"auto"` ou `"tmux"` quando seu terminal é iTerm2 e tmux está disponível como fallback.

Para substituir o padrão, defina [`teammateMode`](/docs/pt/settings-reference#teammatemode) em `~/.claude/settings.json`:

```json theme={null}
{
  "teammateMode": "auto"
}
```

Para definir o modo para uma única sessão, passe como um sinalizador:

```bash theme={null}
claude --teammate-mode auto
```

O sinalizador `--teammate-mode` é experimental e não aparece em `claude --help`.

O modo split-pane requer [tmux](https://github.com/tmux/tmux/wiki) ou iTerm2 com o CLI [`it2`](https://github.com/mkusaka/it2). Para instalar manualmente:

* **tmux**: instale através do gerenciador de pacotes do seu sistema. Veja o [wiki tmux](https://github.com/tmux/tmux/wiki/Installing) para instruções específicas da plataforma.
* **iTerm2**: instale o CLI [`it2`](https://github.com/mkusaka/it2), depois ative a API Python em **iTerm2 → Settings → General → Magic → Enable Python API**.

<h3 id="specify-teammates-and-models">
  Especifique companheiros de equipe e modelos
</h3>

Claude decide o número de companheiros de equipe a gerar com base em sua tarefa, ou você pode especificar exatamente o que deseja:

```text wrap theme={null}
Spawn 4 teammates to refactor these modules in parallel. Use Sonnet for
each teammate.
```

Claude Code escolhe o modelo de cada companheiro de equipe a partir do primeiro destes que se aplica:

1. O modelo que seu prompt de spawn nomeia para esse companheiro de equipe.
2. Para um companheiro de equipe gerado a partir de uma [definição de subagente](#use-subagent-definitions-for-teammates), o `model` da definição, onde `inherit` seleciona o modelo do líder.
3. [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/pt/model-config#environment-variables), quando está definido para qualquer coisa diferente de `inherit`.
4. O modelo atual do líder.

Se você definir [`CLAUDE_CODE_SUBAGENT_MODEL_FORCE=1`](/docs/pt/sub-agents#run-every-subagent-on-one-model), as duas primeiras fontes não se aplicam. Claude Code escolhe o modelo de cada companheiro de equipe a partir de `CLAUDE_CODE_SUBAGENT_MODEL` quando está definido para qualquer coisa diferente de `inherit`, e a partir do modelo atual do líder caso contrário. Requer Claude Code v2.1.257 ou posterior.

Antes da v2.1.251, `CLAUDE_CODE_SUBAGENT_MODEL` vinha primeiro nesta ordem.

<Note>
  `teammateDefaultModel` foi removido na v2.1.234; Claude Code ignora um valor remanescente. Nomeie o modelo em seu prompt em vez disso.
</Note>

Claude Code verifica o modelo que seleciona para um companheiro de equipe contra a lista de permissões [`availableModels`](/docs/pt/model-config#restrict-model-selection) da sua organização. Quando a lista de permissões bloqueia um valor, Claude Code substitui outro modelo:

* **Alias de família como `opus`**: Na API Anthropic e Claude Platform na AWS, Claude Code executa o companheiro de equipe na versão mais recente dessa família que a lista de permissões permite. Em provedores com IDs de modelo específicos do provedor, onde a [substituição não opera](/docs/pt/model-config#restrict-model-selection), um alias bloqueado volta como qualquer outro valor bloqueado de acordo com o próximo ponto
* **Qualquer outro valor bloqueado, incluindo um alias de família em provedores onde a substituição não opera, ou um cuja família não tem versão permitida**: Claude Code executa o companheiro de equipe no modelo do líder em vez disso. Se você definir `CLAUDE_CODE_SUBAGENT_MODEL`, Claude Code tenta esse modelo primeiro, sob essas mesmas regras

Os companheiros de equipe herdam o [nível de esforço](/docs/pt/model-config#adjust-effort-level) do líder. No modo split-pane isso se aplica a partir da v2.1.186; versões anteriores não passavam o esforço da sessão do líder para companheiros de equipe em split-pane.

<h3 id="have-teammates-plan-before-implementing">
  Tenha companheiros de equipe planejando antes de implementar
</h3>

Para tarefas complexas ou arriscadas, você pode ter companheiros de equipe planejando antes de implementar. Um companheiro de equipe que Claude gera enquanto o líder está em [modo de plano](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode) funciona em modo de plano somente leitura até que seu plano esteja pronto. Mude o líder para modo de plano primeiro, depois peça pelo companheiro de equipe:

```text wrap theme={null}
Spawn an architect teammate to refactor the authentication module.
```

Quando um companheiro de equipe termina o planejamento, ele envia uma solicitação de aprovação de plano ao líder. Claude Code aprova o plano na sessão do líder assim que a solicitação chega, sem o líder revisá-lo. Os edits e comandos do companheiro de equipe ainda passam pelos prompts de permissão descritos em [Permissions](#permissions). Uma vez aprovado, o companheiro de equipe sai do modo de plano e começa a implementação.

<h3 id="talk-to-teammates-directly">
  Fale com companheiros de equipe diretamente
</h3>

Cada companheiro de equipe é uma sessão Claude Code completa e independente. Você pode enviar mensagens para qualquer companheiro de equipe diretamente para dar instruções adicionais, fazer perguntas de acompanhamento ou redirecionar sua abordagem.

* **Modo in-process**: use as teclas de seta para cima e para baixo no painel de agentes para selecionar um companheiro de equipe, depois pressione Enter para visualizar sua sessão e digite para enviar uma mensagem. Pressione `x` em um companheiro de equipe selecionado para interrompê-lo. Pressione Ctrl+T para alternar a lista de tarefas.
* **Modo split-pane**: clique em um painel de companheiro de equipe para interagir com sua sessão diretamente. Cada companheiro de equipe tem uma visualização completa de seu próprio terminal.

Enquanto você está visualizando um companheiro de equipe in-process, texto simples e [skills](/docs/pt/skills) vão para esse companheiro de equipe, mas comandos integrados ainda são executados na sessão do líder.

O modelo e modo rápido de um companheiro de equipe são fixos quando ele é gerado, portanto `/model` e `/fast` apenas alteram as configurações do líder. A partir da v2.1.199, digitar qualquer comando enquanto visualiza um companheiro de equipe mostra um aviso de que a alteração se aplica ao líder; versões anteriores a aplicavam ao líder sem indicação. `/effort` ainda se aplica aos turnos posteriores do companheiro de equipe visualizado, porque os companheiros de equipe seguem o [nível de esforço](/docs/pt/model-config#adjust-effort-level) do líder.

<h3 id="assign-and-claim-tasks">
  Atribuir e reivindicar tarefas
</h3>

A lista de tarefas compartilhada coordena o trabalho em toda a equipe. O líder cria tarefas e os companheiros de equipe as trabalham. As tarefas têm três estados: pendente, em progresso e concluída. As tarefas também podem depender de outras tarefas: uma tarefa pendente com dependências não resolvidas não pode ser reivindicada até que essas dependências sejam concluídas.

Agentes [sem as ferramentas Task](/docs/pt/tools-reference#task-tool-availability) coordenam através de mensagens em vez da lista de tarefas compartilhada.

O líder pode atribuir tarefas explicitamente ou os companheiros de equipe podem auto-reivindicar:

* **Líder atribui**: diga ao líder qual tarefa dar a qual companheiro de equipe
* **Auto-reivindicar**: após terminar uma tarefa, um companheiro de equipe pega a próxima tarefa não atribuída e desbloqueada por conta própria

A reivindicação de tarefas usa bloqueio de arquivo para evitar condições de corrida quando múltiplos companheiros de equipe tentam reivindicar a mesma tarefa simultaneamente.

<h3 id="shut-down-teammates">
  Encerrar companheiros de equipe
</h3>

Para encerrar graciosamente a sessão de um companheiro de equipe, refira-se a ele pelo nome. Por exemplo, com um companheiro de equipe chamado pesquisador:

```text wrap theme={null}
Ask the researcher teammate to shut down
```

O líder envia uma solicitação de encerramento. O companheiro de equipe pode aprovar, saindo graciosamente, ou rejeitar com uma explicação.

Os diretórios compartilhados da equipe são limpos automaticamente quando a sessão termina, portanto não há uma etapa de limpeza separada. Veja [Architecture](#architecture) para saber quais diretórios são removidos e quais persistem para sessões retomadas.

<h3 id="enforce-quality-gates-with-hooks">
  Aplicar gates de qualidade com hooks
</h3>

Use [hooks](/docs/pt/hooks) para aplicar regras quando os companheiros de equipe terminam o trabalho ou as tarefas são criadas ou concluídas:

* [`TeammateIdle`](/docs/pt/hooks#teammateidle): é executado quando um companheiro de equipe está prestes a ficar ocioso. Saia com código 2 para enviar feedback e manter o companheiro de equipe trabalhando.
* [`TaskCreated`](/docs/pt/hooks#taskcreated): é executado quando uma tarefa está sendo criada. Saia com código 2 para evitar criação e enviar feedback.
* [`TaskCompleted`](/docs/pt/hooks#taskcompleted): é executado quando uma tarefa está sendo marcada como concluída. Saia com código 2 para evitar conclusão e enviar feedback.

<h2 id="how-agent-teams-work">
  Como funcionam as equipes de agentes
</h2>

Esta seção cobre a arquitetura e a mecânica por trás das equipes de agentes. Se você quiser começar a usá-las, veja [Controle sua equipe de agentes](#control-your-agent-team) acima.

<h3 id="how-claude-starts-agent-teams">
  Como Claude inicia equipes de agentes
</h3>

Para iniciar uma equipe, peça ao Claude por companheiros de equipe. Claude lança um companheiro de equipe quando chama a [ferramenta Agent](/docs/pt/tools-reference) com um [`name`](/docs/pt/sub-agents#subagent-names) enquanto as equipes de agentes estão habilitadas, a menos que a chamada seja um [fork](/docs/pt/sub-agents#fork-the-current-conversation) ou passe `isolation` na própria chamada. Claude Code não pede que você confirme o lançamento.

Claude também nomeia subagentes ordinários por conta própria para que possa enviá-los mensagens depois. Essas chamadas seguem a mesma regra, portanto as equipes podem se formar mesmo quando você não pediu por uma. Se você quiser subagentes em vez disso, [desative as equipes de agentes](#claude-spawns-teammates-instead-of-subagents).

<h3 id="architecture">
  Arquitetura
</h3>

Uma equipe de agentes consiste em:

| Componente    | Papel                                                                                          |
| :------------ | :--------------------------------------------------------------------------------------------- |
| **Team lead** | A sessão Claude Code principal que gera companheiros de equipe e coordena o trabalho           |
| **Teammates** | Instâncias Claude Code separadas que cada uma trabalha em tarefas atribuídas                   |
| **Task list** | Lista compartilhada de itens de trabalho que os companheiros de equipe reivindicam e completam |
| **Mailbox**   | Sistema de mensagens para comunicação entre agentes                                            |

A caixa de correio de cada agente é um arquivo JSON em `~/.claude/teams/{team-name}/inboxes/{agent-name}.json`. Claude Code valida cada entrada quando lê um arquivo de caixa de correio. Entradas que não correspondem ao formato de mensagem são relatadas como erros e removidas do arquivo; as mensagens válidas ainda são entregues. Antes da v2.1.207, uma única entrada de caixa de correio malformada causava um erro repetido a cada segundo e bloqueava a entrega para essa caixa de correio até que você deletasse o arquivo manualmente.

Claude Code relata uma mensagem como enviada apenas quando a escrita no arquivo de caixa de correio do destinatário é bem-sucedida, seja a mensagem texto simples ou uma mensagem de protocolo estruturada, como uma aprovação de plano ou solicitação de encerramento. Quando a escrita falha, por exemplo porque o disco está cheio ou o diretório de caixa de correio não é gravável, o agente remetente recebe um erro e nada é enviado. Veja [Failed to write to a teammate's inbox](/docs/pt/errors#failed-to-write-to-a-teammate-inbox) para as mensagens de erro e etapas de recuperação.

Claude Code gerencia dependências de tarefas automaticamente: quando um companheiro de equipe completa uma tarefa da qual outras tarefas dependem, ele desbloqueia as tarefas dependentes sem nenhuma ação sua.

Equipes e tarefas são armazenadas localmente sob um nome derivado da sessão. O nome é `session-` seguido pelos primeiros oito caracteres do ID da sessão:

* **Team config**: `~/.claude/teams/{team-name}/config.json`
* **Task list**: `~/.claude/tasks/{team-name}/`

Claude Code gera ambas automaticamente na inicialização da sessão e as atualiza conforme os companheiros de equipe entram, ficam ociosos ou saem. O diretório de configuração da equipe é removido quando a sessão termina. O diretório da lista de tarefas persiste localmente e nunca é carregado, portanto as sessões retomadas mantêm suas tarefas. A retenção é governada pelo mesmo [`cleanupPeriodDays`](/docs/pt/settings-reference#cleanupperioddays) que você já controla para transcrições de sessão, seguindo as [regras de limpeza de retenção](/docs/pt/claude-directory#cleaned-up-automatically).

A configuração da equipe contém estado de tempo de execução, como IDs de sessão e IDs de painel tmux, portanto não a edite manualmente ou a crie previamente: suas alterações são sobrescritas na próxima atualização de estado.

Para definir papéis de companheiros de equipe reutilizáveis, use [definições de subagent](#use-subagent-definitions-for-teammates) em vez disso.

A configuração da equipe contém um array `members` com o nome de cada companheiro de equipe e ID do agente. A entrada do líder sempre carrega o tipo de agente `team-lead`. A entrada de um companheiro de equipe carrega qualquer tipo de agente que o líder nomeou ao gerá-lo, seja um [tipo integrado](/docs/pt/sub-agents#built-in-subagents) ou uma [definição de subagent](#use-subagent-definitions-for-teammates), e omite o campo quando o líder não nomeou nenhum. Os companheiros de equipe podem ler este arquivo para descobrir outros membros da equipe.

Não há equivalente em nível de projeto da configuração da equipe. Um arquivo como `.claude/teams/teams.json` no seu diretório de projeto não é reconhecido como configuração; Claude o trata como um arquivo ordinário.

<h3 id="use-subagent-definitions-for-teammates">
  Use subagent definitions for teammates
</h3>

Ao gerar um companheiro de equipe em qualquer modo de exibição, você pode referenciar um tipo de [subagent](/docs/pt/sub-agents) do projeto, usuário ou escopo de subagent gerenciado [subagent scope](/docs/pt/sub-agents#choose-the-subagent-scope). Isso permite que você defina um papel uma vez, como um revisor de segurança ou executor de testes, e o reutilize tanto como um subagent delegado quanto como um companheiro de equipe de equipe de agentes.

Para usar uma definição de subagent, mencione-a pelo nome ao pedir ao Claude para gerar o companheiro de equipe:

```text wrap theme={null}
Spawn a teammate using the security-reviewer agent type to audit the auth module.
```

Claude Code lê a definição de subagent que você nomeou e aplica estas partes dela ao companheiro de equipe. Onde uma parte depende do [modo de exibição](#choose-a-display-mode) do companheiro de equipe, a entrada diz assim:

* **`tools`**: Claude Code limita o companheiro de equipe às ferramentas na lista `tools` da definição. Para um companheiro de equipe em processo, Claude Code adiciona `SendMessage` a essa lista, e em uma [sessão que tem as ferramentas Task](/docs/pt/tools-reference#task-tool-availability) ele também adiciona `TaskCreate`, `TaskGet`, `TaskList` e `TaskUpdate`.
* **`model`**: Claude Code usa o `model` da definição em qualquer modo de exibição quando seu prompt de geração não nomeia um. Veja [como Claude Code escolhe o modelo de um companheiro de equipe](#specify-teammates-and-models).
* **Body**: para um companheiro de equipe em processo, Claude Code anexa o corpo da definição ao seu prompt do sistema padrão como instruções adicionais. Para um companheiro de equipe em painel dividido, Claude Code usa o corpo no lugar do seu prompt do sistema padrão.
* **`skills`**: Claude Code não aplica o `skills` da definição a um companheiro de equipe em qualquer modo de exibição. O companheiro de equipe carrega skills de suas configurações de projeto e usuário.
* **`mcpServers`**: para um companheiro de equipe em painel dividido, Claude Code aplica o `mcpServers` da definição sob as [regras para esse campo](/docs/pt/sub-agents#scope-mcp-servers-to-a-subagent), que cobrem uma sessão iniciada com `--agent` também. Um companheiro de equipe em processo ignora o campo e carrega servidores MCP de suas configurações de projeto e usuário.

Quando Claude envia uma mensagem para um companheiro de equipe em processo que não está mais em execução, Claude Code o traz de volta na mesma sessão, restaura qualquer conversa salva para ele e lhe dá a mensagem como seu próximo prompt. Depois que você retoma uma sessão, os companheiros de equipe não são trazidos de volta dessa forma, de acordo com [a limitação de retomada](#limitations).

Para um companheiro de equipe que ele traz de volta, Claude Code reaplicará uma definição que veio do diretório `.claude/agents/` de um projeto ou de um diretório `--add-dir` apenas se você tiver [confiado na pasta em que o arquivo do agente está](/docs/pt/permissions#what-runs-before-you-trust-a-folder). Confiar em uma pasta pai não conta. Até então, o companheiro de equipe volta com nenhuma das ferramentas ou instruções da definição, mantendo apenas as ferramentas que Claude Code adiciona a cada companheiro de equipe em processo. Veja [a definição de agente do companheiro de equipe não foi restaurada](/docs/pt/errors#teammate-agent-definition-not-restored) para o texto do aviso.

<h3 id="permissions">
  Permissões
</h3>

Os companheiros de equipe começam com o modo de permissão do líder, exceto o modo [`dontAsk`](/docs/pt/permission-modes#allow-only-pre-approved-tools-with-dontask-mode), que eles não herdam. Se o líder for executado com `--dangerously-skip-permissions`, todos os companheiros de equipe também. Após gerar, você pode alterar o modo de permissão de um companheiro de equipe individual, mas não pode definir modos de permissão por companheiro de equipe no tempo de geração.

Os prompts de permissão de companheiros de equipe aparecem na sessão líder, portanto aprove-os lá você mesmo. [Aprovação de plano](#have-teammates-plan-before-implementing) é a exceção projetada: a sessão líder concede aprovações de plano de companheiros de equipe sem um prompt separado para você.

<h4 id="messages-between-agents">
  Mensagens entre agentes
</h4>

Quando um agente envia uma mensagem para outro sobre `SendMessage`, Claude Code informa ao agente receptor que a mensagem veio de outra sessão Claude, não de você. Um companheiro de equipe não pode aprovar um prompt de permissão ou fornecer consentimento em seu nome, e um companheiro de equipe que foi negado uma ação não pode retransmiti-la para outro companheiro de equipe para contornar a verificação. As mesmas regras se aplicam a uma mensagem que chega de [uma de suas outras sessões Claude Code](/docs/pt/cross-session-messaging#how-a-session-treats-an-incoming-message), fora da equipe inteiramente.

Em [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), o classificador aplica duas verificações a mensagens entre agentes:

* Ele trata uma reivindicação de aprovação retransmitida de outro agente como entrada não confiável em vez de confirmação de você.
* Ele revisa cada mensagem antes que Claude Code a entregue, seja uma mensagem simples ou uma mensagem de protocolo estruturada, como uma solicitação de encerramento ou resposta de aprovação de plano. Uma mensagem que ele bloqueia nunca chega ao destinatário.

<h3 id="context-and-communication">
  Context e comunicação
</h3>

Cada companheiro de equipe tem sua própria context window. Quando gerado, um companheiro de equipe carrega o mesmo contexto de projeto que uma sessão regular: CLAUDE.md, MCP servers e skills. Ele também recebe o prompt de geração do líder. O histórico de conversa do líder não é transferido.

**Como os companheiros de equipe compartilham informações:**

* **Entrega automática de mensagens**: quando os companheiros de equipe enviam mensagens, elas são entregues automaticamente aos destinatários. O líder não precisa fazer polling para atualizações.
* **Notificações de ociosidade**: quando um companheiro de equipe termina e para, ele notifica automaticamente o líder e inclui sua resposta final na notificação. Um companheiro de equipe cuja vez termina em um erro de API notifica o líder que falhou e inclui o texto do erro.
* **Lista de tarefas compartilhada**: [agentes que têm as ferramentas Task](/docs/pt/tools-reference#task-tool-availability) podem ver o status da tarefa e reivindicar trabalho disponível.
* **Mensagens de companheiros de equipe**: envie uma mensagem para um companheiro de equipe específico pelo nome. Para alcançar todos, envie uma mensagem por destinatário.

O líder atribui a cada companheiro de equipe um nome quando o gera, e qualquer companheiro de equipe pode enviar mensagens para qualquer outro por esse nome. Para obter nomes previsíveis que você possa referenciar em prompts posteriores, diga ao líder como chamar cada companheiro de equipe em sua instrução de geração.

<h3 id="token-usage">
  Uso de tokens
</h3>

Equipes de agentes usam significativamente mais tokens do que uma única sessão. Cada companheiro de equipe tem sua própria context window, e o uso de tokens escala com o número de companheiros de equipe ativos. Para pesquisa, revisão e trabalho de novos recursos, os tokens extras geralmente valem a pena. Para tarefas rotineiras, uma única sessão é mais econômica. Veja [custos de token de equipe de agentes](/docs/pt/costs#agent-team-token-costs) para orientação de uso.

A solicitação de um companheiro de equipe em processo fica fora do bucket TTL de cache da conversa principal]\(/pt/prompt-caching#which-ttl-each-request-gets), portanto seu cache dura cinco minutos por padrão, inclusive em uma assinatura Claude. Para mantê-lo por uma hora, defina [`subagentPromptCacheTtl`](/docs/pt/settings-reference#subagentpromptcachettl) como `1h`. A API cobra gravações de cache de 1 hora a uma taxa mais alta.

<h2 id="use-case-examples">
  Exemplos de casos de uso
</h2>

Estes exemplos mostram como as equipes de agentes lidam com tarefas onde a exploração paralela adiciona valor.

<h3 id="run-a-parallel-code-review">
  Executar uma revisão de código paralela
</h3>

Um único revisor tende a gravitar em torno de um tipo de problema por vez. Dividir critérios de revisão em domínios independentes significa que segurança, desempenho e cobertura de testes recebem atenção completa simultaneamente. O prompt atribui a cada companheiro de equipe uma lente distinta para que não se sobreponham:

```text wrap theme={null}
Spawn three teammates to review PR #142:
- One focused on security implications
- One checking performance impact
- One validating test coverage
Have them each review and report findings.
```

Cada revisor trabalha a partir do mesmo PR, mas aplica um filtro diferente. O líder sintetiza descobertas em todos os três após terminarem.

<h3 id="investigate-with-competing-hypotheses">
  Investigar com hipóteses concorrentes
</h3>

Quando a causa raiz é incerta, um único agente tende a encontrar uma explicação plausível e parar de procurar. O prompt combate isso tornando os companheiros de equipe explicitamente adversários: o trabalho de cada um não é apenas investigar sua própria teoria, mas desafiar as dos outros.

```text wrap theme={null}
Users report the app exits after one message instead of staying connected.
Spawn 5 agent teammates to investigate different hypotheses. Have them talk to
each other to try to disprove each other's theories, like a scientific
debate. Update the findings doc with whatever consensus emerges.
```

A estrutura de debate é o mecanismo-chave aqui. A investigação sequencial sofre de ancoragem: uma vez que uma teoria é explorada, a investigação subsequente é enviesada em relação a ela.

Com múltiplos investigadores independentes tentando ativamente desprovar uns aos outros, a teoria que sobrevive é muito mais provável de ser a causa raiz real.

<h2 id="best-practices">
  Melhores práticas
</h2>

<h3 id="give-teammates-enough-context">
  Dê aos companheiros de equipe contexto suficiente
</h3>

Os companheiros de equipe carregam contexto de projeto automaticamente, incluindo CLAUDE.md, MCP servers e skills, mas não herdam o histórico de conversa do líder. Veja [Context e comunicação](#context-and-communication) para detalhes. Inclua detalhes específicos da tarefa no prompt de geração:

```text wrap theme={null}
Spawn a security reviewer teammate with the prompt: "Review the authentication module
at src/auth/ for security vulnerabilities. Focus on token handling, session
management, and input validation. The app uses JWT tokens stored in
httpOnly cookies. Report any issues with severity ratings."
```

<h3 id="choose-an-appropriate-team-size">
  Escolha um tamanho de equipe apropriado
</h3>

Não há limite rígido no número de companheiros de equipe, mas restrições práticas se aplicam:

* **Custos de token escalam linearmente**: cada companheiro de equipe tem sua própria context window e consome tokens independentemente. Veja [custos de token de equipe de agentes](/docs/pt/costs#agent-team-token-costs) para detalhes.
* **Sobrecarga de coordenação aumenta**: mais companheiros de equipe significa mais comunicação, coordenação de tarefas e potencial para conflitos
* **Retornos decrescentes**: além de um certo ponto, companheiros de equipe adicionais não aceleram o trabalho proporcionalmente

Comece com 3-5 companheiros de equipe para a maioria dos fluxos de trabalho. Isso equilibra o trabalho paralelo com coordenação gerenciável. Se você tiver 15 tarefas independentes, 3 companheiros de equipe é um bom ponto de partida.

Escale apenas quando o trabalho se beneficiar de ter companheiros de equipe trabalhando simultaneamente. Três companheiros de equipe focados frequentemente superam cinco dispersos.

<h3 id="size-tasks-appropriately">
  Dimensione tarefas apropriadamente
</h3>

* **Muito pequeno**: sobrecarga de coordenação excede o benefício
* **Muito grande**: companheiros de equipe trabalham muito tempo sem check-ins, aumentando o risco de esforço desperdiçado
* **Bem dimensionado**: unidades auto-contidas que produzem um entregável claro, como uma função, um arquivo de teste ou uma revisão

<Tip>
  O líder divide o trabalho em tarefas e as atribui aos companheiros de equipe automaticamente. Se não estiver criando tarefas suficientes, peça a ele para dividir o trabalho em pedaços menores. Ter 5-6 tarefas por companheiro de equipe mantém todos produtivos e permite que o líder reatribua trabalho se alguém ficar preso.
</Tip>

<h3 id="wait-for-teammates-to-finish">
  Espere os companheiros de equipe terminarem
</h3>

Às vezes, o líder começa a implementar tarefas em vez de esperar pelos companheiros de equipe. Se você notar isso:

```text wrap theme={null}
Wait for your teammates to complete their tasks before proceeding
```

<h3 id="start-with-research-and-review">
  Comece com pesquisa e revisão
</h3>

Se você é novo em equipes de agentes, comece com tarefas que têm limites claros e não requerem escrever código: revisar um PR, pesquisar uma biblioteca ou investigar um bug. Essas tarefas mostram o valor da exploração paralela sem os desafios de coordenação que vêm com a implementação paralela.

<h3 id="avoid-file-conflicts">
  Evite conflitos de arquivo
</h3>

Dois companheiros de equipe editando o mesmo arquivo leva a sobrescrita. Divida o trabalho para que cada companheiro de equipe possua um conjunto diferente de arquivos.

<h3 id="monitor-and-steer">
  Monitore e direcione
</h3>

Verifique o progresso dos companheiros de equipe, redirecione abordagens que não estão funcionando e sintetize descobertas conforme chegam. Deixar uma equipe executar sem supervisão por muito tempo aumenta o risco de esforço desperdiçado.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="teammates-not-appearing">
  Companheiros de equipe não aparecem
</h3>

Se os companheiros de equipe não aparecerem depois que você pedir ao Claude para criar uma equipe:

* No modo in-process, os companheiros de equipe aparecem no painel de agentes abaixo da entrada de prompt. Use as teclas de seta para cima e para baixo para selecionar um e pressione Enter para visualizá-lo.
* Uma linha de companheiro de equipe que desapareceu após ficar inativa foi ocultada, não interrompida. As linhas inativas se ocultam 30 segundos após o painel inteiro ficar inativo e reaparecem na próxima vez do companheiro de equipe. Quando mais de três companheiros de equipe estão inativos, suas linhas excedentes se recolhem em uma única linha `N idle agents` que Enter expande. Envie uma mensagem ao companheiro de equipe pelo nome para trazer uma linha oculta de volta.
* Verifique se a tarefa que você deu ao Claude era complexa o suficiente para justificar uma equipe. Claude decide se deve gerar companheiros de equipe com base na tarefa.
* Se você explicitamente solicitou split panes, certifique-se de que tmux está instalado e disponível no seu PATH:
  ```bash theme={null}
  which tmux
  ```
* Para iTerm2, verifique se o CLI `it2` está instalado e a API Python está ativada nas preferências do iTerm2.

<h3 id="claude-spawns-teammates-instead-of-subagents">
  Claude gera companheiros de equipe em vez de subagentes
</h3>

Enquanto as equipes de agentes estão ativadas, um subagente que Claude nomeia na sessão do líder é iniciado como um companheiro de equipe. Claude [pode nomear subagentes por conta própria](#how-claude-starts-agent-teams), então isso pode acontecer durante delegação que você nunca enquadrou como trabalho em equipe.

Para fazer com que subagentes nomeados sejam iniciados como subagentes novamente, desative as equipes de agentes definindo `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` como `0`:

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "0"
  }
}
```

Você não precisa iniciar uma nova sessão: Claude Code reaplica os valores `env` do arquivo de configurações à sessão em execução quando você salva, e relê a variável cada vez que Claude gera um subagente, então o próximo subagente que Claude nomeia é iniciado como um subagente.

Definir a variável como `0` no seu `settings.json` do usuário substitui uma exportação de shell. Outras fontes de configurações ainda podem ativar equipes de agentes:

* **Arquivos de configurações de precedência superior**: configurações de projeto, configurações locais e uma carga `--settings` se aplicam após as configurações do usuário, então uma entrada `env` que define a variável como `1` em qualquer uma delas vence. Veja [Precedência de configurações](/docs/pt/settings#settings-precedence).
* **Configurações gerenciadas**: [configurações gerenciadas](/docs/pt/server-managed-settings) se aplicam após todas as outras fontes. Se sua organização ativa equipes de agentes lá, peça ao seu administrador para alterar o valor gerenciado.

Após a alteração, Claude ainda pode nomear subagentes, e o nome continua funcionando como um endereço [`SendMessage`](/docs/pt/sub-agents#resume-subagents). Claude recebe o resultado de cada subagente quando ele é concluído.

<h3 id="too-many-permission-prompts">
  Muitos prompts de permissão
</h3>

Solicitações de permissão de companheiros de equipe surgem para o líder, o que pode criar atrito. Pré-aprove operações comuns nas suas [configurações de permissão](/docs/pt/permissions) antes de gerar companheiros de equipe para reduzir interrupções.

<h3 id="agents-stopping-early">
  Agentes parando cedo
</h3>

Os companheiros de equipe podem parar após encontrar erros em vez de se recuperar. Verifique sua saída selecionando o companheiro de equipe no painel de agentes e pressionando Enter no modo in-process, ou clicando no painel no modo split, depois:

* Dê a eles instruções adicionais diretamente
* Gere um companheiro de equipe de substituição para continuar o trabalho

Uma mensagem do líder ou de outro companheiro de equipe acorda um companheiro de equipe in-process que está aguardando para tentar novamente uma solicitação de API com falha, para que ele tente novamente imediatamente em vez de aguardar o atraso de repetição completo.

<h3 id="orphaned-tmux-sessions">
  Sessões tmux órfãs
</h3>

Se uma sessão tmux persistir após a sessão do Claude Code terminar, pode não ter sido totalmente limpa. Liste as sessões e encerre a criada pela equipe:

```bash theme={null}
tmux ls
tmux kill-session -t <session-name>
```

<h2 id="limitations">
  Limitações
</h2>

Equipes de agentes são experimentais. Limitações atuais a serem observadas:

* **Sem retomada de sessão com companheiros de equipe in-process**: `/resume` e `/rewind` não restauram companheiros de equipe in-process. Após retomar uma sessão, o líder pode tentar enviar mensagens para companheiros de equipe que não existem mais. Se isso acontecer, diga ao líder para gerar novos companheiros de equipe.
* **Status da tarefa pode ficar atrasado**: os companheiros de equipe às vezes falham em marcar tarefas como concluídas, o que bloqueia tarefas dependentes. Se uma tarefa parecer presa, verifique se o trabalho está realmente pronto e atualize o status da tarefa manualmente ou diga ao líder para dar um empurrão ao companheiro de equipe.
* **Encerramento pode ser lento**: os companheiros de equipe terminam sua solicitação atual ou chamada de ferramenta antes de encerrar, o que pode levar tempo.
* **Uma equipe por sessão**: uma sessão tem exatamente uma equipe, com escopo para essa sessão. Você não pode criar equipes nomeadas adicionais ou compartilhar uma equipe entre sessões.
* **Sem equipes aninhadas**: os companheiros de equipe não podem gerar seus próprios companheiros de equipe. Apenas o líder pode gerenciar a equipe.
* **Sem subagentes em segundo plano de companheiros de equipe in-process**: os próprios subagentes de um companheiro de equipe in-process são executados em primeiro plano, porque o trabalho em segundo plano de um companheiro de equipe não pode sobreviver ao processo do líder. Claude Code retorna um erro quando um companheiro de equipe gera um subagente cuja definição define `background: true`. Uma solicitação `run_in_background: true` de um companheiro de equipe também falha, seja com um erro ou executando silenciosamente em primeiro plano, conforme descrito em [como Claude Code escolhe primeiro plano ou segundo plano](/docs/pt/sub-agents#run-subagents-in-foreground-or-background). Subagentes lançados da conversa principal seguem o [padrão de segundo plano](/docs/pt/sub-agents#run-subagents-in-foreground-or-background).
* **Líder é fixo**: a sessão principal é o líder por sua vida útil. Você não pode promover um companheiro de equipe a líder ou transferir liderança.
* **Permissões definidas no tempo de geração**: os companheiros de equipe começam com o modo de permissão descrito em [Permissões](#permissions). Você pode alterar o modo de permissão de um companheiro de equipe individual após gerar, mas não pode definir modos de permissão por companheiro de equipe no tempo de geração.
* **Split panes requerem tmux ou iTerm2**: o modo in-process padrão funciona em qualquer terminal. O modo split-pane não é suportado no terminal integrado do VS Code, Windows Terminal ou Ghostty.

<h2 id="next-steps">
  Próximos passos
</h2>

Explore abordagens relacionadas para trabalho paralelo e delegação:

* **Delegação leve**: [subagents](/docs/pt/sub-agents) geram agentes auxiliares para pesquisa ou verificação dentro de sua sessão, melhor para tarefas que não precisam de coordenação entre agentes
* **Mensagens entre suas próprias sessões**: [cross-session messaging](/docs/pt/cross-session-messaging) permite que Claude passe descobertas entre as sessões que você executa
* **Sessões paralelas manuais**: [Git worktrees](/docs/pt/worktrees) permitem que você execute múltiplas sessões Claude Code você mesmo sem coordenação de equipe automatizada
