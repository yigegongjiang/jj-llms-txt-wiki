> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orquestre subagentos em escala com fluxos de trabalho dinâmicos

> Fluxos de trabalho dinâmicos orquestram muitos subagentos a partir de um script que Claude escreve e você pode executar novamente. Use-os para auditorias de base de código, grandes migrações e pesquisa com verificação cruzada.

<Note>
  Fluxos de trabalho dinâmicos estão disponíveis em todos os planos pagos, com acesso à API Anthropic, e no Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. No Pro, ative-os na linha Dynamic workflows em `/config`.
</Note>

Um fluxo de trabalho dinâmico é um script JavaScript que orquestra muitos [subagentos](/docs/pt/sub-agents) de uma vez. Claude escreve o script para a tarefa que você descreve, e um runtime o executa em segundo plano enquanto sua sessão permanece responsiva.

Recorra a um fluxo de trabalho quando uma tarefa precisar de mais agentes do que uma conversa pode coordenar, ou quando você quiser que a orquestração seja codificada como um script que você possa ler e executar novamente. Os exemplos incluem uma varredura de bugs em toda a base de código, uma migração de 500 arquivos, uma pergunta de pesquisa que precisa ter fontes verificadas cruzadamente uma contra a outra, e um plano difícil que vale a pena ser elaborado de vários ângulos independentes antes de você se comprometer com um.

<h2 id="when-to-use-a-workflow">
  Quando usar um fluxo de trabalho
</h2>

[Subagentos](/docs/pt/sub-agents), [skills](/docs/pt/skills), [equipes de agentes](/docs/pt/agent-teams) e fluxos de trabalho podem todos executar uma tarefa com várias etapas. A diferença é quem mantém o plano:

|                                         | Subagentos                          | Skills                       | Equipes de agentes                                  | Fluxos de trabalho                         |
| :-------------------------------------- | :---------------------------------- | :--------------------------- | :-------------------------------------------------- | :----------------------------------------- |
| O que é                                 | Um worker Claude que spawna         | Instruções que Claude segue  | Um agente líder supervisionando sessões entre pares | Um script que o runtime executa            |
| Quem decide o que é executado a seguir  | Claude, turno por turno             | Claude, seguindo o prompt    | O agente líder, turno por turno                     | O script                                   |
| Onde os resultados intermediários vivem | Janela de contexto de Claude        | Janela de contexto de Claude | Uma lista de tarefas compartilhada                  | Variáveis de script                        |
| O que é repetível                       | A definição do worker               | As instruções                | A definição da equipe                               | A orquestração em si                       |
| Escala                                  | Algumas tarefas delegadas por turno | Igual aos subagentos         | Um punhado de pares de longa duração                | Dezenas a centenas de agentes por execução |
| Interrupção                             | Reinicia o turno                    | Reinicia o turno             | Os companheiros de equipe continuam executando      | Retomável na mesma sessão                  |

Um fluxo de trabalho move o plano para o código. Com subagentos, skills e equipes de agentes, Claude é o orquestrador: ele decide turno por turno o que spawnar ou atribuir a seguir, e cada resultado chega à janela de contexto. Um script de fluxo de trabalho mantém o loop, a ramificação e os resultados intermediários em si, então o contexto de Claude contém apenas a resposta final.

Mover o plano para o código também permite que um fluxo de trabalho aplique um padrão de qualidade repetível, não apenas execute mais agentes: ele pode ter agentes independentes revisando adversarialmente as descobertas um do outro antes de serem relatadas, ou elaborar um plano de vários ângulos e pesá-los um contra o outro, para que você obtenha um resultado mais confiável do que uma única passagem.

<h2 id="run-a-bundled-workflow">
  Executar um workflow agrupado
</h2>

