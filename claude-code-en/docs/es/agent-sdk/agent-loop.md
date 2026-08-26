> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Cómo funciona el bucle del agente

> Comprenda el ciclo de vida de los mensajes, la ejecución de herramientas, la ventana de contexto y la arquitectura que potencia sus agentes SDK.

El Agent SDK le permite incrustar el bucle de agente autónomo de Claude Code en sus propias aplicaciones. El SDK es un paquete independiente que le proporciona control programático sobre herramientas, permisos, límites de costos y salida. No necesita tener instalado el CLI de Claude Code para usarlo.

Cuando inicia un agente, el SDK ejecuta el mismo [bucle de ejecución que potencia Claude Code](/docs/es/how-claude-code-works#the-agentic-loop): Claude evalúa su prompt, llama a herramientas para tomar acciones, recibe los resultados y repite hasta que la tarea se complete. Esta página explica qué sucede dentro de ese bucle para que pueda construir, depurar y optimizar sus agentes de manera efectiva.

<h2 id="the-loop-at-a-glance">
  El bucle de un vistazo
</h2>

Cada sesión de agente sigue el mismo ciclo:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-loop-diagram.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=1c6e8f28d80dba14a7287419656f1237" alt="Diagrama del bucle del agente: su prompt entra en el bucle agéntico, donde Claude evalúa y solicita llamadas de herramientas cuyos resultados se retroalimentan en otra evaluación, o devuelve la respuesta final" width="720" height="212" data-path="images/agent-loop-diagram.svg" />

1. **Recibir prompt.** Claude recibe su prompt, junto con el prompt del sistema, las definiciones de herramientas e historial de conversación. El SDK produce un [`SystemMessage`](#message-types) con subtipo `"init"` que contiene metadatos de sesión.
2. **Evaluar y responder.** Claude evalúa el estado actual y determina cómo proceder. Puede responder con texto, solicitar una o más llamadas de herramientas, o ambas. El SDK produce un [`AssistantMessage`](#message-types) que contiene el texto y cualquier solicitud de llamada de herramienta.
3. **Ejecutar herramientas.** El SDK ejecuta cada herramienta solicitada y recopila los resultados. Cada conjunto de resultados de herramientas se retroalimenta a Claude para la siguiente decisión. Puede usar [hooks](/docs/es/agent-sdk/hooks) para interceptar, modificar o bloquear llamadas de herramientas antes de que se ejecuten.
4. **Repetir.** Los pasos 2 y 3 se repiten como un ciclo. Cada ciclo completo es un turno. Claude continúa llamando a herramientas y procesando resultados hasta que produce una respuesta sin llamadas de herramientas.
5. **Devolver resultado.** El SDK produce un [`AssistantMessage`](#message-types) final con la respuesta de texto (sin llamadas de herramientas), seguido de un [`ResultMessage`](#message-types) con el texto final, uso de tokens, costo e ID de sesión.

Una pregunta rápida ("¿qué archivos hay aquí?") podría tomar uno o dos turnos llamando a `Glob` y respondiendo con los resultados. Una tarea compleja ("refactorizar el módulo de autenticación y actualizar las pruebas") puede encadenar docenas de llamadas de herramientas en muchos turnos, leyendo archivos, editando código y ejecutando pruebas, con Claude ajustando su enfoque basado en cada resultado.

<h2 id="turns-and-messages">
  Turnos y mensajes
</h2>

Un turno es un viaje de ida y vuelta dentro del bucle: Claude produce una salida que incluye llamadas de herramientas, el SDK ejecuta esas herramientas y los resultados se devuelven a Claude automáticamente. Esto sucede sin ceder el control a su código. Los turnos continúan hasta que Claude produce una salida sin llamadas de herramientas, momento en el cual el bucle termina y se entrega el resultado final.

Considere cómo podría verse una sesión completa para el prompt "Arregla las pruebas fallidas en auth.ts".

Primero, el SDK envía su prompt a Claude y produce un [`SystemMessage`](#message-types) con los metadatos de sesión. Luego comienza el bucle:

1. **Turno 1:** Claude llama a `Bash` para ejecutar `npm test`. El SDK produce un [`AssistantMessage`](#message-types) con la llamada de herramienta, ejecuta el comando, luego produce un [`UserMessage`](#message-types) con la salida (tres fallos).
2. **Turno 2:** Claude llama a `Read` en `auth.ts` y `auth.test.ts`. El SDK devuelve el contenido del archivo y produce un `AssistantMessage`.
3. **Turno 3:** Claude llama a `Edit` para arreglar `auth.ts`, luego llama a `Bash` para volver a ejecutar `npm test`. Las tres pruebas pasan. El SDK produce un `AssistantMessage`.
4. **Turno final:** Claude produce una respuesta solo de texto sin llamadas de herramientas: "Arreglé el error de autenticación, las tres pruebas pasan ahora." El SDK produce un `AssistantMessage` final con este texto, luego un [`ResultMessage`](#message-types) con el mismo texto más costo y uso.

Eso fueron cuatro turnos: tres con llamadas de herramientas, uno con respuesta final solo de texto.

Puede limitar el bucle con `max_turns` / `maxTurns`, que cuenta solo los turnos de uso de herramientas. Por ejemplo, `max_turns=2` en el bucle anterior se habría detenido antes del paso de edición. También puede usar `max_budget_usd` / `maxBudgetUsd` para limitar los turnos basándose en un umbral de gasto.

Sin límites, el bucle se ejecuta hasta que Claude termine por su cuenta, lo cual está bien para tareas bien delimitadas pero puede ejecutarse durante mucho tiempo en prompts abiertos ("mejora esta base de código"). Establecer un presupuesto es un buen valor predeterminado para agentes de producción. Vea [Turnos y presupuesto](#turns-and-budget) a continuación para la referencia de opciones.

<h2 id="message-types">
  Tipos de mensajes
</h2>

A medida que se ejecuta el bucle, el SDK produce un flujo de mensajes. Cada mensaje lleva un tipo que le indica en qué etapa del bucle se originó. Los cinco tipos principales son:

* **`SystemMessage`:** eventos del ciclo de vida de la sesión. El campo `subtype` los distingue:

  * `"init"`: metadatos de sesión para la ejecución. Cuando un hook `SessionStart` o `Setup` se ejecuta durante el inicio de la sesión, sus [mensajes del ciclo de vida del hook](/docs/es/agent-sdk/typescript#sdkhookstartedmessage) llegan antes del mensaje `init`
  * `"compact_boundary"`: se dispara después de [compactación](#automatic-compaction)
  * `"informational"`: banners de estado de texto sin formato del bucle
  * `"worker_shutting_down"`: el bucle terminará después del turno actual porque el host se está cerrando o Remote Control se desconectó

  En TypeScript, cada subtipo que no sea `"init"` es su propio tipo en la unión [`SDKMessage`](/docs/es/agent-sdk/typescript#sdkmessage) en lugar de un subtipo de `SDKSystemMessage`.
* **`AssistantMessage`:** emitido después de cada respuesta de Claude, incluida la final solo de texto. Contiene bloques de contenido de texto y bloques de llamadas de herramientas de ese turno.
* **`UserMessage`:** emitido después de cada ejecución de herramienta con el contenido del resultado de la herramienta enviado de vuelta a Claude. También se emite para cualquier entrada de usuario que transmita a mitad del bucle.
* **`StreamEvent`:** solo se emite cuando los mensajes parciales están habilitados. Contiene eventos de transmisión de API sin procesar (deltas de texto, fragmentos de entrada de herramientas). Vea [Respuestas de transmisión](/docs/es/agent-sdk/streaming-output).
* **`ResultMessage`:** marca el final del bucle del agente. Contiene el resultado de texto final, uso de tokens, costo e ID de sesión. Verifique el campo `subtype` para determinar si la tarea tuvo éxito o alcanzó un límite. Un pequeño número de eventos del sistema finales, como `prompt_suggestion`, pueden llegar después, así que itere el flujo hasta completarse en lugar de romper en el resultado. Vea [Manejar el resultado](#handle-the-result).

Estos cinco tipos cubren el ciclo de vida completo del bucle del agente en ambos SDK. El SDK de TypeScript también produce eventos de observabilidad adicionales (eventos de hooks, progreso de herramientas, límites de velocidad, notificaciones de tareas) que proporcionan detalles adicionales pero no son necesarios para impulsar el bucle. Vea la [referencia de tipos de mensajes de Python](/docs/es/agent-sdk/python#message-types) y la [referencia de tipos de mensajes de TypeScript](/docs/es/agent-sdk/typescript#message-types) para las listas completas.

<h3 id="handle-messages">
  Manejar mensajes
</h3>

Qué mensajes maneja depende de lo que esté construyendo:

* **Solo resultados finales:** maneje `ResultMessage` para obtener la salida, el costo y si la tarea tuvo éxito o alcanzó un límite.
* **Actualizaciones de progreso:** maneje `AssistantMessage` para ver qué está haciendo Claude en cada turno, incluidas las herramientas que llamó.
* **Transmisión en vivo:** habilite mensajes parciales (`include_partial_messages` en Python, `includePartialMessages` en TypeScript) para obtener mensajes `StreamEvent` en tiempo real. Vea [Respuestas de transmisión en tiempo real](/docs/es/agent-sdk/streaming-output).

Cómo verifica los tipos de mensajes depende del SDK:

* **Python:** verifique los tipos de mensajes con `isinstance()` contra clases importadas de `claude_agent_sdk` (por ejemplo, `isinstance(message, ResultMessage)`).
* **TypeScript:** verifique el campo de cadena `type` (por ejemplo, `message.type === "result"`). `AssistantMessage` y `UserMessage` envuelven el mensaje de API sin procesar en un campo `.message`, por lo que los bloques de contenido están en `message.message.content`, no en `message.content`.

<Accordion title="Ejemplo: Verificar tipos de mensajes y manejar resultados">
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
  Ejecución de herramientas
</h2>

Las herramientas dan a su agente la capacidad de tomar acciones. Sin herramientas, Claude solo puede responder con texto. Con herramientas, Claude puede leer archivos, ejecutar comandos, buscar código e interactuar con servicios externos.

<h3 id="built-in-tools">
  Herramientas integradas
</h3>

El SDK incluye las mismas herramientas que potencian Claude Code:

| Categoría                  | Herramientas                                                    | Qué hacen                                                                              |
| :------------------------- | :-------------------------------------------------------------- | :------------------------------------------------------------------------------------- |
| **Operaciones de archivo** | `Read`, `Edit`, `Write`                                         | Leer, modificar y crear archivos                                                       |
| **Búsqueda**               | `Glob`, `Grep`                                                  | Encontrar archivos por patrón, buscar contenido con regex                              |
| **Ejecución**              | `Bash`                                                          | Ejecutar comandos de shell, scripts, operaciones de git                                |
| **Web**                    | `WebSearch`, `WebFetch`                                         | Buscar en la web, obtener y analizar páginas                                           |
| **Descubrimiento**         | `ToolSearch`                                                    | Encontrar y cargar herramientas dinámicamente bajo demanda en lugar de precargar todas |
| **Orquestación**           | `Agent`, `Skill`, `AskUserQuestion`, `TaskCreate`, `TaskUpdate` | Generar subagentes, invocar skills, preguntar al usuario, rastrear tareas              |

Más allá de las herramientas integradas, puede:

* **Conectar servicios externos** con [servidores MCP](/docs/es/agent-sdk/mcp) (bases de datos, navegadores, APIs)
* **Definir herramientas personalizadas** con [manejadores de herramientas personalizadas](/docs/es/agent-sdk/custom-tools)
* **Cargar skills del proyecto** a través de [fuentes de configuración](/docs/es/agent-sdk/claude-code-features) para flujos de trabajo reutilizables

<h3 id="tool-permissions">
  Permisos de herramientas
</h3>

Claude determina qué herramientas llamar basándose en la tarea, pero usted controla si esas llamadas pueden ejecutarse. Puede aprobar automáticamente herramientas específicas, bloquear otras completamente o requerir aprobación para todo. Tres opciones funcionan juntas para determinar qué se ejecuta:

* **`allowed_tools` / `allowedTools`** aprueba automáticamente las herramientas listadas. Un agente de solo lectura con `["Read", "Glob", "Grep"]` en su lista de herramientas permitidas ejecuta esas herramientas sin solicitar. Las herramientas no listadas aún están disponibles pero requieren permiso.
* **`disallowed_tools` / `disallowedTools`** bloquea las herramientas listadas, independientemente de otras configuraciones. Vea [Permisos](/docs/es/agent-sdk/permissions) para el orden en que se verifican las reglas antes de que se ejecute una herramienta.
* **`permission_mode` / `permissionMode`** controla qué sucede con las herramientas que no están cubiertas por reglas de permitir o denegar. Vea [Modo de permiso](#permission-mode) para los modos disponibles.

También puede limitar herramientas individuales con reglas como `"Bash(npm *)"` para permitir solo comandos específicos. Vea [Permisos](/docs/es/agent-sdk/permissions) para la sintaxis completa de reglas.

Cuando se deniega una herramienta, Claude recibe un mensaje de rechazo como resultado de la herramienta e intenta típicamente un enfoque diferente o reporta que no pudo proceder.

<h3 id="parallel-tool-execution">
  Ejecución paralela de herramientas
</h3>

Cuando Claude solicita múltiples llamadas de herramientas en un solo turno, ambos SDK pueden ejecutarlas concurrentemente o secuencialmente dependiendo de la herramienta. Las herramientas de solo lectura (como `Read`, `Glob`, `Grep` y herramientas MCP marcadas como de solo lectura) pueden ejecutarse concurrentemente. Las herramientas que modifican estado (como `Edit`, `Write` y `Bash`) se ejecutan secuencialmente para evitar conflictos.

Las herramientas personalizadas tienen ejecución secuencial de forma predeterminada. Para habilitar la ejecución paralela para una herramienta personalizada, establezca `readOnlyHint` en sus anotaciones. Ambos SDK de [TypeScript](/docs/es/agent-sdk/typescript#tool) y [Python](/docs/es/agent-sdk/python#tool) usan este nombre de campo del SDK de MCP.

<h2 id="control-how-the-loop-runs">
  Controlar cómo se ejecuta el bucle
</h2>

Puede limitar cuántos turnos toma el bucle, cuánto cuesta, cuán profundamente razona Claude y si las herramientas requieren aprobación antes de ejecutarse. Todos estos son campos en [`ClaudeAgentOptions`](/docs/es/agent-sdk/python#claudeagentoptions) (Python) / [`Options`](/docs/es/agent-sdk/typescript#options) (TypeScript).

<h3 id="turns-and-budget">
  Turnos y presupuesto
</h3>

| Opción                                                 | Qué controla                                            | Predeterminado |
| :----------------------------------------------------- | :------------------------------------------------------ | :------------- |
| Máximo de turnos (`max_turns` / `maxTurns`)            | Máximo de viajes de ida y vuelta de uso de herramientas | Sin límite     |
| Presupuesto máximo (`max_budget_usd` / `maxBudgetUsd`) | Costo máximo antes de detener                           | Sin límite     |

Cuando se alcanza cualquiera de los límites, el SDK devuelve un `ResultMessage` con un subtipo de error correspondiente (`error_max_turns` o `error_max_budget_usd`). Vea [Manejar el resultado](#handle-the-result) para cómo verificar estos subtipos y [`ClaudeAgentOptions`](/docs/es/agent-sdk/python#claudeagentoptions) / [`Options`](/docs/es/agent-sdk/typescript#options) para la sintaxis.

Con [entrada de transmisión](/docs/es/agent-sdk/streaming-vs-single-mode), un mensaje que envía mientras un turno aún se está ejecutando permanece en cola cuando ese turno termina en el límite de máximo de turnos, e inicia su propio turno con su propio límite de máximo de turnos. Antes de v2.1.205, un mensaje que llegaba en la iteración final del turno podría consumirse en el turno final y perderse sin llegar nunca al modelo.

<h3 id="effort-level">
  Nivel de esfuerzo
</h3>

La opción `effort` controla cuánto razonamiento aplica Claude. Los niveles de esfuerzo más bajos usan menos tokens por turno y reducen el costo. No todos los modelos soportan el parámetro de esfuerzo. Vea [Esfuerzo](https://platform.claude.com/docs/en/build-with-claude/effort) para qué modelos lo soportan.

| Nivel      | Comportamiento                          | Bueno para                                                                     |
| :--------- | :-------------------------------------- | :----------------------------------------------------------------------------- |
| `"low"`    | Razonamiento mínimo, respuestas rápidas | Búsquedas de archivos, listado de directorios                                  |
| `"medium"` | Razonamiento equilibrado                | Ediciones rutinarias, tareas estándar                                          |
| `"high"`   | Análisis exhaustivo                     | Refactorizaciones, depuración                                                  |
| `"xhigh"`  | Profundidad de razonamiento extendida   | Tareas de codificación y agentes; recomendado en Fable 5, Opus 4.7+ y Sonnet 5 |
| `"max"`    | Profundidad de razonamiento máxima      | Problemas de múltiples pasos que requieren análisis profundo                   |

Si no establece `effort`, ambos SDK dejan el parámetro sin establecer y se remiten al comportamiento predeterminado del modelo.

<Note>
  `effort` intercambia latencia y costo de token por profundidad de razonamiento dentro de cada respuesta. [Extended thinking](https://platform.claude.com/docs/en/build-with-claude/extended-thinking) es una característica separada que produce bloques de cadena de pensamiento visibles en la salida. Son independientes: puede establecer `effort: "low"` con extended thinking habilitado, o `effort: "max"` sin él.
</Note>

Use esfuerzo más bajo para agentes que realizan tareas simples y bien delimitadas (como listar archivos o ejecutar un único grep) para reducir costo y latencia. Establezca `effort` en las opciones de nivel superior `query()` para toda la sesión, o por subagente con el campo `effort` en [`AgentDefinition`](/docs/es/agent-sdk/subagents#agentdefinition-configuration) para anular el nivel de sesión.

<h3 id="permission-mode">
  Modo de permiso
</h3>

La opción de modo de permiso (`permission_mode` en Python, `permissionMode` en TypeScript) controla si el agente solicita aprobación antes de usar herramientas:

| Modo                  | Comportamiento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `"default"`           | Las herramientas no cubiertas por reglas de permitir activan su devolución de llamada de aprobación; sin devolución de llamada significa denegar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `"acceptEdits"`       | Aprueba automáticamente ediciones de archivo y comandos comunes del sistema de archivos (`mkdir`, `touch`, `mv`, `cp`, etc.); otros comandos de Bash siguen reglas predeterminadas                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `"plan"`              | Claude explora y planifica sin editar sus archivos fuente; las ediciones de archivo nunca se aprueban automáticamente y se solicitan a través de su devolución de llamada `canUseTool`                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `"dontAsk"`           | Nunca solicita. Las herramientas preaprobadas por [reglas de permiso](/docs/es/settings#permission-settings) se ejecutan; todo lo demás se deniega. `AskUserQuestion`, herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) se deniegan incluso si las ha permitido                                                                                                                                                                                            |
| `"auto"`              | Usa un clasificador de modelo para aprobar o denegar cada llamada de herramienta. Vea [Modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) para disponibilidad y comportamiento                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `"bypassPermissions"` | Ejecuta todas las herramientas permitidas sin preguntar, excepto herramientas coincidentes por una regla [`ask`](/docs/es/settings#permission-settings) explícita, herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y herramientas que requieren interacción del usuario; vea [Cómo se evalúan los permisos](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) para el orden de precedencia. No se puede usar cuando se ejecuta como root en Unix. Use solo en entornos aislados donde las acciones del agente no pueden afectar sistemas que le importan |

Para aplicaciones interactivas, use `"default"` con una devolución de llamada de aprobación de herramienta para mostrar solicitudes de aprobación. Para agentes autónomos en una máquina de desarrollo, `"acceptEdits"` aprueba automáticamente ediciones de archivo y comandos comunes del sistema de archivos (`mkdir`, `touch`, `mv`, `cp`, etc.) mientras aún controla otros comandos de `Bash` detrás de reglas de permitir. Reserve `"bypassPermissions"` para CI, contenedores u otros entornos aislados. Vea [Permisos](/docs/es/agent-sdk/permissions) para detalles completos.

<h3 id="model">
  Modelo
</h3>

Si no establece `model`, el SDK usa el predeterminado de Claude Code, que depende de su método de autenticación y suscripción. Establézcalo explícitamente (por ejemplo, `model="claude-sonnet-5"`) para fijar un modelo específico o para usar un modelo más pequeño para agentes más rápidos y económicos. Vea [modelos](https://platform.claude.com/docs/en/about-claude/models) para IDs disponibles.

<h2 id="the-context-window">
  La ventana de contexto
</h2>

La ventana de contexto es la cantidad total de información disponible para Claude durante una sesión. No se reinicia entre turnos dentro de una sesión. Todo se acumula: el prompt del sistema, definiciones de herramientas, historial de conversación, entradas de herramientas y salidas de herramientas. El contenido que permanece igual en todos los turnos (prompt del sistema, definiciones de herramientas, CLAUDE.md) se [almacena automáticamente en caché de prompt](https://platform.claude.com/docs/es/build-with-claude/prompt-caching), lo que reduce el costo y la latencia para prefijos repetidos.

<h3 id="what-consumes-context">
  Qué consume contexto
</h3>

Aquí está cómo cada componente afecta el contexto en el SDK:

| Fuente                           | Cuándo se carga                                                                      | Impacto                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------------------------------- | :----------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Prompt del sistema**           | Cada solicitud                                                                       | Costo fijo pequeño, siempre presente                                                                                                                                                                                                                                                                                                                                                                                              |
| **Archivos CLAUDE.md**           | Inicio de sesión, a través de [`settingSources`](/docs/es/agent-sdk/claude-code-features) | Contenido completo en cada solicitud (pero almacenado en caché de prompt, así que solo la primera solicitud paga el costo completo)                                                                                                                                                                                                                                                                                               |
| **Definiciones de herramientas** | Cada solicitud; esquemas MCP diferidos por defecto                                   | Los esquemas de herramientas integradas se cargan en cada solicitud. [Búsqueda de herramientas](/docs/es/agent-sdk/mcp#mcp-tool-search) difiere esquemas de herramientas MCP por defecto, retrocediendo a carga anticipada en Google Cloud's Agent Platform o una `ANTHROPIC_BASE_URL` que no sea de primera parte. Vea [Configurar búsqueda de herramientas](/docs/es/agent-sdk/tool-search#configure-tool-search) para la matriz completa |
| **Historial de conversación**    | Se acumula en turnos                                                                 | Crece con cada turno: prompts, respuestas, entradas de herramientas, salidas de herramientas                                                                                                                                                                                                                                                                                                                                      |
| **Descripciones de skills**      | Inicio de sesión, a través de fuentes de configuración                               | Resúmenes cortos; el contenido completo se carga solo cuando se invoca                                                                                                                                                                                                                                                                                                                                                            |

Las salidas de herramientas grandes consumen contexto significativo. Leer un archivo grande o ejecutar un comando con salida detallada puede usar miles de tokens en un solo turno. El contexto se acumula en turnos, así que sesiones más largas con muchas llamadas de herramientas acumulan significativamente más contexto que las cortas.

<h3 id="automatic-compaction">
  Compactación automática
</h3>

Cuando la ventana de contexto se acerca a su límite, el SDK compacta automáticamente la conversación: resume el historial anterior para liberar espacio, manteniendo intactos sus intercambios más recientes y decisiones clave. El SDK emite un mensaje con `type: "system"` y `subtype: "compact_boundary"` en el flujo cuando esto sucede (en Python esto es un `SystemMessage`; en TypeScript es un tipo separado `SDKCompactBoundaryMessage`).

La compactación reemplaza mensajes anteriores con un resumen, así que instrucciones específicas del inicio de la conversación pueden no preservarse. Las reglas persistentes pertenecen a CLAUDE.md (cargado a través de [`settingSources`](/docs/es/agent-sdk/claude-code-features)) en lugar de en el prompt inicial, porque el contenido de CLAUDE.md se reinyecta en cada solicitud.

Puede personalizar el comportamiento de compactación de varias maneras:

* **Instrucciones de resumen en CLAUDE.md:** El compactador lee su CLAUDE.md como cualquier otro contexto, así que puede incluir una sección diciéndole qué preservar al resumir. El encabezado de la sección es de forma libre (no una cadena mágica); el compactador coincide en intención.
* **Hook `PreCompact`:** Ejecute lógica personalizada antes de que ocurra la compactación, por ejemplo para archivar la transcripción completa. El hook recibe un campo `trigger` (`manual` o `auto`). Vea [hooks](/docs/es/agent-sdk/hooks).
* **Compactación manual:** Envíe `/compact` como una cadena de prompt para activar la compactación bajo demanda. Los comandos enviados de esta manera son entradas de SDK, no atajos solo de CLI. Vea [slash commands en el SDK](/docs/es/agent-sdk/slash-commands).

<Accordion title="Ejemplo: Instrucciones de resumen en CLAUDE.md">
  Agregue una sección al CLAUDE.md de su proyecto diciéndole al compactador qué preservar. El nombre del encabezado no es especial; use cualquier etiqueta clara.

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
  Mantener el contexto eficiente
</h3>

Algunas estrategias para agentes de larga duración:

* **Use subagentes para subtareas.** Cada subagente comienza con una conversación nueva (sin historial de mensajes anterior, aunque carga su propio prompt del sistema y contexto a nivel de proyecto como CLAUDE.md). No ve los turnos del padre, y solo su respuesta final regresa al padre como resultado de herramienta. El contexto del agente principal crece por ese resumen, no por la transcripción completa de subtarea. Vea [Qué heredan los subagentes](/docs/es/agent-sdk/subagents#what-subagents-inherit) para detalles.
* **Sea selectivo con herramientas.** Cada definición de herramienta toma espacio de contexto. Use el campo `tools` en [`AgentDefinition`](/docs/es/agent-sdk/subagents#agentdefinition-configuration) para limitar subagentes al conjunto mínimo que necesitan.
* **Observe costos de servidor MCP.** [Búsqueda de herramientas MCP](/docs/es/agent-sdk/mcp#mcp-tool-search) difiere esquemas de herramientas MCP por defecto y los carga bajo demanda. Cuando la búsqueda de herramientas está desactivada, en Google Cloud's Agent Platform, o detrás de una `ANTHROPIC_BASE_URL` que no sea de primera parte, cada servidor MCP agrega todos sus esquemas de herramientas a cada solicitud, así que algunos servidores con muchas herramientas pueden consumir contexto significativo antes de que el agente haga ningún trabajo.
* **Use esfuerzo más bajo para tareas rutinarias.** Establezca [esfuerzo](#effort-level) a `"low"` para agentes que solo necesitan leer archivos o listar directorios. Esto reduce el uso de tokens y el costo.

Para un desglose detallado de costos de contexto por característica, vea [Entender costos de contexto](/docs/es/features-overview#understand-context-costs).

<h2 id="sessions-and-continuity">
  Sesiones y continuidad
</h2>

Cada interacción con el SDK crea o continúa una sesión. Capture el ID de sesión de `ResultMessage.session_id` (disponible en ambos SDK) para reanudar más tarde. El SDK de TypeScript también lo expone como un campo directo en el `SystemMessage` de init; en Python está anidado en `SystemMessage.data`.

Cuando reanuda, el contexto completo de turnos anteriores se restaura: archivos que fueron leídos, análisis que fue realizado y acciones que fueron tomadas. También puede bifurcar una sesión para ramificarse en un enfoque diferente sin modificar el original.

Vea [Gestión de sesiones](/docs/es/agent-sdk/sessions) para la guía completa en patrones de reanudar, continuar y bifurcar.

<Note>
  En Python, `ClaudeSDKClient` maneja IDs de sesión automáticamente en múltiples llamadas. Vea la [referencia del SDK de Python](/docs/es/agent-sdk/python#choosing-between-query-and-claudesdkclient) para detalles.
</Note>

<h2 id="handle-the-result">
  Manejar el resultado
</h2>

Cuando el bucle termina, el `ResultMessage` le dice qué sucedió y le proporciona la salida. El campo `subtype` (disponible en ambos SDK) es la forma principal de verificar el estado de terminación.

| Subtipo de resultado                  | Qué sucedió                                                                                                                                                                                                 | ¿Campo `result` disponible? |
| :------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------: |
| `success`                             | Claude terminó la tarea normalmente                                                                                                                                                                         |              Sí             |
| `error_max_turns`                     | Alcanzó el límite de `maxTurns` antes de terminar                                                                                                                                                           |              No             |
| `error_max_budget_usd`                | Alcanzó el límite de `maxBudgetUsd` antes de terminar                                                                                                                                                       |              No             |
| `error_during_execution`              | Un error interrumpió el bucle (por ejemplo, una falla de API o solicitud cancelada)                                                                                                                         |              No             |
| `error_max_structured_output_retries` | No se produjo una salida estructurada válida dentro del límite de reintentos configurado: cada intento falló la validación, o una alternativa de modelo retractó la salida completada sin reintento exitoso |              No             |

El campo `result` (la salida de texto final) solo está presente en la variante `success`, así que siempre verifique el subtipo antes de leerlo. Todos los subtipos de resultado llevan `total_cost_usd`, `usage`, `num_turns` e `session_id` para que pueda rastrear el costo y reanudar incluso después de errores. En Python, `total_cost_usd` y `usage` se escriben como opcionales y pueden ser `None` en algunas rutas de error, así que proteja antes de formatearlos. Vea [Rastrear costos y uso](/docs/es/agent-sdk/cost-tracking) para detalles sobre la interpretación de los campos `usage`.

<Note>
  Cuando una consulta termina en un resultado de error:

  * Una llamada `query()` de un solo disparo produce el mensaje de resultado final y luego genera un error que incluye el texto de fallo, como `Reached maximum number of turns`. La generación es intencional — envuelva el bucle en un bloque try si su código necesita continuar más allá de él. El proceso Claude Code subyacente también sale con un código distinto de cero.
  * Una sesión de entrada de transmisión permanece activa, y puede seguir enviando mensajes.
</Note>

El resultado también incluye un campo `stop_reason` (`string | null` en TypeScript, `str | None` en Python) indicando por qué el modelo dejó de generar en su turno final. Los valores comunes son `end_turn` (modelo terminó normalmente), `max_tokens` (alcanzó el límite de token de salida) y `refusal` (el modelo rechazó la solicitud). En subtipos de resultado de error, `stop_reason` lleva el valor de la última respuesta de asistente antes de que el bucle terminara. Para detectar rechazos, verifique `stop_reason === "refusal"` (TypeScript) o `stop_reason == "refusal"` (Python). Vea [`SDKResultMessage`](/docs/es/agent-sdk/typescript#sdkresultmessage) (TypeScript) o [`ResultMessage`](/docs/es/agent-sdk/python#resultmessage) (Python) para el tipo completo.

<h2 id="hooks">
  Hooks
</h2>

[Hooks](/docs/es/agent-sdk/hooks) son devoluciones de llamada que se disparan en puntos específicos del bucle: antes de que se ejecute una herramienta, después de que regresa, cuando el agente termina, y así sucesivamente. Algunos hooks comúnmente usados son:

| Hook                             | Cuándo se dispara                           | Usos comunes                                      |
| :------------------------------- | :------------------------------------------ | :------------------------------------------------ |
| `PreToolUse`                     | Antes de que se ejecute una herramienta     | Validar entradas, bloquear comandos peligrosos    |
| `PostToolUse`                    | Después de que regresa una herramienta      | Auditar salidas, activar efectos secundarios      |
| `UserPromptSubmit`               | Cuando se envía un prompt                   | Inyectar contexto adicional en prompts            |
| `Stop`                           | Cuando el agente termina                    | Validar el resultado, guardar estado de sesión    |
| `SubagentStart` / `SubagentStop` | Cuando se genera un subagente o se completa | Rastrear y agregar resultados de tareas paralelas |
| `PreCompact`                     | Antes de la compactación de contexto        | Archivar transcripción completa antes de resumir  |

Los hooks se ejecutan en su proceso de aplicación, no dentro de la ventana de contexto del agente, así que no consumen contexto. Los hooks también pueden cortocircuitar el bucle: un hook `PreToolUse` que rechaza una llamada de herramienta evita que se ejecute, y Claude recibe el mensaje de rechazo en su lugar.

Ambos SDK soportan todos los eventos anteriores. El SDK de TypeScript incluye eventos adicionales que Python aún no soporta. Vea [Controlar ejecución con hooks](/docs/es/agent-sdk/hooks) para la lista completa de eventos, disponibilidad por SDK y la API de devolución de llamada completa.

<h2 id="put-it-all-together">
  Ponerlo todo junto
</h2>

Este ejemplo combina los conceptos clave de esta página en un único agente que arregla pruebas fallidas. Configura el agente con herramientas permitidas (aprobadas automáticamente para que el agente se ejecute autónomamente), configuración del proyecto y límites de seguridad en turnos y esfuerzo de razonamiento. A medida que se ejecuta el bucle, captura el ID de sesión para posible reanudación, maneja el resultado final e imprime el costo total.

Debido a que una única llamada `query()` genera una excepción después de producir un resultado de error, el bucle se envuelve en un bloque try para que el script se cierre correctamente cuando se alcanza un límite.

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
  Próximos pasos
</h2>

Ahora que entiende el bucle, aquí está dónde ir dependiendo de lo que esté construyendo:

* **¿Aún no ha ejecutado un agente?** Comience con el [inicio rápido](/docs/es/agent-sdk/quickstart) para obtener el SDK instalado y ver un ejemplo completo ejecutándose de principio a fin.
* **¿Listo para conectarse a su proyecto?** [Cargue CLAUDE.md, skills y hooks del sistema de archivos](/docs/es/agent-sdk/claude-code-features) para que el agente siga automáticamente las convenciones de su proyecto.
* **¿Construyendo una interfaz de usuario interactiva?** Habilite [transmisión](/docs/es/agent-sdk/streaming-output) para mostrar texto en vivo y llamadas de herramientas a medida que se ejecuta el bucle.
* **¿Necesita control más ajustado sobre lo que el agente puede hacer?** Bloquee el acceso a herramientas con [permisos](/docs/es/agent-sdk/permissions) y use [hooks](/docs/es/agent-sdk/hooks) para auditar, bloquear o transformar llamadas de herramientas antes de que se ejecuten.
* **¿Ejecutando tareas largas o costosas?** Descargue trabajo aislado a [subagentes](/docs/es/agent-sdk/subagents) para mantener su contexto principal delgado.

Para la imagen conceptual más amplia del bucle agente (no específica del SDK), vea [Cómo funciona Claude Code](/docs/es/how-claude-code-works). Para una guía práctica sobre el diseño de bucles en Claude Code, desde bucles basados en turnos hasta bucles basados en objetivos y bucles proactivos, vea [Loop engineering: getting started with loops](https://claude.com/blog/getting-started-with-loops) en el blog.
