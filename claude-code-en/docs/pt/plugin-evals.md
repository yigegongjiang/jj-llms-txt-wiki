> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Testar plugins com evals

> Escreva casos de eval para seu plugin Claude Code, execute-os com claude plugin eval, classifique os resultados, compare com uma linha de base sem plugin e gate CI na pontuação.

O comando shell `claude plugin eval` executa seu [plugin](/docs/pt/plugins/overview) contra um conjunto de casos de teste e classifica os resultados. Cada caso é um prompt realista mais um ou mais avaliadores. Um avaliador é uma verificação de aprovação/reprovação sobre o que Claude produziu, como uma regex sobre a resposta, se uma ferramenta particular foi chamada, ou uma rubrica que um segundo modelo julga a resposta.

Você não precisa escrever o conjunto manualmente. `claude plugin eval init` pergunta sobre seu plugin, propõe os casos e avaliadores, tenta-os e escreve os arquivos. Você também pode pedir a Claude para fazer o mesmo a partir de uma sessão que você já tem aberta.

Use evals para:

* Medir com que confiabilidade seu plugin direciona Claude para produzir o resultado correto
* Detectar regressões quando você altera o plugin ou um novo modelo é lançado
* Ver qual é a contribuição do plugin em comparação com nenhum plugin

Esta página é para autores de plugins e skills que têm um plugin funcionando e desejam testar seu comportamento, e para equipes que fazem gate de mudanças de plugin em CI. Seu formato de caso é separado do arquivo `evals/evals.json` que o [skill-creator plugin](/docs/pt/skills#run-evals-with-skill-creator) usa. Para criar um plugin, consulte [Criar um plugin](/docs/pt/plugins/create); para verificar os arquivos de um plugin quanto a erros de sintaxe e esquema em vez de seu comportamento, use [`claude plugin validate`](/docs/pt/plugins/cli-reference#plugin-validate).

<Note>
  Cada execução de eval e cada avaliador de juiz é uma chamada de modelo real em sua conta, contada contra o uso do seu plano ou sua fatura de API, então verifique os [requisitos](#requirements) primeiro. Em seguida, [crie seu primeiro conjunto de eval](#create-your-first-eval-suite), ou vá para [Executar evals em CI](#run-evals-in-ci) se você já tiver um.
</Note>

<h2 id="requirements">
  Requisitos
</h2>

Para executar evals de plugin você precisa:

* Claude Code v2.1.269 ou posterior. Execute `claude --version` para verificar e `claude update` para atualizar.
* Um diretório de plugin com um manifesto `plugin.json` ou `.claude-plugin/plugin.json`, ou um [plugin de diretório de skills](/docs/pt/plugins/loading#plugins-shared-through-a-repository).
* A mesma autenticação e provedor de modelo que suas sessões normais de Claude Code usam. Execuções de eval, avaliadores pontuados por juiz e `claude plugin eval init` chamam o modelo com suas credenciais, então contam contra seus limites de uso do plano ou sua fatura de API. Quando o comando relata um custo, a figura é uma [estimativa de preço de lista](/docs/pt/costs) dessas chamadas.

<h2 id="how-an-eval-run-works">
  Como uma execução de eval funciona
</h2>

Um conjunto de eval vive em um diretório chamado `evals/` dentro de seu plugin, organizado como [Escrever e refinar casos](#write-and-refine-cases) mostra. Cada caso é seu próprio subdiretório com um [prompt](#set-run-limits-and-tools-in-prompt-md) e um ou mais [avaliadores](#grade-the-result). O prompt é algo que uma pessoa usando seu plugin poderia digitar, como uma solicitação que um de seus skills deveria lidar.

<h3 id="what-happens-in-a-run">
  O que acontece em uma execução
</h3>

Para cada execução de um caso, Claude Code inicia uma sessão [isolada](#how-runs-are-isolated) [não-interativa](/docs/pt/headless) fresca com apenas seu plugin carregado, envia o prompt e deixa Claude trabalhar até que termine ou atinja o limite de turno ou tempo do caso. Cada avaliador então verifica a resposta final, a transcrição ou um arquivo que Claude criou, e passa ou falha.

<h3 id="how-a-case-is-scored">
  Como um caso é pontuado
</h3>

Uma execução de um agente não-determinístico diz pouco, então cada caso é executado três vezes por padrão. A pontuação de uma execução é a fração de seus avaliadores que passaram, ponderada se você definir pesos, e a pontuação do caso é a média entre suas execuções. Um caso passa quando sua pontuação atende ao [`--threshold`](#command-options), `1.0` por padrão.

Em chamadas de modelo, um conjunto faz aproximadamente casos × execuções execuções de agente com o plugin e tantas novamente para a [linha de base sem plugin](#the-no-plugin-baseline), mais três chamadas de juiz curtas por avaliador `llm` ou `baseline` por execução.

<h3 id="the-no-plugin-baseline">
  A linha de base sem plugin
</h3>

Uma pontuação alta por si só não diz que o plugin ajudou, porque Claude poderia fazer tão bem sem ele. Para separar os dois, as execuções de cada caso são repetidas sem plugin carregado por padrão, e você obtém duas pontuações, `WITH` e `W/OUT`. Sua diferença, `Δ`, é o que o plugin contribuiu. Se um caso marca 1.0 com e sem o plugin, o plugin não é o que o fez passar.

Os dois conjuntos de execuções são chamados de braço com e braço sem; [Comparar com uma linha de base sem plugin](#compare-against-a-no-plugin-baseline) cobre como os avaliadores são pontuados entre eles e como desativar a linha de base.

<h2 id="create-your-first-eval-suite">
  Crie seu primeiro conjunto de eval
</h2>

Este passo a passo escreve um caso para seu próprio plugin, o executa e lê o resultado. Antes de começar, certifique-se de que você tem:

* Claude Code v2.1.269 ou posterior e os outros [requisitos](#requirements)
* Um terminal aberto no diretório raiz de seu plugin, aquele contendo `plugin.json` ou `.claude-plugin/plugin.json`
* Um skill no plugin que você deseja testar e uma solicitação que um usuário digitaria que deveria acioná-lo

<Steps>
  <Step title="Crie os casos">
    A partir da raiz do plugin, execute:

    ```bash theme={null}
    claude plugin eval init
    ```

    Se Claude Code ainda não confia neste diretório, ele primeiro pergunta `Trust this plugin directory?`; responda `y`.

    Uma sessão interativa de Claude Code então abre. Claude lê seu plugin e pergunta qual é um bom resultado, propõe prompts que devem e não devem acionar o plugin, projeta avaliadores para cada um, testa-os uma vez para verificar se se comportam, e escreve um diretório de caso por prompt sob `evals/`, cada um nomeado após seu prompt.

    Quando Claude diz que o conjunto está pronto, saia dessa sessão com `/exit` ou Ctrl+D para retornar ao seu shell.

    Se você já tiver uma sessão de Claude Code aberta na raiz do plugin, você pode em vez disso pedir a Claude para executar `claude plugin eval init`. Claude executa o comando e então faz as mesmas perguntas nessa conversa.

    Se você preferir escrever um caso você mesmo para ver exatamente o que os arquivos contêm, siga [Escrever um caso manualmente](#write-a-case-manually) e volte aqui para executá-lo.
  </Step>

  <Step title="Execute o conjunto">
    De volta ao seu shell na raiz do plugin, execute cada caso sob `evals/`:

    ```bash theme={null}
    claude plugin eval .
    ```

    Você já confiou neste diretório durante a etapa 1, então a execução começa imediatamente. Se você escreveu o caso manualmente em vez disso, a execução primeiro pergunta `Trust this plugin directory? [y/N]`; responda `y`. [O que uma execução pode acessar](#security) explica no que você está concordando.

    Cada caso é executado três vezes com seu plugin e três vezes sem ele, então um caso é seis execuções. Uma linha de progresso é impressa conforme cada execução termina, com a pontuação dessa execução e o veredicto de cada avaliador.
  </Step>

  <Step title="Leia o resumo">
    Quando o conjunto termina você vê uma tabela de resumo, seguida de onde o relatório foi:

    ```text theme={null}
    CASE        WITH  W/OUT Δ      RUNS COST    NOTES
    first-case  1.00  0.33  +0.67  6    $0.41

    1 case(s) · mean Δ +0.67 · 74s · $0.41
    Report: /Users/you/my-plugin/evals/results/2026-09-10T17-02-11-482Z/report.html
    Published: https://claude.ai/... · keep local next time with --no-publish
    ```

    `WITH` é a pontuação do caso com seu plugin carregado, `W/OUT` é a pontuação sem ele, e um `Δ` positivo significa que o plugin aumentou a pontuação. `COST` é uma estimativa de preço de lista das chamadas de modelo, e `NOTES` mostra a explicação do avaliador de falha de peso mais alto, ou o erro da execução, do braço com.
  </Step>

  <Step title="Abra o relatório e itere">
    Abra a URL `Published:`, ou o caminho `Report:` quando nenhuma linha `Published:` aparecer, para ver o veredicto de cada avaliador e explicação para cada execução, e para avaliadores `llm` os votos do juiz e o trecho que ele julgou. A linha `Published:` aparece apenas quando sua conta pode [publicar relatórios](#html-report).

    O achado mais comum primeiro é um `Δ` próximo a zero com o avaliador `tool_used: Skill` do caso falhando, o que significa que Claude não está escolhendo seu skill em fraseado natural. Ajuste a [`description`](/docs/pt/skills#frontmatter-reference) do skill, execute `claude plugin eval .` novamente e compare.

    Para iterar em um caso barato, execute um único braço uma vez. Uma única execução é barulhenta, então confirme qualquer mudança nas três execuções padrão antes de confiar nela. Com um braço a tabela mostra colunas `SCORE` e `PASS%` em vez de `WITH`, `W/OUT` e `Δ`:

    ```bash theme={null}
    claude plugin eval . --case <case-name> --runs 1 --ablation none
    ```

    Substitua `<case-name>` por um dos nomes de diretório sob `evals/`.
  </Step>
</Steps>

<h2 id="write-and-refine-cases">
  Escrever e refinar casos
</h2>

Os casos que `claude plugin eval init` escreve são arquivos simples que você pode abrir, alterar e adicionar. Um caso é um diretório sob o diretório de eval do plugin que contém um `prompt.md`, um `case.yaml` ou ambos. Para agrupar casos, aninhá-los sob um diretório que não seja em si um caso; qualquer coisa dentro de um diretório de caso, como `graders/` e arquivos de fixture, pertence a esse caso.

Este é o layout que `claude plugin eval init` escreve e o que usar para novos conjuntos. A [referência de conjunto de eval](#eval-suite-reference) tem a árvore completa, incluindo mocks e resultados:

```text theme={null}
my-plugin/
├── .claude-plugin/plugin.json
├── skills/...
└── evals/
    ├── first-case/
    │   ├── prompt.md          # frontmatter: case fields; body: the prompt
    │   ├── graders/
    │   │   ├── criteria.md    # frontmatter: type + options; body: rubric or pattern
    │   │   └── skill-fired.md
    │   └── case.yaml          # optional: only for context.* fields
    ├── ignores-unrelated-request/
    │   └── ...
    └── results/               # written by each run; add to .gitignore
```

<h3 id="write-a-case-manually">
  Escrever um caso manualmente
</h3>

Ter Claude escrever os casos com `claude plugin eval init` é o caminho recomendado. Para escrever um você mesmo em vez disso, comece a partir de um modelo em branco. O comando a seguir escreve um caso nomeado `first-case` com um `prompt.md` de espaço reservado e um avaliador de espaço reservado, e não executa nada:

```bash theme={null}
claude plugin eval init --bare first-case
```

```text theme={null}
evals/first-case/
├── prompt.md            # the prompt sent to Claude, plus run limits
└── graders/
    └── criteria.md      # one grader: how to score the result
```

Em `prompt.md` você escreve a mensagem que Claude recebe em cada execução, e define os limites da execução e as ferramentas que o caso pode usar em seu frontmatter. Abra `evals/first-case/prompt.md` e substitua o corpo do espaço reservado por uma solicitação que um de seus skills deveria lidar, fraseada da maneira que um usuário digitaria em vez de nomear o skill. Este exemplo é para um skill que redige mensagens de commit; use sua própria solicitação:

```markdown theme={null}
---
max_turns: 10
allowed_tools: [Read, Glob, Grep, Skill]
---

Write me a commit message for this change: I renamed getUser to fetchUser and updated the three call sites.
```

Cada execução começa em um diretório de trabalho vazio, então coloque o que a tarefa precisa no próprio prompt, ou [configure o espaço de trabalho](#add-setup-or-history-with-case-yaml) primeiro.

A [lista completa de campos de frontmatter](#prompt-md-fields) cobre o modelo, timeout, tags e variáveis de ambiente.

Cada arquivo sob `graders/` é uma verificação aplicada após a execução. Abra `evals/first-case/graders/criteria.md` e substitua o espaço reservado por uma rubrica para o modelo de juiz, escrita como condições PASS e FAIL concretas:

```markdown theme={null}
---
type: llm
---

PASS if <what a correct response contains>.
FAIL if <what a wrong or missing response looks like>.
```

Em seguida, adicione um segundo avaliador que verifica se seu skill é o que produziu a resposta. Crie `evals/first-case/graders/skill-fired.md`, substituindo `your-skill-name` pelo nome do diretório do skill sob `skills/`, que é o nome que Claude o invoca:

```markdown theme={null}
---
type: tool_used
tool: Skill
input_match: '"skill"\s*:\s*"(?:[\w-]+:)?your-skill-name"'
---
```

Isso passa quando Claude invocou esse skill pelo menos uma vez durante a execução, incluindo por sua forma `plugin-name:skill-name` com namespace.

[Tipos de avaliador](#grader-types) lista as outras verificações disponíveis, como corresponder a uma regex ou confirmar que um arquivo foi criado.

Com ambos os arquivos salvos, execute o caso da maneira que o [quickstart](#create-your-first-eval-suite) faz, com `claude plugin eval .` a partir da raiz do plugin.

<h3 id="set-run-limits-and-tools-in-prompt-md">
  Defina limites de execução e ferramentas em prompt.md
</h3>

Defina `max_turns`, `timeout_seconds`, `model`, `tags` de um caso e o `allowed_tools` que pode usar em frontmatter `prompt.md`; a referência [prompt.md frontmatter](#prompt-md-fields) lista cada campo e seu padrão.

Claude recebe o corpo exatamente como você o escreveu. Menções `@path` nele não são expandidas em anexos de arquivo, então se Claude precisar ler um arquivo, conceda uma ferramenta para ele em `allowed_tools`.

<h3 id="grade-the-result">
  Escolha e pese avaliadores
</h3>

O frontmatter de um avaliador define seu `type` e opcionalmente um `weight` que o faz contar para mais da pontuação da execução e um [`arm`](#compare-against-a-no-plugin-baseline) que controla como é pontuado contra a linha de base. Dos seis tipos, `regex`, `tool_used`, `tool_order` e `file_exists` são computados a partir da transcrição e arquivos e não custam nada, enquanto `llm` e `baseline` chamam um modelo de juiz e adicionam ao custo da execução.

Não há avaliadores de código personalizado.

[Tipos de avaliador](#grader-types) lista as opções de cada tipo e condição de aprovação, e [o que um avaliador pode ver](#what-a-grader-can-look-at) lista os valores que `target` e `focus` aceitam.

O juiz para avaliadores `llm` e `baseline` é um modelo pequeno e rápido por padrão. Passe `--judge-model sonnet` ou um ID de modelo completo para usar um mais forte para rubricas nuançadas.

<h4 id="choose-graders-that-give-a-stable-signal">
  Escolha avaliadores que dão um sinal estável
</h4>

Um avaliador `llm` pede a um modelo um veredicto, então sua resposta pode diferir entre execuções, e difere mais quanto mais longo o texto que tem que ler. Esses hábitos mantêm as pontuações de um conjunto estáveis o suficiente para confiar:

* Para saída longa, como um arquivo gerado, classifique-a com um avaliador `regex` sobre o conteúdo do arquivo, que verifica o arquivo inteiro da mesma forma toda vez. Mantenha avaliadores `llm` para saídas curtas, com rubricas escritas como condições PASS e FAIL concretas.
* Dê a cada caso um avaliador sobre o resultado, como a mensagem final ou um arquivo produzido, e um sobre os passos que Claude tomou para produzi-lo, como `tool_used` ou `tool_order`. Juntos eles dizem se a resposta estava correta e se seu plugin a produziu.
* Se um avaliador `tool_used: Skill` de um caso passa mas `Δ` é negativo, suspeite do juiz antes do plugin. Um modelo de juiz pequeno pode marcar uma resposta correta como errada porque está formatada diferentemente do que a rubrica descreve. Re-execute com `--judge-model sonnet` e aperte a rubrica para que a formatação não decida o veredicto.
* Para verificar que uma compilação ou teste passou dentro da execução, peça a Claude para executá-lo e escrever o resultado em um arquivo, classifique esse arquivo e afirme que o comando foi executado com um avaliador `tool_used` cujo `input_match` nomeia o comando.

<h3 id="compare-against-a-no-plugin-baseline">
  Pontuação contra a linha de base sem plugin
</h3>

Quando um plugin está sob teste, cada caso é executado em dois braços por padrão. O braço com é suas execuções com o plugin carregado, e o braço sem é o mesmo número de execuções sem nenhum plugin. O resumo e relatório mostram ambas as pontuações e `Δ`, a pontuação do braço com menos a pontuação do braço sem.

Passe `--ablation none` para executar apenas o braço com, o que reduz o custo pela metade quando você não precisa da comparação, como ao iterar em avaliadores.

Em uma execução de dois braços, alguns avaliadores são relatados com `scored: false`. Uma verificação como "o skill foi invocado" nunca pode passar sem o plugin, então contá-la empurraria o braço sem para zero e inflaria `Δ`. Para manter os dois braços comparáveis, Claude Code exclui tais avaliadores da pontuação em ambos os braços e os relata no braço com como indicadores de aprovação/reprovação apenas. Isso inclui:

* Cada avaliador `tool_used` cujo `tool` é `Skill`
* Cada avaliador `regex` com `target: mock_calls` e cada avaliador `llm` com `focus: mock_calls`, quando cada [servidor simulado](#mock-mcp-servers) no caso é um que seu plugin declara
* Qualquer avaliador que você marque `arm: with-only`

Três configurações mudam essa exclusão:

* **Cada avaliador excluído**: se cada avaliador em um caso está no conjunto excluído, eles são pontuados normalmente em vez disso, já que não haveria nada deixado para pontuar.
* **`arm: both`**: defina `arm: both` em um avaliador para pontuá-lo em ambos os braços independentemente, que é o que você quer para uma verificação "não deve invocar o skill" com `min: 0` e `max: 0`.
* **`--ablation none`**: sob `--ablation none` nada é excluído, então o mesmo conjunto pode produzir uma pontuação absoluta diferente nos dois modos.

<h3 id="use-a-different-eval-directory">
  Use um diretório de eval diferente
</h3>

Se `evals/` já está sendo usado por outra ferramenta, mantenha o conjunto em um diretório diferente. Você pode registrar esse diretório no `plugin.json` do plugin para que cada execução e cada colaborador o use, ou passe-o na linha de comando para uma única execução:

* **Em `plugin.json`**: adicione `"experimental": { "evals": "quality/evals" }`.
* **Na linha de comando**: passe `--eval-dir quality/evals` para `claude plugin eval` e `claude plugin eval init`.

Se você definir ambos, o diretório da flag é usado. Dê um caminho relativo de nomes de diretório simples como `qa` ou `quality/evals`. Um caminho absoluto ou um contendo `..` não é aceito: como um valor de flag é um erro, enquanto um valor de manifesto inutilizável imprime uma linha `Warning:` e a execução usa `evals/` em vez disso. Casos, resultados e saída `init` todos se movem para esse diretório.

<h2 id="set-up-fixtures-and-mocks">
  Configure fixtures e mocks
</h2>

Um caso pode precisar de mais que um prompt: arquivos ou um repositório git no espaço de trabalho, uma conversa anterior para continuar, ou respostas dos servidores MCP com os quais seu plugin fala. Cada um desses é configurado ao lado do caso para que as execuções permaneçam repetíveis.

<h3 id="add-setup-or-history-with-case-yaml">
  Semeie o espaço de trabalho ou conversa
</h3>

Cada execução começa em um espaço de trabalho vazio. Quando um caso precisa de mais que o prompt, adicione um `case.yaml` ao lado de `prompt.md` com um bloco `context`:

* **Arquivos de fixture ou um repositório git**: escreva um script Bash no diretório de caso e nomeie-o em `context.scaffold_script`. O script é executado como você, fora da sandbox do agente, e apenas quando você passa `--scaffold`, então passe essa flag apenas para conjuntos que você ou sua organização escreveu.
* **Uma conversa anterior para continuar**: salve a transcrição como um arquivo `.jsonl` e nomeie-a em `context.history_file`, e o prompt do caso se torna o próximo turno do usuário.
* **Diretórios de fixture que Claude pode ler durante a execução**: liste-os em `context.add_dirs`.

Um `case.yaml` também precisa de `schema_version: "1.1"` e `name`; a referência [case.yaml fields](#case-yaml-fields) tem a lista completa.

Este `case.yaml` semeia um espaço de trabalho a partir de um script e deixa Claude ler fixtures de um diretório `resources/`:

```yaml theme={null}
schema_version: "1.1"
name: changelog-from-diff
tags: [smoke]
context:
  scaffold_script: fixture.sh
  add_dirs: [resources]
```

<h3 id="mock-mcp-servers">
  Mock MCP servers
</h3>

Você pode avaliar um plugin cujos skills chamam ferramentas MCP sem o serviço real por trás delas. Coloque um arquivo Markdown por ferramenta sob `evals/mocks/<server>/<tool>.md` para o conjunto inteiro, ou sob um diretório `mocks/` próprio de um caso para um caso, onde `<server>` é o nome do servidor na [configuração MCP](/docs/pt/plugins/components#mcp-servers) do seu plugin.

Uma execução nunca inicia seus servidores MCP reais do plugin a menos que você peça. Claude Code registra um substituto sob o próprio nome de cada servidor. Ferramentas com um arquivo mock respondem a partir dele e são permitidas sem uma concessão `--allow-tools`, e uma ferramenta sem arquivo mock não está disponível para Claude. Um servidor sem nenhum mock aparece na linha de progresso `mocked:` do caso como `plugin_<plugin>_<server>[not started: no mock]`.

O corpo do arquivo é o que a ferramenta retorna a Claude. Este mock substitui uma ferramenta `create_issue` em um servidor nomeado `tracker`, verifica a entrada que Claude envia e ecoa o título de volta. Salve-o como `evals/mocks/tracker/create_issue.md`:

```markdown theme={null}
---
expect:
  title: string
  priority: [low, medium, high]
---

Created issue #4821: {{input.title}}
```

O corpo e frontmatter de um arquivo mock aceitam estas opções:

* **Substituições**: insira campos da entrada da chamada com `{{input.<field>}}`, e o conteúdo de um arquivo de fixture ao lado do mock com `{{file:fixtures/{input.<field>}.json}}`.
* **`expect:`**: o bloco `expect:` protege a entrada. Se uma chamada violar, a execução aborta com pontuação 0 e registra por quê, então um caso pode afirmar o que seu plugin pediu ao servidor.
* **`error: true`**: defina `error: true` para retornar o corpo como um erro de ferramenta em vez disso.
* **`type: agent`**: defina `type: agent` para ter um modelo pequeno responder como o servidor a partir de instruções no corpo.

A [referência de arquivo mock](#mock-files) lista cada chave e os arquivos `_server.md` e `_tools.json`.

Para classificar as chamadas em si, aponte um avaliador para `target: mock_calls`.

Para executar contra os servidores MCP reais do plugin em vez disso, passe uma dessas flags. De qualquer forma, esses processos são executados como você, fora da sandbox da execução, e suas ferramentas precisam de uma concessão [`--allow-tools`](#grant-tools):

* **`--allow-real-servers`**: inicie o processo real para cada servidor que você não mockificou e continue respondendo ferramentas mockificadas a partir de seus arquivos
* **`--mocks off`**: ignore `mocks/` inteiramente e inicie cada servidor que o plugin declara

<h4 id="replay-agent-mock-answers">
  Reproduza respostas de mock de agente
</h4>

Um mock `type: agent` responde com uma chamada ao [`--judge-model`](#command-options), então sua saída varia entre execuções e muda se você mudar o juiz. Quando uma execução é concluída sem um erro ou aborto, Claude Code salva cada resposta que um mock de agente deu sob o diretório de resultados em `mock-recordings/`.

Abra `ADOPT.txt` lá para ver cada gravação e o diretório `.replay/<server>/` para copiar para, ao lado do mock que a produziu. Depois de copiar uma gravação lá, execuções posteriores respondem a chamada idêntica a partir dela sem chamada de modelo. Confirme `mocks/.replay/` com o resto de `mocks/` para que as execuções de CI sejam repetíveis.

<h2 id="run-evals">
  Executar evals
</h2>

Uma vez que um conjunto existe, `claude plugin eval` o executa. Você escolhe qual plugin e casos executar com o argumento de destino, concede quaisquer ferramentas que os casos precisem além do conjunto somente leitura com `--allow-tools`, e controla contagem de execução, modelos, custo e saída com as outras opções.

<h3 id="choose-what-to-evaluate">
  Escolha o que avaliar
</h3>

Na maioria das vezes você executa `claude plugin eval .` a partir da raiz do plugin, que executa cada caso no conjunto com o plugin em que você está em pé carregado. Para executar um arquivo de caso único, ou para avaliar um plugin que você instalou em vez de um que está desenvolvendo, passe um destino diferente:

| Destino                                                    | O que é executado                                                                                                                                                                                   |
| :--------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Um diretório raiz do plugin, como `.`                      | Cada caso sob seu diretório de eval, com esse plugin carregado                                                                                                                                      |
| Um arquivo único `prompt.md` ou `case.yaml`                | Esse caso, com seu plugin envolvente carregado                                                                                                                                                      |
| Um plugin instalado por nome, `name` ou `name@marketplace` | Os casos na cópia instalada do diretório de eval, com a cópia instalada carregada. Os resultados são escritos sob `./evals/results/` no seu diretório atual, ou `./<dir>/results/` com `--eval-dir` |
| `name@skills-dir`                                          | O mesmo, para um [plugin de diretório de skills](/docs/pt/plugins/loading#plugins-shared-through-a-repository)                                                                                           |
| Omitido                                                    | O diretório atual como um caminho                                                                                                                                                                   |

Adicione `--case <glob>` para filtrar por nome de caso e `--tag <tag>` para manter casos com qualquer uma das tags fornecidas.

Coloque o destino antes de `--tag`, `--allow-tools` e `--json`. Os dois primeiros pegam uma lista e `--json` pega um caminho opcional, então cada um deles lê um destino que segue como seu próprio valor.

<h3 id="grant-tools">
  Conceda ferramentas
</h3>

As execuções nunca param para pedir permissão. Ferramentas integradas que precisam de uma concessão que você não deu, como `Bash`, `Write`, `Edit`, `WebFetch` e `WebSearch`, são removidas da sessão, então Claude não pode chamá-las.

Uma execução permite apenas as ferramentas somente leitura que o caso lista em `allowed_tools`, de `Read`, `Glob`, `Grep`, `NotebookRead`, `Skill`, `AskUserQuestion`, `Agent`, `TodoWrite` e as ferramentas de tarefa `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate` e `TaskStop`, mais o que você conceder com `--allow-tools`. Essa concessão se aplica a cada caso na execução. Para deixar casos usar `Bash`, `Write`, `Edit`, `WebFetch` ou `WebSearch`, conceda-os você mesmo:

```bash theme={null}
claude plugin eval . --allow-tools Write Edit "Bash(npm test *)"
```

Quando um caso pediu uma ferramenta que você não concedeu, a saída de progresso a lista como `not granted`. Ferramentas em um servidor MCP [mockificado](#mock-mcp-servers) não precisam de concessão. Ferramentas em um servidor MCP de plugin real precisam tanto do servidor iniciado, com `--allow-real-servers` ou `--mocks off`, quanto de uma concessão por nome, como `--allow-tools "mcp__plugin_my-plugin_github__*"`; as ferramentas MCP de um plugin são nomeadas `mcp__plugin_<plugin>_<server>__<tool>`.

Quando você concede `Bash` em qualquer forma, cada comando é executado sob a [sandbox de nível do SO](/docs/pt/sandboxing) do Claude Code. As escritas são confinadas ao espaço de trabalho da execução, seu diretório inicial e configuração de Claude Code são ilegíveis, e o acesso à rede é limitado a domínios que você concede com `--allow-tools "WebFetch(domain:example.com)"`. Se você conceder Bash ou PowerShell em uma máquina sem backend de sandbox, Claude Code recusa cada execução em vez de executá-la sem confinamento, e o caso mostra um erro de execução e geralmente marca 0. Windows nativo não tem backend, então execute conjuntos que concedem shell sob WSL2; no Linux, instale `bubblewrap` e `socat` primeiro. Veja os [pré-requisitos de sandboxing](/docs/pt/sandboxing).

<h3 id="command-options">
  Opções de comando
</h3>

Esta tabela cobre as opções para contagem de execução, modelos, pontuação, custo, concessões de ferramentas, mocks e saída. Execute `claude plugin eval --help` para a lista completa, que também inclui `--case`, `--tag`, `--eval-dir`, `--no-scaffold`, `--report` e `--verbose`.

| Opção                      | Padrão                                                                                   | Efeito                                                                                                                                                                                                                                                                                                                             |
| :------------------------- | :--------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--runs <n>`               | `runs` de cada caso, senão 3                                                             | Execuções por caso por braço                                                                                                                                                                                                                                                                                                       |
| `-j`, `--concurrency <n>`  | `1`                                                                                      | Execute até este número de execuções de agente de uma vez, de 1 a 8. Elas compartilham o limite de taxa de sua conta, então isso encurta o tempo de parede em vez de aumentar a taxa de transferência além desse limite. Os resultados mantêm a ordem do caso                                                                      |
| `--model <model>`          | `model` de cada caso, senão `ANTHROPIC_MODEL` se definido, senão o padrão de Claude Code | Modelo para o agente sob teste. Fixe-o em CI para que um lançamento de modelo não seja confundido com uma regressão de plugin                                                                                                                                                                                                      |
| `--judge-model <model>`    | Um modelo pequeno e rápido                                                               | Modelo para avaliadores `llm` e `baseline`                                                                                                                                                                                                                                                                                         |
| `--ablation <mode>`        | `with-without` quando um plugin resolve, senão `none`                                    | Se também executar cada caso sem o plugin para medir o que ele adiciona. `none` executa um braço; `with-without` adiciona a linha de base sem plugin                                                                                                                                                                               |
| `--threshold <0..1>`       | `1.0`                                                                                    | Um caso passa quando sua pontuação de braço com é pelo menos isso. Qualquer caso abaixo disso faz o comando sair 1                                                                                                                                                                                                                 |
| `--max-cost-usd <usd>`     | Sem teto                                                                                 | Um teto no custo estimado de preço de lista da execução, não no uso do plano. Verificado antes de cada execução começar. Uma vez gasto, nada mais começa; execuções já em voo terminam, então o gasto pode passar o teto por essas execuções. Se alguma execução for deixada não iniciada, o comando sai 2 com resultados parciais |
| `--allow-tools <tools...>` | Nenhum                                                                                   | Conceda ferramentas além do conjunto somente leitura. Veja [Conceda ferramentas](#grant-tools)                                                                                                                                                                                                                                     |
| `--scaffold`               | Desligado                                                                                | Execute o [`scaffold_script`](#add-setup-or-history-with-case-yaml) de cada caso                                                                                                                                                                                                                                                   |
| `--trust-plugin`           | Desligado                                                                                | Pule o prompt de confiança de primeira execução para um plugin cujo código e conjunto você executaria você mesmo. Passe-o em CI para que o trabalho nunca seja recusado ou deixado esperando no prompt. Veja [O que uma execução pode acessar](#security)                                                                          |
| `--mocks <mode>`           | `record`                                                                                 | `record` responde chamadas de ferramenta MCP a partir de [mocks](#mock-mcp-servers), não inicia os servidores MCP reais do plugin e salva respostas de mock de agente para reprodução. `off` ignora mocks e inicia os servidores MCP reais do plugin                                                                               |
| `--allow-real-servers`     | Desligado                                                                                | Com `--mocks record`, também inicie os servidores MCP reais do plugin para servidores que não têm mock                                                                                                                                                                                                                             |
| `--json [path]`            | Desligado                                                                                | Imprima o [documento de resultado](#json-result) para stdout, ou escreva-o em um caminho terminando em `.json`. A execução é silenciosa: sem linhas de progresso ou tabela de resumo                                                                                                                                               |
| `--output-dir <dir>`       | `<eval dir>/results/<timestamp>/`                                                        | Onde `aggregate-result.json` e `report.html` vão                                                                                                                                                                                                                                                                                   |
| `--no-publish`             |                                                                                          | Mantenha o relatório HTML local. Veja [Relatório HTML](#html-report)                                                                                                                                                                                                                                                               |
| `--publish-report`         |                                                                                          | Publique o relatório mesmo onde ficaria local por padrão, como uma execução que uma sessão de Claude Code iniciou                                                                                                                                                                                                                  |
| `--keep-temp`              | Desligado                                                                                | Mantenha o diretório de sandbox de cada execução e imprima seu caminho, para depuração do que Claude produziu                                                                                                                                                                                                                      |

<h3 id="run-evals-in-ci">
  Executar evals em CI
</h3>

Em seu trabalho de CI, execute o conjunto com `--json` para escrever o resultado para arquivamento e falhe a compilação no código de saída. Passe `--trust-plugin` para que o trabalho nunca espere no [prompt de confiança de primeira execução](#security), fixe ambos os modelos para que as pontuações sejam comparáveis ao longo do tempo, mantenha o relatório local e defina um teto de custo como um limite superior:

```bash theme={null}
claude plugin eval . \
  --trust-plugin \
  --json results.json \
  --threshold 0.8 \
  --model claude-sonnet-5 \
  --judge-model claude-haiku-4-5 \
  --no-publish \
  --max-cost-usd 20
```

O código de saída do trabalho diz o que aconteceu:

| Código de saída | Significado                                                                                                                                                                                                                                 |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 0               | Cada caso marcou em ou acima de `--threshold` e cada arquivo de caso carregou                                                                                                                                                               |
| 1               | Um caso marcou abaixo do limite, um arquivo de caso falhou ao carregar, nenhum caso foi encontrado, uma execução não pôde ser iniciada, o diretório do plugin não é confiável e `--trust-plugin` não foi passado, ou uma opção era inválida |
| 2               | Execução parcial: o teto `--max-cost-usd` foi atingido, ou sua credencial foi rejeitada antes ou na primeira execução. `results.json` ainda é escrito com `partial: true` e o motivo                                                        |
| 130             | Interrompido. Resultados parciais são escritos                                                                                                                                                                                              |
| 143             | Terminado, como por um timeout de CI                                                                                                                                                                                                        |

Problemas ao escrever ou publicar o relatório HTML nunca mudam o código de saída.

Para ver por que um caso marcou baixo, execute-o localmente sem `--json` para que o progresso por execução e as linhas do avaliador sejam impressas.

Um executor de CI também precisa destes em vigor:

* **Instalar e credenciais**: um executor de CI precisa de uma instalação de Claude Code e [credenciais no ambiente](/docs/pt/authentication) como `ANTHROPIC_API_KEY`.
* **Confiança**: sem `--trust-plugin`, um trabalho cujo diretório de checkout Claude Code ainda não confia precisa do [prompt de confiança de primeira execução](#trust-the-plugin-directory), e uma execução que não pode perguntar é recusada com saída 1.
* **`init` em CI**: `claude plugin eval init` precisa de um terminal para fazer suas perguntas; em CI, execute `claude plugin eval init --bare <name>` para obter o modelo em branco.

Para manter custos previsíveis, dê a cada conjunto de mudança rápida apenas avaliadores que não chamam um juiz, use `--ablation none` onde você não precisa de `Δ` e deixe documentos `partial: true` e execuções com `skippedPaidGraders` fora de qualquer tendência que você gráfico.

<h2 id="read-the-results">
  Leia os resultados
</h2>

Cada execução com pelo menos um caso escreve um diretório `results/<timestamp>/` dentro do diretório de eval, contendo `aggregate-result.json` e `report.html`. Para um destino de caminho que está sob o plugin; para um plugin que você nomeou, está sob seu diretório atual, como a [tabela de destino](#choose-what-to-evaluate) mostra. A tabela de resumo, o JSON e o relatório todos renderizam os mesmos dados de resultado.

<h3 id="html-report">
  Relatório HTML
</h3>

`report.html` é um arquivo único e autossuficiente que não faz solicitações externas, então você pode anexá-lo a um trabalho de CI ou abri-lo do disco. Este exemplo é o topo de um relatório para uma execução de conjunto de três casos com `--threshold 0.8`; o custo mostrado é uma estimativa de preço de lista e varia com o modelo e o número de casos:

<img src="https://mintcdn.com/claude-code/qq7LHDi_F0aeFHgk/images/plugin-eval-report.png?fit=max&auto=format&n=qq7LHDi_F0aeFHgk&q=85&s=106eb6e6a70a6565f891ea3a4564f87d" alt="Topo de um relatório de eval: uma linha de veredicto lendo &#x22;Plugin effect: +33.3 pts vs baseline, improved 2, flat 1, regressed 0 of 3 cases&#x22;, cinco blocos de resumo para pontuação do conjunto, delta de ablação, pontuação de baseline, casos passando no limite e execuções perfeitas, então o primeiro caso com seu delta, barra de pontuação e uma execução cujos dois avaliadores mostram aprovação" width="1360" height="1032" data-path="images/plugin-eval-report.png" />

Leia-o de cima para baixo:

* **A linha de veredicto e os blocos** respondem se o plugin ajudou em todo o conjunto. A pontuação do conjunto é a média das pontuações com plugin por caso, Ablation Δ é o quão longe isso fica acima ou abaixo da pontuação de baseline, e Cases conta quantos atingiram o limite. Perfect runs é a proporção de execuções com plugin onde cada avaliador passou.
* **Cada cartão de caso** mostra o próprio `Δ` do caso e a pontuação com plugin, com uma marca na barra no limite. Um caso cujo `Δ` é negativo recebe uma borda esquerda vermelha, então as regressões se destacam quando você rola.
* **Dentro de um caso**, as execuções com plugin vêm primeiro e as execuções de baseline depois. Cada execução lista seus avaliadores com um chip de aprovação ou reprovação. Um avaliador reprovado já está expandido com sua explicação, e um avaliador `llm` também mostra os votos do juiz e a evidência que foi mostrada, que é onde você descobre por que uma execução teve uma pontuação baixa. Avaliadores que não contam para a pontuação, como `tool_used: Skill`, carregam um badge de `plugin-fired indicator`.
* **Prompt e Graders**, abaixo das execuções, mostram o prompt do caso e a rubrica ou padrão de cada avaliador, para que alguém lendo o relatório sem o conjunto possa ver o que foi perguntado e o que contou como bom.

Se você estiver conectado com uma assinatura claude.ai e [artifacts](/docs/pt/artifacts) estiverem disponíveis para sua conta, Claude Code também publica o relatório como um artefato privado e imprime `Published: <url>`. Passe `--no-publish` para mantê-lo local. Se nenhuma linha `Published:` aparecer, como com autenticação de chave de API, o arquivo local é o relatório.

Uma execução que uma sessão de Claude Code iniciou, como quando você pede a Claude para executar o conjunto para você, também fica local, e sua linha `Report:` diz `kept local`. Adicione `--publish-report` a esse comando para publicá-lo.

<h3 id="json-result">
  Resultado JSON
</h3>

`aggregate-result.json` e saída `--json` é um documento versionado com `schemaVersion: 1` para scripts de CI analisarem. Os nomes de campo são camelCase e novos campos são adicionados sem renomear os existentes, então escreva seu script para ignorar campos que não reconhece.

Estes são os campos que um script de gating geralmente lê. O documento também carrega a configuração do conjunto, cada definição de avaliador e resultados de avaliador por execução com explicações e evidências:

| Campo                                             | Significado                                                                                                                                                                                                        |
| :------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `partial`, `partialReason`                        | `true` com `cost_ceiling`, `interrupted` ou `auth_failed` quando o conjunto não terminou. Deixe resultados parciais fora de gráficos de tendência                                                                  |
| `aggregates.overallScore`                         | Pontuação média do caso em todo o conjunto                                                                                                                                                                         |
| `aggregates.casesPassed`, `aggregates.casesTotal` | Casos em ou acima de `--threshold` e o total                                                                                                                                                                       |
| `aggregates.meanDelta`                            | `Δ` médio entre casos, sob o modo de dois braços                                                                                                                                                                   |
| `cases[].name`                                    | Nome do caso                                                                                                                                                                                                       |
| `cases[].aggregates.score`                        | Pontuação média de execução de braço com para o caso                                                                                                                                                               |
| `cases[].aggregates.delta`                        | Pontuação de braço com menos pontuação de braço sem. Omitido quando os braços não são comparáveis                                                                                                                  |
| `cases[].arms.with[].error`                       | `null`, ou por que uma execução terminou anormalmente, como `timed out after 300s`. Uma execução que começou mas terminou mal ainda é classificada no que produziu, então um erro não nulo não implica pontuação 0 |
| `cases[].arms.with[].aborted`                     | Presente quando um [mock](#mock-mcp-servers) `expect:` ou `abort_when` parou a execução, com `server`, `tool` e `reason`. A execução marca 0 e `error` permanece `null`                                            |
| `cases[].arms.with[].skippedPaidGraders`          | `true` quando o teto de custo pulou os avaliadores de juiz dessa execução, então sua pontuação não é comparável                                                                                                    |
| `costUsd`, `durationSeconds`, `claudeVersion`     | Custo estimado a preço de lista incluindo chamadas de juiz, segundos de parede e a versão de Claude Code que executou o conjunto                                                                                   |

<h2 id="security">
  O que uma execução pode acessar
</h2>

`claude plugin eval` carrega as skills, hooks e agents do plugin de destino e executa sua suite de avaliação em sua máquina, como você. Apontar para um plugin é a mesma decisão de confiança que `claude --plugin-dir`, portanto, avalie apenas plugins em que você confia.

O isolamento descrito nesta seção limita o que o agent sob teste pode alcançar; não é uma barreira contra o próprio código do plugin, e uma suite que passa não diz nada sobre se o plugin é seguro.

<h3 id="trust-the-plugin-directory">
  Confie no diretório do plugin
</h3>

Na primeira vez que você executa `claude plugin eval` contra um diretório, Claude Code pergunta `Trust this plugin directory?` antes de carregar qualquer coisa dele, a menos que você já tenha aceito o prompt de confiança lá em uma sessão interativa de `claude`. Dentro de um repositório git, responder sim confia em todo o repositório, para sessões interativas também. Quando stdin ou stdout não é um terminal, sob `--json`, ou quando a variável de ambiente `CI` é definida como um valor verdadeiro como `true`, a execução não pode perguntar e é recusada com saída 1; passe `--trust-plugin` para afirmar a confiança você mesmo, apenas para um plugin que você executaria em sua própria máquina. Um alvo que você nomeia em vez de fornecer como um caminho, significando um plugin instalado ou um plugin de diretório de skills, pula o prompt.

Algumas partes do plugin e da suite são executadas apenas quando você passa sua flag para essa execução:

* Um [`scaffold_script`](#add-setup-or-history-with-case-yaml) de caso com `--scaffold`
* [Tools além do conjunto somente leitura](#grant-tools) com `--allow-tools`
* Os [servidores MCP reais](#mock-mcp-servers) do plugin com `--allow-real-servers` ou `--mocks off`

Um `allowed_tools` de caso e um frontmatter `allowed-tools` próprio de uma skill não podem ampliar nenhum deles.

Quando o plugin inclui hooks que você não escreveu, ou você inicia seus servidores MCP reais, trate suas pontuações como consultivas a menos que você o tenha executado em um ambiente isolado como um container ou CI runner, já que hooks e servidores são executados fora do sandbox do agent e poderiam modificar os arquivos que os avaliadores leem.

<h3 id="how-runs-are-isolated">
  Como as execuções são isoladas
</h3>

Cada execução obtém um diretório home temporário, diretório de trabalho e configuração de Claude Code, e o agent sob teste é executado lá como um processo filho `claude -p` com apenas seu plugin carregado. Mantenha essas consequências em mente quando você escrever casos:

* **Nada pessoal ou no nível do projeto é carregado.** Suas configurações de usuário, hooks, arquivos `CLAUDE.md`, servidores MCP, outros plugins instalados, memória e skills estão ausentes, e nenhum `.claude/` com escopo de projeto ou `.mcp.json` acima do sandbox é lido. A maioria do seu ambiente de shell também é retida; apenas uma [lista de permissões](#prompt-md-fields) e variáveis `EVAL_*` alcançam a execução. Se o plugin precisar de configuração, envie-o no plugin, crie-o em um `scaffold_script`, ou passe variáveis `EVAL_*`.
* **A política gerenciada ainda pode restringir uma execução.** Restrições em [configurações gerenciadas](/docs/pt/managed-settings) que um administrador implantou na máquina se aplicam dentro de uma execução, portanto, os resultados em uma máquina gerenciada podem diferir de uma não gerenciada por essa política.
* **A ferramenta Artifact está desativada.** Uma skill que publica um [artifact](/docs/pt/artifacts) pode ser avaliada apenas no que ela produz antes dessa etapa.
* **As definições de caso estão ocultas do agent.** Uma execução não pode ler o diretório eval, portanto, Claude não pode ver o prompt do caso, seus avaliadores ou casos irmãos.
* **Nenhum sandbox de rede fora dos comandos shell.** Comandos shell que você concede são executados sob as regras de rede do sandbox. Uma concessão `WebFetch(domain:…)` alcança esse domínio diretamente, e os hooks próprios do plugin e quaisquer servidores MCP reais que você inicia podem alcançar qualquer host.

<h2 id="eval-suite-reference">
  Referência de conjunto de eval
</h2>

Tudo o que um conjunto de eval pode conter vive sob o diretório de eval do plugin, `evals/` a menos que você [configure outro](#use-a-different-eval-directory). Esta árvore mostra cada arquivo que `claude plugin eval` lê ou escreve lá; apenas `prompt.md` ou `case.yaml` é necessário para um caso existir:

```text theme={null}
evals/
├── <case>/                        # one directory per case; nest under a non-case directory to group
│   ├── prompt.md                  # frontmatter: case and run fields; body: the prompt
│   ├── case.yaml                  # optional: context.* fields, or the whole case in one file
│   ├── graders/
│   │   └── <name>.md              # one grader per file; frontmatter: type and options; body: rubric
│   ├── mocks/                     # optional: mocks for this case only, same layout as below
│   └── <fixtures, scripts, transcripts referenced by case.yaml>
├── mocks/                         # optional: suite-wide MCP mocks
│   ├── <server>/
│   │   ├── <tool>.md              # one mocked tool; body: the tool result
│   │   ├── _server.md             # optional: one agent that answers several tools
│   │   ├── _tools.json            # optional: saved tools/list response for real descriptions and schemas
│   │   └── fixtures/              # files inserted with {{file:fixtures/...}}
│   └── .replay/<server>/          # adopted agent-mock recordings, answered without a model call
└── results/<timestamp>/           # written by each run; add results/ to .gitignore
    ├── aggregate-result.json
    ├── report.html
    └── mock-recordings/           # agent-mock answers from clean runs, with ADOPT.txt
```

<h3 id="prompt-md-fields">
  prompt.md frontmatter
</h3>

O frontmatter `prompt.md` aceita esses campos. Uma chave desconhecida é um erro:

| Campo                  | Padrão                           | Propósito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :--------------------- | :------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `schema_version`       | `"1.1"`, definido para você      | Versão do formato de caso. Casos escritos como `prompt.md` o obtêm automaticamente, então você raramente o define                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `name`                 | O nome do diretório              | Nome do caso. Globs `--case` o combinam e o relatório o chave                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `description`          |                                  | Para humanos. Não usado em tempo de execução                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `tags`                 | `[]`                             | Rótulos para filtragem `--tag`. Um caso é executado se qualquer uma de suas tags corresponder                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `plugins`              | O plugin envolvente mais próximo | Diretórios de plugin sob teste, relativos ao diretório de caso. Defina `plugins: ["../.."]` quando a detecção automática não encontra seu plugin; veja [o plugin não carregou](#the-baseline-arm-shows-no-plugin-or-delta-is-zero)                                                                                                                                                                                                                                                                                                             |
| `runs`                 | `3`                              | Execuções por braço, 1 a 50. `--runs` o substitui                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `expected_outcome`     |                                  | Para humanos. Não usado em tempo de execução                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `model`                | O padrão da sessão filha         | Modelo para o agente sob teste. `--model` o substitui                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `max_turns`            | `10`                             | Limite de turno, até 200. Atingi-lo é registrado como um erro de execução e geralmente reduz a pontuação, então defina-o generosamente                                                                                                                                                                                                                                                                                                                                                                                                         |
| `timeout_seconds`      | `300`                            | Limite de parede por execução, até 3600                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `allowed_tools`        | `[]`                             | Ferramentas que o caso quer, como `[Read, Glob, Grep, Skill]`. Ferramentas somente leitura são concedidas quando listadas aqui; para qualquer outra coisa, veja [Conceda ferramentas](#grant-tools)                                                                                                                                                                                                                                                                                                                                            |
| `append_system_prompt` |                                  | Texto anexado ao prompt do sistema da sessão filha                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `env`                  | `{}`                             | Variáveis de ambiente extras para a sessão filha. As chaves devem corresponder a `EVAL_[A-Z0-9_]*`; qualquer outra chave falha a execução. A execução herda apenas uma lista de permissões do seu shell: básicos como `PATH` e localidade, configurações de proxy e certificado, as variáveis que selecionam e autenticam seu provedor de modelo, a maioria de `ANTHROPIC_*` e `CLAUDE_CODE_*` configuração e `EVAL_*`. Para entregar ao plugin qualquer outra coisa, como uma configuração de toolchain, exporte-a como uma variável `EVAL_*` |

<h3 id="case-yaml-fields">
  case.yaml fields
</h3>

`case.yaml` é uma alternativa ou complemento para `prompt.md`: descreve um caso em YAML e adiciona os campos que apontam para outros arquivos. Requer `schema_version: "1.1"` e `name`. Os campos `prompt.md` `description`, `tags`, `plugins`, `runs` e `expected_outcome` vão no nível superior; `model`, `max_turns`, `timeout_seconds`, `allowed_tools`, `append_system_prompt` e `env` vão sob `execution:`. Quando ambos os arquivos existem, o frontmatter `prompt.md` substitui os campos `case.yaml` correspondentes, o corpo `prompt.md` é o prompt e `graders/*.md` são adicionados após qualquer avaliador listado em `case.yaml`.

Esses campos existem apenas em `case.yaml`:

| Campo                     | Propósito                                                                                                                                                                                                                                                    |
| :------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `context.scaffold_script` | Um script Bash no diretório de caso que é executado no espaço de trabalho vazio antes de Claude começar, para criar arquivos de fixture ou um repositório git. Ele é executado apenas quando você passa [`--scaffold`](#add-setup-or-history-with-case-yaml) |
| `context.history_file`    | Uma transcrição `.jsonl` no diretório de caso para retomar. O prompt do caso se torna o próximo turno do usuário                                                                                                                                             |
| `context.add_dirs`        | Diretórios dentro do diretório de caso que Claude pode ler durante a execução, concedido somente leitura                                                                                                                                                     |
| `execution.prompt`        | O prompt, quando você mantém o caso inteiro em `case.yaml` e omite `prompt.md`                                                                                                                                                                               |
| `graders`                 | Uma lista de avaliadores, cada um com um `name` mais as mesmas chaves que um arquivo `graders/*.md` leva em frontmatter. Para avaliadores `llm`, coloque a rubrica em `criteria`                                                                             |

<h3 id="grader-frontmatter">
  Grader frontmatter
</h3>

Cada arquivo de avaliador sob `graders/` leva essas chaves em frontmatter, mais as opções para seu tipo. O nome do avaliador é o nome do arquivo sem `.md`:

| Chave    | Padrão       | Propósito                                                                                                                                                                                                                 |
| :------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`   | obrigatório  | Um dos [tipos de avaliador](#grader-types)                                                                                                                                                                                |
| `weight` | `1`          | Peso relativo na pontuação da execução. Qualquer número positivo                                                                                                                                                          |
| `arm`    | não definido | `with-only` exclui o avaliador da pontuação em uma [execução de dois braços](#compare-against-a-no-plugin-baseline); `both` força um avaliador que Claude Code excluiria de outra forma a ser pontuado em ambos os braços |

<h4 id="what-a-grader-can-look-at">
  O que um avaliador pode ver
</h4>

Avaliadores `regex` pegam um `target` e avaliadores `llm` pegam um `focus`. Ambos aceitam os mesmos valores:

| Valor                            | O que o avaliador vê                                                                                                                                                                                                                                                                                                                         |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `last_message`                   | Texto da resposta final de Claude. Este é o padrão                                                                                                                                                                                                                                                                                           |
| `trace`                          | A sessão inteira como JSON, uma mensagem por linha. Um avaliador `regex` vê cada mensagem; um juiz `llm` vê os primeiros 12 e os últimos 12. Aspas e quebras de linha dentro dela são escapadas em JSON, então uma regex corresponde a `\"` em vez de `"`                                                                                    |
| `files`                          | A lista de caminhos que Claude criou durante a execução, um por linha. Não seus conteúdos e não arquivos que um scaffold criou ou que Claude apenas modificou                                                                                                                                                                                |
| `{ source: file, path: <path> }` | O conteúdo de um arquivo no espaço de trabalho após a execução. Use isso para classificar o que o plugin produziu. Um arquivo PNG, JPEG, GIF ou WebP é mostrado a um juiz `llm` como uma imagem. Um juiz `llm` recusa outros arquivos binários como `.pptx` ou PDF; renderize-os para uma imagem ou escreva-os como texto e classifique isso |
| `mock_calls`                     | Cada chamada que Claude fez a uma [ferramenta MCP mockificada](#mock-mcp-servers), com sua entrada e a resposta do mock                                                                                                                                                                                                                      |

<h4 id="grader-types">
  Tipos de avaliador
</h4>

Cada tipo de avaliador abaixo lista suas opções e quando passa:

| Tipo          | Opções                                | Passa quando                                                                                                                                                                                                                                                          |
| :------------ | :------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `regex`       | `pattern`, `flags`, `match`, `target` | A regex JavaScript `pattern` é encontrada no destino. Defina `match: not_contains` para exigir ausência ou `match: "count:N"` para exigir exatamente N correspondências. Coloque insensibilidade a maiúsculas/minúsculas em `flags: i`; `(?i)` inline não é suportado |
| `tool_used`   | `tool`, `input_match`, `min`, `max`   | O número de chamadas para `tool` cuja entrada codificada em JSON corresponde à regex `input_match` opcional está entre `min`, padrão 1, e `max`, padrão ilimitado. Para afirmar que uma ferramenta nunca foi chamada, defina ambos `min: 0` e `max: 0`                |
| `tool_order`  | `before`, `after`                     | Ambas as ferramentas foram chamadas e a primeira chamada `before` correspondente precede a primeira chamada `after` correspondente. Cada um é um nome de ferramenta ou `{ tool, input_match }`                                                                        |
| `file_exists` | `path`, `exists`                      | Um arquivo que Claude criou corresponde ao glob `path`, ou nenhum com `exists: false`. Apenas arquivos criados durante a execução contam                                                                                                                              |
| `llm`         | `criteria`, `focus`                   | Um modelo de juiz vota PASS na rubrica em pelo menos dois de três votos. No layout `.md` o corpo do arquivo é os critérios                                                                                                                                            |
| `baseline`    | `baseline_file`, `criteria`           | Um juiz encontra a execução satisfaz os critérios pelo menos tão bem quanto a transcrição de referência em `baseline_file`, um `.jsonl` no diretório de caso                                                                                                          |

<h3 id="mock-files">
  Mock files
</h3>

Um arquivo `<tool>.md` sob `mocks/<server>/` responde uma ferramenta. Seu corpo é o resultado da ferramenta, com substituições `{{input.<field>}}` e `{{file:fixtures/<name>}}`. Seu frontmatter aceita essas chaves:

| Chave        | Padrão       | Propósito                                                                                                                                                                                                                                                                                                     |
| :----------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`       | `fixed`      | `fixed` retorna o corpo como escrito. `agent` trata o corpo como instruções para um modelo pequeno que joga o servidor para a execução e vê chamadas anteriores como histórico                                                                                                                                |
| `expect`     | não definido | Um mapa de caminhos de entrada com pontos para um nome de tipo como `string`, `number`, `boolean`, `array` ou `object`, um `/regex/`, um literal ou uma lista de literais permitidos. Uma chamada que viola aborta a execução com pontuação 0 e é relatada como `aborted` com o servidor, ferramenta e motivo |
| `error`      | `false`      | `fixed` apenas. Retorne o corpo como um erro de ferramenta                                                                                                                                                                                                                                                    |
| `abort_when` | não definido | `agent` apenas. Prosa listando as únicas condições sob as quais o agente pode abortar a execução                                                                                                                                                                                                              |

Dois arquivos opcionais ficam ao lado dos arquivos de ferramenta no diretório de um servidor:

* **`_server.md`**: um único mock `type: agent` que responde várias ferramentas, listadas em sua chave frontmatter `tools:`. Um `<tool>.md` para a mesma ferramenta tem precedência. Coloque uma guarda `expect:` no `<tool>.md` individual, não aqui
* **`_tools.json`**: uma resposta `tools/list` salva do servidor real, para que ferramentas mockificadas carreguem suas descrições reais e esquemas de entrada em vez de um espaço reservado permissivo

O diretório `mocks/` próprio de um caso usa o mesmo layout e substitui os arquivos de mocks do conjunto arquivo por arquivo.

<h2 id="troubleshooting">
  Solução de problemas
</h2>

Estes são os problemas que os autores mais frequentemente encontram, chaveados no que você vê.

<h3 id="plugin-eval-is-currently-in-early-access">
  "plugin eval is currently in early access"
</h3>

Sua compilação é anterior à disponibilidade geral do comando. Execute `claude update` e execute o comando novamente em uma sessão fresca.

<h3 id="plugin-eval-is-currently-unavailable">
  "plugin eval is currently unavailable"
</h3>

Anthropic desligou o comando do lado do servidor. Nada em sua máquina o liga novamente; execute `claude update` e tente novamente em uma sessão fresca mais tarde.

<h3 id="is-not-a-trusted-plugin-directory-and-this-run-cannot-stop-to-ask-you-about-it">
  "is not a trusted plugin directory, and this run cannot stop to ask you about it"
</h3>

Esta é a primeira execução contra um diretório que Claude Code ainda não confia, e não pode perguntar porque stdin ou stdout não é um terminal, você passou `--json`, ou a variável de ambiente `CI` está definida como um valor verdadeiro como `true`. Execute `claude plugin eval <dir>` uma vez em um terminal e responda o prompt, ou passe `--trust-plugin` se você confia no código e conjunto do plugin. Veja [O que uma execução pode acessar](#security).

<h3 id="no-eval-cases-found">
  "No eval cases found"
</h3>

Nenhum `<case>/prompt.md` ou `<case>/case.yaml` existe sob o diretório de eval em vigor, ou seus filtros `--case` e `--tag` não corresponderam a nenhum caso. Execute a partir da raiz do plugin, ou execute `claude plugin eval init` para criar um conjunto.

<h3 id="the-baseline-arm-shows-no-plugin-or-delta-is-zero">
  O braço de linha de base mostra nenhum plugin, ou delta é zero
</h3>

Se o resumo não tem coluna `W/OUT`, ou o caso falha com "ablation requested but no plugin resolved", nenhum plugin foi encontrado para o caso. Adicione `plugins: ["../.."]` ao caso, dando o caminho do diretório de caso para o diretório de plugin.

Se o plugin carregou e `Δ` ainda está próximo a zero com seu avaliador `tool_used: Skill` falhando, isso é geralmente um achado real, significando que a `description` do skill não dispara no fraseado do prompt. Ajuste a descrição e re-execute o mesmo conjunto.

<h3 id="agent-type-’-’-not-found-for-one-of-your-plugin’s-agents">
  "Agent type '...' not found" para um dos agentes do seu plugin
</h3>

Por padrão, cada caso é executado tanto com seu plugin quanto sem ele, e as execuções sem ele são a [linha de base sem plugin](#the-no-plugin-baseline). Quando Claude despacha um dos agentes do seu plugin em uma execução de linha de base, a chamada da ferramenta Agent falha com `Agent type '<plugin>:<agent-name>' not found. Available agents: ...`. A lista nomeia apenas agentes que existem sem o plugin, como os [subagentes integrados](/docs/pt/sub-agents#built-in-subagents).

O erro é esperado, já que `Δ` compara suas execuções do plugin contra a linha de base. No resultado JSON, as execuções de linha de base estão sob `cases[].arms.without`.

Em execuções com seu plugin carregado, um caso que lista `Agent` em `allowed_tools` pode despachar um dos agentes do seu plugin pelo seu nome com namespace, como `my-plugin:code-reviewer` para o agente `code-reviewer` em um plugin nomeado `my-plugin`. Para pular as execuções de linha de base, passe `--ablation none`.

<h3 id="everything-scores-zero-although-the-right-files-were-produced">
  Tudo marca zero embora os arquivos corretos tenham sido produzidos
</h3>

Seus avaliadores visam `files`, a lista de caminhos criados, quando você quis o conteúdo do arquivo. Use `{ source: file, path: <path> }` como o `target` ou `focus`. Separadamente, `file_exists` conta apenas arquivos criados durante a execução, então um arquivo que o scaffold criou ou que Claude apenas editou é invisível para ele; classifique seu conteúdo ou use `tool_used` em `Edit`.

<h3 id="a-regex-over-the-trace-doesn’t-match-text-i-can-see">
  Uma regex sobre o trace não corresponde ao texto que posso ver
</h3>

* **Alvo errado**: o `target` padrão é `last_message`, não o trace.
* **Escape JSON**: quando você visa `trace`, é JSON por linha, então aspas aparecem como `\"`.
* **Sintaxe de regex**: regexes usam sintaxe JavaScript, então coloque `i` em `flags` em vez de escrever `(?i)`.

<h3 id="tools-are-denied-mcp-tools-are-missing-or-bash-won’t-run">
  Ferramentas são negadas, ferramentas MCP estão faltando ou Bash não será executado
</h3>

Qualquer coisa além do conjunto somente leitura precisa de sua concessão, como `--allow-tools Bash Write`. Seus servidores MCP pessoais nunca carregam em uma execução. Os servidores próprios do plugin não começam a menos que você [opte por](#mock-mcp-servers), e suas ferramentas então também precisam de uma concessão `--allow-tools "mcp__plugin_<plugin>_<server>__*"`; uma ferramenta mockificada não precisa de nenhuma.

<h3 id="the-run-exits-1-but-the-results-look-fine">
  A execução sai 1 mas os resultados parecem bons
</h3>

O `--threshold` padrão é 1.0, então o comando sai 1 quando qualquer caso marca abaixo do perfeito. Defina um limite que corresponda à sua barra. Saída 1 também cobre um arquivo de caso que falhou ao carregar, que é relatado em stderr acima da tabela.

<h3 id="json-output-path-must-end-in-json">
  "--json output path must end in .json"
</h3>

Você colocou o destino após `--json`, então foi lido como o caminho de saída. Coloque o destino primeiro, como em `claude plugin eval . --json`, ou dê a `--json` um caminho `.json` explícito.

<h3 id="a-grader-shows-passed-false-under-a-run-that-scored-1-0">
  Um avaliador mostra passed: false sob uma execução que marcou 1.0
</h3>

Esse avaliador é excluído da pontuação por design em uma execução de dois braços, e seu campo `scored` é `false`. Veja [Comparar com uma linha de base sem plugin](#compare-against-a-no-plugin-baseline).

<h3 id="runs-fail-with-a-usage-limit-or-rate-limit-error-partway-through">
  Execuções falham com um erro de limite de uso ou limite de taxa no meio
</h3>

Se sua conta atinge o limite de uso do plano ou um limite de taxa de API enquanto um conjunto está em execução, cada execução posterior termina com esse erro, é classificada no que produziu e geralmente marca 0. O conjunto ainda termina e não é marcado `partial`, então o resultado pode parecer uma regressão. Verifique a coluna `NOTES` ou `cases[].arms.with[].error` no JSON para a mensagem de limite antes de confiar nas pontuações, então re-execute após o limite redefinir, com `--runs 1` ou um filtro `--case` se você precisar ficar abaixo dele.

<h3 id="runs-time-out-or-hit-the-turn-cap">
  Execuções expiram ou atingem o limite de turno
</h3>

Os padrões são 10 turnos e 300 segundos. Aumente `max_turns` e `timeout_seconds` no caso para tarefas que precisam de mais, e use `--max-cost-usd` como o teto de custo em vez de limites apertados por execução.

<h2 id="see-also">
  Veja também
</h2>

* [Criar um plugin](/docs/pt/plugins/create): construa o plugin que você está testando e carregue-o com `--plugin-dir` durante o desenvolvimento
* [Referência de comandos de plugin](/docs/pt/plugins/cli-reference#plugin-eval): as entradas de comando `plugin eval` e `plugin eval init`. A chave [`experimental.evals`](/docs/pt/plugins/manifest-reference#fields) do manifesto está na referência do manifesto
* [Skills](/docs/pt/skills): como a descrição de um skill decide quando Claude o invoca, que é o que um caso que verifica se o skill dispara está medindo
* [Sandboxing](/docs/pt/sandboxing): a sandbox de nível do SO que se aplica quando você concede Bash a uma execução
* [Publicar um plugin](/docs/pt/plugins/publish): publique o plugin uma vez que seu conjunto passa
* [Medir custo e uso do plugin](/docs/pt/plugins/measure): o que o plugin adiciona ao contexto de cada sessão e se as pessoas ainda o usam
