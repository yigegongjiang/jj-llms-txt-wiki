> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rastrear custo e uso

> Aprenda como rastrear o uso de tokens, estimar custos e configurar prompt caching com o Claude Agent SDK.

O Claude Agent SDK fornece informações detalhadas de uso de tokens para cada interação com Claude. Este guia explica como rastrear adequadamente o uso e entender o relatório de custos, especialmente ao lidar com usos de ferramentas paralelas e conversas em múltiplas etapas.

Para documentação completa da API, consulte a [referência do SDK TypeScript](/docs/pt/agent-sdk/typescript) e a [referência do SDK Python](/docs/pt/agent-sdk/python).

<Warning>
  Os campos `total_cost_usd` e `costUSD` são estimativas do lado do cliente, não dados de faturamento autoritários. O SDK os calcula localmente a partir de uma tabela de preços incluída no momento da compilação, portanto, podem divergir do que você é realmente cobrado quando:

  * os preços mudam
  * a versão do SDK instalada não reconhece um modelo
  * regras de faturamento se aplicam que o cliente não consegue modelar

  Use esses campos para obter informações de desenvolvimento e orçamento aproximado. Para faturamento autoritário, use a [API de Uso e Custo](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) ou a página de Uso no [Console Claude](https://platform.claude.com/usage). Não fature usuários finais ou dispare decisões financeiras a partir desses campos.
</Warning>

<h2 id="understand-token-usage">
  Entender o uso de tokens
</h2>

Os SDKs TypeScript e Python expõem os mesmos dados de uso com nomes de campos diferentes:

* **TypeScript** fornece divisões de tokens por etapa em cada mensagem do assistente (`message.message.id`, `message.message.usage`), custo por modelo via `modelUsage` na mensagem de resultado, e um total cumulativo na mensagem de resultado.
* **Python** fornece divisões de tokens por etapa em cada mensagem do assistente (`message.usage`, `message.message_id`), custo por modelo via `model_usage` na mensagem de resultado, e o total acumulado na mensagem de resultado (`total_cost_usd` e dicionário `usage`).

Ambos os SDKs usam o mesmo modelo de custo subjacente e expõem a mesma granularidade. A diferença está na nomenclatura dos campos e onde o uso por etapa está aninhado.

O rastreamento de custos depende de entender como o SDK define o escopo dos dados de uso:

* **Chamada `query()`:** uma invocação da função `query()` do SDK. Uma única chamada pode envolver múltiplas etapas (Claude responde, usa ferramentas, obtém resultados, responde novamente). Cada chamada produz uma mensagem [`result`](/docs/pt/agent-sdk/typescript#sdkresultmessage) no final.
* **Etapa:** um único ciclo de solicitação/resposta dentro de uma chamada `query()`. Cada etapa produz mensagens do assistente com uso de tokens.
* **Sessão:** uma série de chamadas `query()` vinculadas por um ID de sessão (usando a opção `resume`). Cada chamada `query()` dentro de uma sessão relata seu próprio custo independentemente.

O diagrama a seguir mostra o fluxo de mensagens de uma única chamada `query()`, com uso de tokens relatado em cada etapa e a estimativa cumulativa no final:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" alt="Diagrama mostrando uma query produzindo duas etapas de mensagens. A Etapa 1 tem quatro mensagens do assistente compartilhando o mesmo ID e uso (contar uma vez), a Etapa 2 tem uma mensagem do assistente com um novo ID, e a mensagem de resultado final mostra o total_cost_usd estimado." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<Steps>
  <Step title="Cada etapa produz mensagens do assistente">
    Quando Claude responde, ele envia uma ou mais mensagens do assistente. Em TypeScript, cada mensagem do assistente contém uma `BetaMessage` aninhada (acessada via `message.message`) com um `id` e um objeto [`usage`](https://platform.claude.com/docs/en/api/messages) com contagens de tokens (`input_tokens`, `output_tokens`). Em Python, a classe de dados `AssistantMessage` expõe os mesmos dados diretamente via `message.usage` e `message.message_id`. Quando Claude usa múltiplas ferramentas em um turno, todas as mensagens nesse turno compartilham o mesmo ID, portanto, deduplicar por ID para evitar contagem dupla.
  </Step>

  <Step title="A mensagem de resultado fornece a estimativa cumulativa">
    Quando a chamada `query()` é concluída, o SDK emite uma mensagem de resultado com `total_cost_usd` e `usage` cumulativo. Isso está disponível tanto em TypeScript ([`SDKResultMessage`](/docs/pt/agent-sdk/typescript#sdkresultmessage)) quanto em Python ([`ResultMessage`](/docs/pt/agent-sdk/python#resultmessage)). Se você fizer múltiplas chamadas `query()` (por exemplo, em uma sessão multi-turno), cada resultado reflete apenas o custo dessa chamada individual. Se você só precisar do total estimado, pode ignorar o uso por etapa e ler este único valor.
  </Step>
</Steps>

<h2 id="get-the-total-cost-of-a-query">
  Obter o custo total de uma query
</h2>

A mensagem de resultado ([TypeScript](/docs/pt/agent-sdk/typescript#sdkresultmessage), [Python](/docs/pt/agent-sdk/python#resultmessage)) marca o final do loop do agente para uma chamada `query()`. Ela inclui `total_cost_usd`, o custo estimado cumulativo em todas as etapas dessa chamada. Isso funciona tanto para resultados de sucesso quanto de erro. Se você usar sessões para fazer múltiplas chamadas `query()`, cada resultado reflete apenas o custo dessa chamada individual.

Os três campos de nível de resultado diferem no que contam quando o agente gera [subagentes](/docs/pt/agent-sdk/subagents). Use `modelUsage`, ou `model_usage` em Python, para contabilidade de tokens de toda a árvore; o campo `usage` subestima assim que ocorre aninhamento.

| Campo                        | Atividade de subagente                                                                                                            |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Excluído. Conta apenas o loop do agente de nível superior, portanto os tokens consumidos dentro de subagentes não são adicionados |
| `total_cost_usd`             | Incluído. Conta solicitações de subagentes junto com o loop de nível superior                                                     |
| `modelUsage` / `model_usage` | Incluído. Conta solicitações de subagentes junto com o loop de nível superior, dividido por modelo                                |

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
          # failure was an error result, it still carried total_cost_usd and the
          # branch above has already run; connection or process failures yield
          # no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="track-per-step-and-per-model-usage">
  Rastrear uso por etapa e por modelo
</h2>

Os exemplos nesta seção usam nomes de campos TypeScript. Em Python, os campos equivalentes são [`AssistantMessage.usage`](/docs/pt/agent-sdk/python#assistantmessage) e `AssistantMessage.message_id` para uso por etapa, e [`ResultMessage.model_usage`](/docs/pt/agent-sdk/python#resultmessage) para divisões por modelo.

<h3 id="track-per-step-usage">
  Rastrear uso por etapa
</h3>

Cada mensagem do assistente contém uma `BetaMessage` aninhada (acessada via `message.message`) com um `id` e um objeto `usage` com contagens de tokens. Quando Claude usa ferramentas em paralelo, múltiplas mensagens compartilham o mesmo `id` com dados de uso idênticos. Rastreie quais IDs você já contou e pule duplicatas para evitar totais inflacionados.

<Warning>
  Chamadas de ferramentas paralelas produzem múltiplas mensagens do assistente cuja `BetaMessage` aninhada compartilha o mesmo `id` e uso idêntico. Sempre deduplicar por ID para obter contagens de tokens por etapa precisas.
</Warning>

O exemplo a seguir acumula tokens de entrada e saída em todas as etapas, contando cada ID de mensagem único apenas uma vez:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const seenIds = new Set<string>();
let totalInputTokens = 0;
let totalOutputTokens = 0;

try {
  for await (const message of query({ prompt: "Summarize this project" })) {
    if (message.type === "assistant") {
      const msgId = message.message.id;

      // Parallel tool calls share the same ID, only count once
      if (!seenIds.has(msgId)) {
        seenIds.add(msgId);
        totalInputTokens += message.message.usage.input_tokens;
        totalOutputTokens += message.message.usage.output_tokens;
      }
    }
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result, so the
  // totals below still reflect the steps that ran before the failure.
  console.error(`Session ended with an error: ${error}`);
}

console.log(`Steps: ${seenIds.size}`);
console.log(`Input tokens: ${totalInputTokens}`);
console.log(`Output tokens: ${totalOutputTokens}`);
```

<h3 id="break-down-usage-per-model">
  Dividir o uso por modelo
</h3>

A mensagem de resultado inclui [`modelUsage`](/docs/pt/agent-sdk/typescript#modelusage), um mapa de nome de modelo para contagens de tokens por modelo e custo. Isso é útil quando você executa múltiplos modelos (por exemplo, Haiku para subagentos e Opus para o agente principal) e deseja ver para onde os tokens estão indo.

O exemplo a seguir executa uma query e imprime o custo e a divisão de tokens para cada modelo usado:

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

Cada chamada `query()` retorna seu próprio `total_cost_usd`. O SDK não fornece um total no nível da sessão, portanto, se sua aplicação fizer múltiplas chamadas `query()` (por exemplo, em uma sessão multi-turno ou entre diferentes usuários), acumule os totais você mesmo.

Os exemplos a seguir executam duas chamadas `query()` sequencialmente, adicionam o `total_cost_usd` de cada chamada a um total em execução e imprimem tanto o custo por chamada quanto o custo combinado:

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

<h2 id="handle-errors-caching-and-token-discrepancies">
  Lidar com erros, caching e discrepâncias de tokens
</h2>

Para rastreamento de custos preciso, leve em conta conversas falhadas, preços de tokens em cache e inconsistências ocasionais de relatórios.

<h3 id="resolve-output-token-discrepancies">
  Resolver discrepâncias de tokens de saída
</h3>

Em casos raros, você pode observar valores diferentes de `output_tokens` para mensagens com o mesmo ID. Quando isso ocorre:

1. **Use o valor mais alto:** a mensagem final em um grupo normalmente contém o total preciso.
2. **Prefira a mensagem de resultado:** o `total_cost_usd` na mensagem de resultado reflete a estimativa acumulada do SDK em todas as etapas, portanto, é mais confiável do que somar valores por etapa você mesmo. Ainda é uma estimativa e pode diferir da sua conta real.
3. **Relate inconsistências:** abra problemas no [repositório GitHub Claude Code](https://github.com/anthropics/claude-code/issues).

<h3 id="track-costs-on-failed-conversations">
  Rastrear custos em conversas falhadas
</h3>

Tanto as mensagens de resultado de sucesso quanto de erro incluem `usage` e `total_cost_usd`. Se uma conversa falhar no meio do caminho, você ainda consumiu tokens até o ponto de falha. Sempre leia dados de custo da mensagem de resultado independentemente de seu `subtype`.

<h3 id="track-cache-tokens">
  Rastrear tokens em cache
</h3>

O Agent SDK usa automaticamente [prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) para reduzir custos em conteúdo repetido. Você não precisa configurar caching você mesmo. O objeto de uso inclui dois campos adicionais para rastreamento de cache:

* `cache_creation_input_tokens`: tokens usados para criar novas entradas de cache (cobrados a uma taxa mais alta do que tokens de entrada padrão).
* `cache_read_input_tokens`: tokens lidos de entradas de cache existentes (cobrados a uma taxa reduzida).

Rastreie esses separadamente de `input_tokens` para entender a economia de caching. Em TypeScript, esses campos são digitados no objeto [`Usage`](/docs/pt/agent-sdk/typescript#usage). Em Python, eles aparecem como chaves no dicionário [`ResultMessage.usage`](/docs/pt/agent-sdk/python#resultmessage) (por exemplo, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Estender o TTL do cache de prompt para uma hora
</h3>

As entradas de cache escritas pelo SDK usam um TTL de 5 minutos por padrão quando você se autentica com uma chave de API ou executa no Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry. Se sua carga de trabalho executa muitas sessões curtas contra o mesmo prompt do sistema e contexto com lacunas maiores que 5 minutos entre elas, o cache expira entre sessões e cada nova sessão paga o preço de entrada completo.

Para solicitar um TTL de 1 hora em escritas de cache, defina a variável de ambiente [`ENABLE_PROMPT_CACHING_1H`](/docs/pt/env-vars). Você pode exportá-la em seu ambiente de shell ou contêiner, ou passá-la através de `options.env`.

O exemplo a seguir habilita TTL de 1 hora para um agente executando no Amazon Bedrock:

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

Escritas de cache com TTL de 1 hora são cobradas a uma taxa mais alta do que escritas de 5 minutos, portanto, habilitar isso troca custo de escrita mais alto por mais leituras de cache. Consulte [preços de prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) para detalhes. Os usuários de assinatura Claude já recebem TTL de 1 hora automaticamente e não precisam definir essa variável.

<h2 id="related-documentation">
  Documentação relacionada
</h2>

* [Referência do SDK TypeScript](/docs/pt/agent-sdk/typescript) - Documentação completa da API
* [Visão geral do SDK](/docs/pt/agent-sdk/overview) - Começando com o SDK
* [Permissões do SDK](/docs/pt/agent-sdk/permissions) - Gerenciando permissões de ferramentas
