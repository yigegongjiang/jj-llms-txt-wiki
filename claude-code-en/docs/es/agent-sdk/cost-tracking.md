> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rastrear costo y uso

> Aprenda a rastrear el uso de tokens, estimar costos y configurar el almacenamiento en caché de prompts con el SDK del Agente Claude.

El SDK del Agente Claude proporciona información detallada sobre el uso de tokens para cada interacción con Claude. Esta guía explica cómo rastrear correctamente el uso y comprender los informes de costos, especialmente cuando se trata de usos de herramientas paralelas y conversaciones de múltiples pasos.

Para obtener la documentación completa de la API, consulte la [referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript) y la [referencia del SDK de Python](/docs/es/agent-sdk/python).

<Warning>
  Los campos `total_cost_usd` y `costUSD` son estimaciones del lado del cliente, no datos de facturación autorizados. El SDK los calcula localmente a partir de una tabla de precios incluida en el momento de la compilación, a menos que una tabla [`modelPricing`](/docs/es/settings-reference#modelpricing) esté en vigor. Pueden desviarse de lo que realmente se le factura cuando:

  * los precios cambian
  * la versión del SDK instalada no reconoce un modelo
  * se aplican reglas de facturación que el cliente no puede modelar

  Una regla de facturación que el SDK sí modela es la [fijación de precios por residencia de datos](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing). Cuando el `usage` de una respuesta reporta `inference_geo: "us"`, el SDK multiplica el precio de lista de los tokens de esa respuesta por 1.1. Las tarifas por solicitud, como la búsqueda web, no se multiplican. Requiere TypeScript Agent SDK v0.3.239 o posterior, o Python Agent SDK v0.2.144 o posterior.

  Utilice estos campos para obtener información de desarrollo y presupuestos aproximados. Para facturación autorizada, utilice la [API de Uso y Costo](https://platform.claude.com/docs/en/build-with-claude/usage-cost-api) o la página de Uso en la [Consola Claude](https://platform.claude.com/usage). No facture a los usuarios finales ni desencadene decisiones financieras a partir de estos campos.
</Warning>

<h2 id="understand-token-usage">
  Comprender el uso de tokens
</h2>

Los SDKs de TypeScript y Python exponen los mismos datos de uso con nombres de campo diferentes:

* **TypeScript** proporciona desgloses de tokens por paso en cada mensaje del asistente (`message.message.id`, `message.message.usage`), costo por modelo a través de `modelUsage` en el mensaje de resultado, y un total acumulativo en el mensaje de resultado.
* **Python** proporciona desgloses de tokens por paso en cada mensaje del asistente como `message.usage` y `message.message_id`, costo por modelo a través de `model_usage` en el mensaje de resultado, y el total acumulativo en el mensaje de resultado como `total_cost_usd`.

Ambos SDKs utilizan el mismo modelo de costo subyacente y exponen la misma granularidad. La diferencia está en la nomenclatura de campos y dónde se anida el uso por paso.

El seguimiento de costos depende de comprender cómo el SDK delimita los datos de uso:

* **Llamada `query()`:** una invocación de la función `query()` del SDK. Una única llamada puede involucrar múltiples pasos: Claude responde, utiliza herramientas, obtiene resultados y responde nuevamente. Cada llamada produce un mensaje [`result`](/docs/es/agent-sdk/typescript#sdkresultmessage) al final, excepto en [modo de entrada de transmisión](/docs/es/agent-sdk/streaming-vs-single-mode), donde una llamada `query()` lleva múltiples turnos de usuario y cada turno emite su propio mensaje `result`.
* **Paso:** un único ciclo de solicitud/respuesta dentro de una llamada `query()`. Cada paso produce mensajes del asistente con uso de tokens.
* **Sesión:** una serie de llamadas `query()` vinculadas por un ID de sesión a través de la opción `resume`. Una llamada reanudada reporta el gasto total de la sesión, no solo el de esa llamada. Consulte [Acumular costos en múltiples llamadas](#accumulate-costs-across-multiple-calls) para ver cómo se transfieren los totales.

El siguiente diagrama muestra el flujo de mensajes de una única llamada `query()`, con el uso de tokens reportado en cada paso y la estimación acumulativa al final:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/message-usage-flow.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=68497aee338e01cc745323af7aea378e" className="dark:hidden" alt="Diagrama que muestra una consulta que produce dos pasos de mensajes. El paso 1 tiene cuatro mensajes del asistente que comparten el mismo ID y uso (contar una vez), el paso 2 tiene un mensaje del asistente con un nuevo ID, y el mensaje de resultado final muestra el total_cost_usd estimado." width="760" height="520" data-path="images/agent-sdk/message-usage-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/message-usage-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=8ea95085abc0a6b7f55ecef498bd4d14" className="hidden dark:block" alt="Diagrama que muestra una consulta que produce dos pasos de mensajes. El paso 1 tiene cuatro mensajes del asistente que comparten el mismo ID y uso (contar una vez), el paso 2 tiene un mensaje del asistente con un nuevo ID, y el mensaje de resultado final muestra el total_cost_usd estimado." width="760" height="520" data-path="images/agent-sdk/message-usage-flow-dark.svg" />

<Steps>
  <Step title="Cada paso produce mensajes del asistente">
    Cuando Claude responde, envía uno o más mensajes del asistente. En TypeScript, cada mensaje del asistente contiene un `BetaMessage` anidado (accesible a través de `message.message`) con un `id` y un objeto [`usage`](https://platform.claude.com/docs/en/api/messages) con conteos de tokens (`input_tokens`, `output_tokens`). En Python, la clase de datos `AssistantMessage` expone los mismos datos directamente a través de `message.usage` y `message.message_id`. Cuando Claude utiliza múltiples herramientas en un turno, todos los mensajes en ese turno comparten el mismo ID, así que deduplique por ID para evitar contar dos veces.
  </Step>

  <Step title="El mensaje de resultado proporciona la estimación acumulativa">
    Cuando se completa la llamada `query()`, el SDK emite un mensaje de resultado con `total_cost_usd` y `usage` acumulativo, tipado como [`SDKResultMessage`](/docs/es/agent-sdk/typescript#sdkresultmessage) en TypeScript y [`ResultMessage`](/docs/es/agent-sdk/python#resultmessage) en Python. Si solo necesita el total estimado, puede ignorar el uso por paso y leer este único valor.

    Si realiza múltiples llamadas `query()` independientes, cada resultado refleja solo el costo de esa llamada individual. Una llamada que reanuda una sesión también cuenta el gasto anterior de la sesión.

    En modo de entrada de transmisión, cada turno emite su propio mensaje de resultado. Consulte [Rastrear costos en modo de entrada de transmisión](#track-costs-in-streaming-input-mode) para saber cómo leer totales de llamadas en ese modo.
  </Step>
</Steps>

<h2 id="track-costs-in-streaming-input-mode">
  Rastrear costos en modo de entrada de streaming
</h2>

En [modo de entrada de streaming](/docs/es/agent-sdk/streaming-vs-single-mode), una llamada `query()` lleva múltiples turnos de usuario y cada turno emite su propio mensaje de resultado. Los campos de resultado difieren en alcance:

* **`usage`**: cubre solo ese turno, y dentro de él solo el bucle principal del agente, no ningún subagente que haya ejecutado.
* **`total_cost_usd` y `modelUsage`, o `model_usage` en Python**: llevan el total acumulado para toda la llamada hasta ahora, más cualquier gasto restaurado cuando la llamada reanudó una sesión.

En una llamada donde su aplicación nunca envía `/clear`, `/reset` o `/new`, lea el resultado más reciente para los totales de llamada en lugar de sumar entre resultados.

Los totales acumulados se reinician cada vez que su aplicación envía uno de esos tres comandos, y dentro de una llamada `query()` nada más los reinicia. Tres resultados importan para su contabilidad:

* **El resultado del propio turno `/clear`**: cubre solo lo que se ha ejecutado desde el reinicio, y lleva un nuevo `session_id`.
* **Cada resultado posterior**: sigue contando desde ese reinicio.
* **El último resultado antes de cada `/clear`**: contiene el total para los turnos desde el reinicio anterior.

Para totalizar toda la llamada, agregue el último resultado anterior a cada `/clear` al resultado final de la llamada. Todos los demás resultados, incluido el del turno `/clear`, son reemplazados por uno posterior.

En TypeScript, el SDK también emite un [`SDKConversationResetMessage`](/docs/es/agent-sdk/typescript#sdkconversationresetmessage) en cada reinicio, por lo que puede detectar reinicios desde la secuencia. En Python, el SDK asimismo emite un `ConversationResetMessage`. Antes de Python SDK v0.2.137, el iterador de Python descartaba ese mensaje, por lo que en esas versiones cuente los reinicios usted mismo a partir de los turnos `/clear` que su aplicación envía.

`maxBudgetUsd` (TypeScript) o `max_budget_usd` (Python) cuenta solo el gasto de la llamada misma: los totales restaurados de una sesión reanudada no cuentan en su contra, y un `/clear` reinicia el presupuesto.

<h2 id="get-the-total-cost-of-a-query">
  Obtener el costo total de una consulta
</h2>

El mensaje de resultado, tipado como [`SDKResultMessage`](/docs/es/agent-sdk/typescript#sdkresultmessage) en TypeScript y [`ResultMessage`](/docs/es/agent-sdk/python#resultmessage) en Python, marca el final del bucle del agente para una llamada `query()`. Incluye `total_cost_usd`, el costo estimado acumulativo en todos los pasos de esa llamada. Una llamada que reanuda una sesión también cuenta el gasto anterior de la sesión. Se aplican dos advertencias cuando lee el valor:

* En Python el campo se tipea como opcional, así que verifique que no sea `None` antes de leerlo.
* Los resultados de éxito y error llevan ambos, aunque el resultado final de un [bloqueo de sesión](#recover-totals-after-a-session-crash) puede llevarlo en cero.

En modo de entrada de transmisión, lea los totales de llamadas como se describe en [Rastrear costos en modo de entrada de transmisión](#track-costs-in-streaming-input-mode).

Los tres campos a nivel de resultado difieren en lo que cuentan cuando el agente genera [subagentes](/docs/es/agent-sdk/subagents). Use `modelUsage`, o `model_usage` en Python, para contabilidad de tokens de árbol completo; el campo `usage` subestima tan pronto como ocurre anidamiento.

| Campo                        | Actividad de subagente                                                                                                           |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `usage`                      | Excluido. Cuenta solo el bucle del agente de nivel superior, por lo que los tokens consumidos dentro de subagentes no se agregan |
| `total_cost_usd`             | Incluido. Cuenta solicitudes de subagentes junto con el bucle de nivel superior                                                  |
| `modelUsage` / `model_usage` | Incluido. Cuenta solicitudes de subagentes junto con el bucle de nivel superior, desglosado por modelo                           |

En [modo de entrada de mensaje único](/docs/es/agent-sdk/streaming-vs-single-mode#single-message-input), cuando los subagentes de fondo aún se están ejecutando al final del turno final, Claude Code espera por ellos, hasta el límite descrito en [tareas de fondo al salir](/docs/es/headless#background-tasks-at-exit), antes de emitir el resultado. El `total_cost_usd`, `duration_api_ms` y `modelUsage` del resultado, o `model_usage` en Python, incluyen el trabajo realizado durante esa espera.

Los siguientes ejemplos iteran sobre la transmisión de mensajes de una llamada `query()` e imprimen el costo total cuando llega el mensaje `result`:

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

Para limitar cuánto pueden agregar los subagentes a `total_cost_usd`, establezca los [límites de profundidad, concurrencia y gasto](/docs/es/agent-sdk/subagents#cap-subagent-depth-concurrency-and-spend) en la consulta.

<h2 id="track-per-step-and-per-model-usage">
  Rastrear el uso por paso y por modelo
</h2>

Los ejemplos en esta sección utilizan nombres de campos de TypeScript. En Python, los campos equivalentes son [`AssistantMessage.usage`](/docs/es/agent-sdk/python#assistantmessage) y `AssistantMessage.message_id` para el uso por paso, y [`ResultMessage.model_usage`](/docs/es/agent-sdk/python#resultmessage) para los desglose por modelo.

<h3 id="track-per-step-usage">
  Rastrear el uso por paso
</h3>

Cada mensaje del asistente contiene un `BetaMessage` anidado (accedido a través de `message.message`) con un `id` y un objeto `usage` con conteos de tokens. Cuando Claude utiliza herramientas en paralelo, múltiples mensajes comparten el mismo `id` con datos de uso idénticos. Rastreé qué IDs ya ha contado y omita duplicados para evitar totales inflados.

<Warning>
  Los valores deduplicados por paso son precisos para tokens de entrada y caché. El `output_tokens` por paso es un marcador de posición, así que [lea los tokens de salida del mensaje de resultado](#read-output-tokens-from-the-result-message).
</Warning>

El siguiente ejemplo acumula tokens de entrada en todos los pasos, contando cada ID de mensaje del bucle principal único solo una vez y omitiendo mensajes de subagentes, y lee el total de salida del mensaje de resultado, que cubre el bucle principal:

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
  Desglosar el uso por modelo
</h3>

El mensaje de resultado incluye [`modelUsage`](/docs/es/agent-sdk/typescript#modelusage), un mapa del nombre del modelo a los conteos de tokens por modelo y el costo. Esto es útil cuando ejecuta múltiples modelos (por ejemplo, Haiku para subagentes y Opus para el agente principal) y desea ver dónde van los tokens.

El `costBasis` de cada entrada indica qué tabla de precios fijó el precio de la solicitud más reciente de ese modelo: `list` para precio de lista, `managed` para una tabla [`modelPricing`](/docs/es/settings-reference#modelpricing), o `unknown` cuando ninguno coincidió con el ID del modelo. El campo requiere Claude Code v2.1.246 o posterior.

El siguiente ejemplo ejecuta una consulta e imprime el desglose de costo y tokens para cada modelo utilizado:

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

Cada llamada a `query()` devuelve `total_cost_usd` en sus resultados. La forma en que combine los valores depende de si las llamadas comparten una sesión:

* **Llamadas independientes, sin opción `resume` o `continue`**: cada resultado cubre solo su propia llamada, por lo que debe sumar los totales usted mismo, como hacen los ejemplos a continuación.
* **Llamadas que reanudan la misma sesión**: Claude Code guarda los totales de la sesión en su [transcripción](/docs/es/sessions#where-transcripts-are-stored) cuando el proceso se cierra normalmente y los restaura cuando una llamada posterior reanuda o bifurca la sesión. Cada resultado ya incluye el gasto anterior de la sesión. Lea el resultado más reciente para el total de la sesión; sumar resultados cuenta dos veces el gasto restaurado. Antes de v2.1.277, una sesión que reanudaba a través del SDK o `claude -p` iniciaba sus totales en cero, por lo que cada resultado de llamada cubría solo esa llamada.

En modo de entrada de transmisión, lea el total de cada llamada como se describe en [Rastrear costos en modo de entrada de transmisión](#track-costs-in-streaming-input-mode). Para una llamada que terminó en un bloqueo, consulte [Recuperar totales después de un bloqueo de sesión](#recover-totals-after-a-session-crash).

Los siguientes ejemplos ejecutan dos llamadas a `query()` secuencialmente, agregan el `total_cost_usd` de cada llamada a un total acumulado e imprimen tanto el costo por llamada como el costo combinado:

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
  Gestionar errores, almacenamiento en caché y conteos de tokens de salida
</h2>

Para un seguimiento preciso de costos, tenga en cuenta el conteo de salida de marcador de posición en los mensajes del asistente, los tokens que consumió una conversación fallida y los precios de tokens en caché.

<h3 id="read-output-tokens-from-the-result-message">
  Leer tokens de salida del mensaje de resultado
</h3>

Claude Code construye cada mensaje del asistente a partir del uso que la API reportó cuando la respuesta comenzó, por lo que el `output_tokens` del mensaje es solo el conteo que la API había reportado en `message_start`, antes de que se generara la respuesta. Una respuesta de API puede producir varios mensajes del asistente, y cada uno de ellos lleva ese mismo marcador de posición.

La API reporta el conteo de salida real al final de la respuesta, y Claude Code lo añade al mensaje de resultado. Lea los tokens de salida del `usage` del resultado, o de `modelUsage` para un desglose por modelo.

Para observar el conteo de salida de una respuesta crecer mientras se transmite, establezca `includePartialMessages`, o `include_partial_messages` en Python, y lea `usage` de cada evento de flujo `message_delta`, tipado como [`SDKPartialAssistantMessage`](/docs/es/agent-sdk/typescript#sdkpartialassistantmessage) en TypeScript y [`StreamEvent`](/docs/es/agent-sdk/python#streamevent) en Python.

<h3 id="track-costs-on-failed-conversations">
  Rastrear costos en conversaciones fallidas
</h3>

Tanto los mensajes de resultado de éxito como de error incluyen `usage` y `total_cost_usd`; en Python ambos campos se tipan como opcionales, así que verifique que no sean `None` antes de leerlos.

Si una conversación falla a mitad de camino, aún consumió tokens hasta el punto de falla. Lea datos de costo de cada mensaje de resultado, independientemente de si su `subtype` es `success` o uno de los subtipos de error. En algunos resultados de error, `usage` reporta menos de lo que la llamada gastó:

* **`error_during_execution` después de un [bloqueo de sesión](#recover-totals-after-a-session-crash)**: cada campo de costo puede ser puesto a cero.
* **`error_max_budget_usd`**: `usage` omite la respuesta que cruzó el presupuesto, mientras que `total_cost_usd` y `modelUsage` la incluyen.

Donde tenga la opción, contabilice desde `total_cost_usd` o `modelUsage` en lugar de `usage`.

<h3 id="recover-totals-after-a-session-crash">
  Recuperar totales después de un bloqueo de sesión
</h3>

Cuando el proceso de Claude Code se bloquea, emite un resultado final `error_during_execution` y sale, tanto en modo de entrada de un solo disparo como en modo de entrada de transmisión. Ese resultado puede llevar `usage`, `total_cost_usd` y `modelUsage` puestos a cero, así que recupere los totales de la llamada de lo que llegó antes. El paso 1 recupera los totales completos siempre que exista un resultado anterior; la alternativa en el paso 2 recupera solo los tokens de entrada y caché del bucle principal.

1. Use el resultado del turno anterior al bloqueo. En modo de entrada de transmisión, contiene el total acumulado descrito en [Rastrear costos en modo de entrada de transmisión](#track-costs-in-streaming-input-mode). Vaya al paso 2 en su lugar cuando ese resultado no pueda ayudarle:
   * La llamada fue de un solo disparo, así que no existe resultado anterior.
   * El bloqueo ocurrió en el primer turno.
   * El turno anterior al bloqueo fue el `/clear` mismo, así que su resultado cubre solo el reinicio.
2. Sume el `usage` en los mensajes del asistente en su lugar, contando cada respuesta de API una vez, como hace el ejemplo [Track per-step usage](#track-per-step-usage). En modo de un solo disparo, sume todos ellos; en modo de entrada de transmisión, sume los que llegaron después del último resultado. Esto le da los tokens de entrada y caché del bucle principal. El uso de subagentes no es recuperable de esta manera, tampoco lo son los tokens de salida o el costo en USD, porque [el `output_tokens` por paso es un marcador de posición](#read-output-tokens-from-the-result-message).

<h3 id="track-cache-tokens">
  Rastrear tokens en caché
</h3>

El Agent SDK utiliza automáticamente [almacenamiento en caché de prompts](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) para reducir costos en contenido repetido. No necesita configurar el almacenamiento en caché usted mismo. El objeto de uso incluye dos campos adicionales para el seguimiento de caché:

* `cache_creation_input_tokens`: tokens utilizados para crear nuevas entradas de caché (cobrados a una tasa más alta que los tokens de entrada estándar).
* `cache_read_input_tokens`: tokens leídos de entradas de caché existentes (cobrados a una tasa reducida).

Rastreé estos por separado de `input_tokens` para entender los ahorros de almacenamiento en caché. En TypeScript, estos campos se tipan en el objeto [`Usage`](/docs/es/agent-sdk/typescript#usage). En Python, aparecen como claves en el diccionario [`ResultMessage.usage`](/docs/es/agent-sdk/python#resultmessage) (por ejemplo, `message.usage.get("cache_read_input_tokens", 0)`).

<h3 id="extend-the-prompt-cache-ttl-to-one-hour">
  Extender el TTL de caché de prompts a una hora
</h3>

Sus propios turnos caen en el [bucket de TTL de conversación principal](/docs/es/prompt-caching#which-ttl-each-request-gets), junto con los ayudantes que Claude Code ejecuta en línea con ellos. Las solicitudes que Claude Code realiza fuera de esa conversación, como [subagentes](/docs/es/agent-sdk/subagents), tienen un [control de TTL separado](/docs/es/prompt-caching#choose-the-ttl-yourself).

Las entradas de caché para sus propios turnos utilizan un TTL de 5 minutos de forma predeterminada cuando se autentica con una clave de API o ejecuta en Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry, o [Claude Platform en AWS](/docs/es/claude-platform-on-aws). Si su carga de trabajo ejecuta muchas sesiones cortas contra el mismo prompt del sistema y contexto con brechas más largas que 5 minutos entre ellas, el caché expira entre sesiones y cada nueva sesión paga el precio de entrada completo.

Para solicitar un TTL de 1 hora en escrituras de caché, establezca la variable de entorno [`ENABLE_PROMPT_CACHING_1H`](/docs/es/env-vars). Puede exportarla en su entorno de shell o contenedor, o pasarla a través de `options.env`.

El siguiente ejemplo habilita TTL de 1 hora para un agente que se ejecuta en Amazon Bedrock. Porque establece `CLAUDE_CODE_USE_BEDROCK`, requiere credenciales de AWS funcionando para [Amazon Bedrock](/docs/es/amazon-bedrock); sin ellas la consulta falla.

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

Las escrituras de caché con un TTL de 1 hora se facturan a una tasa más alta que las escrituras de 5 minutos, así que habilitar esto intercambia un costo de escritura más alto por más lecturas de caché. Vea [precios de almacenamiento en caché de prompts](https://platform.claude.com/docs/en/build-with-claude/prompt-caching) para detalles. En una suscripción de Claude dentro del uso incluido de su plan, obtiene el TTL de 1 hora en sus propios turnos, y en algunas de las solicitudes de ayuda que Claude Code realiza junto a ellos, sin establecer esta variable, y Claude Code reduce esos turnos al TTL de 5 minutos una vez que está utilizando [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans).

`ENABLE_PROMPT_CACHING_1H` solicita el TTL de 1 hora en cada solicitud en ambos buckets. Para elegir un TTL para cada bucket por separado, use estos controles en su lugar. Cada uno toma `5m` o `1h` y tiene precedencia sobre `ENABLE_PROMPT_CACHING_1H`:

* Conversación principal: la variable de entorno [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/es/env-vars), o la configuración [`promptCacheTtl`](/docs/es/settings-reference#promptcachettl)
* Todo lo demás: la variable de entorno `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`, o la configuración [`subagentPromptCacheTtl`](/docs/es/settings-reference#subagentpromptcachettl)

Establecer `promptCacheTtl` a `1h` mantiene el caché de 1 hora en la conversación principal mientras está utilizando créditos de uso. Para el orden de precedencia completo, vea [elegir el TTL usted mismo](/docs/es/prompt-caching#choose-the-ttl-yourself).

<h2 id="related-documentation">
  Documentación relacionada
</h2>

* [Referencia del SDK de TypeScript](/docs/es/agent-sdk/typescript) - Documentación completa de la API
* [Descripción general del SDK](/docs/es/agent-sdk/overview) - Introducción al SDK
* [Permisos del SDK](/docs/es/agent-sdk/permissions) - Gestión de permisos de herramientas
