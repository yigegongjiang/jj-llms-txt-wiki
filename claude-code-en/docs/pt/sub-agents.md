> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Criar subagentes personalizados

> Crie e use subagentes de IA especializados no Claude Code para fluxos de trabalho específicos de tarefas e gerenciamento de contexto aprimorado.

Subagentes são assistentes de IA especializados que lidam com tipos específicos de tarefas. Use um quando uma tarefa secundária inundaria sua conversa principal com resultados de pesquisa, logs ou conteúdos de arquivo que você não referenciará novamente: o subagente faz esse trabalho em seu próprio contexto e retorna apenas o resumo. Defina um subagente personalizado quando você continua gerando o mesmo tipo de worker com as mesmas instruções.

Cada subagente é executado em sua própria janela de contexto com um prompt de sistema personalizado, acesso a ferramentas específicas e permissões independentes. Quando Claude encontra uma tarefa que corresponde à descrição de um subagente, ele delega para esse subagente, que funciona independentemente e retorna resultados. Para ver a economia de contexto na prática, a [visualização da janela de contexto](/docs/pt/context-window) apresenta uma sessão onde um subagente lida com pesquisa em sua própria janela separada.

<Note>
  Subagentes funcionam dentro de uma única sessão. Para executar muitas sessões independentes em paralelo e monitorá-las de um único lugar, consulte [agentes em segundo plano](/docs/pt/agent-view). Para sessões separadas que passam mensagens uma para a outra, consulte [mensagens entre sessões](/docs/pt/cross-session-messaging). Para uma equipe coordenada de sessões que Claude gera e supervisiona, consulte [equipes de agentes](/docs/pt/agent-teams).
</Note>

Subagentes ajudam você a:

* **Preservar contexto** mantendo exploração e implementação fora de sua conversa principal
* **Aplicar restrições** limitando quais ferramentas um subagente pode usar
* **Reutilizar configurações** entre projetos com subagentes no nível do usuário
* **Especializar comportamento** com prompts de sistema focados para domínios específicos
* **Controlar custos** roteando tarefas para modelos mais rápidos e baratos como Haiku

Claude usa a descrição de cada subagente para decidir quando delegar tarefas. Quando você cria um subagente, escreva uma descrição clara para que Claude saiba quando usá-lo.

