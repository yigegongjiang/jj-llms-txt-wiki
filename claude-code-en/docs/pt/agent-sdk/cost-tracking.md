> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rastrear custo e uso

> Aprenda como rastrear o uso de tokens, estimar custos e configurar cache de prompt com o Claude Agent SDK.

O Claude Agent SDK fornece informações detalhadas de uso de tokens para cada interação com Claude. Este guia explica como rastrear adequadamente o uso e entender o relatório de custos, especialmente ao lidar com usos de ferramentas paralelas e conversas em múltiplas etapas.

Para documentação completa da API, consulte a [referência do SDK TypeScript](/docs/pt/agent-sdk/typescript) e a [referência do SDK Python](/docs/pt/agent-sdk/python).

<Warning>
  Os campos `total_cost_usd` e `costUSD` são estimativas do lado do cliente, não dados de faturamento autoritários. O SDK os calcula localmente a partir de uma tabela de preços incluída no tempo de compilação, a menos que uma tabela [`modelPricing`](/docs/pt/settings-reference#modelpricing) esteja em vigor. Eles podem divergir do que você é realmente faturado quando:

  * os preços mudam
  * a versão do SDK instalada não reconhece um modelo
  * regras de faturamento se aplicam que o cliente não pode modelar

  Uma regra de faturamento que o SDK modela é [preço de residência de dados](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing). Quando a `usage` de uma resposta relata `inference_geo: "us"`, o SDK multiplica o preço de lista dos tokens dessa resposta por 1,1. Taxas por solicitação, como busca na web, não são multiplicadas. Requer TypeScript Agent SDK v0.3.239 ou posterior, ou Python Agent SDK v0.2.144 ou posterior.

  Use esses campos para insight de desenvolvimento e orçamento aproximado. Para faturamento autoritário, use a [API de Uso e Custo](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) ou a página de Uso no [Console Claude](https://platform.claude.com/usage). Não fature usuários finais ou dispare decisões financeiras a partir desses campos.
</Warning>

<h2 id="understand-token-usage">
  Entender o uso de tokens
</h2>

Os SDKs TypeScript e Python expõem os mesmos dados de uso com nomes de campos diferentes:

* **TypeScript** fornece detalhamentos de tokens por etapa em cada mensagem do assistente (`message.message.id`, `message.message.usage`), custo por modelo via `modelUsage` na mensagem de resultado, e um total cumulativo na mensagem de resultado.
* **Python** fornece detalhamentos de tokens por etapa em cada mensagem do assistente como `message.usage` e `message.message_id`, custo por modelo via `model_usage` na mensagem de resultado, e o total cumulativo na mensagem de resultado como `total_cost_usd`.

Ambos os SDKs usam o mesmo modelo de custo subjacente e expõem a mesma granularidade. A diferença está na nomenclatura dos campos e em onde o uso por etapa está aninhado.

O rastreamento de custos depende de entender como o SDK define o escopo dos dados de uso:

* **Chamada `query()`:** uma invocação da função `query()` do SDK. Uma única chamada pode envolver múltiplas etapas: Claude responde, usa ferramentas, obtém resultados e responde novamente. Cada chamada produz uma mensagem [`result`](/docs/pt/agent-sdk/typescript#sdkresultmessage) ao final, exceto no [modo de entrada em streaming](/docs/pt/agent-sdk/streaming-vs-single-mode), onde uma chamada `query()` carrega múltiplos turnos do usuário e cada turno emite sua própria mensagem `result`.
* **Etapa:** um único ciclo de solicitação/resposta dentro de uma chamada `query()`. Cada etapa produz mensagens do assistente com uso de tokens.
* **Sessão:** uma série de chamadas `query()` vinculadas por um ID de sessão através da opção `resume`. Uma chamada retomada relata o gasto total da sessão, não apenas o da própria chamada. Veja [Acumular custos em múltiplas chamadas](#accumulate-costs-across-multiple-calls) para saber como os totais se transferem.

O diagrama a seguir mostra o fluxo de mensagens de uma única chamada `query()`, com uso de tokens relatado em cada etapa e a estimativa cumulativa ao final:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" className="dark:hidden" alt="Diagrama mostrando uma query produzindo duas etapas de mensagens. A Etapa 1 tem quatro mensagens do assistente compartilhando o mesmo ID e uso (contar uma vez), a Etapa 2 tem uma mensagem do assistente com um novo ID, e a mensagem de resultado final mostra o total_cost_usd estimado." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/message-usage-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=8ea95085abc0a6b7f55ecef498bd4d14" className="hidden dark:block" alt="Diagrama mostrando uma query produzindo duas etapas de mensagens. A Etapa 1 tem quatro mensagens do assistente compartilhando o mesmo ID e uso (contar uma vez), a Etapa 2 tem uma mensagem do assistente com um novo ID, e a mensagem de resultado final mostra o total_cost_usd estimado." width="760" height="520" data-path="images/agent-sdk/message-usage-flow-dark.svg" />

<Steps>
  <Step title="Cada etapa produz mensagens do assistente">
    Quando Claude responde, ele envia uma ou mais mensagens do assistente. Em TypeScript, cada mensagem do assistente contém uma `BetaMessage` aninhada (acessada via `message.message`) com um `id` e um objeto [`usage`](https://platform.claude.com/docs/en/api/messages) com contagens de tokens (`input_tokens`, `output_tokens`). Em Python, a classe de dados `AssistantMessage` expõe os mesmos dados diretamente via `message.usage` e `message.message_id`. Quando Claude usa múltiplas ferramentas em um turno, todas as mensagens nesse turno compartilham o mesmo ID, então deduplicar por ID para evitar contagem dupla.
  </Step>

  <Step title="A mensagem de resultado fornece a estimativa cumulativa">
    Quando a chamada `query()` é concluída, o SDK emite uma mensagem de resultado com `total_cost_usd` e `usage` cumulativo, tipado como [`SDKResultMessage`](/docs/pt/agent-sdk/typescript#sdkresultmessage) em TypeScript e [`ResultMessage`](/docs/pt/agent-sdk/python#resultmessage) em Python. Se você só precisar da estimativa total, pode ignorar o uso por etapa e ler este único valor.

    Se você fizer múltiplas chamadas `query()` independentes, cada resultado reflete apenas o custo dessa chamada individual. Uma chamada que retoma uma sessão também conta o gasto anterior da sessão.

    No modo de entrada em streaming, cada turno emite sua própria mensagem de resultado. Veja [Rastrear custos no modo de entrada em streaming](#track-costs-in-streaming-input-mode) para saber como ler totais de chamadas nesse modo.
  </Step>
</Steps>

<h2 id="track-costs-in-streaming-input-mode">
  Rastrear custos no modo de entrada em streaming
</h2>

No [modo de entrada em streaming](/docs/pt/agent-sdk/streaming-vs-single-mode), uma chamada `query()` carrega múltiplos turnos do usuário e cada turno emite sua própria mensagem de resultado. Os campos de resultado diferem em escopo:

* **`usage`**: cobre apenas esse turno, e dentro dele apenas o loop principal do agente, não qualquer subagenteque ele executou.
* **`total_cost_usd` e `modelUsage`, ou `model_usage` em Python**: carregam o total acumulado para toda a chamada até agora, mais qualquer gasto restaurado quando a chamada retomou uma sessão.

Em uma chamada onde seu aplicativo nunca envia `/clear`, `/reset` ou `/new`, leia o resultado mais recente para totais de chamada em vez de somar entre resultados.

Os totais acumulados começam novamente cada vez que seu aplicativo envia um desses três comandos, e dentro de uma chamada `query()` nada mais os redefine. Três resultados importam para sua contabilidade:

* **O resultado do próprio turno `/clear`**: cobre apenas o que foi executado desde a redefinição, e carrega um novo `session_id`.
* **Cada resultado posterior**: continua contando a partir dessa redefinição.
* **O último resultado antes de cada `/clear`**: contém o total para os turnos desde a redefinição anterior.

Para totalizar toda a chamada, adicione o último resultado de antes de cada `/clear` ao resultado final da chamada. Todos os outros resultados, incluindo o do próprio turno `/clear`, são substituídos por um posterior.

Em TypeScript, o SDK também emite uma [`SDKConversationResetMessage`](/docs/pt/agent-sdk/typescript#sdkconversationresetmessage) em cada redefinição, para que você possa detectar redefinições do stream. Em Python, o SDK igualmente emite uma `ConversationResetMessage`. Antes da versão 0.2.137 do SDK Python, o iterador Python descartava essa mensagem, então nessas versões conte as redefinições você mesmo a partir dos turnos `/clear` que seu aplicativo envia.

`maxBudgetUsd` (TypeScript) ou `max_budget_usd` (Python) conta apenas o gasto da própria chamada: totais restaurados de uma sessão retomada não contam contra ele, e um `/clear` inicia o orçamento novamente.

<h2 id="get-the-total-cost-of-a-query">
  Obter o custo total de uma consulta
</h2>

A mensagem de resultado, digitada como [`SDKResultMessage`](/docs/pt/agent-sdk/typescript#sdkresultmessage) em TypeScript e [`ResultMessage`](/docs/pt/agent-sdk/python#resultmessage) em Python, marca o fim do loop do agente para uma chamada `query()`. Ela inclui `total_cost_usd`, o custo estimado cumulativo em todas as etapas dessa chamada. Uma chamada que retoma uma sessão também conta o gasto anterior da sessão. Duas ressalvas se aplicam quando você lê o valor:

* Em Python, o campo é digitado como opcional, portanto verifique se não é `None` antes de lê-lo.
* Os resultados de sucesso e erro carregam ambos, embora o resultado final de um [travamento de sessão](#recover-totals-after-a-session-crash) possa carregá-lo zerado.

No modo de entrada em streaming, leia os totais de chamadas conforme descrito em [Rastrear custos no modo de entrada em streaming](#track-costs-in-streaming-input-mode).

Os três campos de nível de resultado diferem no que contam quando o agente gera [subagentes](/docs/pt/agent-sdk/subagents). Use `modelUsage`, ou `model_usage` em Python, para contabilidade de tokens de toda a árvore; o campo `usage` subestima assim que o aninhamento ocorre.

| Campo                        | Atividade do subagente                                                                                                             |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Excluído. Conta apenas o loop do agente de nível superior, portanto os tokens consumidos dentro dos subagentes não são adicionados |
| `total_cost_usd`             | Incluído. Conta as solicitações de subagentes junto com o loop de nível superior                                                   |
| `modelUsage` / `model_usage` | Incluído. Conta as solicitações de subagentes junto com o loop de nível superior, dividido por modelo                              |

No [modo de entrada de mensagem única](/docs/pt/agent-sdk/streaming-vs-single-mode#single-message-input), quando subagentes em segundo plano ainda estão em execução no final da volta final, Claude Code aguarda por eles, até o limite descrito em [tarefas em segundo plano na saída](/docs/pt/headless#background-tasks-at-exit), antes de emitir o resultado. O `total_cost_usd`, `duration_api_ms` e `modelUsage` do resultado, ou `model_usage` em Python, incluem o trabalho realizado durante essa espera.

Os exemplos a seguir iteram sobre o fluxo de mensagens de uma chamada `query()` e imprimem o custo total quando a mensagem `result` chega:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({ prompt: "Summarize this project" })) {
      if (message.type === "result") {
        console.log(`Total cost: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, it still carried total_cost_usd and the
    // branch above has already run; connection or process failures yield
    // no result message.
    console.error(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      try:
          async for message in query(prompt="Summarize this project"):
              if isinstance(message, ResultMessage):
                  print(f"Total cost: ${message.total_cost_usd or 0}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the branch above has already run;
          # connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Para limitar quanto os subagentes podem adicionar a `total_cost_usd`, defina os [limites de profundidade, concorrência e gastos](/docs/pt/agent-sdk/subagents#cap-subagent-depth-concurrency-and-spend) na consulta.

<h2 id="track-per-step-and-per-model-usage">
  Rastrear uso por etapa e por modelo
</h2>

Os exemplos nesta seção usam nomes de campos TypeScript. Em Python, os campos equivalentes são [`AssistantMessage.usage`](/docs/pt/agent-sdk/python#assistantmessage) e `AssistantMessage.message_id` para uso por etapa, e [`ResultMessage.model_usage`](/docs/pt/agent-sdk/python#resultmessage) para detalhamentos por modelo.

<h3 id="track-per-step-usage">
  Rastrear uso por etapa
</h3>

Cada mensagem do assistente contém um `BetaMessage` aninhado (acessado via `message.message`) com um `id` e um objeto `usage` com contagens de tokens. Quando Claude usa ferramentas em paralelo, várias mensagens compartilham o mesmo `id` com dados de uso idênticos. Rastreie quais IDs você já contou e pule duplicatas para evitar totais inflacionados.

<Warning>
  Os valores por etapa desduplicados são precisos para tokens de entrada e cache. O `output_tokens` por etapa é um espaço reservado, portanto, [leia tokens de saída da mensagem de resultado](#read-output-tokens-from-the-result-message).
</Warning>

O exemplo a seguir acumula tokens de entrada em todas as etapas, contando cada ID de mensagem do loop principal único apenas uma vez e pulando mensagens de subagentos, e lê o total de saída da mensagem de resultado, que cobre o loop principal:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let resultOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant" && !message.parent_tool_use_id) {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
      }
    }
    if (message.type === "result") {
      // Per-step output_tokens is a placeholder; the result message
      // carries the accumulated output total.
      resultOutputTokens = message.usage.output_tokens;
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // input total below still reflects the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${resultOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  Detalhamento de uso por modelo
</h3>

A mensagem de resultado inclui [`modelUsage`](/docs/pt/agent-sdk/typescript#modelusage), um mapa de nome de modelo para contagens de tokens por modelo e custo. Isso é útil quando você executa vários modelos (por exemplo, Haiku para subagentos e Opus para o agente principal) e deseja ver para onde os tokens estão indo.

O `costBasis` de cada entrada diz qual tabela de preços precificou a solicitação mais recente desse modelo: `list` para preço de lista, `managed` para uma tabela [`modelPricing`](/docs/pt/settings-reference#modelpricing), ou `unknown` quando nenhum correspondeu ao ID do modelo. O campo requer Claude Code v2.1.246 ou posterior.

O exemplo a seguir executa uma consulta e imprime o custo e o detalhamento de tokens para cada modelo usado:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type !== "result") continue;

    for (const [modelName, usage] of Object.entries(message.modelUsage)) {
      console.log(`${modelName}: $${usage.costUSD.toFixed(4)}`);
      console.log(`  Input tokens: ${usage.inputTokens}`);
      console.log(`  Output tokens: ${usage.outputTokens}`);
      console.log(`  Cache read: ${usage.cacheReadInputTokens}`);
      console.log(`  Cache creation: ${usage.cacheCreationInputTokens}`);
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result. If the
  // failure was an error result, the per-model breakdown above has already
  // printed; connection or process failures yield no result message.
  console.error(`Session ended with an error: ${error}`);
}
```

<h2 id="accumulate-costs-across-multiple-calls">
  Acumular custos em múltiplas chamadas
</h2>

Cada chamada `query()` retorna `total_cost_usd` em seus resultados. Como você combina os valores depende se as chamadas compartilham uma sessão:

* **Chamadas independentes, sem opção `resume` ou `continue`**: cada resultado cobre apenas sua própria chamada, portanto, adicione os totais você mesmo, como os exemplos abaixo fazem.
* **Chamadas que retomam a mesma sessão**: Claude Code salva os totais da sessão em sua [transcrição](/docs/pt/sessions#where-transcripts-are-stored) quando o processo sai normalmente e os restaura quando uma chamada posterior retoma ou bifurca a sessão. Cada resultado já inclui o gasto anterior da sessão. Leia o resultado mais recente para o total da sessão; somar resultados conta duas vezes o gasto restaurado. Antes da v2.1.277, uma sessão que você retomou através do SDK ou `claude -p` iniciava seus totais em zero, portanto, cada resultado de chamada cobria apenas essa chamada.

No modo de entrada em streaming, leia o total de cada chamada conforme descrito em [Rastrear custos no modo de entrada em streaming](#track-costs-in-streaming-input-mode). Para uma chamada que terminou em uma falha, consulte [Recuperar totais após uma falha de sessão](#recover-totals-after-a-session-crash).

Os exemplos a seguir executam duas chamadas `query()` sequencialmente, adicionam o `total_cost_usd` de cada chamada a um total acumulado e imprimem tanto o custo por chamada quanto o custo combinado:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Track cumulative cost across multiple query() calls
  let totalSpend = 0;

  const prompts = [
    "Read the files in src/ and summarize the architecture",
    "List all exported functions in src/auth.ts"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({ prompt })) {
        if (message.type === "result") {
          totalSpend += message.total_cost_usd;
          console.log(`This call: $${message.total_cost_usd}`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, this call's cost was already counted;
      // connection or process failures yield no result message. Continue
      // with the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }

  console.log(`Total spend: $${totalSpend.toFixed(4)}`);
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ResultMessage
  import asyncio


  async def main():
      # Track cumulative cost across multiple query() calls
      total_spend = 0.0

      prompts = [
          "Read the files in src/ and summarize the architecture",
          "List all exported functions in src/auth.ts",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt):
                  if isinstance(message, ResultMessage):
                      cost = message.total_cost_usd or 0
                      total_spend += cost
                      print(f"This call: ${cost}")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. If
              # the failure was an error result, this call's cost was already
              # counted; connection or process failures yield no result message.
              # Continue with the next prompt.
              print(f"Call failed: {error}")

      print(f"Total spend: ${total_spend:.4f}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="handle-errors-caching-and-output-token-counts">
  Lidar com erros, cache e contagens de tokens de saída
</h2>

Para rastreamento preciso de custos, leve em conta a contagem de saída do espaço reservado em mensagens do assistente, os tokens que uma conversa falhada consumiu e o preço dos tokens de cache.

<h3 id="read-output-tokens-from-the-result-message">
  Ler tokens de saída da mensagem de resultado
</h3>

Claude Code constrói cada mensagem do assistente a partir do uso que a API relatou quando a resposta começou, portanto, o `output_tokens` da mensagem é apenas a contagem que a API havia relatado em `message_start`, antes da resposta ser gerada. Uma resposta da API pode produzir várias mensagens do assistente, e cada uma delas carrega esse mesmo espaço reservado.

A API relata a contagem de saída real no final da resposta, e Claude Code a adiciona à mensagem de resultado. Leia tokens de saída do `usage` do resultado, ou de `modelUsage` para uma divisão por modelo.

Para observar a contagem de saída de uma resposta crescer enquanto ela é transmitida, defina `includePartialMessages`, ou `include_partial_messages` em Python, e leia `usage` de cada evento de fluxo `message_delta`, digitado como [`SDKPartialAssistantMessage`](/docs/pt/agent-sdk/typescript#sdkpartialassistantmessage) em TypeScript e [`StreamEvent`](/docs/pt/agent-sdk/python#streamevent) em Python.

<h3 id="track-costs-on-failed-conversations">
  Rastrear custos em conversas falhadas
</h3>

Tanto as mensagens de resultado de sucesso quanto as de erro incluem `usage` e `total_cost_usd`; em Python, ambos os campos são digitados como opcionais, portanto, verifique se não são `None` antes de lê-los.

Se uma conversa falhar no meio do caminho, você ainda consumiu tokens até o ponto de falha. Leia dados de custo de cada mensagem de resultado, independentemente de seu `subtype` ser `success` ou um dos subtipos de erro. Em alguns resultados de erro, `usage` relata menos do que a chamada gastou:

* **`error_during_execution` após um [travamento de sessão](#recover-totals-after-a-session-crash)**: cada campo de custo pode ser zerado.
* **`error_max_budget_usd`**: `usage` deixa de fora a resposta que ultrapassou o orçamento, enquanto `total_cost_usd` e `modelUsage` a incluem.

Quando você tiver a escolha, contabilize a partir de `total_cost_usd` ou `modelUsage` em vez de `usage`.

<h3 id="recover-totals-after-a-session-crash">
  Recuperar totais após um travamento de sessão
</h3>

Quando o processo Claude Code trava, ele emite um resultado final `error_during_execution` e sai, tanto no modo de entrada única quanto no modo de entrada de transmissão. Esse resultado pode carregar `usage`, `total_cost_usd` e `modelUsage` zerados, portanto, recupere os totais da chamada a partir do que chegou antes dele. A etapa 1 recupera os totais completos sempre que existe um resultado anterior; o fallback na etapa 2 recupera apenas os tokens de entrada e cache do loop principal.

1. Use o resultado da rodada antes do travamento. No modo de entrada de transmissão, ele contém o total em execução descrito em [Rastrear custos no modo de entrada de transmissão](#track-costs-in-streaming-input-mode). Vá para a etapa 2 em vez disso quando esse resultado não puder ajudá-lo:
   * A chamada foi única, portanto, nenhum resultado anterior existe.
   * O travamento aconteceu na primeira rodada.
   * A rodada antes do travamento foi o `/clear` em si, portanto, seu resultado cobre apenas a redefinição.
2. Some o `usage` nas mensagens do assistente em vez disso, contando cada resposta da API uma vez, como o exemplo [Track per-step usage](#track-per-step-usage) faz. No modo de entrada única, some todas elas; no modo de entrada de transmissão, some as que chegaram após o último resultado. Isso fornece os tokens de entrada e cache do loop principal. O uso de subagentos não é recuperável dessa forma, e nem são tokens de saída ou custo em USD, porque [o `output_tokens` por etapa é um espaço reservado](#read-output-tokens-from-the-result-message).

<h3 id="track-cache-tokens">
  Rastrear tokens de cache
</h3>

O Agent SDK usa automaticamente [prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) para reduzir custos em conteúdo repetido. Você não precisa configurar o cache você mesmo. O objeto de uso inclui dois campos adicionais para rastreamento de cache:

* `cache_creation_input_tokens`: tokens usados para criar novas entradas de cache (cobrados a uma taxa mais alta do que tokens de entrada padrão).
* `cache_read_input_tokens`: tokens lidos de entradas de cache existentes (cobrados a uma taxa reduzida).

Rastreie esses separadamente de `input_tokens` para entender a economia de cache. Em TypeScript, esses campos são digitados no objeto [`Usage`](/docs/pt/agent-sdk/typescript#usage). Em Python, eles aparecem como chaves no dicionário [`ResultMessage.usage`](/docs/pt/agent-sdk/python#resultmessage) (por exemplo, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Estender o TTL do cache de prompt para uma hora
</h3>

Suas próprias rodadas caem no [bucket TTL da conversa principal](/docs/pt/prompt-caching#which-ttl-each-request-gets), junto com os auxiliares que Claude Code executa em linha com elas. As solicitações que Claude Code faz fora dessa conversa, como [subagentos](/docs/pt/agent-sdk/subagents), têm um [controle TTL separado](/docs/pt/prompt-caching#choose-the-ttl-yourself).

As entradas de cache para suas próprias rodadas usam um TTL padrão de 5 minutos quando você se autentica com uma chave de API ou executa no Amazon Bedrock, na Agent Platform do Google Cloud, no Microsoft Foundry ou [Claude Platform on AWS](/docs/pt/claude-platform-on-aws). Se sua carga de trabalho executa muitas sessões curtas contra o mesmo prompt do sistema e contexto com intervalos maiores que 5 minutos entre elas, o cache expira entre sessões e cada nova sessão paga o preço de entrada completo.

Para solicitar um TTL de 1 hora em gravações de cache, defina a variável de ambiente [`ENABLE_PROMPT_CACHING_1H`](/docs/pt/env-vars). Você pode exportá-la em seu ambiente de shell ou contêiner, ou passá-la através de `options.env`.

O exemplo a seguir habilita TTL de 1 hora para um agente em execução no Amazon Bedrock. Como define `CLAUDE_CODE_USE_BEDROCK`, requer credenciais AWS funcionais para [Amazon Bedrock](/docs/pt/amazon-bedrock); sem elas, a consulta falha.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import ClaudeAgentOptions, query
  import asyncio


  async def main():
      options = ClaudeAgentOptions(
          env={
              "CLAUDE_CODE_USE_BEDROCK": "1",
              "ENABLE_PROMPT_CACHING_1H": "1",
          },
      )

      async for message in query(prompt="Summarize this project", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    env: {
      ...process.env,
      CLAUDE_CODE_USE_BEDROCK: "1",
      ENABLE_PROMPT_CACHING_1H: "1",
    },
  };

  for await (const message of query({ prompt: "Summarize this project", options })) {
    console.log(message);
  }
  ```
</CodeGroup>

As gravações de cache com TTL de 1 hora são cobradas a uma taxa mais alta do que as gravações de 5 minutos, portanto, habilitar isso troca custo de gravação mais alto por mais leituras de cache. Consulte [preços de prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) para detalhes. Em uma assinatura Claude dentro do uso incluído do seu plano, você obtém o TTL de 1 hora em suas próprias rodadas e em algumas das solicitações auxiliares que Claude Code faz ao lado delas, sem definir essa variável, e Claude Code reduz essas rodadas para o TTL de 5 minutos assim que você está usando [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans).

`ENABLE_PROMPT_CACHING_1H` solicita o TTL de 1 hora em cada solicitação em ambos os buckets. Para escolher um TTL para cada bucket separadamente, use esses controles em vez disso. Cada um leva `5m` ou `1h` e tem precedência sobre `ENABLE_PROMPT_CACHING_1H`:

* Conversa principal: a variável de ambiente [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/pt/env-vars), ou a configuração [`promptCacheTtl`](/docs/pt/settings-reference#promptcachettl)
* Tudo mais: a variável de ambiente `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`, ou a configuração [`subagentPromptCacheTtl`](/docs/pt/settings-reference#subagentpromptcachettl)

Definir `promptCacheTtl` para `1h` mantém o cache de 1 hora na conversa principal enquanto você está usando créditos de uso. Para a ordem de precedência completa, consulte [escolha o TTL você mesmo](/docs/pt/prompt-caching#choose-the-ttl-yourself).

<h2 id="related-documentation">
  Documentação relacionada
</h2>

* [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript) - Documentação completa da API
* [Visão geral do SDK](/docs/pt/agent-sdk/overview) - Começando com o SDK
* [Permissões do SDK](/docs/pt/agent-sdk/permissions) - Gerenciando permissões de ferramentas
