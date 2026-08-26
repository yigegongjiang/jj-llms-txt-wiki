> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rastrear costo y uso

> Aprenda a rastrear el uso de tokens, estimar costos y configurar el almacenamiento en caché de indicaciones con el SDK del Agente Claude.

El SDK del Agente Claude proporciona información detallada sobre el uso de tokens para cada interacción con Claude. Esta guía explica cómo rastrear adecuadamente el uso y comprender los informes de costos, especialmente cuando se trata de usos de herramientas paralelas y conversaciones de múltiples pasos.

Para la documentación completa de la API, consulte la [referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript) y la [referencia del SDK de Python](/docs/es/agent-sdk/python).

<Warning>
  Los campos `total_cost_usd` y `costUSD` son estimaciones del lado del cliente, no datos de facturación autorizados. El SDK los calcula localmente a partir de una tabla de precios incluida en el momento de la compilación, por lo que pueden desviarse de lo que realmente se le factura cuando:

  * los precios cambian
  * la versión del SDK instalada no reconoce un modelo
  * se aplican reglas de facturación que el cliente no puede modelar

  Utilice estos campos para obtener información de desarrollo y presupuestos aproximados. Para facturación autorizada, utilice la [API de Uso y Costo](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) o la página de Uso en la [Consola Claude](https://platform.claude.com/usage). No facture a los usuarios finales ni desencadene decisiones financieras a partir de estos campos.
</Warning>

<h2 id="understand-token-usage">
  Comprender el uso de tokens
</h2>

Los SDK de TypeScript y Python exponen los mismos datos de uso con nombres de campo diferentes:

* **TypeScript** proporciona desgloses de tokens por paso en cada mensaje del asistente (`message.message.id`, `message.message.usage`), costo por modelo a través de `modelUsage` en el mensaje de resultado, y un total acumulativo en el mensaje de resultado.
* **Python** proporciona desgloses de tokens por paso en cada mensaje del asistente (`message.usage`, `message.message_id`), costo por modelo a través de `model_usage` en el mensaje de resultado, y el total acumulado en el mensaje de resultado (`total_cost_usd` y diccionario `usage`).

Ambos SDK utilizan el mismo modelo de costo subyacente y exponen la misma granularidad. La diferencia está en la nomenclatura de campos y dónde se anida el uso por paso.

El rastreo de costos depende de comprender cómo el SDK delimita los datos de uso:

* **Llamada `query()`:** una invocación de la función `query()` del SDK. Una sola llamada puede implicar múltiples pasos (Claude responde, usa herramientas, obtiene resultados, responde nuevamente). Cada llamada produce un mensaje [`result`](/docs/es/agent-sdk/typescript#sdkresultmessage) al final.
* **Paso:** un ciclo único de solicitud/respuesta dentro de una llamada `query()`. Cada paso produce mensajes del asistente con uso de tokens.
* **Sesión:** una serie de llamadas `query()` vinculadas por un ID de sesión (usando la opción `resume`). Cada llamada `query()` dentro de una sesión reporta su propio costo de forma independiente.

El siguiente diagrama muestra el flujo de mensajes de una sola llamada `query()`, con el uso de tokens reportado en cada paso y la estimación acumulativa al final:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" alt="Diagrama que muestra una consulta que produce dos pasos de mensajes. El paso 1 tiene cuatro mensajes del asistente que comparten el mismo ID y uso (contar una vez), el paso 2 tiene un mensaje del asistente con un nuevo ID, y el mensaje de resultado final muestra el total_cost_usd estimado." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<Steps>
  <Step title="Cada paso produce mensajes del asistente">
    Cuando Claude responde, envía uno o más mensajes del asistente. En TypeScript, cada mensaje del asistente contiene un `BetaMessage` anidado (accesible a través de `message.message`) con un `id` y un objeto [`usage`](https://platform.claude.com/docs/en/api/messages) con conteos de tokens (`input_tokens`, `output_tokens`). En Python, la clase de datos `AssistantMessage` expone los mismos datos directamente a través de `message.usage` y `message.message_id`. Cuando Claude usa múltiples herramientas en un turno, todos los mensajes en ese turno comparten el mismo ID, así que deduplique por ID para evitar contar dos veces.
  </Step>

  <Step title="El mensaje de resultado proporciona la estimación acumulativa">
    Cuando se completa la llamada `query()`, el SDK emite un mensaje de resultado con `total_cost_usd` y `usage` acumulativo. Esto está disponible tanto en TypeScript ([`SDKResultMessage`](/docs/es/agent-sdk/typescript#sdkresultmessage)) como en Python ([`ResultMessage`](/docs/es/agent-sdk/python#resultmessage)). Si realiza múltiples llamadas `query()` (por ejemplo, en una sesión de múltiples turnos), cada resultado solo refleja el costo de esa llamada individual. Si solo necesita el total estimado, puede ignorar el uso por paso y leer este valor único.
  </Step>
</Steps>

<h2 id="get-the-total-cost-of-a-query">
  Obtener el costo total de una consulta
</h2>

El mensaje de resultado ([TypeScript](/docs/es/agent-sdk/typescript#sdkresultmessage), [Python](/docs/es/agent-sdk/python#resultmessage)) marca el final del bucle del agente para una llamada `query()`. Incluye `total_cost_usd`, el costo estimado acumulativo en todos los pasos de esa llamada. Esto funciona tanto para resultados de éxito como de error. Si utiliza sesiones para realizar múltiples llamadas `query()`, cada resultado solo refleja el costo de esa llamada individual.

Los tres campos a nivel de resultado difieren en lo que cuentan cuando el agente genera [subagentes](/docs/es/agent-sdk/subagents). Utilice `modelUsage`, o `model_usage` en Python, para la contabilidad de tokens de todo el árbol; el campo `usage` subestima tan pronto como ocurre anidamiento.

| Campo                        | Actividad de subagente                                                                                                           |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Excluido. Cuenta solo el bucle del agente de nivel superior, por lo que los tokens consumidos dentro de subagentes no se agregan |
| `total_cost_usd`             | Incluido. Cuenta las solicitudes de subagentes junto con el bucle de nivel superior                                              |
| `modelUsage` / `model_usage` | Incluido. Cuenta las solicitudes de subagentes junto con el bucle de nivel superior, desglosado por modelo                       |

Los siguientes ejemplos iteran sobre el flujo de mensajes de una llamada `query()` e imprimen el costo total cuando llega el mensaje `result`:

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
  Rastrear el uso por paso y por modelo
</h2>

Los ejemplos en esta sección utilizan nombres de campo de TypeScript. En Python, los campos equivalentes son [`AssistantMessage.usage`](/docs/es/agent-sdk/python#assistantmessage) y `AssistantMessage.message_id` para el uso por paso, y [`ResultMessage.model_usage`](/docs/es/agent-sdk/python#resultmessage) para desgloses por modelo.

<h3 id="track-per-step-usage">
  Rastrear el uso por paso
</h3>

Cada mensaje del asistente contiene un `BetaMessage` anidado (accesible a través de `message.message`) con un `id` y un objeto `usage` con conteos de tokens. Cuando Claude usa herramientas en paralelo, múltiples mensajes comparten el mismo `id` con datos de uso idénticos. Realice un seguimiento de qué ID ya ha contado y omita los duplicados para evitar totales inflados.

<Warning>
  Las llamadas de herramientas paralelas producen múltiples mensajes del asistente cuyo `BetaMessage` anidado comparte el mismo `id` y uso idéntico. Siempre deduplique por ID para obtener conteos de tokens por paso precisos.
</Warning>

El siguiente ejemplo acumula tokens de entrada y salida en todos los pasos, contando cada ID de mensaje único solo una vez:

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
  Desglosar el uso por modelo
</h3>

El mensaje de resultado incluye [`modelUsage`](/docs/es/agent-sdk/typescript#modelusage), un mapa del nombre del modelo a conteos de tokens y costo por modelo. Esto es útil cuando ejecuta múltiples modelos (por ejemplo, Haiku para suagentes y Opus para el agente principal) y desea ver dónde van los tokens.

El siguiente ejemplo ejecuta una consulta e imprime el costo y desglose de tokens para cada modelo utilizado:

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
  Acumular costos en múltiples llamadas
</h2>

Cada llamada `query()` devuelve su propio `total_cost_usd`. El SDK no proporciona un total a nivel de sesión, así que si su aplicación realiza múltiples llamadas `query()` (por ejemplo, en una sesión de múltiples turnos o entre diferentes usuarios), acumule los totales usted mismo.

Los siguientes ejemplos ejecutan dos llamadas `query()` secuencialmente, agregan el `total_cost_usd` de cada llamada a un total acumulado, e imprimen tanto el costo por llamada como el combinado:

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
  Manejar errores, almacenamiento en caché y discrepancias de tokens
</h2>

Para un rastreo de costos preciso, tenga en cuenta conversaciones fallidas, precios de tokens en caché e inconsistencias ocasionales en los informes.

<h3 id="resolve-output-token-discrepancies">
  Resolver discrepancias de tokens de salida
</h3>

En casos raros, puede observar diferentes valores de `output_tokens` para mensajes con el mismo ID. Cuando esto ocurra:

1. **Utilice el valor más alto:** el mensaje final en un grupo típicamente contiene el total preciso.
2. **Prefiera el mensaje de resultado:** el `total_cost_usd` en el mensaje de resultado refleja la estimación acumulada del SDK en todos los pasos, por lo que es más confiable que sumar valores por paso usted mismo. Sigue siendo una estimación y puede diferir de su factura real.
3. **Reporte inconsistencias:** presente problemas en el [repositorio de GitHub de Claude Code](https://github.com/anthropics/claude-code/issues).

<h3 id="track-costs-on-failed-conversations">
  Rastrear costos en conversaciones fallidas
</h3>

Tanto los mensajes de resultado de éxito como de error incluyen `usage` y `total_cost_usd`. Si una conversación falla a mitad de camino, aún consumió tokens hasta el punto de falla. Siempre lea datos de costo del mensaje de resultado independientemente de su `subtype`.

<h3 id="track-cache-tokens">
  Rastrear tokens en caché
</h3>

El SDK del Agente utiliza automáticamente [almacenamiento en caché de indicaciones](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) para reducir costos en contenido repetido. No necesita configurar el almacenamiento en caché usted mismo. El objeto de uso incluye dos campos adicionales para rastreo de caché:

* `cache_creation_input_tokens`: tokens utilizados para crear nuevas entradas de caché (facturados a una tasa más alta que los tokens de entrada estándar).
* `cache_read_input_tokens`: tokens leídos de entradas de caché existentes (facturados a una tasa reducida).

Rastree estos por separado de `input_tokens` para comprender los ahorros de almacenamiento en caché. En TypeScript, estos campos se escriben en el objeto [`Usage`](/docs/es/agent-sdk/typescript#usage). En Python, aparecen como claves en el diccionario [`ResultMessage.usage`](/docs/es/agent-sdk/python#resultmessage) (por ejemplo, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Extender el TTL de caché de indicaciones a una hora
</h3>

Las entradas de caché escritas por el SDK utilizan un TTL de 5 minutos de forma predeterminada cuando se autentica con una clave de API o se ejecuta en Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. Si su carga de trabajo ejecuta muchas sesiones cortas contra el mismo indicador del sistema y contexto con brechas más largas que 5 minutos entre ellas, el caché expira entre sesiones y cada nueva sesión paga el precio de entrada completo.

Para solicitar un TTL de 1 hora en escrituras de caché, establezca la variable de entorno [`ENABLE_PROMPT_CACHING_1H`](/docs/es/env-vars). Puede exportarla en su entorno de shell o contenedor, o pasarla a través de `options.env`.

El siguiente ejemplo habilita TTL de 1 hora para un agente que se ejecuta en Amazon Bedrock:

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

Las escrituras de caché con TTL de 1 hora se facturan a una tasa más alta que las escrituras de 5 minutos, por lo que habilitar esto intercambia un costo de escritura más alto por más lecturas de caché. Consulte [precios de almacenamiento en caché de indicaciones](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) para obtener detalles. Los usuarios de suscripción de Claude ya reciben TTL de 1 hora automáticamente y no necesitan establecer esta variable.

<h2 id="related-documentation">
  Documentación relacionada
</h2>

* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript) - Documentación completa de la API
* [Descripción general del SDK](/docs/es/agent-sdk/overview) - Introducción al SDK
* [Permisos del SDK](/docs/es/agent-sdk/permissions) - Gestión de permisos de herramientas
