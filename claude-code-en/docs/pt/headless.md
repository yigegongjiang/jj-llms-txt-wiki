> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Executar Claude Code programaticamente

> Use o Agent SDK para executar Claude Code programaticamente a partir da CLI, Python ou TypeScript.

O [Agent SDK](/docs/pt/agent-sdk/overview) oferece as mesmas ferramentas, loop de agente e gerenciamento de contexto que alimentam Claude Code. Está disponível como uma CLI para scripts e CI/CD, ou como pacotes [Python](/docs/pt/agent-sdk/python) e [TypeScript](/docs/pt/agent-sdk/typescript) para controle programático completo.

Para executar Claude Code em modo não interativo, passe `-p` com seu prompt e as [opções de CLI](/docs/pt/cli-reference) que você precisa:

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Esta página aborda o uso do Agent SDK via CLI (`claude -p`). Para os pacotes SDK Python e TypeScript com saídas estruturadas, callbacks de aprovação de ferramentas e objetos de mensagem nativos, consulte a [documentação completa do Agent SDK](/docs/pt/agent-sdk/overview).

<h2 id="basic-usage">
  Uso básico
</h2>

Adicione o sinalizador `-p` (ou `--print`) a qualquer comando `claude` para executá-lo de forma não interativa. Nem todas as [opções de CLI](/docs/pt/cli-reference) se combinam com `-p`. Claude Code rejeita `--bg` e rejeita `--cloud` com uma descrição de tarefa, com um erro nomeando o conflito; `--cloud` com um ID de sessão e `-p` em vez disso [enfileira uma mensagem nessa sessão na nuvem](/docs/pt/claude-code-on-the-web#send-follow-ups-from-the-cli) e sai. As opções que você combinará com `-p` geralmente incluem:

* `--continue` para [continuar conversas](#continue-conversations)
* `--allowedTools` para [aprovar ferramentas automaticamente](#auto-approve-tools)
* `--output-format` para [saída estruturada](#get-structured-output)

Este exemplo faz uma pergunta ao Claude sobre sua base de código e imprime a resposta:

```bash theme={null}
claude -p "What does the auth module do?"
```

Claude Code sai com código 0 em caso de sucesso e com um código diferente de zero quando a execução falha, para que seus scripts possam ramificar no status de saída. Se você passar um sinalizador inválido, Claude Code relata o erro para stderr antes do início da execução. Quando uma falha ocorre dentro da execução, como autenticação ausente, Claude Code imprime a falha como resultado em stdout.

<h3 id="start-faster-with-bare-mode">
  Comece mais rápido com modo bare
</h3>

Adicione `--bare` para reduzir o tempo de inicialização pulando a descoberta automática de hooks, skills, comandos personalizados, [subagentos](/docs/pt/sub-agents), plugins instalados, servidores MCP, memória automática e CLAUDE.md. Sem ele, `claude -p` carrega o mesmo [contexto](/docs/pt/how-claude-code-works#the-context-window) que uma sessão interativa carregaria, incluindo qualquer coisa configurada no diretório de trabalho ou `~/.claude`.

O modo bare é útil para CI e scripts onde você precisa do mesmo resultado em cada máquina. Um hook no `~/.claude` de um colega de trabalho ou um servidor MCP no `.mcp.json` do projeto não serão executados, porque o modo bare nunca os lê. Um diretório que você nomeia com `--add-dir` é uma exceção parcial: o modo bare carrega skills de sua pasta `.claude/skills/`, mas ainda pula suas pastas `.claude/commands/` e `.claude/agents/`. [Skills de diretórios adicionais](/docs/pt/skills#skills-from-additional-directories) cobre o que carrega e o que não carrega.

Sem `--bare`, uma sessão `-p` executa os hooks no `settings.json` de um projeto e conecta os servidores em seu `.mcp.json`, mesmo em uma pasta que você nunca confiou. Uma sessão `-p` não mostra nenhum diálogo de confiança de workspace e nenhum prompt de aprovação por servidor. [O que é executado antes de você confiar em uma pasta](/docs/pt/permissions#what-runs-before-you-trust-a-folder) cobre cada tipo de conteúdo de repositório sob `-p` e como mantê-lo fora.

Este exemplo executa uma tarefa de resumo única em modo bare e pré-aprova a ferramenta Read para que a chamada seja concluída sem um prompt de permissão. Defina `ANTHROPIC_API_KEY` antes de executá-lo, porque o modo bare não usa seu login de assinatura:

```bash theme={null}
claude --bare -p "Summarize README.md" --allowedTools "Read"
```

No modo bare, Claude Code nunca lê credenciais OAuth ou o keychain do sistema. Para a API Anthropic, defina `ANTHROPIC_API_KEY` no ambiente, com uma chave criada no [Claude Console](https://platform.claude.com), ou forneça um `apiKeyHelper` no JSON `--settings`. Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry continuam a ler suas próprias credenciais de provedor como de costume.

No modo bare Claude tem acesso às ferramentas Bash, leitura de arquivo e edição de arquivo. Passe qualquer contexto que você precise com um sinalizador:

| Para carregar                | Use                                                     |
| ---------------------------- | ------------------------------------------------------- |
| Adições de prompt do sistema | `--append-system-prompt`, `--append-system-prompt-file` |
| Configurações                | `--settings <file-or-json>`                             |
| Servidores MCP               | `--mcp-config <file-or-json>`                           |
| Agentes personalizados       | `--agents <json>`                                       |
| Um plugin                    | `--plugin-dir <path>`, `--plugin-url <url>`             |

<Note>
  `--bare` é o modo recomendado para chamadas com script e SDK, e se tornará o padrão para `-p` em uma versão futura.
</Note>

<h3 id="background-tasks-at-exit">
  Tarefas em segundo plano ao sair
</h3>

Se Claude iniciar uma [tarefa Bash em segundo plano](/docs/pt/tools-reference#bash-tool-behavior) durante uma execução de `claude -p`, por exemplo um servidor de desenvolvimento ou uma compilação de observação, esse shell será encerrado cerca de cinco segundos após Claude retornar seu resultado final e stdin ter sido fechado. O período de carência permite que uma tarefa que termina logo após o resultado ainda entregue sua saída.

Se Claude iniciar um [subagentos](/docs/pt/sub-agents) em segundo plano ou fluxo de trabalho, `claude -p` em vez disso permanece aberto até que esse trabalho seja concluído, porque seu resultado faz parte da saída final.

Por padrão, a espera termina após 10 minutos de espera contínua inativa, para que um subagentos ou fluxo de trabalho travado não possa manter o processo aberto indefinidamente. Nesse ponto, Claude Code para o que ainda está em execução e descarta seu resultado parcial. Para alterar o limite, defina [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/pt/env-vars), ou defina-o como `0` para aguardar sem um.

Se Claude iniciar uma observação [Monitor](/docs/pt/tools-reference#monitor-tool) durante uma execução de `claude -p`, Claude Code aguarda a observação até que ela expire ou o limite de dez minutos termine a espera, o que vier primeiro. Enquanto aguarda, Claude continua respondendo ao que a observação relata. Por padrão, uma observação expira cinco minutos após Claude iniciá-la.

<h3 id="stop-a-run-with-sigterm">
  Parar uma execução com SIGTERM
</h3>

Se você parar uma execução de `claude -p` com SIGTERM, por exemplo com `kill` ou de um supervisor de processo, Claude Code sai com código 143. Claude Code deixa a volta que estava em progresso inacabada e não registra nenhum resultado para ela. Para encerrar a volta em vez disso, envie SIGINT ou chame `interrupt()` do Agent SDK, antes de parar o processo.

No SIGTERM, Claude Code encerra a árvore de processos de qualquer comando Bash que ainda está em execução. Claude Code então executa [hooks `SessionEnd`](/docs/pt/hooks#sessionend) e sai. Ao sair, Claude Code não inicia nenhuma nova chamada de ferramenta, não envia nenhuma nova solicitação de modelo e não executa nenhum hook além de `SessionEnd`. Se a execução estava no meio de um comando ou aguardando uma resposta a um prompt de permissão quando o sinal chegou, Claude Code trata essa etapa da seguinte forma:

* **Executando um comando**: Claude Code registra o comando como eliminado na sessão.
* **Aguardando uma resposta a um prompt de permissão**: se você enviar SIGTERM para o processo, Claude Code deixa o prompt sem resposta. Se seu programa fechar a sessão através do Agent SDK, o SDK encerra a entrada de Claude Code antes de enviar qualquer sinal, e Claude Code cancela o prompt assim que a entrada termina.

Quando você [retoma a sessão](#continue-conversations), Claude Code continua a volta que SIGTERM deixou inacabada.

<h2 id="examples">
  Exemplos
</h2>

Estes exemplos destacam padrões comuns de CLI. Quando um comando nomeia um arquivo como `auth.py` ou `build-error.txt`, substitua por um arquivo do seu próprio projeto. Em CI ou outros ambientes com script, adicione [`--bare`](#start-faster-with-bare-mode) para que Claude Code inicie sem carregar os hooks, plugins, memória automática ou `CLAUDE.md` do host.

<h3 id="pipe-data-through-claude">
  Canalizar dados através do Claude
</h3>

O modo não interativo lê stdin, então você pode canalizar dados e redirecionar a resposta como qualquer outra ferramenta de linha de comando.

Este exemplo canaliza um log de compilação para Claude e escreve a explicação em um arquivo:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Com `--output-format json`, a carga de resposta inclui `total_cost_usd` e um detalhamento de custo por modelo, para que os chamadores com script possam rastrear gastos sem consultar o [painel de uso](/docs/pt/costs). Quando você continua uma conversa anterior com `--continue` ou `--resume`, a execução relata o total da conversa, [gastos de execuções anteriores inclusos](/docs/pt/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Ambas as figuras são [estimativas do lado do cliente](/docs/pt/agent-sdk/cost-tracking) e podem diferir da sua fatura real.

<Note>
  Stdin canalizado é limitado a 10MB. Se você exceder o limite, Claude Code sai com um erro claro e um status diferente de zero. Para trabalhar com entradas maiores, escreva o conteúdo em um arquivo e faça referência ao caminho do arquivo em seu prompt em vez de canalizá-lo.
</Note>

Se Claude Code não conseguir ler stdin, por exemplo porque o processo que o iniciou desconectou sua extremidade, Claude Code imprime um aviso para stderr e continua com o prompt da linha de comando. Antes da v2.1.211, um stdin ilegível no Windows causava falha na sessão ou a fazia sair silenciosamente sem saída.

<h3 id="add-claude-to-a-build-script">
  Adicionar Claude a um script de compilação
</h3>

Você pode envolver uma chamada não interativa em um script para usar Claude como um linter ou revisor específico do projeto.

Este script `package.json` canaliza o diff contra `main` para Claude e pede que ele relate erros de digitação. Canalizar o diff significa que Claude não precisa de permissão Bash para lê-lo, e as aspas duplas escapadas mantêm o script portável para Windows:

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

Execute com `npm run lint:claude`.

<h3 id="get-structured-output">
  Obter saída estruturada
</h3>

Use `--output-format` para controlar como as respostas são retornadas:

* `text` (padrão): saída de texto simples
* `json`: JSON estruturado com resultado, ID de sessão e metadados
* `stream-json`: JSON delimitado por quebra de linha para streaming em tempo real

Este exemplo retorna um resumo do projeto como JSON com metadados de sessão, com o resultado de texto no campo `result`:

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

Para obter saída em conformidade com um esquema específico, use `--output-format json` com `--json-schema` e uma definição de [JSON Schema](https://json-schema.org/). A resposta inclui metadados sobre a solicitação (ID de sessão, uso, etc.) com a saída estruturada no campo `structured_output`.

Este exemplo extrai nomes de funções e os retorna como uma matriz de strings:

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

Se o valor não for um JSON Schema válido, `claude` sai com `Error: --json-schema is not a valid JSON Schema` seguido pelo diagnóstico do validador. Claude Code aceita esquemas que usam a palavra-chave `format`, como `"format": "email"`, mas trata `format` como uma anotação e não a impõe. Antes da v2.1.205, Claude Code ignorava silenciosamente um esquema inválido e retornava texto não estruturado, e tratava qualquer esquema contendo `format` como inválido.

<Tip>
  Use uma ferramenta como [jq](https://jqlang.org/) para analisar a resposta e extrair campos específicos:

  ```bash theme={null}
  # Extract the text result
  claude -p "Summarize this project" --output-format json | jq -r '.result'

  # Extract structured output
  claude -p "Extract function names from auth.py" \
    --output-format json \
    --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
    | jq '.structured_output'
  ```
</Tip>

<h3 id="stream-responses">
  Respostas de stream
</h3>

Use `--output-format stream-json` com `--verbose` e `--include-partial-messages` para receber tokens conforme são gerados. Cada linha é um objeto JSON representando um evento:

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

A última linha do stream é uma mensagem `result` com o texto de resposta final, custo e metadados de sessão.

Se seu consumidor ler o stream lentamente, Claude Code aguarda a fila de saída drenar antes de sair, dimensionando a espera com o quanto ainda está na fila, limitado a 30 segundos. Antes da v2.1.214, a espera de saída era limitada a cerca de dois segundos, o que poderia cortar o final de uma resposta grande.

O exemplo a seguir usa [jq](https://jqlang.org/) para filtrar deltas de texto e exibir apenas o texto de streaming. O sinalizador `-r` produz strings brutas (sem aspas) e `-j` une sem quebras de linha para que os tokens façam streaming continuamente:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Para streaming programático com callbacks e objetos de mensagem, consulte [Stream responses in real-time](/docs/pt/agent-sdk/streaming-output) na documentação do Agent SDK.

<h4 id="follow-subagent-messages">
  Seguir mensagens de subagentes
</h4>

Mensagens de [subagentes](/docs/pt/sub-agents) aparecem no stream como mensagens `assistant` e `user` cujo campo `parent_tool_use_id` é o ID da chamada de ferramenta que gerou o subagente. Mensagens da conversa principal carregam `null` nesse campo.

A primeira mensagem de um subagente em execução em [primeiro plano](/docs/pt/sub-agents#run-subagents-in-foreground-or-background) é uma mensagem `user` carregando o prompt que o conduz. Após essa primeira mensagem, Claude Code emite:

* **Por padrão**: os blocos `tool_use` e `tool_result` do subagente.
* **Com [`--forward-subagent-text`](/docs/pt/cli-reference#cli-flags) ou [`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`](/docs/pt/env-vars)**: os blocos de texto e pensamento do subagente também, para que você possa reconstruir a transcrição de cada subagente. Isso requer Claude Code v2.1.211 ou posterior.

Quando você habilita uma das opções, Claude Code encaminha mensagens de [subagentes em cada profundidade de aninhamento](/docs/pt/sub-agents#let-subagents-spawn-their-own-subagents), independentemente de cada um ter sido gerado com a ferramenta Agent ou iniciado como uma [skill bifurcada](/docs/pt/skills#run-skills-in-a-subagent). Mensagens de subagentes que uma skill bifurcada gera, e de skills bifurcadas iniciadas dentro de um subagente ou outra skill bifurcada, requerem Claude Code v2.1.275 ou posterior. Em `parent_tool_use_id`, as mensagens do subagente aninhado carregam o ID da chamada de ferramenta Agent ou Skill que o iniciou, para que você possa reconstruir a árvore de aninhamento completa seguindo esses IDs. Antes da v2.1.219, mensagens de subagentes aninhados não apareciam no stream.

Skills que [executam em um subagente](/docs/pt/skills#run-skills-in-a-subagent) aparecem no stream da mesma forma: a primeira mensagem da skill bifurcada é uma mensagem `user` carregando o conteúdo da skill que conduz a execução. Se você habilitar uma das opções, o stream também carrega os blocos de texto e pensamento da skill bifurcada. Antes da v2.1.265, apenas os blocos `tool_use` e `tool_result` de uma skill bifurcada apareciam no stream.

<h4 id="handle-api-retries">
  Lidar com tentativas de API
</h4>

Quando uma solicitação de API falha com um erro que pode ser repetido, Claude Code emite um evento `system/api_retry` antes de tentar novamente. Na v2.1.246 ou posterior, quando um `401` ou `403` rejeita uma credencial [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper), Claude Code faz as duas primeiras tentativas silenciosamente sem evento, depois emite o evento como usual a partir da terceira tentativa consecutiva em diante. As tentativas silenciosas ainda contam para `attempt`. Você pode usar o evento para mostrar progresso de repetição em sua própria interface.

| Campo            | Tipo             | Descrição                                                                                                                                                                                                                                                                                                                                                                                                |
| ---------------- | ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`       | tipo de mensagem                                                                                                                                                                                                                                                                                                                                                                                         |
| `subtype`        | `"api_retry"`    | identifica isso como um evento de repetição                                                                                                                                                                                                                                                                                                                                                              |
| `attempt`        | inteiro          | número da tentativa atual, começando em 1                                                                                                                                                                                                                                                                                                                                                                |
| `max_retries`    | inteiro          | total de repetições permitidas para a causa dessa falha, que pode ser menor que o orçamento de toda a sessão                                                                                                                                                                                                                                                                                             |
| `retry_delay_ms` | inteiro          | milissegundos até a próxima tentativa                                                                                                                                                                                                                                                                                                                                                                    |
| `error_status`   | inteiro ou nulo  | código de status HTTP da tentativa falhada, ou `null` quando a tentativa não obteve resposta HTTP da API                                                                                                                                                                                                                                                                                                 |
| `no_response`    | objeto, opcional | presente apenas quando a tentativa falhada obteve [nenhum cabeçalho de resposta a tempo](/docs/pt/errors#no-response-from-api). `waited_ms` é quanto tempo essa tentativa aguardou e `retry_wait_ms` é quanto tempo a repetição aguardará. Nesses eventos, `max_retries` reflete a uma repetição que essa causa normalmente obtém, não o orçamento de toda a sessão. Requer Claude Code v2.1.261 ou posterior |
| `error`          | string           | categoria de erro: `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, ou `unknown`                                                                                                                                                   |
| `uuid`           | string           | identificador único do evento                                                                                                                                                                                                                                                                                                                                                                            |
| `session_id`     | string           | sessão à qual o evento pertence                                                                                                                                                                                                                                                                                                                                                                          |

<h4 id="read-session-metadata">
  Ler metadados de sessão
</h4>

O evento `system/init` relata metadados de sessão incluindo o modelo, ferramentas, servidores MCP e plugins carregados. É o primeiro evento no stream a menos que eventos de inicialização o precedam:

* eventos `plugin_install`, quando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/pt/env-vars) está definido.
* [eventos `hook_started`, `hook_progress` e `hook_response`](/docs/pt/agent-sdk/typescript#sdkhookstartedmessage), enquanto um hook [`SessionStart`](/docs/pt/hooks#sessionstart) ou [`Setup`](/docs/pt/hooks#setup) configurado é executado. Estes fazem stream conforme o hook os produz. Claude Code v2.1.169 através v2.1.203 os entregou em um lote após o hook ser concluído, ainda à frente de `system/init`; v2.1.204 restaurou a entrega ao vivo.

O evento também carrega uma matriz `capabilities` opcional de strings nomeando os comportamentos do protocolo que esta versão do Claude Code implementa, como `interrupt_receipt_v1` ou `interrupt_cancel_queued_v1`. Verifique-a para detectar recursos em vez de comparar strings de versão, e ignore valores que você não reconheça. O campo requer Claude Code v2.1.205 ou posterior e está ausente em versões anteriores. Consulte [`SDKSystemMessage`](/docs/pt/agent-sdk/typescript#sdksystemmessage) para a lista de capacidades.

<h4 id="fail-ci-when-a-plugin-or-mcp-server-doesn’t-load">
  Falhar CI quando um plugin ou servidor MCP não carrega
</h4>

Use os campos de plugin no evento `system/init` para capturar um plugin que não foi carregado:

| Campo           | Tipo  | Descrição                                                                                                                                                                                                                                                                                                                 |
| --------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | array | plugins que foram carregados com sucesso, cada um com `name` e `path`                                                                                                                                                                                                                                                     |
| `plugin_errors` | array | erros de tempo de carregamento de plugin, cada um com `plugin`, `type` e `message`. Inclui versões de dependência insatisfeitas e falhas de carregamento de `--plugin-dir` como um caminho ausente ou arquivo inválido. Os plugins afetados são rebaixados e ausentes de `plugins`. A chave é omitida quando não há erros |

Use os campos de servidor MCP da mesma forma. Quando você passa [`--mcp-config`](/docs/pt/cli-reference#cli-flags) com `-p`, Claude Code aguarda servidores ainda pendentes antes de executar a primeira volta, até o tempo limite de inicialização [`MCP_TIMEOUT`](/docs/pt/env-vars), 30 segundos por padrão. Um servidor remoto com uma [lista de ferramentas em cache](/docs/pt/agent-sdk/mcp#connection-timing) pula a espera, mostra `pending` em `system/init` e se conecta em sua primeira chamada de ferramenta. A espera requer Claude Code v2.1.221 ou posterior.

Claude Code valida cada entrada `--mcp-config` na inicialização e pula entradas que falham na validação, por exemplo uma entrada `url` sem `type`. A execução continua e sai limpa, então verifique esses campos para capturar um servidor que nunca foi carregado:

| Campo               | Tipo  | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `mcp_servers`       | array | servidores MCP na sessão, cada um com `name` e `status`                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `mcp_server_errors` | array | entradas `--mcp-config` puladas pela validação de config, cada uma com `name`, `type` e `message`. `type` é uma categoria de pulo como `unknown_type`, `url_missing_type`, `invalid_config` ou `reserved_name`; trate valores que você não reconheça como um pulo genérico. Os servidores afetados estão ausentes de `mcp_servers`. A chave é omitida quando não há erros, então um portão de CI pode falhar em uma matriz não vazia. Requer Claude Code v2.1.219 ou posterior |

Quando você executa o comando manualmente em um terminal, Claude Code também imprime um aviso de inicialização para stderr, como `Warning: 1 MCP server skipped due to invalid config:`, seguido pela razão de cada entrada pulada. Quando você redireciona stderr, ou quando um programa como um executor de CI ou um host SDK o captura, Claude Code não imprime aviso e relata as entradas puladas apenas no campo `mcp_server_errors`. O aviso requer Claude Code v2.1.219 ou posterior.

<h4 id="track-plugin-installs">
  Rastrear instalações de plugin
</h4>

Quando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/pt/env-vars) está definido, Claude Code emite eventos `system/plugin_install` enquanto plugins do marketplace instalam antes da primeira volta. Use estes para exibir o progresso de instalação em sua própria UI.

| Campo        | Tipo                                                     | Descrição                                                                                                    |
| ------------ | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `type`       | `"system"`                                               | tipo de mensagem                                                                                             |
| `subtype`    | `"plugin_install"`                                       | identifica isso como um evento de instalação de plugin                                                       |
| `status`     | `"started"`, `"installed"`, `"failed"`, ou `"completed"` | `started` e `completed` envolvem a instalação geral; `installed` e `failed` relatam marketplaces individuais |
| `name`       | string, opcional                                         | nome do marketplace, presente em `installed` e `failed`                                                      |
| `error`      | string, opcional                                         | mensagem de falha, presente em `failed`                                                                      |
| `uuid`       | string                                                   | identificador único do evento                                                                                |
| `session_id` | string                                                   | sessão à qual o evento pertence                                                                              |

<h3 id="auto-approve-tools">
  Aprovar ferramentas automaticamente
</h3>

Use `--allowedTools` para permitir que Claude use certas ferramentas sem solicitar. Este exemplo executa um conjunto de testes e corrige falhas, permitindo que Claude execute comandos Bash e leia/edite arquivos sem pedir permissão:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Para definir uma linha de base para toda a sessão em vez de listar ferramentas individuais, passe um [modo de permissão](/docs/pt/permission-modes). Para `-p`, o [modo de permissão inicial integrado](/docs/pt/permission-modes#which-mode-a-session-starts-in) é Manual em todos os planos, então passe o modo de permissão que você deseja:

* **`auto`**: passe `--permission-mode auto` para ter um classificador revisar a maioria das ações em vez de você
* **`dontAsk`**: Claude Code nega qualquer chamada que de outra forma solicitaria, o que é útil para execuções de CI bloqueadas. Ações que não precisam de aprovação no modo Manual ainda são executadas, como leituras de arquivo em seus diretórios de trabalho e o [conjunto de comandos somente leitura](/docs/pt/permissions#read-only-commands), e também ações que suas entradas `--allowedTools` ou regras `permissions.allow` cobrem. `AskUserQuestion`, ferramentas de conector [que sua organização definiu como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools), e ferramentas MCP marcadas [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool) são negadas mesmo quando uma regra de permissão corresponde
* **`acceptEdits`**: Claude escreve arquivos sem solicitar, e Claude Code aprova automaticamente comandos comuns do sistema de arquivos como `mkdir`, `touch`, `mv` e `cp`. As [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves) ainda se aplicam. Além do conjunto de comandos somente leitura, outros comandos de shell e solicitações de rede ainda precisam de uma entrada `--allowedTools` ou uma regra `permissions.allow`. Consulte [o que `acceptEdits` aprova automaticamente](/docs/pt/permission-modes#auto-approve-file-edits-with-acceptedits-mode) para a lista completa

Este exemplo aplica correções de lint com `acceptEdits` como a linha de base:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="turn-off-permission-prompts-in-unattended-runs">
  Desativar prompts de permissão em execuções autônomas
</h3>

Passe `--permission-prompts none` quando ninguém estiver disponível para responder prompts de permissão, por exemplo em um trabalho agendado. O sinalizador é mais importante quando sua execução tem um host de permissão: um aplicativo Agent SDK com um callback [`canUseTool`](/docs/pt/agent-sdk/user-input), ou uma ferramenta MCP que você passa com [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags). Sem o sinalizador, sua execução aguarda que esse host responda cada solicitação de permissão.

Com o sinalizador, sua execução não consulta o host ou aguarda por ele. Qualquer coisa que solicitaria é negada a menos que um hook `PermissionRequest` a permita, Claude é informado que ninguém pode aprovar a solicitação e não deve tentar novamente, e a execução continua. Em uma execução `-p` sem host, essas solicitações são negadas de qualquer forma, e o sinalizador também diz a Claude não tentar novamente. Regras de permissão, [hooks `PermissionRequest`](/docs/pt/hooks#permissionrequest) e o modo de permissão que você definir ainda decidem cada chamada primeiro; Claude Code nega apenas as solicitações que nada mais resolve.

Este exemplo executa uma tarefa autônoma em [modo auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode). O classificador revisa cada ação como usual, e Claude Code nega qualquer coisa que teria caído de volta para um prompt:

```bash theme={null}
claude -p "Update the dependency pins and run the tests" --permission-mode auto --permission-prompts none
```

Com `--permission-prompts none`, Claude Code remove as ferramentas que precisam de uma resposta de uma pessoa, como [`AskUserQuestion`](/docs/pt/tools-reference#askuserquestion-tool-behavior), para que Claude não possa chamá-las. Qualquer [solicitação de elicitação MCP](/docs/pt/mcp#respond-to-mcp-elicitation-requests) que nenhum hook [`Elicitation`](/docs/pt/hooks#elicitation) responde é cancelada.

Com `--output-format stream-json`, negações aparecem como mensagens de sistema `permission_denied`, e a mensagem de resultado final as lista em `permission_denials`.

<Note>
  O sinalizador `--permission-prompts` requer Claude Code v2.1.259 ou posterior. Versões anteriores o rejeitam com um erro de opção desconhecida.
</Note>

<h3 id="create-a-commit">
  Criar um commit
</h3>

Este exemplo revisa as alterações preparadas e cria um commit com uma mensagem apropriada:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

O sinalizador `--allowedTools` usa [sintaxe de regra de permissão](/docs/pt/settings-reference#permission-rule-syntax). O ` *` à direita habilita correspondência de prefixo, então `Bash(git diff *)` permite qualquer comando começando com `git diff`. O espaço antes de `*` é importante: sem ele, `Bash(git diff*)` também corresponderia a `git diff-index`.

<Note>
  O suporte a comandos difere no modo `-p`:

  * [Skills](/docs/pt/skills) invocadas pelo usuário e comandos personalizados funcionam. Inclua `/skill-name` na string de prompt e Claude Code o expande antes de executar.
  * Comandos integrados que abrem um diálogo interativo, como `/login`, não estão disponíveis no modo `-p`.
  * `/model`, `/effort`, `/fast`, `/color` e `/rename` aceitam o valor como um argumento, por exemplo `/model sonnet`, e `/mcp` sem argumento imprime um resumo de texto do status do servidor. Essas formas requerem Claude Code v2.1.205 ou posterior e seguem as [notas de disponibilidade de cada comando](/docs/pt/commands#all-commands).
  * Para alterar uma configuração, passe `key=value` para `/config`, por exemplo `/config thinking=false`.
  * `/output-style <style>` alterna [estilos de saída](/docs/pt/output-styles) e `/output-style` sozinho os lista. Requer Claude Code v2.1.269 ou posterior.
</Note>

<h3 id="customize-the-system-prompt">
  Personalizar o prompt do sistema
</h3>

Use `--append-system-prompt` para adicionar instruções mantendo o comportamento padrão do Claude Code. Este exemplo envia um diff de PR para Claude e o instrui a revisar vulnerabilidades de segurança. Salve como um script de shell, por exemplo `review.sh`:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

No script, `"$1"` representa o primeiro argumento que você passa na linha de comando. Execute `bash review.sh 123` e o shell substitui `"$1"` por `123`, então o script busca o diff para PR 123. Claude Code imprime a revisão como JSON, com o texto no campo `result`.

Consulte [system prompt flags](/docs/pt/cli-reference#system-prompt-flags) para mais opções, incluindo `--system-prompt` para substituir completamente o prompt padrão.

<h3 id="continue-conversations">
  Continuar conversas
</h3>

Use `--continue` para continuar a conversa mais recente, ou `--resume` com um ID de sessão para continuar uma conversa específica. Na Claude Code v2.1.257 ou posterior, quando você passa `--continue`, Claude Code abre uma [sessão em background](/docs/pt/sessions#resume-a-session) que terminou, mas não uma que ainda está em execução. Este exemplo executa uma revisão e depois envia prompts de acompanhamento:

```bash theme={null}
# First request
claude -p "Review this codebase for performance issues"

# Continue the most recent conversation
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

Se você estiver executando várias conversas, capture o ID da sessão para retomar uma específica:

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

Você pode executar os dois comandos de diretórios diferentes: Claude Code [encontra a sessão por seu ID](/docs/pt/sessions#resume-a-session) em qualquer projeto nesta máquina. Antes da v2.1.223, Claude Code procurava o ID apenas no diretório do projeto atual e seus git worktrees, então você tinha que executar ambos os comandos do mesmo diretório.

No lugar do ID da sessão, você pode passar para `--resume` o caminho absoluto para o arquivo de [transcrição](/docs/pt/sessions#where-transcripts-are-stored) `.jsonl` de uma sessão, e Claude Code continua a conversa armazenada nesse arquivo.

<h2 id="next-steps">
  Próximas etapas
</h2>

* [Agent SDK quickstart](/docs/pt/agent-sdk/quickstart): construa seu primeiro agente com Python ou TypeScript
* [CLI reference](/docs/pt/cli-reference): todos os sinalizadores e opções de CLI
* [GitHub Actions](/docs/pt/github-actions): use o Agent SDK em fluxos de trabalho do GitHub
* [GitLab CI/CD](/docs/pt/gitlab-ci-cd): use o Agent SDK em pipelines do GitLab
