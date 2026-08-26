> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Como o loop do agente funciona

> Entenda o ciclo de vida das mensagens, execução de ferramentas, janela de contexto e arquitetura que alimentam seus agentes SDK.

O Agent SDK permite que você incorpore o loop do agente autônomo do Claude Code em suas próprias aplicações. O SDK é um pacote independente que oferece controle programático sobre ferramentas, permissões, limites de custo e saída. Você não precisa ter o CLI do Claude Code instalado para usá-lo.

Quando você inicia um agente, o SDK executa o mesmo [loop de execução que alimenta o Claude Code](/docs/pt/how-claude-code-works#the-agentic-loop): Claude avalia seu prompt, chama ferramentas para agir, recebe os resultados e repete até que a tarefa seja concluída. Esta página explica o que acontece dentro desse loop para que você possa construir, depurar e otimizar seus agentes de forma eficaz.

<h2 id="the-loop-at-a-glance">
  O loop em um relance
</h2>

Cada sessão de agente segue o mesmo ciclo:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-loop-diagram.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=1c6e8f28d80dba14a7287419656f1237" alt="Diagrama do loop do agente: seu prompt entra no loop agentic, onde Claude avalia e solicita chamadas de ferramentas, cujos resultados retornam para outra avaliação, ou retorna a resposta final" width="720" height="212" data-path="images/agent-loop-diagram.svg" />

1. **Receber prompt.** Claude recebe seu prompt, junto com o prompt do sistema, definições de ferramentas e histórico de conversa. O SDK produz uma [`SystemMessage`](#message-types) com subtipo `"init"` contendo metadados da sessão.
2. **Avaliar e responder.** Claude avalia o estado atual e determina como proceder. Pode responder com texto, solicitar uma ou mais chamadas de ferramentas, ou ambos. O SDK produz uma [`AssistantMessage`](#message-types) contendo o texto e quaisquer solicitações de chamadas de ferramentas.
3. **Executar ferramentas.** O SDK executa cada ferramenta solicitada e coleta os resultados. Cada conjunto de resultados de ferramentas retorna para Claude para a próxima decisão. Você pode usar [hooks](/docs/pt/agent-sdk/hooks) para interceptar, modificar ou bloquear chamadas de ferramentas antes de serem executadas.
4. **Repetir.** Os passos 2 e 3 se repetem como um ciclo. Cada ciclo completo é uma volta. Claude continua chamando ferramentas e processando resultados até produzir uma resposta sem chamadas de ferramentas.
5. **Retornar resultado.** O SDK produz uma [`AssistantMessage`](#message-types) final com a resposta de texto (sem chamadas de ferramentas), seguida por uma [`ResultMessage`](#message-types) com o texto final, uso de tokens, custo e ID da sessão.

Uma pergunta rápida ("quais arquivos estão aqui?") pode levar uma ou duas voltas chamando `Glob` e respondendo com os resultados. Uma tarefa complexa ("refatore o módulo de autenticação e atualize os testes") pode encadear dezenas de chamadas de ferramentas em muitas voltas, lendo arquivos, editando código e executando testes, com Claude ajustando sua abordagem com base em cada resultado.

<h2 id="turns-and-messages">
  Voltas e mensagens
</h2>

Uma volta é uma rodada dentro do loop: Claude produz saída que inclui chamadas de ferramentas, o SDK executa essas ferramentas e os resultados retornam para Claude automaticamente. Isso acontece sem ceder o controle de volta para seu código. As voltas continuam até que Claude produza saída sem chamadas de ferramentas, momento em que o loop termina e o resultado final é entregue.

Considere como uma sessão completa pode parecer para o prompt "Corrija os testes falhando em auth.ts".

Primeiro, o SDK envia seu prompt para Claude e produz uma [`SystemMessage`](#message-types) com os metadados da sessão. Então o loop começa:

1. **Volta 1:** Claude chama `Bash` para executar `npm test`. O SDK produz uma [`AssistantMessage`](#message-types) com a chamada de ferramenta, executa o comando, então produz uma [`UserMessage`](#message-types) com a saída (três falhas).
2. **Volta 2:** Claude chama `Read` em `auth.ts` e `auth.test.ts`. O SDK retorna o conteúdo dos arquivos e produz uma `AssistantMessage`.
3. **Volta 3:** Claude chama `Edit` para corrigir `auth.ts`, então chama `Bash` para executar novamente `npm test`. Todos os três testes passam. O SDK produz uma `AssistantMessage`.
4. **Volta final:** Claude produz uma resposta apenas com texto sem chamadas de ferramentas: "Corrigi o bug de autenticação, todos os três testes passam agora." O SDK produz uma `AssistantMessage` final com este texto, então uma [`ResultMessage`](#message-types) com o mesmo texto mais custo e uso.

Foram quatro voltas: três com chamadas de ferramentas, uma resposta final apenas com texto.

Você pode limitar o loop com `max_turns` / `maxTurns`, que conta apenas voltas de uso de ferramentas. Por exemplo, `max_turns=2` no loop acima teria parado antes da etapa de edição. Você também pode usar `max_budget_usd` / `maxBudgetUsd` para limitar voltas com base em um limite de gastos.

Sem limites, o loop é executado até que Claude termine por conta própria, o que é bom para tarefas bem definidas, mas pode ser longo em prompts abertos ("melhore esta base de código"). Definir um orçamento é um bom padrão para agentes em produção. Veja [Voltas e orçamento](#turns-and-budget) abaixo para a referência de opções.

<h2 id="message-types">
  Tipos de mensagens
</h2>

Conforme o loop é executado, o SDK produz um fluxo de mensagens. Cada mensagem carrega um tipo que informa em qual estágio do loop ela veio. Os cinco tipos principais são:

* **`SystemMessage`:** eventos do ciclo de vida da sessão. O campo `subtype` os distingue:

  * `"init"`: metadados da sessão para a execução. Quando um hook `SessionStart` ou `Setup` é executado durante a inicialização da sessão, suas [mensagens do ciclo de vida do hook](/docs/pt/agent-sdk/typescript#sdkhookstartedmessage) chegam antes da mensagem `init`
  * `"compact_boundary"`: dispara após [compactação](#automatic-compaction)
  * `"informational"`: banners de status em texto simples do loop
  * `"worker_shutting_down"`: o loop terminará após a volta atual porque o host está saindo ou Remote Control foi desconectado

  Em TypeScript, cada subtipo diferente de `"init"` é seu próprio tipo na união [`SDKMessage`](/docs/pt/agent-sdk/typescript#sdkmessage) em vez de um subtipo de `SDKSystemMessage`.
* **`AssistantMessage`:** emitida após cada resposta do Claude, incluindo a final apenas com texto. Contém blocos de conteúdo de texto e blocos de chamadas de ferramentas dessa volta.
* **`UserMessage`:** emitida após cada execução de ferramenta com o conteúdo do resultado da ferramenta enviado de volta para Claude. Também emitida para quaisquer entradas do usuário que você transmita no meio do loop.
* **`StreamEvent`:** emitida apenas quando mensagens parciais estão habilitadas. Contém eventos brutos de streaming da API (deltas de texto, pedaços de entrada de ferramentas). Veja [Respostas de streaming](/docs/pt/agent-sdk/streaming-output).
* **`ResultMessage`:** marca o fim do loop do agente. Contém o resultado de texto final, uso de tokens, custo e ID da sessão. Verifique o campo `subtype` para determinar se a tarefa foi bem-sucedida ou atingiu um limite. Um pequeno número de eventos de sistema finais, como `prompt_suggestion`, pode chegar após ele, então itere o fluxo até a conclusão em vez de quebrar no resultado. Veja [Lidar com o resultado](#handle-the-result).

Esses cinco tipos cobrem o ciclo de vida completo do loop do agente em ambos os SDKs. O SDK TypeScript também produz eventos de observabilidade adicionais (eventos de hooks, progresso de ferramentas, limites de taxa, notificações de tarefas) que fornecem detalhes extras, mas não são necessários para conduzir o loop. Veja a [referência de tipos de mensagens Python](/docs/pt/agent-sdk/python#message-types) e [referência de tipos de mensagens TypeScript](/docs/pt/agent-sdk/typescript#message-types) para as listas completas.

<h3 id="handle-messages">
  Lidar com mensagens
</h3>

Quais mensagens você lida depende do que você está construindo:

* **Apenas resultados finais:** lide com `ResultMessage` para obter a saída, custo e se a tarefa foi bem-sucedida ou atingiu um limite.
* **Atualizações de progresso:** lide com `AssistantMessage` para ver o que Claude está fazendo a cada volta, incluindo quais ferramentas chamou.
* **Streaming ao vivo:** habilite mensagens parciais (`include_partial_messages` em Python, `includePartialMessages` em TypeScript) para obter mensagens `StreamEvent` em tempo real. Veja [Respostas de streaming em tempo real](/docs/pt/agent-sdk/streaming-output).

Como você verifica tipos de mensagens depende do SDK:

* **Python:** verifique tipos de mensagens com `isinstance()` contra classes importadas de `claude_agent_sdk` (por exemplo, `isinstance(message, ResultMessage)`).
* **TypeScript:** verifique o campo de string `type` (por exemplo, `message.type === "result"`). `AssistantMessage` e `UserMessage` envolvem a mensagem bruta da API em um campo `.message`, então blocos de conteúdo estão em `message.message.content`, não em `message.content`.

<Accordion title="Exemplo: Verificar tipos de mensagens e lidar com resultados">
  <CodeGroup>
    ```python Python theme={null}
    import asyncio
    from claude_agent_sdk import query, AssistantMessage, ResultMessage


    async def main():
        try:
            async for message in query(prompt="Summarize this project"):
                if isinstance(message, AssistantMessage):
                    print(f"Turn completed: {len(message.content)} content blocks")
                if isinstance(message, ResultMessage):
                    if message.subtype == "success":
                        print(message.result)
                    else:
                        print(f"Stopped: {message.subtype}")
        except Exception as error:
            # A single-shot query() raises after yielding an error result. If the
            # failure was an error result, the error subtype branches above have
            # already run; connection or process failures yield no result message.
            print(f"Session ended with an error: {error}")


    asyncio.run(main())
    ```

    ```typescript TypeScript theme={null}
    import { query } from "@anthropic-ai/claude-agent-sdk";

    try {
      for await (const message of query({ prompt: "Summarize this project" })) {
        if (message.type === "assistant") {
          console.log(`Turn completed: ${message.message.content.length} content blocks`);
        }
        if (message.type === "result") {
          if (message.subtype === "success") {
            console.log(message.result);
          } else {
            console.log(`Stopped: ${message.subtype}`);
          }
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, the error subtype branches above have
      // already run; connection or process failures yield no result message.
      console.log(`Session ended with an error: ${error}`);
    }
    ```
  </CodeGroup>
</Accordion>

<h2 id="tool-execution">
  Execução de ferramentas
</h2>

Ferramentas dão ao seu agente a capacidade de agir. Sem ferramentas, Claude pode apenas responder com texto. Com ferramentas, Claude pode ler arquivos, executar comandos, pesquisar código e interagir com serviços externos.

<h3 id="built-in-tools">
  Ferramentas integradas
</h3>

O SDK inclui as mesmas ferramentas que alimentam o Claude Code:

| Categoria                | Ferramentas                                                     | O que fazem                                                                                  |
| :----------------------- | :-------------------------------------------------------------- | :------------------------------------------------------------------------------------------- |
| **Operações de arquivo** | `Read`, `Edit`, `Write`                                         | Ler, modificar e criar arquivos                                                              |
| **Pesquisa**             | `Glob`, `Grep`                                                  | Encontrar arquivos por padrão, pesquisar conteúdo com regex                                  |
| **Execução**             | `Bash`                                                          | Executar comandos de shell, scripts, operações git                                           |
| **Web**                  | `WebSearch`, `WebFetch`                                         | Pesquisar a web, buscar e analisar páginas                                                   |
| **Descoberta**           | `ToolSearch`                                                    | Encontrar e carregar ferramentas dinamicamente sob demanda em vez de pré-carregar todas elas |
| **Orquestração**         | `Agent`, `Skill`, `AskUserQuestion`, `TaskCreate`, `TaskUpdate` | Gerar subagentes, invocar skills, perguntar ao usuário, rastrear tarefas                     |

Além das ferramentas integradas, você pode:

* **Conectar serviços externos** com [servidores MCP](/docs/pt/agent-sdk/mcp) (bancos de dados, navegadores, APIs)
* **Definir ferramentas personalizadas** com [manipuladores de ferramentas personalizadas](/docs/pt/agent-sdk/custom-tools)
* **Carregar skills do projeto** via [fontes de configuração](/docs/pt/agent-sdk/claude-code-features) para fluxos de trabalho reutilizáveis

<h3 id="tool-permissions">
  Permissões de ferramentas
</h3>

Claude determina quais ferramentas chamar com base na tarefa, mas você controla se essas chamadas podem ser executadas. Você pode aprovar automaticamente ferramentas específicas, bloquear outras completamente ou exigir aprovação para tudo. Três opções funcionam juntas para determinar o que é executado:

* **`allowed_tools` / `allowedTools`** aprova automaticamente ferramentas listadas. Um agente somente leitura com `["Read", "Glob", "Grep"]` em sua lista de ferramentas permitidas executa essas ferramentas sem avisar. Ferramentas não listadas ainda estão disponíveis, mas exigem permissão.
* **`disallowed_tools` / `disallowedTools`** bloqueia ferramentas listadas, independentemente de outras configurações. Veja [Permissões](/docs/pt/agent-sdk/permissions) para a ordem em que as regras são verificadas antes de uma ferramenta ser executada.
* **`permission_mode` / `permissionMode`** controla o que acontece com ferramentas que não são cobertas por regras de permissão ou negação. Veja [Modo de permissão](#permission-mode) para os modos disponíveis.

Você também pode escopo ferramentas individuais com regras como `"Bash(npm *)"` para permitir apenas comandos específicos. Veja [Permissões](/docs/pt/agent-sdk/permissions) para a sintaxe completa de regras.

Quando uma ferramenta é negada, Claude recebe uma mensagem de rejeição como resultado da ferramenta e normalmente tenta uma abordagem diferente ou relata que não pôde prosseguir.

<h3 id="parallel-tool-execution">
  Execução paralela de ferramentas
</h3>

Quando Claude solicita múltiplas chamadas de ferramentas em uma única volta, ambos os SDKs podem executá-las concorrentemente ou sequencialmente dependendo da ferramenta. Ferramentas somente leitura (como `Read`, `Glob`, `Grep` e ferramentas MCP marcadas como somente leitura) podem ser executadas concorrentemente. Ferramentas que modificam estado (como `Edit`, `Write` e `Bash`) são executadas sequencialmente para evitar conflitos.

Ferramentas personalizadas padrão para execução sequencial. Para habilitar execução paralela para uma ferramenta personalizada, defina `readOnlyHint` em suas anotações. Ambos os SDKs [TypeScript](/docs/pt/agent-sdk/typescript#tool) e [Python](/docs/pt/agent-sdk/python#tool) usam este nome de campo do SDK MCP.

<h2 id="control-how-the-loop-runs">
  Controlar como o loop é executado
</h2>

Você pode limitar quantas voltas o loop leva, quanto custa, quão profundamente Claude raciocina e se as ferramentas exigem aprovação antes de serem executadas. Todas essas são campos em [`ClaudeAgentOptions`](/docs/pt/agent-sdk/python#claudeagentoptions) (Python) / [`Options`](/docs/pt/agent-sdk/typescript#options) (TypeScript).

<h3 id="turns-and-budget">
  Voltas e orçamento
</h3>

| Opção                                          | O que controla                          | Padrão     |
| :--------------------------------------------- | :-------------------------------------- | :--------- |
| Max turns (`max_turns` / `maxTurns`)           | Máximo de rodadas de uso de ferramentas | Sem limite |
| Max budget (`max_budget_usd` / `maxBudgetUsd`) | Custo máximo antes de parar             | Sem limite |

Quando qualquer limite é atingido, o SDK retorna uma `ResultMessage` com um subtipo de erro correspondente (`error_max_turns` ou `error_max_budget_usd`). Veja [Lidar com o resultado](#handle-the-result) para como verificar esses subtipos e [`ClaudeAgentOptions`](/docs/pt/agent-sdk/python#claudeagentoptions) / [`Options`](/docs/pt/agent-sdk/typescript#options) para sintaxe.

Com [entrada em streaming](/docs/pt/agent-sdk/streaming-vs-single-mode), uma mensagem que você envia enquanto uma volta ainda está em execução fica enfileirada quando essa volta termina no limite de máximo de voltas, e ela inicia sua própria volta com seu próprio limite de máximo de voltas. Antes da v2.1.205, uma mensagem que chegava na iteração final da volta poderia ser consumida na volta final e perdida sem nunca chegar ao modelo.

<h3 id="effort-level">
  Nível de esforço
</h3>

A opção `effort` controla quanto raciocínio Claude aplica. Níveis de esforço mais baixos usam menos tokens por volta e reduzem custo. Nem todos os modelos suportam o parâmetro de esforço. Veja [Effort](https://platform.claude.com/docs/en/build-with-claude/effort) para quais modelos o suportam.

| Nível      | Comportamento                        | Bom para                                                                       |
| :--------- | :----------------------------------- | :----------------------------------------------------------------------------- |
| `"low"`    | Raciocínio mínimo, respostas rápidas | Buscas de arquivo, listagem de diretórios                                      |
| `"medium"` | Raciocínio equilibrado               | Edições rotineiras, tarefas padrão                                             |
| `"high"`   | Análise completa                     | Refatorações, depuração                                                        |
| `"xhigh"`  | Profundidade de raciocínio estendida | Tarefas de codificação e agentes; recomendado em Fable 5, Opus 4.7+ e Sonnet 5 |
| `"max"`    | Profundidade máxima de raciocínio    | Problemas multi-etapas que exigem análise profunda                             |

Se você não definir `effort`, ambos os SDKs deixam o parâmetro indefinido e deferem para o comportamento padrão do modelo.

<Note>
  `effort` negocia latência e custo de token por profundidade de raciocínio dentro de cada resposta. [Extended thinking](https://platform.claude.com/docs/en/build-with-claude/extended-thinking) é um recurso separado que produz blocos de cadeia de pensamento visíveis na saída. Eles são independentes: você pode definir `effort: "low"` com extended thinking habilitado, ou `effort: "max"` sem ele.
</Note>

Use esforço mais baixo para agentes fazendo tarefas simples e bem definidas (como listar arquivos ou executar um único grep) para reduzir custo e latência. Defina `effort` nas opções de nível superior `query()` para toda a sessão, ou por subagente com o campo `effort` em [`AgentDefinition`](/docs/pt/agent-sdk/subagents#agentdefinition-configuration) para sobrescrever o nível de sessão.

<h3 id="permission-mode">
  Modo de permissão
</h3>

A opção de modo de permissão (`permission_mode` em Python, `permissionMode` em TypeScript) controla se o agente pede aprovação antes de usar ferramentas:

| Modo                  | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"default"`           | Ferramentas não cobertas por regras de permissão acionam seu callback de aprovação; sem callback significa negar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `"acceptEdits"`       | Aprova automaticamente edições de arquivo e comandos comuns do sistema de arquivos (`mkdir`, `touch`, `mv`, `cp`, etc.); outros comandos Bash seguem regras padrão                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `"plan"`              | Claude explora e planeja sem editar seus arquivos de origem; edições de arquivo nunca são aprovadas automaticamente e solicitam através de seu callback `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `"dontAsk"`           | Nunca avisa. Ferramentas pré-aprovadas por [regras de permissão](/docs/pt/settings#permission-settings) são executadas, tudo mais é negado. `AskUserQuestion`, ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas MCP marcadas [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool) são negadas mesmo se você as permitiu                                                                                                                                                                                           |
| `"auto"`              | Usa um classificador de modelo para aprovar ou negar cada chamada de ferramenta. Veja [Modo Auto](/docs/pt/permission-modes#eliminate-prompts-with-auto-mode) para disponibilidade e comportamento                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `"bypassPermissions"` | Executa todas as ferramentas permitidas sem avisar, exceto ferramentas correspondidas por uma regra [`ask`](/docs/pt/settings#permission-settings) explícita, ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) e ferramentas que exigem interação do usuário; veja [Como as permissões são avaliadas](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) para a ordem de precedência. Não pode ser usado ao executar como root em Unix. Use apenas em ambientes isolados onde as ações do agente não podem afetar sistemas que você se importa |

Para aplicações interativas, use `"default"` com um callback de aprovação de ferramenta para exibir avisos de aprovação. Para agentes autônomos em uma máquina de desenvolvimento, `"acceptEdits"` aprova automaticamente edições de arquivo e comandos comuns do sistema de arquivos (`mkdir`, `touch`, `mv`, `cp`, etc.) enquanto ainda controla outros comandos `Bash` atrás de regras de permissão. Reserve `"bypassPermissions"` para CI, contêineres ou outros ambientes isolados. Veja [Permissões](/docs/pt/agent-sdk/permissions) para detalhes completos.

<h3 id="model">
  Modelo
</h3>

Se você não definir `model`, o SDK usa o padrão do Claude Code, que depende do seu método de autenticação e assinatura. Defina explicitamente (por exemplo, `model="claude-sonnet-5"`) para fixar um modelo específico ou usar um modelo menor para agentes mais rápidos e baratos. Veja [modelos](https://platform.claude.com/docs/en/about-claude/models) para IDs disponíveis.

<h2 id="the-context-window">
  A janela de contexto
</h2>

A janela de contexto é a quantidade total de informações disponíveis para Claude durante uma sessão. Ela não é redefinida entre voltas dentro de uma sessão. Tudo se acumula: o prompt do sistema, definições de ferramentas, histórico de conversa, entradas de ferramentas e saídas de ferramentas. Conteúdo que permanece igual entre voltas (prompt do sistema, definições de ferramentas, CLAUDE.md) é automaticamente [prompt cached](https://platform.claude.com/docs/pt/build-with-claude/prompt-caching), o que reduz custo e latência para prefixos repetidos.

<h3 id="what-consumes-context">
  O que consome contexto
</h3>

Aqui está como cada componente afeta o contexto no SDK:

| Fonte                         | Quando carrega                                                               | Impacto                                                                                                                                                                                                                                                                                                                                                                           |
| :---------------------------- | :--------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Prompt do sistema**         | Cada requisição                                                              | Custo fixo pequeno, sempre presente                                                                                                                                                                                                                                                                                                                                               |
| **Arquivos CLAUDE.md**        | Início da sessão, via [`settingSources`](/docs/pt/agent-sdk/claude-code-features) | Conteúdo completo em cada requisição (mas prompt-cached, então apenas a primeira requisição paga o custo completo)                                                                                                                                                                                                                                                                |
| **Definições de ferramentas** | Cada requisição; esquemas MCP adiados por padrão                             | Esquemas de ferramentas integradas carregam a cada requisição. [Tool search](/docs/pt/agent-sdk/mcp#mcp-tool-search) adia esquemas de ferramentas MCP por padrão, voltando ao carregamento antecipado no Vertex AI ou em um `ANTHROPIC_BASE_URL` não de primeira parte. Veja [Configurar busca de ferramentas](/docs/pt/agent-sdk/tool-search#configure-tool-search) para a matriz completa |
| **Histórico de conversa**     | Acumula entre voltas                                                         | Cresce a cada volta: prompts, respostas, entradas de ferramentas, saídas de ferramentas                                                                                                                                                                                                                                                                                           |
| **Descrições de skills**      | Início da sessão, via fontes de configuração                                 | Resumos curtos; conteúdo completo carrega apenas quando invocado                                                                                                                                                                                                                                                                                                                  |

Grandes saídas de ferramentas consomem contexto significativo. Ler um arquivo grande ou executar um comando com saída detalhada pode usar milhares de tokens em uma única volta. O contexto se acumula entre voltas, então sessões mais longas com muitas chamadas de ferramentas acumulam significativamente mais contexto do que as curtas.

<h3 id="automatic-compaction">
  Compactação automática
</h3>

Quando a janela de contexto se aproxima de seu limite, o SDK compacta automaticamente a conversa: resume o histórico mais antigo para liberar espaço, mantendo suas trocas mais recentes e decisões-chave intactas. O SDK emite uma mensagem com `type: "system"` e `subtype: "compact_boundary"` no fluxo quando isso acontece (em Python isso é uma `SystemMessage`; em TypeScript é um tipo separado `SDKCompactBoundaryMessage`).

A compactação substitui mensagens mais antigas por um resumo, então instruções específicas do início da conversa podem não ser preservadas. Regras persistentes pertencem a CLAUDE.md (carregado via [`settingSources`](/docs/pt/agent-sdk/claude-code-features)) em vez do prompt inicial, porque o conteúdo CLAUDE.md é reinjetado em cada requisição.

Você pode personalizar o comportamento de compactação de várias maneiras:

* **Instruções de sumarização em CLAUDE.md:** O compactador lê seu CLAUDE.md como qualquer outro contexto, então você pode incluir uma seção dizendo a ele o que preservar ao resumir. O cabeçalho da seção é livre (não uma string mágica); o compactador corresponde à intenção.
* **Hook `PreCompact`:** Execute lógica personalizada antes da compactação ocorrer, por exemplo para arquivar a transcrição completa. O hook recebe um campo `trigger` (`manual` ou `auto`). Veja [hooks](/docs/pt/agent-sdk/hooks).
* **Compactação manual:** Envie `/compact` como uma string de prompt para acionar compactação sob demanda. Comandos enviados desta forma são entradas SDK, não atalhos apenas CLI. Veja [slash commands no SDK](/docs/pt/agent-sdk/slash-commands).

<Accordion title="Exemplo: Instruções de sumarização em CLAUDE.md">
  Adicione uma seção ao CLAUDE.md do seu projeto dizendo ao compactador o que preservar. O nome do cabeçalho não é especial; use qualquer rótulo claro.

  ```markdown CLAUDE.md theme={null}
  # Summary instructions

  When summarizing this conversation, always preserve:
  - The current task objective and acceptance criteria
  - File paths that have been read or modified
  - Test results and error messages
  - Decisions made and the reasoning behind them
  ```
</Accordion>

<h3 id="keep-context-efficient">
  Manter contexto eficiente
</h3>

Algumas estratégias para agentes de longa duração:

* **Use subagentes para subtarefas.** Cada subagente começa com uma conversa fresca (sem histórico de mensagens anterior, embora carregue seu próprio prompt do sistema e contexto de nível de projeto como CLAUDE.md). Ele não vê as voltas do pai, e apenas sua resposta final retorna ao pai como resultado de ferramenta. O contexto do agente principal cresce por esse resumo, não pela transcrição completa da subtarefa. Veja [O que subagentes herdam](/docs/pt/agent-sdk/subagents#what-subagents-inherit) para detalhes.
* **Seja seletivo com ferramentas.** Cada definição de ferramenta ocupa espaço de contexto. Use o campo `tools` em [`AgentDefinition`](/docs/pt/agent-sdk/subagents#agentdefinition-configuration) para escopo subagentes ao conjunto mínimo que precisam.
* **Observe custos de servidor MCP.** [MCP tool search](/docs/pt/agent-sdk/mcp#mcp-tool-search) adia esquemas de ferramentas MCP por padrão e carrega-os sob demanda. Quando a busca de ferramentas está desativada, no Vertex AI, ou atrás de um `ANTHROPIC_BASE_URL` não de primeira parte, cada servidor MCP adiciona todos os seus esquemas de ferramentas a cada requisição, então alguns servidores com muitas ferramentas podem consumir contexto significativo antes do agente fazer qualquer trabalho.
* **Use esforço mais baixo para tarefas rotineiras.** Defina [esforço](#effort-level) para `"low"` para agentes que apenas precisam ler arquivos ou listar diretórios. Isso reduz uso de tokens e custo.

Para um detalhamento detalhado dos custos de contexto por recurso, veja [Entender custos de contexto](/docs/pt/features-overview#understand-context-costs).

<h2 id="sessions-and-continuity">
  Sessões e continuidade
</h2>

Cada interação com o SDK cria ou continua uma sessão. Capture o ID da sessão de `ResultMessage.session_id` (disponível em ambos os SDKs) para retomar mais tarde. O SDK TypeScript também o expõe como um campo direto na `SystemMessage` init; em Python está aninhado em `SystemMessage.data`.

Quando você retoma, o contexto completo de voltas anteriores é restaurado: arquivos que foram lidos, análise que foi realizada e ações que foram tomadas. Você também pode bifurcar uma sessão para ramificar em uma abordagem diferente sem modificar a original.

Veja [Gerenciamento de sessão](/docs/pt/agent-sdk/sessions) para o guia completo em padrões de retomada, continuação e bifurcação.

<Note>
  Em Python, `ClaudeSDKClient` lida com IDs de sessão automaticamente em múltiplas chamadas. Veja a [referência do SDK Python](/docs/pt/agent-sdk/python#choosing-between-query-and-claudesdkclient) para detalhes.
</Note>

<h2 id="handle-the-result">
  Lidar com o resultado
</h2>

Quando o loop termina, a `ResultMessage` informa o que aconteceu e fornece a saída. O campo `subtype` (disponível em ambos os SDKs) é a forma principal de verificar o estado de término.

| Subtipo de resultado                  | O que aconteceu                                                                                                                                                                                                              | Campo `result` disponível? |
| :------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------: |
| `success`                             | Claude terminou a tarefa normalmente                                                                                                                                                                                         |             Sim            |
| `error_max_turns`                     | Atingiu o limite de `maxTurns` antes de terminar                                                                                                                                                                             |             Não            |
| `error_max_budget_usd`                | Atingiu o limite de `maxBudgetUsd` antes de terminar                                                                                                                                                                         |             Não            |
| `error_during_execution`              | Um erro interrompeu o loop (por exemplo, falha de API ou requisição cancelada)                                                                                                                                               |             Não            |
| `error_max_structured_output_retries` | Nenhuma saída estruturada válida foi produzida dentro do limite de tentativas configurado: todas as tentativas falharam na validação, ou um fallback de modelo retratou a saída concluída sem nenhuma tentativa bem-sucedida |             Não            |

O campo `result` (a saída de texto final) está presente apenas na variante `success`, então sempre verifique o subtipo antes de lê-lo. Todos os subtipos de resultado carregam `total_cost_usd`, `usage`, `num_turns` e `session_id` para que você possa rastrear custo e retomar mesmo após erros. Em Python, `total_cost_usd` e `usage` são digitados como opcionais e podem ser `None` em alguns caminhos de erro, então proteja antes de formatá-los. Veja [Rastreamento de custos e uso](/docs/pt/agent-sdk/cost-tracking) para detalhes sobre interpretação dos campos `usage`.

<Note>
  Quando uma consulta termina em um resultado de erro:

  * Uma chamada `query()` de uma única vez produz a mensagem de resultado final e depois lança um erro que inclui o texto de falha, como `Reached maximum number of turns`. O lançamento é intencional — envolva o loop em um bloco try se seu código precisar continuar após ele. O processo Claude Code subjacente também sai com um código diferente de zero.
  * Uma sessão de entrada de streaming permanece ativa e você pode continuar enviando mensagens.
</Note>

O resultado também inclui um campo `stop_reason` (`string | null` em TypeScript, `str | None` em Python) indicando por que o modelo parou de gerar em sua volta final. Valores comuns são `end_turn` (modelo terminou normalmente), `max_tokens` (atingiu o limite de token de saída) e `refusal` (o modelo recusou a requisição). Em subtipos de resultado de erro, `stop_reason` carrega o valor da última resposta do assistente antes do loop terminar. Para detectar recusas, verifique `stop_reason === "refusal"` (TypeScript) ou `stop_reason == "refusal"` (Python). Veja [`SDKResultMessage`](/docs/pt/agent-sdk/typescript#sdkresultmessage) (TypeScript) ou [`ResultMessage`](/docs/pt/agent-sdk/python#resultmessage) (Python) para o tipo completo.

<h2 id="hooks">
  Hooks
</h2>

[Hooks](/docs/pt/agent-sdk/hooks) são callbacks que disparam em pontos específicos do loop: antes de uma ferramenta ser executada, depois que retorna, quando o agente termina e assim por diante. Alguns hooks comumente usados são:

| Hook                             | Quando dispara                           | Usos comuns                                        |
| :------------------------------- | :--------------------------------------- | :------------------------------------------------- |
| `PreToolUse`                     | Antes de uma ferramenta ser executada    | Validar entradas, bloquear comandos perigosos      |
| `PostToolUse`                    | Depois que uma ferramenta retorna        | Auditar saídas, acionar efeitos colaterais         |
| `UserPromptSubmit`               | Quando um prompt é enviado               | Injetar contexto adicional em prompts              |
| `Stop`                           | Quando o agente termina                  | Validar o resultado, salvar estado da sessão       |
| `SubagentStart` / `SubagentStop` | Quando um subagente é gerado ou completa | Rastrear e agregar resultados de tarefas paralelas |
| `PreCompact`                     | Antes da compactação de contexto         | Arquivar transcrição completa antes de resumir     |

Hooks são executados no processo de sua aplicação, não dentro da janela de contexto do agente, então não consomem contexto. Hooks também podem interromper o loop: um hook `PreToolUse` que rejeita uma chamada de ferramenta impede sua execução, e Claude recebe a mensagem de rejeição em vez disso.

Ambos os SDKs suportam todos os eventos acima. O SDK TypeScript inclui eventos adicionais que Python ainda não suporta. Veja [Controlar execução com hooks](/docs/pt/agent-sdk/hooks) para a lista completa de eventos, disponibilidade por SDK e a API de callback completa.

<h2 id="put-it-all-together">
  Juntar tudo
</h2>

Este exemplo combina os conceitos-chave desta página em um único agente que corrige testes falhando. Ele configura o agente com ferramentas permitidas (pré-aprovadas para que o agente seja executado autonomamente), configurações de projeto e limites de segurança em voltas e esforço de raciocínio. Conforme o loop é executado, ele captura o ID da sessão para possível retomada, lida com o resultado final e imprime o custo total.

Como uma única chamada `query()` levanta uma exceção após produzir um resultado de erro, o loop é envolvido em um bloco try para que o script saia de forma limpa quando um limite é atingido.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def run_agent():
      session_id = None

      try:
          async for message in query(
              prompt="Find and fix the bug causing test failures in the auth module",
              options=ClaudeAgentOptions(
                  allowed_tools=[
                      "Read",
                      "Edit",
                      "Bash",
                      "Glob",
                      "Grep",
                  ],  # Listing tools here auto-approves them (no prompting)
                  setting_sources=[
                      "project"
                  ],  # Load CLAUDE.md, skills, hooks from current directory
                  max_turns=30,  # Prevent runaway sessions
                  effort="high",  # Thorough reasoning for complex debugging
              ),
          ):
              # Handle the final result
              if isinstance(message, ResultMessage):
                  session_id = message.session_id  # Save for potential resumption

                  if message.subtype == "success":
                      print(f"Done: {message.result}")
                  elif message.subtype == "error_max_turns":
                      # Agent ran out of turns. Resume with a higher limit.
                      print(f"Hit turn limit. Resume session {session_id} to continue.")
                  elif message.subtype == "error_max_budget_usd":
                      print("Hit budget limit.")
                  else:
                      print(f"Stopped: {message.subtype}")
                  if message.total_cost_usd is not None:
                      print(f"Cost: ${message.total_cost_usd:.4f}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branches above have
          # already run; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(run_agent())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  try {
    for await (const message of query({
      prompt: "Find and fix the bug causing test failures in the auth module",
      options: {
        allowedTools: ["Read", "Edit", "Bash", "Glob", "Grep"], // Listing tools here auto-approves them (no prompting)
        settingSources: ["project"], // Load CLAUDE.md, skills, hooks from current directory
        maxTurns: 30, // Prevent runaway sessions
        effort: "high" // Thorough reasoning for complex debugging
      }
    })) {
      // Save the session ID to resume later if needed
      if (message.type === "system" && message.subtype === "init") {
        sessionId = message.session_id;
      }

      // Handle the final result
      if (message.type === "result") {
        if (message.subtype === "success") {
          console.log(`Done: ${message.result}`);
        } else if (message.subtype === "error_max_turns") {
          // Agent ran out of turns. Resume with a higher limit.
          console.log(`Hit turn limit. Resume session ${sessionId} to continue.`);
        } else if (message.subtype === "error_max_budget_usd") {
          console.log("Hit budget limit.");
        } else {
          console.log(`Stopped: ${message.subtype}`);
        }
        console.log(`Cost: $${message.total_cost_usd.toFixed(4)}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branches above have
    // already run; connection or process failures yield no result message.
    console.log(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Próximos passos
</h2>

Agora que você entende o loop, aqui está para onde ir dependendo do que você está construindo:

* **Ainda não executou um agente?** Comece com o [quickstart](/docs/pt/agent-sdk/quickstart) para obter o SDK instalado e ver um exemplo completo sendo executado de ponta a ponta.
* **Pronto para conectar ao seu projeto?** [Carregue CLAUDE.md, skills e hooks do sistema de arquivos](/docs/pt/agent-sdk/claude-code-features) para que o agente siga suas convenções de projeto automaticamente.
* **Construindo uma UI interativa?** Habilite [streaming](/docs/pt/agent-sdk/streaming-output) para mostrar texto ao vivo e chamadas de ferramentas conforme o loop é executado.
* **Precisa de controle mais apertado sobre o que o agente pode fazer?** Bloqueie o acesso a ferramentas com [permissões](/docs/pt/agent-sdk/permissions) e use [hooks](/docs/pt/agent-sdk/hooks) para auditar, bloquear ou transformar chamadas de ferramentas antes de serem executadas.
* **Executando tarefas longas ou caras?** Descarregue trabalho isolado para [subagentes](/docs/pt/agent-sdk/subagents) para manter seu contexto principal enxuto.

Para a imagem conceitual mais ampla do loop agente (não específica do SDK), veja [Como o Claude Code funciona](/docs/pt/how-claude-code-works). Para um guia prático sobre como projetar loops no Claude Code, desde loops baseados em turnos até loops baseados em objetivos e loops proativos, veja [Loop engineering: getting started with loops](https://claude.com/blog/getting-started-with-loops) no blog.