A forma mais rápida de ver um workflow em ação é executar `/deep-research`, o [workflow integrado](#bundled-workflows) que Claude Code inclui para investigar uma pergunta em muitas fontes. Você verá agentes trabalhando através de um conjunto de fases em segundo plano enquanto sua sessão permanece livre, e obterá um relatório no final em vez de uma transcrição turno a turno.

<Steps>
  <Step title="Executar o workflow">
    Execute `/deep-research` com uma pergunta que você deseja investigar. Ele distribui buscas na web em vários ângulos, busca e verifica cruzadamente as fontes que encontra, e sintetiza um relatório citado.

    ```text wrap theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Permitir workflows">
    Claude Code pergunta se deve permitir o workflow. Selecione **Sim** para continuar. O prompt exato depende do seu modo de permissão. Consulte [Aprovar o plano antes de ser executado](#approve-the-plan-before-it-runs) para as opções por modo.
  </Step>

  <Step title="Acompanhar o progresso">
    A execução começa em segundo plano. Execute `/workflows`, use as setas para selecionar a execução e pressione Enter para abrir sua visualização de progresso:

    ```text wrap theme={null}
    /workflows
    ```

    A visualização mostra cada fase com sua contagem de agentes, total de tokens e tempo decorrido. Aprofunde-se em qualquer fase para ver seus agentes e o que cada um encontrou. Consulte [Acompanhar a execução](#watch-the-run) para o conjunto completo de controles.

    Você também pode acompanhar a partir do painel de tarefas abaixo da caixa de entrada: um resumo de progresso de uma linha aparece lá enquanto a execução está em andamento. Pressione a seta para baixo para focá-lo e depois Enter para expandir.
  </Step>

  <Step title="Ler o relatório">
    Quando a execução termina, o relatório chega em sua sessão. Ele cita as fontes de cada afirmação, com afirmações que não sobreviveram à verificação cruzada já filtradas.

    Quando os agentes verificadores não conseguem verificar uma afirmação, como após um limite de taxa ou erro de API, o relatório lista essa afirmação como não verificada em vez de contá-la como refutada.
  </Step>
</Steps>

Para executar um workflow para sua própria tarefa, [peça a Claude para escrever um](#have-claude-write-a-workflow), e uma vez que uma execução faça o que você desejava, você pode [salvá-lo](#save-the-workflow-for-reuse) como um comando seu.

<h3 id="bundled-workflows">
  Workflows agrupados
</h3>

Claude Code inclui `/deep-research` como um workflow integrado:

| Comando                     | O que faz                                                                                                                                                                                                                                                                                                                                     |
| :-------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/deep-research <question>` | Distribui buscas na web em uma pergunta em vários ângulos, busca e verifica cruzadamente as fontes que encontra, vota em cada afirmação e retorna um relatório citado com afirmações que não sobreviveram à verificação cruzada filtradas. Requer que a [ferramenta WebSearch](/docs/pt/tools-reference#websearch-tool-behavior) esteja disponível |

`/deep-research` é executado apenas quando você o invoca.

[Workflows que você salva](#save-the-workflow-for-reuse) você mesmo se tornam comandos da mesma forma e aparecem na autocompletar `/` junto com os agrupados.

<h3 id="watch-the-run">
  Acompanhar a execução
</h3>

Workflows são executados em segundo plano, portanto a sessão permanece responsiva enquanto os agentes trabalham. Execute `/workflows` a qualquer momento para listar workflows em execução e concluídos, depois selecione um para abrir sua visualização de progresso.

A visualização de progresso mostra cada fase com suas contagens de agentes, totais de tokens e tempo decorrido. O rodapé lista a chave para cada ação:

| Chave          | Ação                                                                                                       |
| :------------- | :--------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`      | Selecionar uma fase ou agente                                                                              |
| `Enter` ou `→` | Aprofundar-se na fase selecionada e depois no detalhe de um agente. No detalhe, `Enter` expande ou recolhe |
| `Esc` ou `←`   | Voltar um nível. Na v2.1.203 até v2.1.205, `←` não recuou de uma fase ou agente; use `Esc` nessas versões  |
| `j` / `k`      | Rolar dentro do detalhe do agente quando ele transborda                                                    |
| `f`            | Filtrar a lista de agentes na fase selecionada por status. Pressione novamente para ciclar                 |
| `p`            | Pausar ou retomar a execução                                                                               |
| `x`            | Parar o agente selecionado ou parar todo o workflow quando o foco está na execução                         |
| `r`            | Reiniciar o agente em execução selecionado                                                                 |
| `s`            | [Salvar](#save-the-workflow-for-reuse) o script da execução como um comando                                |

O detalhe do agente lista o prompt do agente, suas chamadas de ferramentas recentes e seu resultado. Cada chamada mostra seu estado, como ainda em execução ou falha. Quando o agente mantém uma lista de tarefas própria, o detalhe a mostra também, com o status de cada tarefa.

Pressione `Enter` para expandir o detalhe. O prompt e o resultado então aparecem em sua totalidade, e cada chamada listada mostra sua entrada e o início de seu resultado.

<h2 id="have-claude-write-a-workflow">
  Fazer Claude escrever um fluxo de trabalho
</h2>

Você pode fazer Claude escrever um fluxo de trabalho para sua tarefa de duas maneiras:

* [Peça um fluxo de trabalho](#ask-for-a-workflow-in-your-prompt) em seu prompt, seja com suas próprias palavras ou incluindo a palavra-chave `ultracode`, e Claude escreve um para a tarefa.
* [Deixe Claude decidir com ultracode](#let-claude-decide-with-ultracode): defina `/effort ultracode` e Claude planeja um fluxo de trabalho para cada tarefa substancial na sessão.

Você também pode executar um comando de fluxo de trabalho que já existe: um [fluxo de trabalho agrupado](#bundled-workflows) como `/deep-research`, ou um que você [salvou](#save-the-workflow-for-reuse).

<h3 id="ask-for-a-workflow-in-your-prompt">
  Peça um fluxo de trabalho em seu prompt
</h3>

Para executar uma única tarefa como um fluxo de trabalho sem alterar o nível de esforço da sessão, inclua a palavra-chave `ultracode` em seu prompt. Pedir com suas próprias palavras, por exemplo "use um fluxo de trabalho" ou "execute um fluxo de trabalho", também funciona: Claude trata uma solicitação direta como o mesmo opt-in.

```text wrap theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code destaca a palavra-chave em sua entrada e Claude escreve um script de fluxo de trabalho para a tarefa em vez de trabalhar através dela turno por turno. A palavra-chave apenas escolhe como Claude estrutura o trabalho: as chamadas de ferramentas dos agentes recebem as mesmas verificações de permissão e [sandboxing](/docs/pt/sandboxing) como qualquer outra chamada de ferramenta na sessão.

Se a execução fez o que você queria, você pode [salvá-la como um comando](#save-the-workflow-for-reuse) depois. Se você já tem um orquestrador construído de outra forma, como uma pasta de prompts de subagentos ou uma skill que distribui trabalho, você pode apontar Claude para ele e pedir um fluxo de trabalho que faça a mesma coisa.

<h4 id="dismiss-or-turn-off-the-keyword">
  Descartar ou desativar a palavra-chave
</h4>

Se você não pretendia iniciar um fluxo de trabalho, pressione `Option+W` no macOS ou `Alt+W` no Windows e Linux para descartar o destaque para este prompt, ou pressione backspace enquanto o cursor está logo após a palavra-chave destacada. Para impedir que a palavra-chave dispare, desative o gatilho de palavra-chave Ultracode em `/config`.

<h4 id="where-the-keyword-works">
  Onde a palavra-chave funciona
</h4>

A palavra-chave é um opt-in apenas em um prompt que você digita você mesmo: no prompt interativo, em um painel de extensão IDE, em um cliente [Remote Control](/docs/pt/remote-control), ou em uma aplicação Agent SDK que marca a [`origin`](/docs/pt/agent-sdk/typescript#sdkmessageorigin) de sua entrada de teclado como `{ kind: "human" }`. Ela não inicia um fluxo de trabalho quando chega à sessão de outra forma:

* um prompt passado com `-p`
* um prompt que uma aplicação Agent SDK envia sem marcá-lo como entrada humana
* um prompt de tarefa agendada
* uma carga útil de webhook ou comentário de pull request retransmitido para a conversa

<Note>
  Antes da v2.1.210, a palavra-chave iniciava um fluxo de trabalho de qualquer uma dessas rotas também, incluindo uma carga útil de webhook ou comentário de pull request retransmitido para a conversa.
</Note>

<h3 id="let-claude-decide-with-ultracode">
  Deixe Claude decidir com ultracode
</h3>

Ultracode é uma configuração de Claude Code que combina `xhigh` [esforço de raciocínio](/docs/pt/model-config#adjust-effort-level) com orquestração automática de fluxo de trabalho. Com ele ativado, Claude planeja um fluxo de trabalho para cada tarefa substancial em vez de esperar você pedir.

```text wrap theme={null}
/effort ultracode
```

Para iniciar uma sessão com ultracode já ativado, inicie com `claude --effort ultracode`. Requer Claude Code v2.1.203 ou posterior.

Para ativá-lo enquanto você escolhe um modelo, mova o controle deslizante de esforço do seletor `/model` para `ultracode` com as teclas de seta. [Ajustar nível de esforço](/docs/pt/model-config#adjust-effort-level) lista as rotas que ativam ultracode.

Com ultracode ativado, Claude decide quando uma tarefa justifica um fluxo de trabalho. Uma única solicitação pode se transformar em vários fluxos de trabalho seguidos: um para entender o código, um para fazer a alteração e um para verificá-la. Isso se aplica a cada tarefa na sessão, então cada solicitação usa mais tokens e leva mais tempo do que em níveis de esforço mais baixos.

`/effort ultracode` dura para a sessão atual; para ter cada sessão iniciada com ele, defina a configuração [`ultracode`](/docs/pt/settings-reference#ultracode). Volte com `/effort high` quando retornar ao trabalho de rotina. O menu `/effort` o oferece apenas [quando ultracode está disponível](/docs/pt/model-config#when-ultracode-is-available).

<h3 id="approve-the-plan-before-it-runs">
  Aprovar o plano antes de ser executado
</h3>

Na CLI, o prompt por execução mostra as fases planejadas e estas opções:

* **Sim, execute**: inicie a execução
* **Sim, e não pergunte novamente para `<name>` em `<path>`**: inicie e pule este prompt para este fluxo de trabalho neste projeto a partir de agora. Claude Code oferece esta opção quando você executa um fluxo de trabalho agrupado, salvo ou de plugin por nome, não para um script que Claude escreveu para a tarefa atual.
* **Ver script bruto**: leia o script antes de decidir
* **Não**: cancelar

`Ctrl+G` abre o script em seu editor. `Tab` permite que você ajuste o prompt antes da execução começar.

Se você vê este prompt depende do seu [modo de permissão](/docs/pt/permission-modes):

| Modo de permissão       | Quando você é solicitado                                                                                                                                                                                       |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Auto                    | Apenas no primeiro lançamento. Qualquer **Sim** registra consentimento em suas configurações de usuário, e lançamentos posteriores começam sem solicitar. Ignorado completamente quando ultracode está ativado |
| Manual, aceitar edições | Cada execução, a menos que você tenha selecionado **Sim, e não pergunte novamente** para esse fluxo de trabalho neste projeto                                                                                  |
| Contornar permissões    | Claude Code não o solicita. A execução começa imediatamente                                                                                                                                                    |
| `claude -p`, Agent SDK  | Claude Code não o solicita                                                                                                                                                                                     |

Em `claude -p` e no Agent SDK, Claude Code nunca mostra este prompt. Ele executa a chamada da ferramenta Workflow através da mesma [avaliação de permissão](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) como o resto da sessão, então regras de negação, regras de solicitação e modo `dontAsk` se aplicam ao lançamento como se aplicam a cada chamada de ferramenta. Para permitir que o fluxo de trabalho inicie nessas execuções, use um destes:

* **Regra de permissão**: `Workflow` em suas regras de permissão aprova cada fluxo de trabalho, e `Workflow(<name>)` aprova um fluxo de trabalho salvo por nome.
* **Modo de permissão automático**: o [classificador](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) revisa a chamada e pode aprová-la.
* **Modo de contorno de permissões**: Claude Code aprova a chamada.
* **Um hook `PreToolUse`**: um [hook](/docs/pt/hooks#pretooluse) que retorna `allow` para a chamada a aprova.
* **Seu host**: um [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags) a aprova, ou, com o Agent SDK, um callback [`canUseTool`](/docs/pt/agent-sdk/permissions) ou um hook [`PermissionRequest`](/docs/pt/hooks#permissionrequest) a aprova.

No aplicativo Desktop, um cartão de aprovação mostra o nome do fluxo de trabalho, a lista de fases e um aviso de uso de token, com ações **Uma vez**, **Sempre** e **Negar**. A visualização de progresso aparece no painel de tarefas em segundo plano.

Os subagentos que o fluxo de trabalho spawna usam suas [regras de permissão](/docs/pt/settings-reference#permission-settings), e Claude Code escolhe seu modo de permissão pelas regras em [qual modo de permissão um subagentos é executado](/docs/pt/sub-agents#permission-modes). Para evitar prompts em uma execução longa, adicione as ferramentas que os agentes precisam às suas regras de permissão antes de começar.

<h3 id="save-the-workflow-for-reuse">
  Salvar o fluxo de trabalho para reutilização
</h3>

Quando Claude escreve um fluxo de trabalho para uma tarefa que você repetirá, você pode salvar o script dessa execução como um comando. Um processo como uma revisão que você executa em cada branch então executa a mesma orquestração cada vez.

Execute `/workflows`, selecione a execução que você deseja manter e pressione `s`. Na caixa de diálogo de salvamento, Tab alterna entre os dois locais de salvamento:

* `.claude/workflows/` em seu projeto: compartilhado com todos que clonam o repositório
* `~/.claude/workflows/` em seu diretório inicial: disponível em cada projeto, visível apenas para você. Se você definir [`CLAUDE_CONFIG_DIR`](/docs/pt/env-vars), este local é o diretório `workflows/` sob esse caminho.

A caixa de diálogo de salvamento mostra o caminho resolvido para o local pessoal.

Pressione Enter para salvar. O fluxo de trabalho é executado como `/<name>` em futuras sessões de qualquer local.

Claude Code verifica o local de salvamento para symlinks antes de escrever e mostra um erro em vez de escrever através de um. O que ele verifica depende de onde você salva:

* Local do projeto: Claude Code recusa se `.claude`, `.claude/workflows` ou o arquivo de destino for um symlink.
* Local pessoal: Claude Code recusa apenas se o arquivo de destino em si for um symlink, então um diretório `~/.claude` gerenciado por uma ferramenta de dotfiles ainda funciona.

Antes da v2.1.216, Claude Code seguia o link, o que poderia colocar o arquivo fora do local que você escolheu.

Em um monorepo com vários diretórios `.claude/`, você pode manter fluxos de trabalho ao lado do pacote ao qual se aplicam. Salvar no local do projeto escreve no diretório `.claude/workflows/` mais próximo que já existe entre seu diretório de trabalho e a raiz do repositório, ou para a raiz do repositório se nenhum existir ainda. Os fluxos de trabalho do projeto também carregam de cada `.claude/workflows/` ao longo desse caminho, e quando mais de um define o mesmo nome Claude Code executa o mais próximo do diretório de trabalho.

Se um fluxo de trabalho de projeto e um fluxo de trabalho pessoal compartilham um nome, o do projeto é executado.

<h3 id="distribute-a-workflow-in-a-plugin">
  Distribuir um fluxo de trabalho em um plugin
</h3>

Para compartilhar um fluxo de trabalho entre equipes ou repositórios, inclua-o em um [plugin](/docs/pt/plugins/overview). Coloque o script em um diretório `workflows/` na raiz do plugin, ou aponte para um local diferente com o [campo de manifesto `workflows`](/docs/pt/plugins/manifest-reference#fields).

Os fluxos de trabalho de plugin são nomeados pelo nome do plugin. Um plugin chamado `acme-tools` contendo um script cujo `meta.name` é `release-audit` é executado como `/acme-tools:release-audit`.

<h3 id="pass-input-to-a-saved-workflow">
  Passar entrada para um fluxo de trabalho salvo
</h3>

Um fluxo de trabalho salvo pode aceitar entrada através do parâmetro `args`. O script o lê como um global nomeado `args`. Use isso para fornecer uma pergunta de pesquisa, uma lista de caminhos de destino ou um objeto de configuração no momento da invocação em vez de editar o script para cada execução.

O prompt a seguir executa um fluxo de trabalho salvo com uma lista de números de problemas:

```text wrap theme={null}
Run /triage-issues on issues 1024, 1025, and 1030
```

Claude passa a lista como dados estruturados, então o script pode chamar métodos de array e objeto em `args` diretamente sem analisá-lo primeiro. Se `args` for omitido, o global é `undefined` dentro do script.

<h2 id="example-workflow-prompts">
  Exemplos de prompts de fluxo de trabalho
</h2>

Um fluxo de trabalho se encaixa melhor quando a tarefa é maior do que um agente pode manter em contexto, ou quando a mesma etapa precisa ser executada em muitos itens. Os prompts abaixo mostram formas comuns. Cada um pede ao Claude para escrever e executar um fluxo de trabalho para essa tarefa; você não escreve o script você mesmo.

<h3 id="audit-many-files-for-the-same-issue">
  Auditar muitos arquivos para o mesmo problema
</h3>

Distribua um agente por arquivo e depois colete e verifique os achados.

```text wrap theme={null}
use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Continue corrigindo até que uma verificação seja aprovada
</h3>

Execute um verificador, corrija o que falhou e repita até que seja aprovado ou pare de fazer progresso.

```text wrap theme={null}
use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Migrar muitos arquivos em paralelo
</h3>

Descubra os arquivos a migrar, transforme cada um em uma cópia isolada para que as edições não entrem em conflito e verifique cada resultado.

```text wrap theme={null}
use a workflow to migrate every component under src/components/ from JavaScript to TypeScript, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Revisar cada arquivo alterado e escrever um resumo
</h3>

Execute um revisor por arquivo e depois passe todos os achados para um agente que os classifica e deduplica.

```text wrap theme={null}
use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Pesquisar um tópico em muitas fontes
</h3>

Distribua leitores entre changelogs, problemas e documentação e depois sintetize. O fluxo de trabalho `/deep-research` incluído faz isso; você também pode descrever uma versão mais restrita.

```text wrap theme={null}
use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Encontrar problemas até que a lista pare de crescer
</h3>

Continue pesquisando em rodadas e pare quando novas rodadas não encontrarem nada novo.

```text wrap theme={null}
use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
```

<h3 id="what-the-saved-script-looks-like">
  Como o script salvo se parece
</h3>

Quando você [salva um fluxo de trabalho](#save-the-workflow-for-reuse), o arquivo em `.claude/workflows/` contém um bloco `meta` seguido por um corpo de script que orquestra subagentes. Você geralmente não precisa editá-lo, mas aqui está a forma de um pequeno para que você possa reconhecer o que Claude gerou:

```javascript theme={null}
export const meta = {
  name: 'audit-routes',
  description: 'Audit every route handler for missing auth checks',
}

const found = await agent('List every .ts file under src/routes/.', {
  schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
})

const audits = await pipeline(found.files, file =>
  agent(`Audit ${file} for missing authentication checks.`, { label: file }),
)

return audits.filter(Boolean)
```

O corpo é JavaScript simples com `await` no nível superior. `agent()` gera um subagente, `pipeline()` executa um por item em uma lista, e `parallel()` executa um conjunto de tarefas de agente ao mesmo tempo e aguarda todas elas.

Uma chamada `agent()` é resolvida para `null` se você a interromper no meio da execução ou se ela atingir um erro de API irrecuperável. `pipeline()` mantém cada `null` na matriz de resultados, e é por isso que o exemplo termina com `.filter(Boolean)` para descartar essas entradas.

No [modo automático](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), o prompt que seu script passa para `agent()` não conta como uma solicitação sua quando o classificador revisa as ações desse subagente, porque Claude Code o marca como texto que o script calculou.

Se você passar um `schema` em uma chamada `agent()`, esse subagente retorna JSON correspondente à forma em vez de prosa. Claude Code verifica o schema antes de iniciar o subagente: quando pode provar que o schema se contradiz, a chamada falha com um erro nomeando a contradição, e o subagente nunca é iniciado. Uma contradição que pode provar é uma chave `required` que `additionalProperties: false` exclui.

Se a saída do subagente ainda falhar na validação após cinco tentativas, a chamada falha com um erro que inclui a última falha de validação. Para alterar a contagem de tentativas, defina [`MAX_STRUCTURED_OUTPUT_RETRIES`](/docs/pt/env-vars).

<h3 id="edit-a-saved-script">
  Editar um script salvo
</h3>

Para alterar um [fluxo de trabalho que você salvou](#save-the-workflow-for-reuse), edite seu arquivo `.js` ou peça ao Claude para fazer a alteração. Antes de editar ou pedir, execute a [skill incluída](/docs/pt/skills#bundled-skills) `/workflow-authoring` para carregar a referência de escrita de script com a qual Claude trabalha. A skill requer Claude Code v2.1.248 ou posterior.

Para executar a versão editada na sessão atual, execute [`/reload-skills`](/docs/pt/commands#all-commands) para reler os diretórios de fluxo de trabalho e depois execute `/<name>` novamente.

Claude Code aplica essas regras a cada parte do arquivo quando carrega e executa o script:

* **Bloco `meta`**: mantenha `export const meta` como a primeira instrução e mantenha-o um objeto literal simples com um `name` e uma `description`. Se contiver algo diferente de valores literais, como uma variável, uma chamada de função ou um spread, Claude Code remove `/<name>` do autocomplete `/`.
* **Corpo**: além de `agent()`, `pipeline()` e `parallel()`, você pode chamar `phase()` para agrupar os agentes que seguem sob um título na visualização de progresso, chamar `log()` para mostrar uma mensagem acima das fases e ler o global [`args`](#pass-input-to-a-saved-workflow). Se o corpo tiver um erro de sintaxe, Claude Code o reportará quando você executar o fluxo de trabalho.
* **`phases`**: se você listá-las em `meta`, dê a cada entrada exatamente o título que você passa para `phase()`. Um título `phase()` sem entrada obtém seu próprio grupo de progresso.
* **Timestamps e aleatoriedade**: Claude Code faz `Date.now()`, `Math.random()` e um `new Date()` sem argumentos lançarem dentro do script, para que uma [execução relançada](#resume-after-a-pause) repita as mesmas chamadas `agent()`. Passe um timestamp através de `args` em vez disso.

Você também pode editar [o script de uma única execução](#how-a-workflow-runs) em vez da cópia salva. [Retomar após uma pausa](#resume-after-a-pause) cobre quais agentes são executados novamente quando você relança um script editado. Para as entradas da ferramenta Workflow, consulte sua entrada na [referência do Agent SDK](/docs/pt/agent-sdk/typescript#workflow).

<h2 id="how-a-workflow-runs">
  Como um fluxo de trabalho é executado
</h2>

O runtime do fluxo de trabalho executa o script em um ambiente isolado, separado de sua conversa. Os resultados intermediários permanecem em variáveis de script em vez de chegar ao contexto de Claude.

Cada execução escreve seu script em um arquivo sob o diretório da sua sessão em `~/.claude/projects/`. Claude recebe o caminho quando a execução começa, então você pode pedir por ele. Você pode abrir esse arquivo para ler a orquestração que Claude escreveu, compará-lo com o script de uma execução anterior, ou editá-lo e pedir a Claude para relançar a partir da versão editada.

Claude pode iniciar um fluxo de trabalho apenas a partir de um arquivo de script que a sessão já tem permissão para ler. Para executar um script mantido fora do seu diretório de trabalho, adicione seu diretório com [`/add-dir`](/docs/pt/permissions#working-directories) ou uma [regra de permissão Read](/docs/pt/permissions#read-and-edit) primeiro.

O runtime rastreia o resultado de cada agente conforme a execução progride, o que é o que torna uma execução [retomável](#resume-after-a-pause) dentro da mesma sessão.

<h3 id="prompt-caching-in-a-fan-out">
  Prompt caching em um fan-out
</h3>

Agentes na mesma execução podem ler o [prompt cache](/docs/pt/prompt-caching#subagents-and-the-cache) um do outro. Dois agentes que executam com o mesmo modelo, nível de esforço, tipo de agente, ferramentas, esquema de saída e diretório de trabalho constroem o mesmo prefixo de ferramentas e prompt do sistema, então um agente que começa após a resposta de um irmão correspondente ter começado lê o cache desse irmão em sua primeira solicitação.

As solicitações de um agente de fluxo de trabalho ficam fora do [bucket de TTL de cache](/docs/pt/prompt-caching#which-ttl-each-request-gets) da conversa principal, então seu cache dura cinco minutos por padrão, inclusive em uma assinatura Claude. Para mantê-lo por uma hora, defina [`subagentPromptCacheTtl`](/docs/pt/settings-reference#subagentpromptcachettl) como `1h`. A API cobra gravações de cache de 1 hora a uma taxa mais alta.

Quando um fan-out inicia vários agentes correspondentes de uma vez, Claude Code mantém todos exceto o primeiro até que a resposta do primeiro agente comece, depois libera os agentes mantidos juntos para que suas primeiras solicitações leiam o prefixo compartilhado em vez de cada um processá-lo sem cache. Claude Code limita a retenção a [`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS`](/docs/pt/env-vars) milissegundos, `5000` por padrão. Defina como `0` para desabilitar a retenção.

<h3 id="behavior-and-limits">
  Comportamento e limites
</h3>

O runtime aplica as seguintes restrições:

| Restrição                                                                                                                                                                                                                                                                                                                    | Por quê                                                                                                                                                                                                                                    |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sem entrada do usuário durante a execução                                                                                                                                                                                                                                                                                    | Uma execução pausa por conta própria apenas para prompts de permissão de agente e uma [espera de limite de uso](#when-a-run-hits-your-usage-limit). Para aprovação entre estágios, execute cada estágio como seu próprio fluxo de trabalho |
| Sem acesso direto ao sistema de arquivos ou shell do próprio fluxo de trabalho                                                                                                                                                                                                                                               | Agentes leem, escrevem e executam comandos. O script coordena os agentes                                                                                                                                                                   |
| Sem carregamento de módulo: um script que contém `import()` falha antes da execução começar                                                                                                                                                                                                                                  | O corpo do script é JavaScript simples. Coloque o trabalho que precisa de uma biblioteca na tarefa de um agente                                                                                                                            |
| Até 16 agentes simultâneos por padrão, menos quando Claude Code tem menos CPUs disponíveis, inclusive dentro de um container com CPU limitada. Para alterar o limite, defina [`CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS`](/docs/pt/env-vars#variables) para um valor de 1 a 256, o que requer Claude Code v2.1.269 ou posterior | Limita o uso de recursos locais                                                                                                                                                                                                            |
| Em um fan-out, agentes que compartilham o prefixo de prompt-cache do primeiro agente iniciam até 5 segundos depois por padrão                                                                                                                                                                                                | Todos exceto o primeiro leem o [prefixo que o primeiro agente armazenou em cache](#prompt-caching-in-a-fan-out) em vez de cada um processá-lo sem cache                                                                                    |
| Até 4.096 itens em uma única chamada `parallel()` ou `pipeline()`: o runtime rejeita uma lista mais longa com um erro                                                                                                                                                                                                        | Um limite silencioso deixaria cair parte da carga de trabalho sem informar o script                                                                                                                                                        |
| 1.000 agentes totais por execução                                                                                                                                                                                                                                                                                            | Previne loops descontrolados                                                                                                                                                                                                               |

<h2 id="manage-runs">
  Gerenciar execuções
</h2>

Uma vez que uma execução começa, você a gerencia a partir da visualização `/workflows`, ou expandindo sua linha de progresso no painel de tarefas abaixo da caixa de entrada.

Quando você para uma execução, ela permanece no painel de tarefas enquanto qualquer um dos processos de seus agentes ainda estiver em execução. Se você parar novamente, Claude Code re-sinaliza esses processos.

<h3 id="resume-after-a-pause">
  Retomar após uma pausa
</h3>

Retome uma execução pausada de `/workflows` selecionando-a e pressionando `p`. Para uma execução que você parou, peça a Claude para relançar o fluxo de trabalho com o mesmo script. Se agentes da execução parada ainda não tiverem saído, Claude Code recusa o relançamento até que saiam, para que uma segunda cópia desses agentes não possa ser executada ao lado deles.

Claude Code reproduz a execução na ordem em que os agentes começaram, e cada agente retorna seu resultado salvo ou é executado novamente:

* **Concluído**: retorna seu resultado salvo. O primeiro agente cujo prompt difere da execução anterior, porque você editou o script ou um agente anterior retornou algo diferente, é executado novamente, assim como todos os agentes depois dele, mesmo os que foram concluídos.
* **Ainda em execução quando você parou**: começa novamente. Parar toda a execução não conta nenhum agente como falho.
* **Falhou**: é executado novamente, assim como todos os agentes que começaram depois dele, mesmo os que foram concluídos. Parar um agente sozinho, selecionando-o em [`/workflows`](#watch-the-run) e pressionando `x`, conta como falha.

Esse último caso significa que uma falha no meio de um fan-out reexecuta trabalho que já foi concluído. Se um script inicia A, B, C e D nessa ordem e B falha, relançar retorna A do cache e executa B, C e D novamente.

Você pode retomar uma execução dentro da mesma sessão de Claude Code. O que acontece com um fluxo de trabalho em execução quando você sai da sessão depende de como você sai:

* Se você [colocar a sessão em segundo plano](/docs/pt/agent-view#what-carries-over-when-you-background), Claude Code reproduz a execução da mesma forma na sessão em segundo plano e continua.
* Se você sair de Claude Code enquanto um fluxo de trabalho está em execução e [agent view está ativado](/docs/pt/agent-view#from-inside-a-session), o diálogo de saída oferece `Move to background and exit`, que transfere a execução da mesma forma. Se você escolher `Exit and stop tasks` em vez disso, ou a opção não for oferecida, a execução para com a sessão. Claude Code mantém os resultados salvos da execução no diretório dessa sessão em `~/.claude/projects/`, para que uma sessão que você retome com `claude --resume` possa reproduzi-los quando você pedir a Claude para relançar o fluxo de trabalho. Em uma sessão que você inicia do zero, Claude não tem nenhuma execução anterior para relançar e inicia o fluxo de trabalho novamente como uma nova execução.

Em uma [sessão em nuvem](/docs/pt/claude-code-on-the-web), Claude Code também salva os resultados da execução com o histórico de conversa da sessão, que sobrevive quando a VM da sessão é recuperada. Quando você [reabre tal sessão](/docs/pt/claude-code-on-the-web#environment-expired) e pede a Claude para relançar o fluxo de trabalho, agentes concluídos ainda retornam seus resultados salvos.

Em sessões locais e em nuvem, quando Claude relança uma execução anterior e Claude Code não consegue encontrar os resultados salvos dessa execução, o relançamento falha com um erro `nothing to resume` em vez de iniciar a execução novamente por conta própria. Peça a Claude para iniciar o fluxo de trabalho novamente como uma nova execução.

<h3 id="when-a-run-hits-your-usage-limit">
  Quando uma execução atinge seu limite de uso
</h3>

Quando um agente atinge seu [limite de uso](/docs/pt/interactive-mode#wait-for-a-usage-limit-to-reset) do claude.ai, a execução pausa em vez de falhar esse agente: os agentes que atingem o limite aguardam a redefinição, e nenhum novo agente inicia. Pouco depois que o limite é redefinido, os agentes em espera são executados novamente e a execução continua por conta própria. Requer Claude Code v2.1.271 ou posterior; em versões anteriores, os agentes afetados falham.

Enquanto a execução aguarda, sua linha de progresso no painel de tarefas e o cabeçalho [`/workflows`](#watch-the-run) mostram quando o limite é redefinido.

A execução pausa apenas quando todos esses itens se aplicam; quando um não se aplica, o agente afetado falha em vez disso:

* A sessão é interativa e conectada com uma assinatura claude.ai. Uma execução não pausa em [modo não interativo](/docs/pt/headless) com `claude -p` ou no [Agent SDK](/docs/pt/agent-sdk/overview), em uma [sessão em segundo plano](/docs/pt/agent-view), ou em uma sessão de colega de [Remote Control](/docs/pt/remote-control) ou [agent team](/docs/pt/agent-teams).
* [`autoContinueAtUsageLimit`](/docs/pt/settings-reference#autocontinueatusagelimit) está ativado, a mesma configuração que permite que a sessão em si [aguarde a redefinição de um limite de uso](/docs/pt/interactive-mode#wait-for-a-usage-limit-to-reset). Se você desativá-lo durante uma espera, a espera termina e os agentes em espera falham.
* O limite é redefinido dentro de 24 horas. Um limite semanal pode ser redefinido mais adiante.
* A execução ainda não aguardou duas vezes. Quando atinge o limite pela terceira vez, o agente falha.

<h3 id="cost">
  Custo
</h3>

Um fluxo de trabalho spawna muitos agentes, então uma única execução pode usar significativamente mais tokens do que trabalhar através da mesma tarefa em conversa. As execuções contam para o uso do seu plano e limites de taxa.

Para avaliar o gasto antes de se comprometer com uma tarefa grande, execute o fluxo de trabalho em um pequeno recorte primeiro: um diretório em vez de todo o repositório, ou uma pergunta estreita em vez de uma ampla. A visualização `/workflows` mostra o uso de tokens de cada agente conforme a execução progride, e você pode parar a execução lá a qualquer momento, geralmente sem perder o trabalho concluído. [Retomar após uma pausa](#resume-after-a-pause) cobre o que uma execução parada mantém. Os [limites de agente](#behavior-and-limits) do runtime limitam quantos agentes uma única execução pode spawnar, o que limita o custo de um script descontrolado. Para manter execuções com menos agentes, escolha a diretriz de tamanho `small` [](#set-a-size-guideline).

Claude Code também sinaliza uma execução que cresce incomumente grande. Quando um fluxo de trabalho agenda mais de 25 agentes, ou sua projeção de token total passa 1,5 milhão, sua linha de progresso no painel de tarefas abaixo da caixa de entrada mostra um aviso `Large workflow`. O aviso aponta você para [`/workflows`](#watch-the-run), onde você pode parar a execução.

O aviso é consultivo: ele não pausa ou limita a execução. Duas configurações mudam quando você o vê:

* Se você escolher uma [diretriz de tamanho](#set-a-size-guideline) você mesmo, a contagem de agentes dela substitui o limite de 25 agentes. A diretriz padrão integrada deixa o limite em 25.
* Sessões com [ultracode](#let-claude-decide-with-ultracode) ativado não mostram o aviso, porque ativar ultracode já o opta para execuções grandes.

Claude Code escolhe o modelo de cada agente de fluxo de trabalho na mesma [ordem que usa para subagentes](/docs/pt/sub-agents#choose-a-model). Um modelo que o script nomeia para um estágio conta como o modelo por invocação nessa ordem. Quando nada mais atribui um, o agente é executado no modelo da sua sessão.

Para controlar o custo do modelo:

* Verifique `/model` antes de uma execução grande se você geralmente muda para um modelo menor para trabalho de rotina
* Peça a Claude para usar um modelo menor para estágios que não precisam do mais forte quando você descreve a tarefa

Quando a [`availableModels` allowlist](/docs/pt/model-config#restrict-model-selection) da sua organização bloqueia um modelo que o script solicita para um agente, esse agente é executado em um modelo substituído em vez disso, seguindo as mesmas [regras de substituição que subagentes](/docs/pt/sub-agents#choose-a-model). A visualização de progresso da execução em [`/workflows`](#watch-the-run) mostra um aviso nomeando tanto o modelo solicitado quanto o substituído.

<h3 id="set-a-size-guideline">
  Defina uma diretriz de tamanho
</h3>

Uma diretriz de tamanho diz a Claude quantos agentes visar quando escreve um fluxo de trabalho dinâmico. Claude Code envia a diretriz para Claude como conselho, não um limite, então um prompt que chama por uma escala diferente ainda a substitui. Requer Claude Code v2.1.202 ou posterior.

Cada valor mapeia para uma contagem de agentes:

| Valor          | Contagem de agentes que Claude visa                               |
| :------------- | :---------------------------------------------------------------- |
| `unrestricted` | Sem diretriz: Claude dimensiona o fluxo de trabalho para a tarefa |
| `small`        | Menos de 5 agentes                                                |
| `medium`       | Menos de 10 agentes                                               |
| `large`        | Menos de 50 agentes                                               |

O padrão é `medium`, ou `small` quando você está conectado em um plano Pro com Claude Code v2.1.271 ou posterior. Até você escolher um valor, a linha `/config` marca o valor como padrão, e a linha `Running in background` do fluxo de trabalho nomeia o tamanho em vigor. Requer Claude Code v2.1.219 ou posterior; versões anteriores padrão para `unrestricted`.

Para alterar a diretriz, escolha um valor para a configuração Dynamic workflow size em `/config`, ou execute `/config workflowSizeGuideline=small`. Na v2.1.219 e posterior, você também pode definir a chave [`workflowSizeGuideline`](/docs/pt/settings-reference#workflowsizeguideline) em qualquer arquivo de configurações; esse valor tem precedência sobre `/config`, e Claude Code oculta a linha `/config` enquanto um arquivo de configurações fornece um.

As alterações entram em vigor no próximo prompt. Os [limites de agente do runtime](#behavior-and-limits) ainda se aplicam independentemente da configuração.

<h3 id="turn-workflows-off">
  Desativar fluxos de trabalho
</h3>

Fluxos de trabalho estão disponíveis na CLI, no aplicativo Desktop, nas extensões IDE, [modo não interativo](/docs/pt/headless) com `claude -p`, e no [Agent SDK](/docs/pt/agent-sdk/overview). As mesmas configurações de desativação se aplicam em cada superfície.

Para desativar fluxos de trabalho para você:

* Alterne Dynamic workflows desativado em `/config`. Persiste entre sessões.
* Defina `"disableWorkflows": true` em `~/.claude/settings.json`. Persiste entre sessões.
* Defina `CLAUDE_CODE_DISABLE_WORKFLOWS=1`. Lido na inicialização, então se aplica onde quer que você o defina.

Para desativar fluxos de trabalho para toda a sua organização, defina `"disableWorkflows": true` em [configurações gerenciadas](/docs/pt/server-managed-settings), ou use o alternador na página [configurações de administrador de Claude Code](https://claude.ai/admin-settings/claude-code).

Quando fluxos de trabalho estão desativados, os comandos de fluxo de trabalho agrupados e a skill `/workflow-authoring` não estão disponíveis, a palavra-chave `ultracode` não dispara mais uma execução, e `ultracode` é removido do menu `/effort`.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Executar agentes em paralelo](/docs/pt/agents): comparar subagentos, visualização de agente, equipes de agentes e fluxos de trabalho
* [Criar subagentos personalizados](/docs/pt/sub-agents): a primitiva de worker que fluxos de trabalho orquestram
* [Gerenciar custos](/docs/pt/costs): como execuções multi-agente contam para limites de uso