Essas descrições ocupam contexto, então mantenha-as breves. Quando as descrições combinadas de seus subagentes, exceto os integrados, excedem 15.000 tokens, Claude Code mostra um [aviso na inicialização com a contagem total de tokens](/docs/pt/errors#agent-descriptions-are-over-the-15000-token-limit). Reduza os campos `description` de seus subagentes e mova detalhes para o prompt de sistema de cada subagente, que é carregado apenas quando esse subagente é executado.

<h2 id="built-in-subagents">
  Subagentes integrados
</h2>

Claude Code inclui subagentes integrados que Claude usa automaticamente quando apropriado. Cada um herda as permissões da conversa pai; a maioria é executada com um conjunto de ferramentas restrito.

Explore e Plan pulam seus arquivos CLAUDE.md e o snapshot de status git para manter a pesquisa rápida e econômica. Todos os outros subagentes integrados e [subagentes personalizados](#configure-subagents) carregam ambos, a menos que sua definição defina o campo [`omitClaudeMd`](#supported-frontmatter-fields) para pular os arquivos CLAUDE.md do usuário, projeto e local. Para o detalhamento completo do que chega a um subagente, consulte [o que é carregado na inicialização](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Um agente rápido e somente leitura otimizado para pesquisar e analisar bases de código.

    * **Model**: herda da conversa principal, limitado a Opus na Claude API, portanto Explore nunca é executado em um modelo mais caro do que aquele que você já escolheu para a sessão, a menos que você defina `CLAUDE_CODE_SUBAGENT_MODEL` e [force-o em cada subagente](#run-every-subagent-on-one-model)
    * **Tools**: ferramentas somente leitura; Write e Edit são negados
    * **Purpose**: descoberta de arquivos, pesquisa de código, exploração de base de código

    A partir da v2.1.198, Explore herda o modelo da conversa principal em vez de sempre ser executado em Haiku. Na Claude API, o modelo herdado é limitado a Opus: uma conversa principal em um nível superior executa Explore em Opus, e uma conversa principal em Sonnet ou Haiku executa Explore nesse mesmo modelo. Em qualquer outro provedor, como [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, ou Claude Platform on AWS](/docs/pt/third-party-integrations), Explore herda o modelo da conversa principal diretamente.

    Um [subagente de usuário ou projeto](#choose-the-subagent-scope) nomeado `Explore` substitui o integrado e mantém seu próprio campo `model`, portanto defina um com `model: haiku` para manter a exploração em um modelo de menor custo.

    Claude delega para Explore quando precisa pesquisar ou entender uma base de código sem fazer alterações. Isso mantém os resultados da exploração fora do contexto da sua conversa principal.

    Ao invocar Explore, Claude especifica um nível de minuciosidade: **quick** para buscas direcionadas, **medium** para exploração equilibrada, ou **very thorough** para análise abrangente.
  </Tab>

  <Tab title="Plan">
    Um agente de pesquisa usado durante [plan mode](/docs/pt/permission-modes#analyze-before-you-edit-with-plan-mode) para reunir contexto antes de apresentar um plano.

    * **Model**: herda da conversa principal, a menos que você defina `CLAUDE_CODE_SUBAGENT_MODEL` e [force-o em cada subagente](#run-every-subagent-on-one-model)
    * **Tools**: ferramentas somente leitura; Write e Edit são negados
    * **Purpose**: pesquisa de base de código para planejamento

    Quando você está em plan mode e Claude precisa entender sua base de código, ele delega a pesquisa para o subagente Plan para que a saída de exploração permaneça em uma janela de contexto separada enquanto a conversa principal permanece somente leitura.
  </Tab>

  <Tab title="General-purpose">
    Um agente capaz para tarefas complexas e multi-etapas que requerem exploração e ação.

    * **Model**: o modelo [`CLAUDE_CODE_SUBAGENT_MODEL`](#choose-a-model) se você definir um e nada atribuir um modelo de outra forma, caso contrário o modelo da conversa principal; [Escolha um modelo](#choose-a-model) declara a ordem completa, e [Execute cada subagente em um modelo](#run-every-subagent-on-one-model) mostra como fazer a variável substituir essas fontes
    * **Tools**: todas as ferramentas [disponíveis para subagentes](#available-tools)
    * **Purpose**: pesquisa complexa, operações multi-etapas, modificações de código

    Claude delega para general-purpose quando a tarefa requer exploração e modificação, raciocínio complexo para interpretar resultados, ou múltiplas etapas dependentes.
  </Tab>

  <Tab title="Other">
    Claude Code inclui agentes auxiliares adicionais para tarefas específicas. Estes são normalmente invocados automaticamente, então você não precisa usá-los diretamente.

    | Agent             | Model                                                                                             | When Claude uses it                                                                                                                                                                                                                                                                                                                                                    |
    | :---------------- | :------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | claude            | Nenhum próprio; segue a [ordem de modelo](#choose-a-model) quando Claude o gera como um subagente | Quando uma tarefa não se encaixa em um agente mais especializado. Um catch-all com todas as ferramentas [disponíveis para subagentes](#available-tools). Também o agente padrão para uma [sessão em background](/docs/pt/agent-view) despachada; [qual modo de permissão ele inicia](/docs/pt/agent-view#permission-mode-model-and-effort) depende de como a sessão foi iniciada |
    | statusline-setup  | Sonnet                                                                                            | Quando você executa `/statusline` para configurar sua linha de status                                                                                                                                                                                                                                                                                                  |
    | claude-code-guide | Haiku                                                                                             | Quando você faz perguntas sobre recursos do Claude Code                                                                                                                                                                                                                                                                                                                |
  </Tab>
</Tabs>

Os subagentes integrados são registrados por padrão em sessões interativas. Para restringi-los:

* Para bloquear um tipo integrado específico, adicione-o a `permissions.deny` conforme mostrado em [Desabilitar subagentes específicos](#disable-specific-subagents).
* Para impedir que Claude delegue a qualquer subagente, negue a ferramenta `Agent` em si com [`permissions.deny`](/docs/pt/permissions#tool-specific-permission-rules).
* Para remover apenas os subagentes integrados `Explore` e `Plan`, defina [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/pt/env-vars). Claude lê e explora arquivos diretamente em vez de delegar para eles. Requer Claude Code v2.1.198 ou posterior.
* Em [modo não interativo](/docs/pt/headless) e no [Agent SDK](/docs/pt/agent-sdk/overview), defina [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/pt/env-vars) para remover todos os tipos integrados e fornecer apenas os seus próprios.

Uma chamada de ferramenta Agent que omite `subagent_type` falha com [`subagent_type is required`](/docs/pt/errors#subagent-type-is-required) quando a sessão não tem nenhum subagente `general-purpose` para recorrer.

Além desses subagentes integrados, você pode criar os seus próprios com prompts personalizados, restrições de ferramentas, modos de permissão, hooks e skills. As seções a seguir mostram como começar e personalizar subagentes.

<h2 id="quickstart-create-your-first-subagent">
  Quickstart: criar seu primeiro subagente
</h2>

Subagentes são arquivos Markdown com frontmatter YAML. Para criar um, peça ao Claude para escrevê-lo para você, ou [escreva o arquivo você mesmo](#write-subagent-files).

A partir da v2.1.198, o comando `/agents` não abre mais o assistente de criação interativo; executá-lo imprime um lembrete para pedir ao Claude ou editar `.claude/agents/` diretamente. Os arquivos de subagente, campos de frontmatter e os locais `.claude/agents/` e `~/.claude/agents/` permanecem inalterados; apenas o assistente de terminal foi removido.

Este passo a passo cria um subagente no nível do usuário que revisa código e sugere melhorias.

<Steps>
  <Step title="Peça ao Claude para criar o subagente">
    No Claude Code, descreva o subagente que você deseja e onde salvá-lo:

    ```text wrap theme={null}
    Create a personal code-improver subagent in ~/.claude/agents/ that scans
    files and suggests improvements for readability, performance, and best
    practices. It should explain each issue, show the current code, and
    provide an improved version. Make it read-only and have it use Sonnet.
    ```

    Claude escreve o arquivo com um `name`, uma `description`, uma lista de `tools`, um `model` e um prompt de sistema.
  </Step>

  <Step title="Revise o arquivo">
    Abra `~/.claude/agents/code-improver.md` e confirme que o frontmatter corresponde ao que você pediu. O resultado se parece com isto:

    ```markdown theme={null}
    ---
    name: code-improver
    description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
    tools: Read, Grep, Glob
    model: sonnet
    ---

    You are a code improvement specialist. For each issue you find, explain
    the problem, show the current code, and provide an improved version.
    ```

    Como o arquivo está em `~/.claude/agents/`, o subagente está disponível em todos os projetos em sua máquina. Para limitá-lo a um projeto, mova-o para o diretório `.claude/agents/` desse projeto. [Escolha o escopo do subagente](#choose-the-subagent-scope) compara os dois.
  </Step>

  <Step title="Teste-o">
    Peça ao Claude para delegar para o novo subagente:

    ```text wrap theme={null}
    Use the code-improver agent to suggest improvements in this project
    ```

    Claude delega para seu novo subagente, que verifica a base de código e retorna sugestões de melhoria. Na transcrição, a delegação aparece como uma linha de chamada de ferramenta mostrando o nome do subagente seguido por uma breve descrição de tarefa, como `code-improver(Suggest code improvements)`.

    Se Claude não conseguir encontrar o novo subagente, reinicie o Claude Code e tente novamente. Isso acontece apenas quando `~/.claude/agents/` não existia antes da sessão começar, porque uma sessão em execução não detecta um diretório `agents` recém-criado.
  </Step>
</Steps>

Agora você tem um subagente que pode usar em qualquer projeto em sua máquina para analisar bases de código e sugerir melhorias.

Você também pode escrever arquivos de subagente manualmente, defini-los via flags CLI ou distribuí-los através de plugins. As seções a seguir cobrem todas as opções de configuração.

<Note>
  No Claude Code v2.1.197 e anterior, `/agents` abre um assistente interativo com uma aba **Running** que lista subagentes ativos e uma aba **Library** para criá-los, editá-los e deletá-los.&#x20;
</Note>

<h2 id="configure-subagents">
  Configurar subagentes
</h2>

A localização do arquivo de um subagente determina quem tem acesso a ele, e seu frontmatter determina o que ele pode fazer. Esta seção aborda onde os arquivos de subagente residem e cada campo que eles suportam.

<h3 id="choose-the-subagent-scope">
  Escolher o escopo do subagente
</h3>

Armazene arquivos de subagente em locais diferentes dependendo do escopo. Quando múltiplos subagentes compartilham o mesmo nome, Claude Code usa o que está no local de prioridade mais alta.

| Location                     | Scope                   | Priority    | How to create                                  |
| :--------------------------- | :---------------------- | :---------- | :--------------------------------------------- |
| Managed settings             | Organization-wide       | 1 (highest) | Deployed via [managed settings](/docs/pt/settings)  |
| `--agents` CLI flag          | Current session         | 2           | Pass JSON when launching Claude Code           |
| `.claude/agents/`            | Current project         | 3           | Ask Claude, or create the file manually        |
| `~/.claude/agents/`          | All your projects       | 4           | Ask Claude, or create the file manually        |
| Plugin's `agents/` directory | Where plugin is enabled | 5 (lowest)  | Installed with [plugins](/docs/pt/plugins/overview) |

**Subagentes de projeto** (`.claude/agents/`) são ideais para subagentes específicos de uma base de código. Verifique-os no controle de versão para que sua equipe possa usá-los e melhorá-los colaborativamente.

Subagentes de projeto são descobertos caminhando para cima a partir do diretório de trabalho atual, portanto cada `.claude/agents/` entre lá e a raiz do repositório é verificado. Quando mais de um desses diretórios aninhados define o mesmo `name`, Claude Code usa a definição mais próxima do diretório de trabalho.

Quando você adiciona um diretório com `--add-dir` ou `/add-dir`, Claude Code também carrega sua pasta `.claude/agents/`, junto com seus subagentes de projeto. Veja [Diretórios adicionais](/docs/pt/permissions#additional-directories-grant-file-access-not-configuration) para quais outros tipos de configuração carregam de `--add-dir`. Para compartilhar subagentes entre projetos sem `--add-dir`, use `~/.claude/agents/` ou um [plugin](/docs/pt/plugins/overview).

**Subagentes de usuário** (`~/.claude/agents/`) são subagentes pessoais disponíveis em todos os seus projetos.

Claude Code verifica `.claude/agents/` e `~/.claude/agents/` recursivamente, para que você possa organizar definições em subpastas como `agents/review/` ou `agents/research/`. O caminho do subdiretório não afeta como um subagente é identificado ou invocado, porque a identidade vem apenas do campo `name` do frontmatter.

Mantenha valores de `name` únicos em toda a árvore: se dois arquivos sob o mesmo diretório `.claude/agents/`, incluindo suas subpastas, declaram o mesmo nome, Claude Code carrega apenas um deles, escolhido pela ordem de leitura do sistema de arquivos em vez de uma precedência documentada. Entre diretórios de projeto aninhados, a definição mais próxima do diretório de trabalho vence, conforme descrito acima. O verificador de configuração [`/doctor`](/docs/pt/commands#all-commands) relata arquivos no mesmo diretório que compartilham um nome e propõe renomear ou remover todos exceto um. Antes da v2.1.205, `/doctor` abria uma tela de diagnósticos que listava duplicatas e mostrava qual definição estava ativa.

Diretórios `agents/` de plugin também são verificados recursivamente. Diferentemente dos escopos de projeto e usuário, uma subpasta dentro do diretório `agents/` de um plugin se torna parte do [identificador com escopo](#invoke-subagents-explicitly): um arquivo em `agents/review/security.md` no plugin `my-plugin` se registra como `my-plugin:review:security`.

**Subagentes definidos por CLI** são passados como JSON ao iniciar Claude Code. Eles existem apenas para essa sessão e não são salvos em disco, tornando-os úteis para testes rápidos ou scripts de automação. Você pode definir múltiplos subagentes em uma única chamada `--agents`:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    claude --agents '{
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }'
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    claude --agents @'
    {
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }
    '@
    ```
  </Tab>
</Tabs>

O flag `--agents` aceita JSON com um campo `prompt` mais estes campos de [frontmatter](#supported-frontmatter-fields): `description`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `omitClaudeMd` e `isolation`. Use `prompt` para o prompt de sistema, equivalente ao corpo markdown em subagentes baseados em arquivo. `color` e `experimental` não são aceitos aqui e são ignorados em vez de rejeitados.

Cada chave de nível superior no JSON é o nome do agente. Não comece um nome com `-`.

Para o que Claude Code faz com um valor que não consegue carregar, e os flags e variável de ambiente que pulam essa verificação, veja [`Invalid --agents configuration`](/docs/pt/errors#invalid-agents-configuration).

**Subagentes gerenciados** são implantados por administradores da organização. Coloque arquivos markdown em `.claude/agents/` dentro do [diretório de configurações gerenciadas](/docs/pt/managed-settings#delivery-mechanisms), usando o mesmo formato de frontmatter que subagentes de projeto e usuário. Definições gerenciadas têm precedência sobre subagentes de projeto e usuário com o mesmo nome.

**Subagentes de plugin** vêm de [plugins](/docs/pt/plugins/overview) que você instalou. Eles carregam automaticamente junto com seus subagentes personalizados e aparecem na digitação de @-menção sob seu nome com escopo. Veja a [referência de componentes de plugin](/docs/pt/plugins/components#agents) para detalhes sobre como criar subagentes de plugin.

<Note>
  Por razões de segurança, subagentes de plugin não suportam os campos de frontmatter `hooks`, `mcpServers` ou `permissionMode`. Estes campos são ignorados ao carregar agentes de um plugin. Se você precisar deles, copie o arquivo do agente para `.claude/agents/` ou `~/.claude/agents/`. Você também pode adicionar regras a [`permissions.allow`](/docs/pt/settings-reference#permissions-allow) em `settings.json` ou `settings.local.json`, mas estas regras se aplicam a toda a sessão, não apenas ao subagente do plugin.
</Note>

Definições de subagente de qualquer um desses escopos também estão disponíveis para [equipes de agentes](/docs/pt/agent-teams#use-subagent-definitions-for-teammates): ao gerar um colega de trabalho, você pode referenciar um tipo de subagente, e Claude Code aplica partes dessa definição ao colega de trabalho. Veja [equipes de agentes](/docs/pt/agent-teams#use-subagent-definitions-for-teammates) para quais partes se aplicam em cada modo de exibição.

<h3 id="write-subagent-files">
  Escrever arquivos de subagente
</h3>

Arquivos de subagente usam frontmatter YAML para configuração, seguido pelo prompt de sistema em Markdown:

<Note>
  Claude Code observa `~/.claude/agents/` e `.claude/agents/`. Quando você adiciona ou edita um arquivo de subagente no disco, ou pede a Claude para escrever um para você, Claude Code detecta a alteração em alguns segundos e a próxima delegação usa a definição atualizada, sem necessidade de reinicialização.

  Três casos ainda precisam de uma reinicialização:

  * O observador cobre apenas diretórios que existiam quando a sessão começou, portanto após criar o primeiro arquivo de agente de um escopo em um novo diretório `agents`, reinicie para carregá-lo.
  * Claude Code não observa `.claude/agents/` dentro de diretórios adicionados com `--add-dir` ou `/add-dir`, portanto após adicionar ou editar um subagente lá, reinicie para carregar a alteração.
  * Sessões iniciadas com `--disable-slash-commands` não observam esses diretórios.
</Note>

```markdown .claude/agents/code-reviewer.md theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

O frontmatter define os metadados e configuração do subagente. O corpo se torna o prompt de sistema que guia o comportamento do subagente. Subagentes recebem apenas este prompt de sistema mais detalhes básicos de ambiente como diretório de trabalho, não o prompt de sistema do Claude Code.

Em [modo não interativo](/docs/pt/headless), passe [`--append-subagent-system-prompt`](/docs/pt/cli-reference#cli-flags) para anexar seu texto ao final do prompt de sistema de cada subagente, incluindo subagentes aninhados, exceto um [subagente bifurcado](#fork-the-current-conversation), que reutiliza o prompt da conversa. Requer Claude Code v2.1.205 ou posterior. Se seu texto for muito longo para passar na linha de comando, salve-o em um arquivo e passe o caminho com `--append-subagent-system-prompt-file` em vez disso. O flag de arquivo requer Claude Code v2.1.261 ou posterior.

Um subagente começa no diretório de trabalho atual da conversa principal. Dentro de um subagente, comandos `cd` não persistem entre chamadas de ferramentas Bash ou PowerShell e não afetam o diretório de trabalho da conversa principal. Para dar ao subagente uma cópia isolada do repositório em vez disso, defina [`isolation: worktree`](#supported-frontmatter-fields).

Um subagente com `isolation: worktree` executa seus comandos Bash e PowerShell dentro de seu worktree. Um comando cujo diretório de trabalho se resolve para seu checkout principal, por exemplo porque o diretório worktree foi removido enquanto o subagente estava em execução, falha com um erro. Antes da v2.1.203, tal comando poderia ser executado no checkout principal.

Esta verificação de diretório de trabalho cobre todo o repositório contendo o diretório a partir do qual você iniciou Claude Code. Quando sua sessão é executada em um [worktree](/docs/pt/worktrees) vinculado de sua própria, a verificação também cobre o checkout principal do qual esse worktree está vinculado. Antes da v2.1.210, a verificação cobria apenas o diretório de inicialização em si. Um comando cujo diretório de trabalho se resolveu em outro lugar no mesmo repositório, como a raiz do repositório quando você iniciou Claude Code a partir de um subdiretório de monorepo, era executado lá em vez de falhar.

Para comandos Bash, Claude Code também verifica o comando em si de duas maneiras:

* Ele bloqueia um comando que redireciona git para o checkout principal.
* Ele recusa um comando quando não consegue verificar a partir do texto do comando que qualquer git que o comando executa permanece dentro do worktree, por exemplo quando o nome do comando é calculado em tempo de execução.

Os vetores de redirecionamento e as regras de forma estão listados em [Como Claude Code impõe isolamento](/docs/pt/worktrees#how-claude-code-enforces-isolation). Comandos PowerShell recebem apenas a verificação de diretório de trabalho.

Comandos [Monitor](/docs/pt/tools-reference#monitor-tool) passam pelas mesmas verificações de diretório de trabalho e conteúdo de comando que comandos Bash.

Quando a conversa principal em si é executada isolada em um worktree, Claude Code aplica as mesmas verificações à sessão e a cada subagente que ela gera, incluindo subagentes sem `isolation: worktree`; veja [Como Claude Code impõe isolamento](/docs/pt/worktrees#how-claude-code-enforces-isolation).

<h3 id="supported-frontmatter-fields">
  Referência de frontmatter
</h3>

Configure um subagente com [frontmatter](/docs/pt/glossary#frontmatter) YAML entre marcadores `---` no topo de seu arquivo, e escreva seu prompt de sistema como Markdown após o `---` de fechamento. Apenas `name` e `description` são obrigatórios.

Nomes de campo com múltiplas palavras usam camelCase, como `maxTurns` e `disallowedTools`, e devem corresponder à tabela exatamente: Claude Code ignora um campo que não reconhece sem relatar um erro. Para descobrir por que um arquivo de subagente não carregou, veja [Arquivos de subagente que Claude Code pula](#subagent-files-claude-code-skips).

| Field             | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :---------------- | :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Yes      | Identificador único, como `code-reviewer` ou `reviewer-v2`. [Hooks](/docs/pt/hooks#subagentstart) recebem este valor como `agent_type`. O nome do arquivo não precisa corresponder. Nomes não podem conter `:`, que é reservado para [identificadores com escopo de plugin](/docs/pt/plugins/overview) como `my-plugin:reviewer`. Claude Code não carrega um arquivo cujo nome contém um e registra um erro no log de debug. Antes da v2.1.218, tais nomes eram aceitos                                                               |
| `description`     | Yes      | Quando Claude deve delegar para este subagente                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `tools`           | No       | [Ferramentas](#available-tools) que o subagente pode usar, como uma string separada por vírgulas como `Read, Grep, Bash` ou uma lista YAML. Herda todas as ferramentas disponíveis para subagentes se omitido. Se nenhuma entrada na lista se resolver para uma ferramenta, o subagente geralmente [falha ao iniciar](/docs/pt/errors#agent-would-be-spawned-with-zero-tools) com um erro nomeando as entradas. Para pré-carregar Skills no contexto, use o campo `skills` em vez de listar `Skill` aqui                         |
| `disallowedTools` | No       | Ferramentas a negar, removidas da lista herdada ou especificada. Mesmo formato que `tools`. Uma entrada com um especificador, como `Bash(git push *)`, ainda [remove a ferramenta inteira](#available-tools)                                                                                                                                                                                                                                                                                                                |
| `model`           | No       | [Modelo](#choose-a-model) a usar: `sonnet`, `opus`, `haiku`, `fable`, um ID de modelo completo como `claude-opus-5-5`, ou `inherit`. Quando você omite, Claude Code escolhe o modelo na [ordem de modelo de subagente](#choose-a-model)                                                                                                                                                                                                                                                                                     |
| `permissionMode`  | No       | [Modo de permissão](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, ou `manual` como um alias para `default`. O alias `manual` requer Claude Code v2.1.200 ou posterior. Ignorado para [subagentes de plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                                     |
| `maxTurns`        | No       | Número máximo de turnos de agente antes do subagente parar. Quando o subagente atinge o limite, Claude Code retorna sua saída marcada como parcial, e Claude pode [retomá-lo](#resume-subagents) para continuar. A marcação parcial requer Claude Code v2.1.246 ou posterior                                                                                                                                                                                                                                                |
| `skills`          | No       | [Skills](/docs/pt/skills) a pré-carregar no contexto do subagente na inicialização. O conteúdo completo da skill é injetado, não apenas a descrição. Subagentes ainda podem invocar skills de projeto, usuário e plugin não listadas através da ferramenta Skill                                                                                                                                                                                                                                                                 |
| `mcpServers`      | No       | [MCP servers](/docs/pt/mcp) disponíveis para este subagente. Cada entrada é um nome de servidor referenciando um servidor já configurado (por exemplo, `"slack"`) ou uma definição inline com o nome do servidor como chave e uma [configuração completa de MCP server](/docs/pt/mcp#installing-mcp-servers) como valor. Ignorado para [subagentes de plugin](#choose-the-subagent-scope)                                                                                                                                             |
| `hooks`           | No       | [Lifecycle hooks](#define-hooks-for-subagents) com escopo para este subagente. Ignorado para [subagentes de plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                                                                                                                                                                             |
| `memory`          | No       | [Escopo de memória persistente](#enable-persistent-memory): `user`, `project`, ou `local`. Habilita aprendizado entre sessões                                                                                                                                                                                                                                                                                                                                                                                               |
| `background`      | No       | Defina como `true` para manter este subagente em background mesmo quando Claude pede para executá-lo em foreground. Onde [fork mode](#turn-fork-mode-on-or-off) está ativado, Claude Code já executa os subagentes que Claude gera [em background](#run-subagents-in-foreground-or-background)                                                                                                                                                                                                                              |
| `omitClaudeMd`    | No       | Defina como `true` para iniciar este subagente sem os arquivos CLAUDE.md de usuário, projeto e local; [arquivos de política gerenciada](/docs/pt/memory#how-claude-md-files-load) ainda carregam, exceto para [subagentes gerenciados](#choose-the-subagent-scope). Use-o para subagentes que pegam tudo que precisam do [prompt de delegação](#what-loads-at-startup). Ignorado quando o agente é executado como o agente da sessão principal via `--agent` ou a configuração `agent`. Requer Claude Code v2.1.271 ou posterior |
| `effort`          | No       | Nível de esforço quando este subagente está ativo. Sobrescreve o nível de esforço da sessão. Padrão: herda da sessão. Opções: `low`, `medium`, `high`, `xhigh`, `max`; os níveis disponíveis dependem do modelo                                                                                                                                                                                                                                                                                                             |
| `isolation`       | No       | Defina como `worktree` para executar o subagente em um [git worktree](/docs/pt/worktrees) temporário, dando-lhe uma cópia isolada do repositório ramificada por padrão a partir de sua [branch padrão](/docs/pt/worktrees#choose-the-base-branch) em vez do `HEAD` da sessão pai. O worktree é automaticamente limpo se o subagente não fizer alterações                                                                                                                                                                              |
| `color`           | No       | Cor de exibição para o subagente na lista de tarefas e transcrição. Aceita `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, ou `cyan`                                                                                                                                                                                                                                                                                                                                                                          |
| `initialPrompt`   | No       | Auto-enviado como o primeiro turno do usuário quando este agente é executado como o agente da sessão principal (via `--agent` ou a configuração `agent`). [Comandos](/docs/pt/commands) e [skills](/docs/pt/skills) são processados. Preposto a qualquer prompt fornecido pelo usuário. Ignorado para [subagentes de plugin](#choose-the-subagent-scope)                                                                                                                                                                              |
| `experimental`    | No       | Mapa de opções experimentais. Defina sua chave `cacheTtl` como `5m` ou `1h` para escolher o [tempo de vida do cache de prompt](/docs/pt/prompt-caching#choose-the-ttl-yourself) para as solicitações deste subagente, no lugar da [precedência de tempo de vida do cache](/docs/pt/prompt-caching#choose-the-ttl-yourself). Claude Code ignora qualquer outro valor, ignora `1h` enquanto sua assinatura Claude está usando créditos de uso, e lê o campo apenas de arquivos de subagente. Requer Claude Code v2.1.248 ou posterior   |

Escreva `cacheTtl` dentro do mapa `experimental`, não no nível superior do frontmatter.

```yaml theme={null}
---
name: repo-auditor
description: Audits a large repository and reports what it finds
experimental:
  cacheTtl: 1h
---
```

<h4 id="subagent-files-claude-code-skips">
  Arquivos de subagente que Claude Code pula
</h4>

Claude Code pula um arquivo em um diretório `agents` de projeto, usuário ou gerenciado, ou em um sob um diretório que você adiciona com `--add-dir`, sem relatá-lo na sessão, quando o frontmatter tem qualquer um desses problemas:

* **Sem `name`**: Claude Code trata o arquivo como documentação mantida ao lado de seus agentes.
* **Um `---` de abertura que não é a primeira linha do arquivo**: Claude Code lê o arquivo como não tendo frontmatter e o trata como documentação.
* **Um `name` que começa com `-` ou contém `:`**: Claude Code pula o arquivo e escreve um erro no log de debug. Veja a linha `name` na tabela acima.
* **Um `name` mas sem `description`**: Claude Code pula o arquivo e escreve o motivo no log de debug.
* **YAML que não analisa**: Claude Code não lê campos do arquivo, o pula e escreve o erro de análise no log de debug.

Para ver o log de debug, execute Claude Code com `--debug`.

Um [subagente de plugin](/docs/pt/plugins/components#agents) cujo frontmatter não tem `name` ou não analisa ainda carrega, sob seu nome de arquivo.

<h5 id="check-an-agents-directory-before-a-session">
  Verificar um diretório `agents` antes de uma sessão
</h5>

Para encontrar arquivos em um diretório `agents` cujo frontmatter não analisa, execute `claude plugin validate` contra o diretório, por exemplo `.claude/agents` ou `~/.claude/agents`. Claude Code verifica apenas [o diretório que você nomeia](/docs/pt/plugins/cli-reference#validate-a-directory), e não sinaliza um arquivo cujo frontmatter analisa mas não tem `name`. Requer Claude Code v2.1.233 ou posterior.

<h3 id="choose-a-model">
  Escolher um modelo
</h3>

O campo `model` controla qual modelo o subagente usa:

* **Alias de modelo**: use um dos aliases disponíveis: `sonnet`, `opus`, `haiku`, ou `fable`
* **ID de modelo completo**: use um ID de modelo completo como `claude-opus-5-5` ou `claude-sonnet-5`. Aceita os mesmos valores que o flag `--model`
* **inherit**: use o mesmo modelo que a conversa principal

Quando Claude invoca um subagente, ele também pode passar um parâmetro `model` para essa invocação específica. Claude Code resolve o modelo do subagente nesta ordem:

1. O parâmetro `model` por invocação
2. O frontmatter `model` da definição do subagente, onde `inherit` seleciona o modelo da conversa principal
3. A variável de ambiente [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/pt/model-config#environment-variables), quando você a define para um alias de modelo ou ID de modelo
4. O modelo da conversa principal

Em dois casos, um alias de família como `opus` no parâmetro por invocação ou no frontmatter se resolve para o modelo da conversa principal em vez da [versão para a qual o alias aponta](/docs/pt/model-config#model-aliases):

* **O modelo da conversa principal pertence a essa família**: o subagente é executado no modelo exato da conversa principal, incluindo qualquer sufixo `[1m]`, portanto obtém a mesma janela de [contexto estendido](/docs/pt/model-config#extended-context) que a conversa principal.
* **Claude Code não consegue dizer a família do modelo da conversa principal, em [um provedor diferente da API Anthropic](/docs/pt/third-party-integrations)**: isso pode acontecer com um [ARN de perfil de inferência de aplicação](/docs/pt/amazon-bedrock#iam-configuration) no Amazon Bedrock que Claude Code não resolveu para um modelo de suporte. Este caso cobre apenas o alias `opus`, e não se aplica quando você define [`ANTHROPIC_DEFAULT_OPUS_MODEL`](/docs/pt/model-config#environment-variables), já que `opus` então se resolve para o modelo que você definiu.

Um alias em `CLAUDE_CODE_SUBAGENT_MODEL` sempre se resolve para a versão para a qual o alias aponta, mesmo quando nomeia a família da conversa principal.

Definir `CLAUDE_CODE_SUBAGENT_MODEL` por si só não muda o modelo em que os subagentes Explore e Plan integrados são executados. Para mudá-lo, veja [Executar cada subagente em um modelo](#run-every-subagent-on-one-model).

Antes da v2.1.251, `CLAUDE_CODE_SUBAGENT_MODEL` vinha primeiro nesta ordem e sobrescrevia tanto o parâmetro por invocação quanto o frontmatter, incluindo `model: inherit`.

Definir a variável para `inherit` é o mesmo que deixá-la indefinida. Antes da v2.1.196, esse valor forçava subagentes para o modelo da conversa principal e ignorava as outras fontes.

Claude Code verifica o parâmetro por invocação, frontmatter e valores de variável de ambiente contra a lista de permissões [`availableModels`](/docs/pt/model-config#restrict-model-selection) da sua organização. Para um valor bloqueado, ele substitui outro modelo:

* Quando o valor bloqueado é um alias de família como `opus`, Claude Code executa o subagente na versão mais recente dessa família que a lista de permissões permite, seguindo as mesmas [regras de substituição e escopo de provedor](/docs/pt/model-config#restrict-model-selection) que `/model`. Antes da v2.1.222, Claude Code executava o subagente no modelo herdado para um alias de família bloqueado também.
* Para qualquer outro valor bloqueado, em provedores onde essa substituição não opera, ou quando a lista de permissões não permite nenhuma versão da família, Claude Code executa o subagente no modelo herdado em vez disso. Se você definir `CLAUDE_CODE_SUBAGENT_MODEL`, Claude Code tenta esse modelo primeiro, sob essas mesmas regras.

Em sessões interativas, Claude Code mostra um aviso nomeando o modelo solicitado e o modelo em que o subagente é executado, para qualquer substituição.

Para verificar qual modelo um subagente está executando, execute [`/tasks`](/docs/pt/commands). Claude Code nomeia o modelo na linha do subagente, e adiciona o [nível de esforço](/docs/pt/model-config#adjust-effort-level) quando a definição do subagente, ou a skill da qual ele se bifurcou, define [`effort`](#supported-frontmatter-fields). Requer Claude Code v2.1.242 ou posterior.

Um parâmetro `model` por invocação também se aplica quando o subagente é [retomado ou enviado uma mensagem de acompanhamento](#resume-subagents), portanto o subagente permanece nesse modelo. Antes da v2.1.211, retomar descartava o valor por invocação e o subagente revertia para o campo `model` de sua definição ou, sem um, o modelo da conversa principal.

A partir da v2.1.198, subagentes também herdam a configuração de [pensamento estendido](/docs/pt/model-config#extended-thinking) da conversa principal: se o pensamento está ativado em sua sessão, está ativado para o subagente, e se está desativado, permanece desativado. Não há configuração de pensamento por subagente. Antes da v2.1.198, subagentes eram executados com pensamento estendido desabilitado independentemente da configuração da conversa principal.

<h4 id="run-every-subagent-on-one-model">
  Executar cada subagente em um modelo
</h4>

`CLAUDE_CODE_SUBAGENT_MODEL` é um padrão, portanto a definição de um subagente ou um modelo que Claude passa ainda tem precedência sobre ele. Para aplicar um modelo a cada subagente, [colega de trabalho](/docs/pt/agent-teams#specify-teammates-and-models) e [agente de workflow](/docs/pt/workflows), também defina `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` para `1`. Requer Claude Code v2.1.257 ou posterior.

* Se você definir ambas as variáveis, subagentes são executados no modelo em `CLAUDE_CODE_SUBAGENT_MODEL`.
* Se você definir apenas `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, subagentes são executados no modelo da conversa principal.

Por exemplo, para executar cada subagente em Haiku, defina ambas as variáveis no bloco `env` de um [arquivo de configurações](/docs/pt/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_SUBAGENT_MODEL": "haiku",
    "CLAUDE_CODE_SUBAGENT_MODEL_FORCE": "1"
  }
}
```

Para verificar que a configuração entrou em vigor, execute [`/tasks`](/docs/pt/commands) enquanto um subagente está em execução. A linha do subagente mostra o modelo em que ele é executado.

Enquanto `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` está [ativado](/docs/pt/env-vars), Claude Code ignora o campo `model` de cada definição de subagente, incluindo os subagentes Explore e Plan integrados, e Claude não pode passar um modelo quando inicia um subagente. Dois tipos de subagente ainda são executados no modelo da conversa principal:

* Uma [bifurcação](#fork-the-current-conversation)
* Uma [skill que é executada em um subagente](/docs/pt/skills#run-skills-in-a-subagent) com `model: inherit`

Quando você define apenas `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, o subagente Explore integrado mantém seu [limite de modelo](#built-in-subagents).

<h3 id="control-subagent-capabilities">
  Controlar capacidades do subagente
</h3>

Você pode controlar o que subagentes podem fazer através de acesso a ferramentas, modos de permissão e regras condicionais.

<h4 id="available-tools">
  Ferramentas disponíveis
</h4>

Subagentes herdam as [ferramentas integradas](/docs/pt/tools-reference) e ferramentas MCP disponíveis na conversa principal, reduzidas por dois filtros: o primeiro remove uma lista curta de ferramentas de cada subagente, e o segundo reduz o conjunto de ferramentas integradas para subagentes que são executados em [background](#run-subagents-in-foreground-or-background), que é o padrão. Em macOS, Linux e WSL, um subagente também pode receber as ferramentas Glob e Grep quando a conversa principal não as tem, conforme descrito em [Comportamento da ferramenta Glob](/docs/pt/tools-reference#glob-tool-behavior). [Bifurcações](#fork-the-current-conversation) pulam ambos os filtros e recebem o pool de ferramentas exato da conversa principal. O primeiro filtro remove essas ferramentas, mesmo quando listadas no campo `tools`:

* `Agent`, quando o subagente está no [limite de profundidade](#let-subagents-spawn-their-own-subagents); em uma [bifurcação](#fork-the-current-conversation) a ferramenta permanece listada mas retorna um erro em vez de gerar
* `AskUserQuestion`
* `EndConversation`, que pode encerrar apenas a conversa principal; veja [comportamento da ferramenta EndConversation](/docs/pt/tools-reference#endconversation-tool-behavior)
* `EnterPlanMode`
* `ExitPlanMode`, a menos que o [`permissionMode`](#permission-modes) do subagente seja `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`
* `Workflow`

O segundo filtro se aplica a subagentes em execução em background. Além de `Agent` e `ExitPlanMode`, que seguem as condições do primeiro filtro onde quer que o subagente seja executado, um subagente em background mantém cada ferramenta MCP mas apenas essas ferramentas integradas: `Read`, `Grep`, `Glob`, `LSP`, `Bash`, `PowerShell`, `Edit`, `Write`, `NotebookEdit`, `WebFetch`, `WebSearch`, `TodoWrite`, `Skill`, `ToolSearch`, `EnterWorktree`, `ExitWorktree`, `Monitor`, `TaskStop`, `SendMessage` e `Artifact`, além de [`SubagentHandback`](/docs/pt/tools-reference) para um subagente que relata através dele. Claude Code remove todas as outras ferramentas integradas de um subagente em background, seja herdadas ou listadas no campo `tools`, portanto a mesma definição pode se resolver para ferramentas diferentes em foreground e background. A remoção não relata erro a menos que deixe a lista `tools` [se resolvendo para nada](/docs/pt/errors#agent-would-be-spawned-with-zero-tools).

Antes da v2.1.280, subagentes em background não podiam usar `LSP`.

[`ListAgents`](/docs/pt/cross-session-messaging) segue esses filtros como qualquer ferramenta integrada: um subagente em foreground a herda em sessões onde mensagens entre sessões estão habilitadas, e um subagente em background não a mantém.

Colegas de trabalho em [equipes de agentes](/docs/pt/agent-teams) adicionalmente mantêm as ferramentas de tarefa e ferramentas cron: `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, `CronCreate`, `CronDelete` e `CronList`.

Em uma [sessão sem as ferramentas Task](/docs/pt/tools-reference#task-tool-availability), Claude Code não fornece as ferramentas de tarefa a subagentes também, mesmo quando o subagente executa um modelo diferente. Um colega de trabalho em processo segue sua sessão da mesma forma, enquanto um colega de trabalho em seu próprio [painel dividido](/docs/pt/agent-teams#choose-a-display-mode) é executado como um processo Claude Code separado, portanto seu próprio modelo decide.

Para restringir ferramentas, use o campo `tools` como uma lista de permissões ou o campo `disallowedTools` como uma lista de negação. Este exemplo usa `tools` para permitir apenas Read, Grep, Glob e Bash. O subagente não pode editar arquivos, escrever arquivos ou usar qualquer ferramenta MCP:

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Este exemplo usa `disallowedTools` para herdar o pool de ferramentas do subagente exceto Write e Edit. O subagente mantém Bash, ferramentas MCP e o resto de seu pool:

```yaml theme={null}
---
name: no-writes
description: Inherits the available tools except file writes
disallowedTools: Write, Edit
---
```

Se ambos forem definidos, `disallowedTools` é aplicado primeiro, depois `tools` é resolvido contra o pool restante. Uma ferramenta listada em ambos é removida.

Quando nada na lista `tools` se resolve para uma ferramenta, por exemplo porque cada entrada está com erro de digitação ou nomeia uma ferramenta que não está disponível para subagentes, Claude Code geralmente recusa iniciar o subagente e a ferramenta Agent retorna um erro nomeando as entradas não resolvidas; veja [Agent would be spawned with zero tools](/docs/pt/errors#agent-would-be-spawned-with-zero-tools) para a mensagem e como corrigir cada entrada. Antes da v2.1.208, esse subagente era iniciado sem ferramentas e poderia retornar um resultado vazio ou confuso.

Ambos os campos aceitam padrões de nível de servidor MCP além de nomes de ferramentas exatos: `mcp__<server>` ou `mcp__<server>__*` concede ou remove todas as ferramentas do servidor nomeado. Em `disallowedTools`, `mcp__*` também remove todas as ferramentas MCP de qualquer servidor. Este exemplo remove todas as ferramentas do servidor MCP `github` enquanto mantém ferramentas de outros servidores e as ferramentas integradas em seu pool:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

Uma entrada `disallowedTools` com um especificador, como `Bash(git push *)`, ainda remove a ferramenta inteira do subagente, não apenas os comandos correspondentes. Para manter Bash e bloquear comandos específicos, adicione uma [regra de negação Bash](/docs/pt/permissions#bash) como `Bash(git push *)` a `permissions.deny` em suas configurações. A regra se aplica à conversa principal e aos subagentes.

<h4 id="restrict-which-subagents-can-be-spawned">
  Restringir quais subagentes podem ser gerados
</h4>

Quando um agente é executado como thread principal com `claude --agent`, ele pode gerar subagentes usando a ferramenta Agent. Para restringir quais tipos de subagente ele pode gerar, use a sintaxe `Agent(agent_type)` no campo `tools`.

<Note>Na versão 2.1.63, a ferramenta Task foi renomeada para Agent. Referências existentes de `Task(...)` em configurações e definições de agente ainda funcionam como aliases.</Note>

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

Esta é uma lista de permissões: apenas os subagentes `worker` e `researcher` podem ser gerados. Se o agente tentar gerar qualquer outro tipo, a solicitação falha e o agente vê apenas os tipos permitidos em seu prompt. Para bloquear agentes específicos enquanto permite todos os outros, use [`permissions.deny`](#disable-specific-subagents) em vez disso.

Para permitir gerar qualquer subagente sem restrições, use `Agent` sem parênteses:

```yaml theme={null}
tools: Agent, Read, Bash
```

Se `Agent` for omitido da lista `tools` inteiramente, o agente não pode gerar nenhum subagente com a ferramenta Agent.

A sintaxe de lista de permissões `Agent(agent_type)` se aplica apenas a um agente executado como thread principal com `claude --agent`. Em uma definição de subagente, listar `Agent` em `tools` permite que esse subagente gere subagentes de sua própria conta enquanto o [limite de profundidade](#let-subagents-spawn-their-own-subagents) permite, mas qualquer lista de tipo dentro dos parênteses é ignorada.

<h4 id="scope-mcp-servers-to-a-subagent">
  Escopo de MCP servers para um subagente
</h4>

Use o campo `mcpServers` para dar a um subagente acesso a [MCP](/docs/pt/mcp) servers que não estão disponíveis na conversa principal. Servidores inline definidos aqui são conectados quando o subagente inicia, sujeitos à [regra de confiança para a pasta do arquivo do agente](#inline-server-trust), e desconectados quando termina. Referências de string compartilham a conexão da sessão pai.

<Note>
  O campo `mcpServers` se aplica em ambos os contextos onde um arquivo de agente pode ser executado:

  * Como um subagente, gerado através da ferramenta Agent ou uma @-menção
  * Como a sessão principal, iniciada com [`--agent`](#invoke-subagents-explicitly) ou a configuração `agent`

  Quando o agente é a sessão principal, definições de servidor inline se conectam na inicialização junto com servidores de [`.mcp.json`](/docs/pt/mcp) e arquivos de configurações, sob a mesma [regra de confiança para a pasta do arquivo do agente](#inline-server-trust). Em `/mcp`, um servidor remoto (HTTP ou SSE) que você usou antes pode mostrar o status [`cached`](/docs/pt/mcp#managing-your-servers) em vez disso; Claude Code o conecta quando Claude primeiro chama uma de suas ferramentas.
</Note>

Cada entrada na lista é uma definição de servidor inline ou uma string referenciando um MCP server já configurado em sua sessão:

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

Definições inline usam o mesmo schema que entradas de servidor `.mcp.json`, com chave pelo nome do servidor, e suportam os tipos `stdio`, `http`, `sse` e `ws`.

Para manter um MCP server fora da conversa principal inteiramente e evitar que suas descrições de ferramentas consumam contexto lá, defina-o inline aqui em vez de em `.mcp.json`. O subagente obtém as ferramentas; a conversa pai não.

<span id="inline-server-trust" />Claude Code carrega um servidor inline de um arquivo de agente em seu diretório `.claude/agents/` do projeto, ou em um diretório `.claude/agents/` de um diretório adicionado com `--add-dir`, apenas depois que você [confia na pasta de onde o arquivo do agente veio](/docs/pt/permissions#what-runs-before-you-trust-a-folder). Antes da v2.1.238, Claude Code carregava esses servidores sem verificar confiança.

* **Confiança que não conta**: confiança de uma pasta pai, e a confiança automática que uma sessão `-p` ou SDK obtém para [hooks em arquivos de configurações](/docs/pt/permissions#what-runs-before-you-trust-a-folder)
* **Até então**: Claude Code pula cada servidor inline naquele arquivo de agente e escreve a chave exata `projects["<path>"].hasTrustDialogAccepted` para `~/.claude.json` no log de debug
* **Diretórios `--add-dir`**: um diretório fora do repositório do espaço de trabalho confiável de sua organização precisa de sua própria entrada de confiança, já que seus arquivos `.claude/agents/` não herdam a confiança do seu espaço de trabalho

Claude Code carrega dois tipos de servidor sem verificar confiança para a pasta de onde o arquivo do agente veio:

* Um nome que referencia um servidor que você já configurou
* Um servidor inline em um arquivo de agente de `~/.claude/agents/`, em um que você passa com `--agents` ou a opção `agents` do SDK, ou em um que as configurações gerenciadas fornecem

As restrições de MCP que se aplicam à sessão principal também cobrem servidores declarados no frontmatter do subagente:

* [`--strict-mcp-config`](/docs/pt/cli-reference) e [`--bare`](/docs/pt/cli-reference)
* [Configuração de MCP gerenciada pela empresa](/docs/pt/managed-mcp)
* [Políticas `allowedMcpServers` e `deniedMcpServers`](/docs/pt/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Quando um destes bloqueia um servidor, Claude Code o ignora e mostra um aviso nomeando os servidores bloqueados.

Restrições de configurações gerenciadas se aplicam a cada subagente independentemente de como é definido. `--strict-mcp-config` não filtra servidores que você passa inline via `--agents` ou a opção `agents` do SDK, já que esses são entrada explícita do chamador.

<h4 id="permission-modes">
  Modos de permissão
</h4>

Defina `permissionMode` para escolher o modo de permissão em que um subagente é executado. Use os valores de configuração dos modos, portanto o modo Manual é `default`. Se você deixar indefinido, o subagente herda o modo da conversa principal, que começa como [modo auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) em planos Pro, Max e Team a menos que suas configurações ou sua organização o alterem.

O modo de permissão da conversa principal decide se Claude Code usa o valor que você definiu:

* Quando a conversa principal está em `bypassPermissions`, `acceptEdits`, ou [modo auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode), o subagente é executado nesse mesmo modo e Claude Code ignora o `permissionMode` que você definiu. Sob modo auto, o classificador avalia as chamadas de ferramentas do subagente com as regras de bloqueio e permissão da conversa principal. Quando o subagente termina, o classificador também revisa seu trabalho e seu relatório final antes do relatório ser entregue, conforme [Como o modo auto lida com subagentes](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) descreve.
* Quando a conversa principal está em `default`, `dontAsk`, ou modo `plan`, o subagente é executado no modo de permissão que você definiu, exceto `bypassPermissions`. Um subagente que declara `bypassPermissions` mantém o modo da conversa principal em vez disso. A exceção `bypassPermissions` requer Claude Code v2.1.267 ou posterior.

`permissionMode` aceita estes valores, e `manual` como um alias para `default`:

| Mode                | Behavior                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Modo Manual: solicita permissão                                                                                                                                                                                                                                                                                                                                                                                             |
| `acceptEdits`       | Auto-aceitar edições de arquivo e comandos comuns do sistema de arquivos para caminhos no diretório de trabalho ou `additionalDirectories`                                                                                                                                                                                                                                                                                  |
| `auto`              | [Modo auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode): um classificador de IA revisa comandos e escritas em diretório protegido                                                                                                                                                                                                                                                                                |
| `dontAsk`           | Auto-negar prompts de permissão. Ferramentas explicitamente permitidas ainda funcionam; `AskUserQuestion`, ferramentas MCP marcadas [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool), e ferramentas de conector [sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) em sessões onde essa configuração chega a Claude Code são negadas mesmo se você as permitiu |
| `bypassPermissions` | [Pular prompts de permissão](/docs/pt/permission-modes#skip-all-checks-with-bypasspermissions-mode). Um subagente é executado neste modo apenas quando a conversa principal o faz                                                                                                                                                                                                                                                |
| `plan`              | Plan mode (exploração somente leitura)                                                                                                                                                                                                                                                                                                                                                                                      |

<h4 id="preload-skills-into-subagents">
  Pré-carregar skills em subagentes
</h4>

Use o campo `skills` para injetar conteúdo de skill no contexto de um subagente na inicialização. Isso dá ao subagente conhecimento de domínio sem exigir que ele descubra e carregue skills durante a execução.

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

O conteúdo completo de cada skill listada é injetado no contexto do subagente na inicialização. Este campo controla quais skills são pré-carregadas, não quais skills o subagente pode acessar: sem ele, o subagente ainda pode descobrir e invocar skills de projeto, usuário e plugin através da ferramenta Skill durante a execução. Para impedir que um subagente invoque skills inteiramente, omita `Skill` da lista [`tools`](#available-tools) ou adicione-o a `disallowedTools`.

Você não pode pré-carregar skills que definem [`disable-model-invocation: true`](/docs/pt/skills#control-who-invokes-a-skill), já que pré-carregar extrai do mesmo conjunto de skills que Claude pode invocar. Isso inclui a skill `/verify` integrada: apenas você pode executá-la, portanto ela não pode ser pré-carregada também.

Se uma skill listada estiver faltando ou desabilitada, por exemplo pela política de sua organização, Claude Code a ignora e registra um aviso no log de debug.

<Note>
  Isto é o inverso de [executar uma skill em um subagente](/docs/pt/skills#run-skills-in-a-subagent). Com `skills` em um subagente, o subagente controla o prompt de sistema e carrega conteúdo de skill. Com `context: fork` em uma skill, o conteúdo de skill é injetado no agente que você especificar. Em ambos os casos o subagente começa sem seu histórico de conversa.
</Note>

<h4 id="enable-persistent-memory">
  Habilitar memória persistente
</h4>

O campo `memory` dá ao subagente um diretório persistente que sobrevive entre conversas. O subagente usa este diretório para construir conhecimento ao longo do tempo, como padrões de base de código, insights de debugging e decisões arquiteturais.

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

Escolha um escopo baseado em quão amplamente a memória deve se aplicar:

| Scope     | Location                                      | Use when                                                                                              |
| :-------- | :-------------------------------------------- | :---------------------------------------------------------------------------------------------------- |
| `user`    | `~/.claude/agent-memory/<name-of-agent>/`     | o subagente deve lembrar aprendizados entre todos os projetos                                         |
| `project` | `.claude/agent-memory/<name-of-agent>/`       | o conhecimento do subagente é específico do projeto e compartilhável via controle de versão           |
| `local`   | `.claude/agent-memory-local/<name-of-agent>/` | o conhecimento do subagente é específico do projeto mas não deve ser verificado no controle de versão |

A memória do subagente faz parte da [memória automática](/docs/pt/memory#auto-memory): se você desativar a memória automática, com a configuração `autoMemoryEnabled` ou `CLAUDE_CODE_DISABLE_AUTO_MEMORY`, o campo `memory` não tem efeito e o subagente é iniciado sem as instruções de memória ou o acesso à ferramenta de memória descrito abaixo.

Quando a memória está habilitada:

* O prompt de sistema do subagente inclui instruções para ler e escrever no diretório de memória.
* O prompt de sistema do subagente também inclui as primeiras 200 linhas ou 25KB de `MEMORY.md` no diretório de memória, o que for menor, com instruções para curar `MEMORY.md` se exceder esse limite.
* Ferramentas Read, Write e Edit são automaticamente habilitadas para que o subagente possa gerenciar seus arquivos de memória.

<h5 id="persistent-memory-tips">
  Dicas de memória persistente
</h5>

* `project` é o escopo padrão recomendado. Ele torna o conhecimento do subagente compartilhável via controle de versão.
* Peça ao subagente para consultar sua memória antes de começar o trabalho: "Review this PR, and check your memory for patterns you've seen before."
* Peça ao subagente para atualizar sua memória após completar uma tarefa: "Now that you're done, save what you learned to your memory." Ao longo do tempo, isso constrói uma base de conhecimento que torna o subagente mais eficaz.
* Inclua instruções de memória diretamente no arquivo markdown do subagente para que ele mantenha proativamente sua própria base de conhecimento:

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

<h4 id="conditional-rules-with-hooks">
  Regras condicionais com hooks
</h4>

Para controle mais dinâmico sobre uso de ferramentas, use hooks `PreToolUse` para validar operações antes de serem executadas. Isso é útil quando você precisa permitir algumas operações de uma ferramenta enquanto bloqueia outras.

Este exemplo cria um subagente que apenas permite consultas de banco de dados somente leitura. O hook `PreToolUse` executa o script especificado em `command` antes de cada comando Bash ser executado:

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code [passa entrada de hook como JSON](/docs/pt/hooks#pretooluse-input) via stdin para comandos de hook. O script de validação lê este JSON, extrai o comando Bash e [sai com código 2](/docs/pt/hooks#exit-code-2-behavior-per-event) para bloquear operações de escrita:

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

Em macOS e Linux, torne o script executável, ou o hook falha em vez de bloquear qualquer coisa:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Para testar a regra, peça ao subagente para executar uma instrução `UPDATE`: o script sai com código 2, Claude Code bloqueia o comando, e o subagente vê a mensagem `Blocked: Only SELECT queries are allowed`.

Veja [Hook input](/docs/pt/hooks#pretooluse-input) para o schema de entrada completo e [exit codes](/docs/pt/hooks#exit-code-output) para como códigos de saída afetam o comportamento. No Windows, escreva scripts de hook em PowerShell e adicione `shell: powershell` à entrada de hook conforme mostrado em [executando hooks em PowerShell](/docs/pt/hooks#windows-powershell-tool).

<h4 id="disable-specific-subagents">
  Desabilitar subagentes específicos
</h4>

Você pode impedir que Claude use subagentes específicos adicionando-os ao array `deny` em suas [configurações](/docs/pt/settings-reference#permission-settings). Use o formato `Agent(subagent-name)` onde `subagent-name` corresponde ao campo name do subagente.

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

Isso funciona para subagentes integrados e personalizados. Você também pode usar o flag CLI `--disallowedTools`:

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

Veja [documentação de Permissões](/docs/pt/permissions#tool-specific-permission-rules) para mais detalhes sobre regras de permissão.

<h3 id="define-hooks-for-subagents">
  Definir hooks para subagentes
</h3>

Subagentes podem definir [hooks](/docs/pt/hooks) que são executados durante o ciclo de vida do subagente. Existem duas formas de configurar hooks:

* **No frontmatter do subagente**: defina hooks que são executados apenas enquanto esse subagente específico está ativo
* **Em `settings.json`**: defina hooks em toda a sessão que também disparam dentro de subagentes. Eventos de ferramentas como `PreToolUse` e `PostToolUse` disparam para as chamadas de ferramentas do subagente da mesma forma que na conversa principal, e `SubagentStart` e `SubagentStop` disparam quando um subagente inicia ou termina

Hooks de [arquivos de configurações, configurações de política gerenciada e plugins](/docs/pt/hooks#hook-locations) todos se aplicam dentro de subagentes, portanto um hook `PreToolUse` em `settings.json` também é executado antes de cada ferramenta que um subagente usa.

<h4 id="hooks-in-subagent-frontmatter">
  Hooks no frontmatter do subagente
</h4>

Defina hooks diretamente no arquivo markdown do subagente. Estes hooks são executados apenas enquanto esse subagente específico está ativo e são limpos quando termina.

<Note>
  Hooks de frontmatter disparam quando o agente é gerado como um subagente através da ferramenta Agent ou uma @-menção, e quando o agente é executado como a sessão principal via [`--agent`](#invoke-subagents-explicitly) ou a configuração `agent`. No caso de sessão principal, eles são executados junto com qualquer hook definido em [`settings.json`](/docs/pt/hooks).
</Note>

Para permitir que os hooks de frontmatter de um subagente no nível do projeto sejam executados, aceite o [diálogo de confiança do espaço de trabalho](/docs/pt/permissions#project-allow-rules-and-workspace-trust) para a pasta que contém o arquivo do agente. Hooks de subagentes no nível do usuário em `~/.claude/agents/` e de definições que você passa com `--agents` são executados sem esta etapa. Se você adicionou uma pasta com `--add-dir` de fora do repositório do espaço de trabalho confiável de sua organização, confie nessa pasta separadamente: seus hooks `.claude/agents/` não herdam a confiança do espaço de trabalho.

Até que você confie na pasta, o subagente ainda é executado, mas Claude Code pula seus hooks de frontmatter e registra um erro no log de debug explicando como confiar na pasta. Esta é uma regra mais rigorosa do que a para hooks em arquivos de configurações: confiar em uma pasta pai não é suficiente, e uma sessão `-p` não conta como confiável. [O que é executado antes de você confiar em uma pasta](/docs/pt/permissions#what-runs-before-you-trust-a-folder) compara os dois. Antes da v2.1.218, hooks de frontmatter podiam ser executados de pastas que você não tinha confiado, incluindo em sessões não interativas.

Todos os [eventos de hook](/docs/pt/hooks#hook-events) são suportados. Os eventos mais comuns para subagentes são:

| Event         | Matcher input      | When it fires                                                                    |
| :------------ | :----------------- | :------------------------------------------------------------------------------- |
| `PreToolUse`  | Nome da ferramenta | Antes do subagente usar uma ferramenta                                           |
| `PostToolUse` | Nome da ferramenta | Depois do subagente usar uma ferramenta                                          |
| `Stop`        | (nenhum)           | Quando o subagente termina (convertido para `SubagentStop` em tempo de execução) |

Este exemplo valida comandos Bash com o hook `PreToolUse` e executa um linter após edições de arquivo com `PostToolUse`:

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh $TOOL_INPUT"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

Quando o agente é invocado como um subagente, hooks `Stop` no frontmatter são automaticamente convertidos para eventos `SubagentStop`.

<h4 id="project-level-hooks-for-subagent-events">
  Hooks no nível do projeto para eventos de subagente
</h4>

Configure hooks em `settings.json` que respondem a eventos de ciclo de vida de subagente na sessão principal.

| Event           | Matcher input          | When it fires                         |
| :-------------- | :--------------------- | :------------------------------------ |
| `SubagentStart` | Nome do tipo de agente | Quando um subagente começa a execução |
| `SubagentStop`  | Nome do tipo de agente | Quando um subagente completa          |

Ambos os eventos suportam matchers para direcionar tipos de agente específicos por nome. O valor do matcher é o `name` do frontmatter do agente para subagentes no nível de projeto e usuário, ou o identificador com escopo de plugin como `my-plugin:db-agent` para [subagentes de plugin](/docs/pt/plugins/components#agents). Um nome com escopo contém dois-pontos, portanto é avaliado como uma [expressão regular sem âncora](/docs/pt/hooks#matcher-patterns); ancorá-lo com `^` e `$`, como em `^my-plugin:db-agent$`, para corresponder apenas a esse agente.

Este exemplo executa um script de configuração apenas quando o subagente `db-agent` inicia, e um script de limpeza quando qualquer subagente para:

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

Um matcher com hífens como `db-agent` corresponde exatamente no Claude Code v2.1.195 ou posterior. Em versões anteriores, é avaliado como uma expressão regular sem âncora e também dispara para qualquer tipo de agente que o contenha, como `prod-db-agent`; ancorá-lo como `^db-agent$` nessas versões.

Veja [Hooks](/docs/pt/hooks) para o formato de configuração de hook completo.

<h2 id="work-with-subagents">
  Trabalhar com subagentes
</h2>

<h3 id="understand-automatic-delegation">
  Entender delegação automática
</h3>

Claude delega tarefas automaticamente com base na descrição da tarefa em sua solicitação, no campo `description` nas configurações de subagentes e no contexto atual. Para incentivar delegação proativa, inclua frases como "use proativamente" no campo de descrição do seu subagente.

Mantenha as descrições breves: Claude Code mostra um aviso de inicialização quando as descrições combinadas de seus subagentes ultrapassam o [limite de 15.000 tokens](/docs/pt/errors#agent-descriptions-are-over-the-15000-token-limit), e ainda carrega todos os subagentes.

Se o subagente é fornecido em um [plugin](/docs/pt/plugins/overview), você pode medir o quão confiável Claude delega a ele em prompts realistas em vez de verificar um de cada vez: [`claude plugin eval`](/docs/pt/plugin-evals) executa cada prompt com e sem o plugin e pontua os resultados.

<h3 id="invoke-subagents-explicitly">
  Invocar subagentes explicitamente
</h3>

Quando a delegação automática não é suficiente, você pode solicitar um subagente você mesmo. Três padrões escalam de uma sugestão única para um padrão em toda a sessão:

* **Linguagem natural**: nomeie o subagente em seu prompt; Claude decide se deve delegar
* **@-mention**: garante que o subagente seja executado para uma tarefa
* **Em toda a sessão**: toda a sessão usa o prompt do sistema, restrições de ferramentas e modelo desse subagente via sinalizador `--agent` ou configuração `agent`

Para linguagem natural, não há sintaxe especial. Nomeie o subagente e Claude normalmente delega:

```text wrap theme={null}
Use o subagente test-runner para corrigir testes com falha
Peça ao subagente code-reviewer para analisar minhas mudanças recentes
```

**@-mention o subagente.** Digite `@` e escolha o subagente na lista de sugestões, da mesma forma que você @-menciona arquivos. Isso garante que esse subagente específico seja executado em vez de deixar a escolha para Claude:

```text wrap theme={null}
@"code-reviewer (agent)" analise as mudanças de autenticação
```

Sua mensagem completa ainda vai para Claude, que escreve o prompt de tarefa do subagente com base no que você pediu. O @-mention controla qual subagente Claude invoca, não qual prompt ele recebe.

Subagentes fornecidos por um [plugin](/docs/pt/plugins/overview) habilitado aparecem na lista de sugestões sob seu nome com escopo, como `my-plugin:code-reviewer` ou `my-plugin:review:security` quando o plugin [organiza agentes em subpastas](#choose-the-subagent-scope). Subagentes de fundo nomeados atualmente em execução na sessão também aparecem na lista de sugestões, mostrando seu status ao lado do nome.

Você também pode digitar a menção manualmente sem usar o seletor: `@agent-<name>` para subagentes locais, ou `@agent-` seguido pelo nome com escopo para subagentes de plugin, por exemplo `@agent-my-plugin:code-reviewer`. Enquanto você digita este formulário, a lista de sugestões mostra correspondências de arquivo em vez de agentes. A menção do agente ainda é resolvida quando você envia.

**Execute toda a sessão como um subagente.** Passe [`--agent <name>`](/docs/pt/cli-reference) para iniciar uma sessão onde o thread principal em si assume o prompt do sistema, restrições de ferramentas e modelo desse subagente:

```bash theme={null}
claude --agent code-reviewer
```

O prompt do sistema do subagente substitui completamente o prompt do sistema padrão do Claude Code, da mesma forma que [`--system-prompt`](/docs/pt/cli-reference) faz. Os arquivos `CLAUDE.md` e a memória do projeto ainda são carregados através do fluxo de mensagens normal, mesmo quando a definição do agente define [`omitClaudeMd`](#supported-frontmatter-fields).

O nome do agente aparece como `@<name>` no cabeçalho de inicialização para que você possa confirmar que está ativo.

Isso funciona com subagentes integrados e personalizados, e a escolha persiste quando você retoma a sessão: Claude Code restaura as restrições de ferramentas e o modelo do agente junto com a conversa. Se o agente não existir mais quando você retomar, a sessão continua com as ferramentas padrão e mostra um [aviso nomeando o agente](/docs/pt/errors#session-agent-no-longer-available). Para o prompt do sistema em ambos os casos, consulte [Sinalizadores de prompt do sistema em conversas retomadas](/docs/pt/cli-reference#system-prompt-flags-in-resumed-conversations).

Para um subagente fornecido por plugin, você pode passar apenas o nome do agente e Claude Code o encontra:

```bash theme={null}
claude --agent security-reviewer
```

Se vários plugins fornecerem agentes com o mesmo nome, passe o nome com escopo para desambiguar:

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

Se o plugin colocar o agente em uma subpasta de seu diretório `agents/`, inclua a subpasta no nome com escopo, por exemplo `claude --agent my-plugin:review:security`.

Para torná-lo o padrão para cada sessão em um projeto, defina `agent` em `.claude/settings.json`:

```json theme={null}
{
  "agent": "code-reviewer"
}
```

O sinalizador CLI substitui a configuração se ambos estiverem presentes.

<h3 id="run-subagents-in-foreground-or-background">
  Executar subagentes em primeiro plano ou segundo plano
</h3>

Subagentes podem ser executados em primeiro plano ou segundo plano:

* **Subagentes em primeiro plano** bloqueiam a conversa principal até a conclusão. Prompts de permissão são passados para você conforme surgem.
* **Subagentes em segundo plano** são executados simultaneamente enquanto você continua trabalhando. Quando um subagente em segundo plano atinge uma chamada de ferramenta que precisa de permissão, Claude Code exibe o prompt em sua sessão principal e nomeia o subagente que está pedindo. Aprove para deixar o subagente continuar, ou pressione Esc para negar essa chamada de ferramenta sem parar o subagente.

Para cada subagente que Claude gera com a ferramenta Agent, Claude Code escolhe primeiro plano ou segundo plano do primeiro desses casos que se aplica:

* Se um colega de [equipe de agentes](/docs/pt/agent-teams#limitations) em processo gerou o subagente, Claude Code o executa em primeiro plano. Claude Code recusa com um erro para gerar um subagente de colega cuja definição define [`background: true`](#supported-frontmatter-fields). Onde o [modo fork](#turn-fork-mode-on-or-off) está desativado e você não [desativou tarefas em segundo plano](/docs/pt/env-vars), Claude Code também recusa com um erro quando um colega define `run_in_background: true`.
* Se você definir [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/pt/env-vars) como `1`, Claude Code executa o subagente em primeiro plano, em todo tipo de sessão e independentemente de o modo fork estar ativado.
* Onde o [modo fork](#turn-fork-mode-on-or-off) está ativado, como é por padrão em uma sessão interativa, Claude Code executa o subagente em segundo plano, subagentes fork e não-fork, e Claude não pode pedir o primeiro plano.
* Onde o modo fork está desativado, Claude executa o subagente em segundo plano por padrão e em primeiro plano quando precisa do resultado antes de continuar. O modo fork está desativado no [modo não interativo](/docs/pt/headless) com `-p` e no Agent SDK a menos que você o ative. Para manter um subagente particular em segundo plano mesmo quando Claude quer o resultado, defina seu campo frontmatter [`background`](#supported-frontmatter-fields) como `true`.

Para uma skill com `context: fork`, Claude Code segue as regras em [Executar skills em um subagente](/docs/pt/skills#run-skills-in-a-subagent), independentemente de o modo fork estar ativado.

Subagentes em segundo plano são executados com um [conjunto de ferramentas integradas menor](#available-tools) do que subagentes em primeiro plano, exceto para forks de conversa e subagentes em primeiro plano [retomados](#resume-subagents).

Subagentes em segundo plano exibem cada prompt de permissão em sua sessão principal. Quando você responde um desses prompts com uma escolha que dura além dessa chamada de ferramenta, como uma concessão que dura o resto da sessão, Claude Code aplica sua resposta a toda a sessão, incluindo sua conversa principal.

Um subagente em segundo plano pode deixar um comando [Bash ou PowerShell](/docs/pt/tools-reference#background-commands) em segundo plano [em execução após o final de seu turno](/docs/pt/interactive-mode#how-backgrounding-works). Quando esse comando termina, Claude Code envia ao subagente uma notificação.

Os resultados de um subagente em segundo plano chegam a Claude como uma notificação de conclusão em um turno posterior. Claude aguarda essa notificação antes de relatar os resultados do subagente, e se você perguntar sobre o progresso primeiro, ele relata que o subagente ainda está em execução. Antes da v2.1.211, Claude às vezes relatava resultados para um subagente em segundo plano que não havia terminado.

Você também pode orientar isso você mesmo:

* Onde o modo fork está desativado, peça a Claude para executar uma tarefa em segundo plano ou em primeiro plano
* Pressione **Ctrl+B** para colocar uma tarefa em execução em segundo plano

Claude Code limpa a linha de um subagente em segundo plano do painel de subagentes abaixo da entrada de prompt de uma de duas maneiras, dependendo de como o subagente terminou:

* Quando um subagente termina com sucesso, Claude Code remove sua linha imediatamente e, exceto no [modo leitor de tela](/docs/pt/accessibility), mostra `/tasks para ver subagentes` no rodapé por 30 segundos. Durante esses 30 segundos, execute [`/tasks`](/docs/pt/commands) e pressione `Enter` no subagente para abrir sua transcrição. Antes da v2.1.232, Claude Code mantinha a linha por 30 segundos após o subagente terminar, o mesmo que um com falha, e não mostrava dica de rodapé.
* Quando um subagente falha ou você o para, Claude Code mantém sua linha por 30 segundos. Para limpar a linha mais cedo, selecione-a e pressione `x`.

Um subagente em segundo plano que é concluído permanece listado em [`/tasks`](/docs/pt/commands), marcado como concluído e classificado abaixo do trabalho em execução, pelos mesmos 30 segundos que a dica de rodapé. Sua visualização de detalhes permanece aberta quando o subagente termina. Subagentes que falham ou que você para saem da lista. Antes da v2.1.208, um subagente concluído saía da lista no momento em que terminava e sua visualização de detalhes fechava.

<h3 id="subagent-names">
  Nomes de subagentes
</h3>

Claude pode dar um nome a um subagente passando um parâmetro `name` na chamada da ferramenta Agent, e pode fazer isso por conta própria, sem pedir a você primeiro. O nome torna o subagente endereçável: Claude pode [mensagear ou retomá-lo pelo nome](#resume-subagents) após terminar.

Em uma sessão interativa com [equipes de agentes](/docs/pt/agent-teams) habilitadas, um subagente que Claude gera da conversa principal com um `name` é lançado como um colega, a menos que a chamada seja um [fork](#fork-the-current-conversation) ou passe `isolation` na chamada em si. Um valor `isolation` no frontmatter do subagente não o impede, e o colega então é executado no diretório de trabalho da sessão principal. Consulte [Como Claude inicia equipes de agentes](/docs/pt/agent-teams#how-claude-starts-agent-teams).

<h3 id="api-errors-in-subagents">
  Erros de API em subagentes
</h3>

Quando algo [interrompe a resposta de um subagente no meio do fluxo](/docs/pt/errors#the-response-above-may-be-incomplete), e a resposta parcial contém texto mas nenhuma chamada de ferramenta, Claude Code solicita ao subagente que continue em vez de encerrar a execução. Isso também acontece em sessões interativas. A execução termina no erro apenas uma vez que essas continuações são usadas.

A partir da v2.1.199, um subagente cuja execução termina em um erro de API, como um limite de uso ou um erro de servidor repetido, relata essa falha de volta a Claude em vez de retornar o texto de erro como se fossem as descobertas do subagente. O que Claude recebe depende de onde o subagente foi executado:

* **Primeiro plano**: se um limite de taxa, sobrecarga ou erro de servidor interromper um subagente que já produziu saída de texto, a ferramenta Agent retorna essa saída parcial com uma nota de que o subagente foi interrompido e não completou sua tarefa. Um subagente que não produziu nada, ou cuja única saída foram chamadas de ferramenta, falha com [`Agent terminated early due to an API error`](/docs/pt/errors#agent-terminated-early-due-to-an-api-error), seguido pelo detalhe do erro. Na v2.1.199, um limite de taxa, sobrecarga ou erro de servidor que interrompeu a forma de chamadas de ferramenta apenas retornou um resultado parcial vazio contendo apenas a nota de interrupção.
* **Segundo plano**: o subagente é marcado como com falha, e a mensagem que Claude recebe quando termina nomeia o erro de API e inclui a última saída do subagente, para que o trabalho parcial não seja perdido.

Quando você configura uma [cadeia de modelo de fallback](/docs/pt/model-config#fallback-model-chains) e um subagente encontra uma falha que a cadeia cobre, como seu modelo estar indisponível, Claude Code muda o subagente para o primeiro modelo na cadeia que aceita a solicitação. O subagente continua trabalhando em vez de terminar no erro.

Uma vez que o erro de API subjacente seja resolvido, peça a Claude para tentar novamente a tarefa ou [retomar o subagente](#resume-subagents).

<h3 id="subagent-output-scanning">
  Verificação de saída de subagente
</h3>

Claude Code verifica o relatório final de cada subagente antes de Claude lê-lo. Um subagente pode ter lido arquivos, páginas da web ou saída de comando que você nunca revisou, e texto dessas fontes pode carregar instruções destinadas à conversa principal. A verificação nunca remove ou reformula nada; ela faz dois tipos de mudança que você pode notar em um relatório:

* **Inserção de barra invertida**: a verificação insere uma barra invertida em texto que imita a própria saída do Claude Code, como uma tag `<system-reminder>` ou uma linha começando com `Human:` ou `Assistant:`, para que a imitação seja lida como texto comum em vez de ser confundida com parte da conversa.
* **Linha de marcador**: a verificação prepara uma linha começando com `[harness: subagent output matched instruction-shaped pattern(s):` quando o relatório imita uma tag como `<system-reminder>` ou menciona configurações de permissão como `bypassPermissions` ou `--dangerously-skip-permissions`. Menções de configuração de permissão recebem a linha de marcador, mas o texto em si permanece como escrito.

A verificação não julga se o conteúdo é malicioso, e não muda o que uma instrução em um relatório pode fazer: uma chamada de ferramenta que o relatório leva Claude a fazer ainda passa pelas [verificações de permissão](/docs/pt/permissions) e [sandboxing](/docs/pt/sandboxing) da sessão. Não é um substituto para [restringir o que um subagente pode alcançar](#control-subagent-capabilities).

Um relatório que retorna a Claude como o resultado do subagente também chega sob um cabeçalho marcando-o como saída de subagente. O cabeçalho afirma que instruções ou reivindicações de aprovação dentro do relatório são as palavras do subagente e não carregam autoridade de você.

Um [relatório de subagente em segundo plano](#run-subagents-in-foreground-or-background) chega dentro de uma notificação de conclusão, que é marcada como um evento automatizado em vez de uma mensagem de você.

<Note>
  A verificação de saída de subagente requer Claude Code v2.1.210 ou posterior.
</Note>

<h3 id="common-patterns">
  Padrões comuns
</h3>

<h4 id="isolate-high-volume-operations">
  Isolar operações de alto volume
</h4>

Um dos usos mais eficazes para subagentes é isolar operações que produzem grandes quantidades de saída. Executar testes, buscar documentação ou processar arquivos de log pode consumir contexto significativo. Ao delegar isso a um subagente, a saída detalhada permanece no contexto do subagente enquanto apenas o resumo relevante retorna à sua conversa principal.

```text wrap theme={null}
Use um subagente para executar o conjunto de testes e relatar apenas os testes com falha com suas mensagens de erro
```

<h4 id="run-parallel-research">
  Executar pesquisa paralela
</h4>

Para investigações independentes, gere vários subagentes para trabalhar simultaneamente:

```text wrap theme={null}
Pesquise os módulos de autenticação, banco de dados e API em paralelo usando subagentes separados
```

Cada subagente explora sua área independentemente, então Claude sintetiza as descobertas. Isso funciona melhor quando os caminhos de pesquisa não dependem um do outro.

<Warning>
  Quando subagentes são concluídos, seus resultados retornam à sua conversa principal. Executar muitos subagentes que cada um retorna resultados detalhados pode consumir contexto significativo.
</Warning>

Para trabalho que precisa continuar em paralelo ou não caberá em uma janela de contexto, execute-o em [sessões separadas](/docs/pt/agents) e deixe Claude [passar descobertas entre elas](/docs/pt/cross-session-messaging).

<h4 id="chain-subagents">
  Encadear subagentes
</h4>

Para fluxos de trabalho de várias etapas, peça a Claude para usar subagentes em sequência. Cada subagente completa sua tarefa e retorna resultados a Claude, que então passa contexto relevante para o próximo subagente.

```text wrap theme={null}
Use o subagente code-reviewer para encontrar problemas de desempenho, depois use o subagente optimizer para corrigi-los
```

<h3 id="choose-between-subagents-and-main-conversation">
  Escolher entre subagentes e conversa principal
</h3>

Use a **conversa principal** quando:

* A tarefa precisa de frequente ida e volta ou refinamento iterativo
* Múltiplas fases compartilham contexto significativo, como planejamento, implementação e testes
* Você está fazendo uma mudança rápida e direcionada
* A latência importa. Um subagente que não é um [fork](#fork-the-current-conversation) começa do zero e pode precisar de tempo para reunir contexto

Use **subagentes** quando:

* A tarefa produz saída detalhada que você não precisa em seu contexto principal
* Você quer impor restrições de ferramentas ou permissões específicas
* O trabalho é autossuficiente e pode retornar um resumo

Considere [Skills](/docs/pt/skills) em vez disso quando você quer prompts ou fluxos de trabalho reutilizáveis que são executados no contexto da conversa principal em vez de contexto de subagente isolado.

Para uma pergunta sobre algo já em sua conversa, use [`/btw`](/docs/pt/interactive-mode#side-questions-with-%2Fbtw) em vez de um subagente. Ele vê seu contexto completo mas não tem acesso a ferramentas, e a resposta não é adicionada ao histórico.

<h3 id="let-subagents-spawn-their-own-subagents">
  Deixar subagentes gerar seus próprios subagentes
</h3>

Por padrão, um subagente pode gerar subagentes de seu próprio, até três camadas abaixo da conversa principal. No limite de profundidade, Claude Code retém a ferramenta `Agent` de cada subagente, exceto um [fork](#fork-the-current-conversation), para que um subagente no limite faça seu trabalho delegado em si e retorne um resumo. Um fork no limite mantém `Agent` em sua lista de ferramentas herdada, mas a ferramenta retorna um erro em vez de gerar.

Subagentes aninhados são adequados para uma tarefa delegada que em si se divide em subtarefas paralelas, como um subagente revisor que despacha um verificador por descoberta. Em uma sessão interativa, apenas o resumo do subagente de nível superior retorna para você e a saída intermediária permanece fora de sua conversa principal: um subagente que lança subagentes em segundo plano aguarda seus resultados antes de terminar. Em [modo não interativo](/docs/pt/headless) e no Agent SDK, o subagente de lançamento não aguarda, então um subagente em segundo plano aninhado que termina após seu iniciador ter terminado relata à sua conversa principal em vez disso.

Para alterar o limite, defina [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/pt/env-vars) para o número de camadas de subagente que você quer abaixo de sua conversa principal. Por exemplo, esta entrada em [`settings.json`](/docs/pt/settings) limita o aninhamento a duas camadas:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "2"
  }
}
```

Com este valor, seus subagentes podem delegar a uma segunda camada de seus próprios, e essa segunda camada não pode delegar mais. Defina `1` para desativar o aninhamento.

Um subagente aninhado é configurado da mesma forma que um de nível superior e é resolvido dos mesmos [escopos](#choose-the-subagent-scope). Para manter um subagente de gerar enquanto o aninhamento está ativado, como um revisor que deve permanecer somente leitura, omita `Agent` de sua lista [`tools`](#available-tools) ou adicione-o a `disallowedTools`.

Claude Code mostra subagentes aninhados como uma árvore no painel de subagentes abaixo da entrada de prompt e marca cada linha que ainda tem descendentes no painel com uma contagem `(+N)` deles. Abra uma linha para ver os irmãos e filhos diretos desse subagente com um caminho de volta para `main`.

<Note>
  Versões anteriores usavam padrões diferentes:

  * **v2.1.172 a v2.1.216**: subagentes podiam aninhar por padrão, até cinco camadas de profundidade, e o limite não podia ser alterado.
  * **v2.1.217 a v2.1.218**: o limite era padrão para um, então um subagente não podia gerar seu próprio a menos que você o aumentasse; v2.1.219 aumentou o padrão para três.
</Note>

<h3 id="concurrent-subagent-limit">
  Limite de subagente concorrente
</h3>

Dois limites controlam o uso de subagentes, cada um com sua própria variável: este impede que Claude gere mais subagentes enquanto muitos estão em execução, e o [limite de profundidade](#let-subagents-spawn-their-own-subagents) limita o quão profundamente os subagentes se aninham. Não há limite no número total de subagentes que Claude pode gerar ao longo de uma sessão.

Por padrão, quando 20 subagentes estão em execução em uma sessão, gerar outro com a ferramenta Agent falha com `Concurrent subagent limit reached`, e o erro diz a Claude para não tentar novamente. A geração é bem-sucedida novamente quando a contagem em execução cai abaixo do limite. Para alterar o limite, defina [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/pt/env-vars) para qualquer número inteiro positivo. Sessões com [ultracode](/docs/pt/model-config#adjust-effort-level) ativo estão isentas: o limite não é imposto lá. Requer Claude Code v2.1.217 ou posterior.

O limite bloqueia apenas subagentes que Claude gera com a ferramenta Agent, mas outras execuções ocupam os mesmos slots:

* Um fork em sessão que você inicia com [`/subtask`](#fork-the-current-conversation) ocupa um slot enquanto é executado e nunca é bloqueado pelo limite.
* [Retomar um subagente](#resume-subagents) que já terminou ocupa um slot novo sem verificar o limite, para que retomadas possam empurrar a contagem em execução além dele.

Agentes que outros recursos executam, como agentes [workflow](/docs/pt/workflows) e colegas de [equipe de agentes](/docs/pt/agent-teams), seguem seus próprios limites em vez disso.

<h3 id="manage-subagent-context">
  Gerenciar contexto de subagente
</h3>

<h4 id="what-loads-at-startup">
  O que é carregado na inicialização
</h4>

Cada subagente começa com uma janela de contexto fresca e isolada. Ele não vê seu histórico de conversa, as skills que você já invocou ou os arquivos que Claude já leu. Claude compõe uma mensagem de delegação que resume a tarefa, e o subagente trabalha a partir daí. A exceção é um [fork](#fork-the-current-conversation), que herda a conversa pai em vez de começar do zero.

O contexto inicial de um subagente não-fork contém:

* **Prompt do sistema**: o próprio prompt do agente mais detalhes de ambiente que Claude Code acrescenta, não o prompt do sistema do Claude Code. Subagentes personalizados definem o seu no [corpo markdown](#write-subagent-files) ou campo `prompt`. Agentes integrados têm prompts predefinidos.
* **Mensagem de tarefa**: o prompt de delegação que Claude escreve quando passa o trabalho.
* **Arquivos CLAUDE.md**: cada nível da [hierarquia CLAUDE.md](/docs/pt/memory#how-claude-md-files-load) que a conversa principal carrega, incluindo `~/.claude/CLAUDE.md`, regras do projeto, `CLAUDE.local.md`, arquivos de política gerenciada e qualquer arquivo [`AGENTS.md`](/docs/pt/memory#agents-md) carregado como instruções do projeto. Os agentes Explore e Plan integrados pulam isso. Um subagente cuja definição define [`omitClaudeMd`](#supported-frontmatter-fields) carrega apenas os arquivos de política gerenciada, ou nenhum quando a definição vem de [configurações gerenciadas](#choose-the-subagent-scope).
* **Status Git**: um snapshot tirado no início da sessão pai. Ausente quando o diretório de trabalho não é um repositório Git ou quando [`includeGitInstructions`](/docs/pt/settings-reference#includegitinstructions) é `false`. Explore e Plan pulam independentemente.
* **Skills pré-carregadas**: conteúdo completo de qualquer skill nomeada no campo [`skills`](#preload-skills-into-subagents) do agente. Agentes integrados não pré-carregam skills.
* **Roster de irmãos**: um lembrete do sistema listando `main` e cada outro agente nomeado na sessão, cada um um valor `to` válido para [`SendMessage`](#resume-subagents). Requer Claude Code v2.1.206 ou posterior. O roster aparece apenas quando as ferramentas do subagente incluem `SendMessage` e pelo menos um outro agente tem um nome, seja Claude o nomeou ao gerá-lo ou ele é executado como um colega de [equipe de agentes](/docs/pt/agent-teams). É um snapshot tirado quando o subagente começa, então agentes nomeados depois não aparecem.

Para lançar um de seus próprios subagentes sem os arquivos CLAUDE.md do usuário, projeto e local, defina [`omitClaudeMd: true`](#supported-frontmatter-fields) em seu frontmatter ou `--agents` JSON.

A conversa principal ainda tem seu CLAUDE.md completo quando lê os resultados desses subagentes, então a maioria das regras não precisa alcançar o subagente em si. Se uma regra deve, como "ignore o diretório `vendor/`," reafirme-a no prompt que você dá a Claude ao delegar.

Você não pode alterar quais subagentes recebem status git. Apenas Explore e Plan pulam.

Algum estado da conversa principal nunca alcança um subagente não-fork:

* **Estilo de saída**: um subagente executa seu próprio prompt do sistema, então seu [estilo de saída](/docs/pt/output-styles) não molda suas respostas, exceto em um [fork](#fork-the-current-conversation).
* **Memória automática**: a [memória automática](/docs/pt/memory#auto-memory) da conversa principal não é carregada. Para dar a um subagente memória persistente de seu próprio, use o campo [`memory`](#enable-persistent-memory).
* **Tamanho da janela de contexto**: a janela de contexto de um subagente é dimensionada por seu próprio modelo, não pelo pai. Delegar a um modelo com uma janela menor dá a esse subagente a janela menor.

<h4 id="resume-subagents">
  Retomar subagentes
</h4>

Cada invocação de subagente cria uma nova instância em vez de continuar uma anterior. Para continuar o trabalho de um subagente existente em vez de começar do zero, peça a Claude para retomá-lo.

Subagentes retomados retêm seu histórico de conversa completo, incluindo todas as chamadas de ferramenta anteriores, resultados e raciocínio. Se o subagente gerou [subagentes em segundo plano de seu próprio](#let-subagents-spawn-their-own-subagents), esse histórico inclui os resultados que entregaram enquanto era executado. O subagente continua exatamente de onde parou em vez de começar do zero.

* Quando um subagente é concluído, Claude recebe seu ID de agente.
* Os agentes Explore e Plan integrados são de uma única execução e não retornam um ID de agente, então Claude não pode retomá-los. Use `general-purpose` ou um subagente personalizado quando você precisa continuar o trabalho.
* Quando um subagente para em seu limite [`maxTurns`](#supported-frontmatter-fields), Claude Code marca a saída retornada como parcial. Para subagentes que retornam um ID de agente, Claude Code também observa no resultado que Claude pode mensagear o subagente para continuar de onde parou.

Claude usa a ferramenta `SendMessage` com o ID ou nome do agente como o campo `to` para retomá-lo. `SendMessage` não requer que [equipes de agentes](/docs/pt/agent-teams) estejam habilitadas; apenas mensagens de protocolo de equipe estruturadas como `shutdown_request` e `plan_approval_response` fazem. Além de subagentes e colegas, em sessões onde mensagens entre sessões estão habilitadas, Claude pode usar a mesma ferramenta para mensagear [suas outras sessões do Claude Code](/docs/pt/cross-session-messaging), nesta máquina ou [além dela](/docs/pt/cross-session-messaging#message-sessions-on-other-machines).

Para retomar um subagente, peça a Claude para continuar o trabalho anterior:

```text wrap theme={null}
Use o subagente code-reviewer para revisar o módulo de autenticação
[Agent completes]

Continue essa revisão de código e agora analise a lógica de autorização
[Claude retoma o subagente com contexto completo da conversa anterior]
```

Quando Claude envia a um subagente concluído uma mensagem com a ferramenta `SendMessage`, o subagente retoma em segundo plano sem uma nova invocação `Agent`. O mesmo se aplica a um subagente que Claude parou com a ferramenta `TaskStop`, uma vez que sua execução parada tenha saído. A execução retomada mantém o [conjunto de ferramentas de onde o subagente foi executado primeiro](#run-subagents-in-foreground-or-background) e pode continuar lendo o [cache de prompt que a execução original aqueceu](/docs/pt/prompt-caching#subagents-and-the-cache).

Um subagente que tem a ferramenta `SendMessage` pode enviar essa mensagem também. Em uma sessão interativa, o agente retomado então relata de volta ao subagente que o retomou, não à sua conversa principal. Esse subagente aguarda o resultado antes de terminar seu próprio trabalho. Quando um subagente mensageia um agente ao qual relata, como seu próprio iniciador, Claude Code retoma esse agente sem redirecionar seus resultados.

Um subagente que você parou você mesmo, com `x` em `/tasks` ou uma solicitação SDK `stop_task`, não retoma automaticamente. Se Claude enviar uma mensagem a ele, a mensagem é recusada e Claude é informado de que o agente foi cancelado.

Enquanto [a linha desse subagente ainda está no painel de subagentes](#run-subagents-in-foreground-or-background), digite em sua transcrição para retomá-lo você mesmo. Depois disso, uma mensagem de Claude pode retomá-lo automaticamente novamente.

Retomar inicia uma nova execução do agente sob o mesmo ID, então um subagente que já havia falhado ou sido concluído mostra como em execução novamente na lista de tarefas e nos eventos de tarefa do Agent SDK. Antes da v2.1.205, ele mantinha seu status anterior de falha ou conclusão enquanto a execução retomada estava funcionando.

A partir da v2.1.199, `SendMessage` verifica que um nome ainda se refere ao mesmo agente que alcançou anteriormente na conversa. Se um agente mais novo assumiu o nome, como um agente em segundo plano re-gerado que o reutilizou, Claude Code recusa o envio em vez de entregá-lo ao agente errado, e o erro relata qual agente o nome agora alcança para que Claude possa redirecionar. Para alcançar o agente anterior enquanto ainda está em execução, Claude o endereça pelo ID do agente que recebeu quando gerou esse agente. A verificação é escopo para a conversa atual e é redefinida em `/clear`.

A partir da v2.1.198, um subagente trata mensagens do agente que o lançou como direção de tarefa normal, incluindo correções de curso no meio da tarefa, e age sobre elas dentro de suas próprias configurações de permissão. Dois limites ainda se mantêm independentemente de quem enviou a mensagem: nenhuma mensagem de qualquer agente conta como sua aprovação para um prompt de permissão pendente, e nenhuma mensagem de agente pode alterar as configurações de permissão, `CLAUDE.md` ou configuração de um subagente. Apenas o sistema de permissão ou suas próprias mensagens podem conceder aprovação.

Você também pode pedir a Claude o ID do agente se quiser referenciá-lo explicitamente, ou encontrar IDs nos arquivos de transcrição em `~/.claude/projects/{project}/{sessionId}/subagents/`. Cada transcrição é armazenada como `agent-{agentId}.jsonl`.

Transcrições de subagentes persistem independentemente da conversa principal:

* **Compactação da conversa principal**: quando a conversa principal é compactada, as transcrições de subagentes não são afetadas. Elas são armazenadas em arquivos separados.
* **Persistência de sessão**: as transcrições de subagentes persistem dentro de sua sessão. Você pode [retomar um subagente](#resume-subagents) após reiniciar Claude Code retomando a mesma sessão.
* **Limpeza automática**: Claude Code deleta transcrições de subagentes após o período de retenção `cleanupPeriodDays`, 30 dias por padrão, seguindo as [regras de varredura de retenção](/docs/pt/claude-directory#cleaned-up-automatically).

<h4 id="auto-compaction">
  Auto-compactação
</h4>

Subagentes suportam compactação automática usando a mesma lógica que a conversa principal. A compactação é acionada sob as mesmas condições, e `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` se aplica a subagentes também. Consulte [variáveis de ambiente](/docs/pt/env-vars) para quando a substituição entra em vigor.

Eventos de compactação são registrados em arquivos de transcrição de subagentes:

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

O valor `preTokens` mostra quantos tokens foram usados antes da compactação ocorrer.

<h2 id="fork-the-current-conversation">
  Bifurcar a conversa atual
</h2>

<Note>
  Execute um subagente bifurcado com `/subtask`, que requer Claude Code v2.1.212 ou posterior. Quando [a visualização de agente está desativada](/docs/pt/agent-view#turn-off-agent-view), `/subtask` não está disponível e `/fork` inicia o subagente bifurcado; caso contrário, `/fork` copia toda a sessão para uma nova [sessão em background](/docs/pt/agent-view#from-inside-a-session).
</Note>

Uma bifurcação é um subagente que herda toda a conversa até agora em vez de começar do zero. Isso remove o isolamento de entrada que subagentes de outra forma fornecem: uma bifurcação vê o mesmo prompt de sistema, ferramentas, modelo e histórico de mensagens que a sessão principal, para que você possa entregar uma tarefa secundária sem re-explicar a situação. As chamadas de ferramentas da bifurcação ainda ficam fora de sua conversa e apenas seu resultado final volta, para que sua janela de contexto principal permaneça limpa. Use uma bifurcação quando qualquer outro subagente precisaria de muito contexto para ser útil, ou quando você quer tentar várias abordagens em paralelo a partir do mesmo ponto de partida.

Claude inicia uma bifurcação solicitando o tipo de subagente `fork` através da ferramenta Agent. Você controla se pode com [modo de bifurcação](#turn-fork-mode-on-or-off), que está ativado por padrão em sessões interativas.

Você pode iniciar uma bifurcação você mesmo com `/subtask` seguido de uma tarefa, independentemente de o modo de bifurcação estar ativado. Na v2.1.161 até v2.1.211, o comando é `/fork`. Claude Code nomeia a bifurcação a partir das primeiras palavras da tarefa. O exemplo a seguir bifurca a conversa para rascunhar casos de teste enquanto você continua com a implementação na sessão principal:

```text wrap theme={null}
/subtask draft unit tests for the parser changes so far
```

A bifurcação aparece em um painel abaixo do seu prompt e é executada em background enquanto você continua trabalhando. Quando termina, seu resultado chega como uma mensagem em sua conversa principal. A próxima seção cobre os controles do painel para observar e orientar bifurcações enquanto são executadas.

<h3 id="observe-and-steer-running-forks">
  Observar e orientar bifurcações em execução
</h3>

Bifurcações em execução aparecem em um painel abaixo da entrada de prompt, com uma linha para a sessão principal e uma para cada bifurcação.

Quando uma bifurcação termina com sucesso, Claude Code remove sua linha. Claude Code mantém a linha de uma bifurcação que falhou ou que você parou por 30 segundos, [o mesmo que para qualquer outro subagente em background](#run-subagents-in-foreground-or-background). Antes da v2.1.232, Claude Code também mantinha a linha de uma bifurcação terminada por 30 segundos.

Use estas teclas para interagir com o painel:

| Key       | Action                                                                                                                                                                                                                                            |
| :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `↑` / `↓` | Mover entre linhas                                                                                                                                                                                                                                |
| `Enter`   | Abrir a transcrição da bifurcação selecionada e enviar mensagens de acompanhamento                                                                                                                                                                |
| `x`       | Parar a bifurcação selecionada se estiver em execução, ou descartar sua linha se não estiver mais em execução. Na linha da sessão principal, ou na linha da bifurcação cuja transcrição você abriu com `Enter`, `x` digita no prompt em vez disso |
| `Esc`     | Retornar foco para a entrada de prompt                                                                                                                                                                                                            |

Com a transcrição de uma bifurcação ou subagente aberta, mensagens de acompanhamento e [skills](/docs/pt/skills) vão para esse agente, mas comandos integrados ainda são executados em sua conversa principal. A partir da v2.1.199, digitar `/model` ou `/fast` nessa visualização mostra um aviso de que isso muda o modelo da conversa principal ou modo rápido, não do agente visualizado, em vez de executá-lo silenciosamente.

<h3 id="how-forks-differ-from-other-subagents">
  Como bifurcações diferem de outros subagentes
</h3>

Uma bifurcação herda tudo que a sessão principal tem no momento em que é gerada. Qualquer outro subagente começa do zero a partir de sua definição.

|                         | Bifurcação                           | Subagente não-bifurcado                                                                                                 |
| :---------------------- | :----------------------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| Context                 | Histórico de conversa completo       | Contexto fresco com o prompt que você passa                                                                             |
| System prompt and tools | Mesmo que a sessão principal         | Da [definição file](#write-subagent-files) do subagente, [filtrado para execuções em background](#available-tools)      |
| Model                   | Mesmo que a sessão principal         | Do campo `model` do subagente                                                                                           |
| Permissions             | Prompts aparecem em seu terminal     | [Prompts aparecem em sua sessão principal](#run-subagents-in-foreground-or-background) quando em execução em background |
| Prompt cache            | Compartilhado com a sessão principal | Cache separado                                                                                                          |

Porque o prompt de sistema de uma bifurcação e as definições de ferramentas são idênticas ao pai, sua primeira solicitação reutiliza o [prompt cache](/docs/pt/prompt-caching#subagents-and-the-cache) do pai. Isso torna bifurcação mais barata do que gerar um subagente fresco para tarefas que precisam do mesmo contexto.

Quando Claude gera uma bifurcação através da ferramenta Agent, ele pode passar `isolation: "worktree"` para que as edições de arquivo da bifurcação sejam escritas em um git worktree separado em vez de seu checkout. Uma bifurcação não pode gerar bifurcações adicionais.

<h3 id="turn-fork-mode-on-or-off">
  Ativar ou desativar o modo de bifurcação
</h3>

Claude Code ativa o modo de bifurcação por padrão em sessões interativas e o deixa desativado por padrão em [modo não-interativo](/docs/pt/headless) com `-p` e no Agent SDK. O padrão interativo requer Claude Code v2.1.232 ou posterior. Em versões anteriores, defina `CLAUDE_CODE_FORK_SUBAGENT` para `1` para ativar o modo de bifurcação.

Você pode dizer que o modo de bifurcação está ativado pela forma como Claude Code lida com a ferramenta Agent:

* Claude pode gerar uma bifurcação solicitando o tipo de subagente `fork`. Quando Claude não solicita um tipo, ele obtém o subagente [general-purpose](#built-in-subagents), se a sessão ainda tiver esse tipo. Subagentes gerados a partir de uma definição, como Explore, funcionam como de costume.
* Claude Code executa os subagentes que Claude gera em background, bifurcações e subagentes não-bifurcados, além dos [casos que permanecem em foreground](#run-subagents-in-foreground-or-background). Claude Code também remove o parâmetro `run_in_background` da ferramenta Agent, para que Claude não possa solicitar o foreground.

Defina a variável de ambiente [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/pt/env-vars) para substituir os padrões:

* `1` ativa o modo de bifurcação em modo não-interativo e no Agent SDK também
* `0` desativa o modo de bifurcação em todo tipo de sessão

Para manter o modo de bifurcação ativado mas impedir que Claude gere bifurcações, [negue o tipo de subagente `fork`](#disable-specific-subagents) com uma regra `Agent(fork)`. Claude Code ainda executa os subagentes que Claude gera em background, além dos mesmos [casos que permanecem em foreground](#run-subagents-in-foreground-or-background).

<h2 id="example-subagents">
  Subagentes de exemplo
</h2>

Estes exemplos demonstram padrões eficazes para construir subagentes. Use-os como pontos de partida, ou gere uma versão personalizada com Claude.

<Tip>
  **Melhores práticas:**

  * **Projete subagentes focados:** cada subagente deve se destacar em uma tarefa específica
  * **Escreva descrições que destaquem um subagente:** Claude usa a descrição para decidir quando delegar. Faça cada descrição específica o suficiente para rotear para o subagente correto, e mantenha o conjunto combinado dentro do [orçamento de descrição de 15.000 tokens](#understand-automatic-delegation)
  * **Limite acesso a ferramentas:** conceda apenas permissões necessárias para segurança e foco
  * **Verifique no controle de versão:** compartilhe subagentes de projeto com sua equipe
</Tip>

<h3 id="code-reviewer">
  Revisor de código
</h3>

Um subagente somente leitura que revisa código sem modificá-lo. Este exemplo mostra como projetar um subagente focado com acesso limitado a ferramentas que exclui Edit e Write, e um prompt detalhado que especifica exatamente o que procurar e como formatar a saída.

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

<h3 id="debugger">
  Debugger
</h3>

Um subagente que pode analisar e corrigir problemas. Diferentemente do revisor de código, este inclui Edit porque corrigir bugs requer modificar código. O prompt fornece um fluxo de trabalho claro de diagnóstico para verificação.

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

<h3 id="data-scientist">
  Cientista de dados
</h3>

Um subagente específico de domínio para trabalho de análise de dados. Este exemplo mostra como criar subagentes para fluxos de trabalho especializados fora de tarefas de codificação típicas. Ele explicitamente define `model: sonnet` para análise mais capaz.

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

<h3 id="database-query-validator">
  Validador de consulta de banco de dados
</h3>

Um subagente que permite acesso Bash mas valida comandos para permitir apenas consultas SQL somente leitura. Este exemplo mostra como usar hooks `PreToolUse` para validação condicional quando você precisa de controle mais fino do que o campo `tools` fornece.

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code [passa entrada de hook como JSON](/docs/pt/hooks#pretooluse-input) via stdin para comandos de hook. O script de validação lê este JSON, extrai o comando sendo executado e o verifica contra uma lista de operações de escrita SQL. Se uma operação de escrita é detectada, o script [sai com código 2](/docs/pt/hooks#exit-code-2-behavior-per-event) para bloquear execução e retorna uma mensagem de erro para Claude via stderr.

Crie o script de validação em qualquer lugar em seu projeto. O caminho deve corresponder ao campo `command` em sua configuração de hook:

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

No macOS e Linux, torne o script executável:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

No Windows, escreva o script de validação em PowerShell e adicione `shell: powershell` à entrada de hook. Veja [executando hooks em PowerShell](/docs/pt/hooks#windows-powershell-tool).

O hook recebe JSON via stdin com o comando Bash em `tool_input.command`. Código de saída 2 bloqueia a operação e alimenta a mensagem de erro de volta para Claude. Veja [Hooks](/docs/pt/hooks#exit-code-output) para detalhes sobre códigos de saída e [Hook input](/docs/pt/hooks#pretooluse-input) para o schema de entrada completo.

O prompt do sistema diz ao subagente para recusar solicitações de escrita, então o hook é um backstop: se o subagente tentar uma escrita mesmo assim, Claude Code bloqueia o comando e o subagente vê a mensagem `Blocked: Write operations not allowed. Use SELECT queries only.`.

<h2 id="next-steps">
  Próximos passos
</h2>

Agora que você entende subagentes, explore estes recursos relacionados:

* [Distribuir subagentes com plugins](/docs/pt/plugins/components#agents) para compartilhar subagentes entre equipes ou projetos
* [Executar Claude Code programaticamente](/docs/pt/headless) com o Agent SDK para CI/CD e automação
* [Usar MCP servers](/docs/pt/mcp) para dar aos subagentes acesso a ferramentas e dados externos
