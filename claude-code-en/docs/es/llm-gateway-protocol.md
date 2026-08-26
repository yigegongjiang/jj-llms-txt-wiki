> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia del protocolo de puerta de enlace

> El contrato de API entre Claude Code y una puerta de enlace LLM: puntos finales, encabezados y campos de cuerpo para reenviar, degradación de características cuando se eliminan campos, encabezados de atribución para seguimiento de costos y descubrimiento de modelos.

Esta página documenta las solicitudes que Claude Code envía a una puerta de enlace, incluidos los puntos finales que llama, los encabezados y campos de cuerpo que la puerta de enlace debe reenviar, y qué características dejan de funcionar cuando no lo hace. Está escrita para operadores que configuran un producto de puerta de enlace para trabajar con Claude Code.

Una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) en ejecución sirve una versión legible por máquina de este contrato en `GET /protocol`, cubriendo los mismos requisitos de reenvío más los puntos finales específicos de la puerta de enlace de aplicaciones Claude para inicio de sesión SSO, entrega de configuración administrada y telemetría. La puerta de enlace de aplicaciones Claude se ejecuta desde el mismo binario `claude` que la CLI, por lo que el [inicio rápido de la puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway#quickstart) es el camino más corto hacia una instancia en ejecución desde la que puede obtener la especificación.

<Note>
  * Para implementar una puerta de enlace existente o de terceros para su organización, consulte [Implementar una puerta de enlace LLM](/docs/es/llm-gateway-rollout)
  * Si es un desarrollador individual que autentica Claude Code en una puerta de enlace con una credencial que le proporcionaron, consulte [Conectar Claude Code a una puerta de enlace LLM](/docs/es/llm-gateway-connect)
</Note>

Esta página cubre:

* [Formatos de API](#api-formats) y los puntos finales a servir para cada uno
* [Encabezados de solicitud](#request-headers): cuáles deben llegar al proveedor ascendente y cuáles su puerta de enlace puede consumir
* El [bloque de atribución del mensaje del sistema](#system-prompt-attribution-block) y cómo interactúa con el almacenamiento en caché de solicitudes
* [Paso de características](#feature-pass-through): qué se rompe cuando se eliminan encabezados o campos de cuerpo
* [Descubrimiento de modelos](#model-discovery)

Esta página utiliza dos términos para lo que su puerta de enlace hace con cada encabezado y campo de cuerpo:

* **Reenviar sin cambios**: pasarlo al proveedor ascendente byte por byte
* **Consumir**: la puerta de enlace puede leerlo para enrutamiento, atribución o seguimiento y no necesita reenviarlo

Cualquier cosa no marcada como reenviar sin cambios es suya para consumir o ignorar.

<h2 id="api-formats">
  Formatos de API
</h2>

Una puerta de enlace debe exponer al menos uno de los siguientes formatos de API a los clientes de Claude Code. Qué formato habla Claude Code está determinado por la configuración del cliente: la variable en la columna Seleccionado por de la tabla a continuación apunta Claude Code a su puerta de enlace en ese formato. Google Cloud's Agent Platform es el punto de conexión de Claude de Google Cloud, anteriormente Vertex AI; sus nombres de variables mantienen la ortografía `VERTEX`.

| Formato                                  | Seleccionado por                                             | Puntos finales                                                           | Reenviar sin cambios                                                                                                   |
| :--------------------------------------- | :----------------------------------------------------------- | :----------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                         | `/v1/messages`, `/v1/messages/count_tokens` (opcional)                   | encabezados de solicitud `anthropic-beta` y `anthropic-version`                                                        |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` con `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`    | campos de cuerpo de solicitud `anthropic_beta` y `anthropic_version`                                                   |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` con `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (opcional) | encabezados de solicitud `anthropic-beta` y `anthropic-version`, y el campo de cuerpo de solicitud `anthropic_version` |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry y Claude Platform en AWS
</h3>

Microsoft Foundry y la [Claude Platform en AWS](/docs/es/claude-platform-on-aws) implementan el formato Anthropic Messages. Claude Code se enruta a ellos a través de sus propias variables, `ANTHROPIC_FOUNDRY_BASE_URL` y `ANTHROPIC_AWS_BASE_URL`, pero una puerta de enlace que está frente a cualquiera de ellos implementa la fila Anthropic Messages anterior. Una puerta de enlace que está frente a Claude Platform en AWS también debe reenviar el encabezado `anthropic-workspace-id`, que [esa plataforma requiere en cada solicitud](/docs/es/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Puntos finales opcionales y tráfico de inicio
</h3>

Los puntos finales de conteo de tokens son los únicos opcionales: cuando están ausentes, Claude Code estima el uso de contexto localmente. Las solicitudes de inferencia se publican en `/v1/messages?beta=true`, así que coincida con la ruta, no con la URL completa. Los sufijos del método Google Cloud's Agent Platform se adjuntan a la ruta del modelo del editor, como en `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`.

Una puerta de enlace también ve tráfico de inicio de mejor esfuerzo que puede rechazar sin romper nada: una sonda de conectividad `HEAD /`, y en puertas de enlace con formato Amazon Bedrock una solicitud `GET /inference-profiles?type=SYSTEM_DEFINED`.

<h3 id="streaming">
  Transmisión
</h3>

Las respuestas de inferencia deben transmitirse. Claude Code consume eventos enviados por el servidor a medida que llegan, por lo que una puerta de enlace que almacena en búfer respuestas completas antes de retransmitirlas detiene el cliente.

<h3 id="format-mismatch-with-the-upstream">
  Desajuste de formato con el proveedor ascendente
</h3>

Qué formato habla el cliente determina lo que su puerta de enlace recibe. El modo de fallo común es un desajuste entre el formato que el cliente envía a su puerta de enlace y el formato que el proveedor ascendente detrás de él acepta.

* Cuando el cliente habla el formato Amazon Bedrock o Google Cloud's Agent Platform, Claude Code envía solo el subconjunto de su conjunto de capacidades completo que esos proveedores aceptan
* Cuando el cliente habla el formato Anthropic Messages, Claude Code envía el conjunto completo, incluso si su puerta de enlace se reenvía a un proveedor ascendente Amazon Bedrock o Google Cloud's Agent Platform

Cerrar esa diferencia es el trabajo de su puerta de enlace. [Paso de características](#feature-pass-through) describe qué se rompe cuando no lo hace.

<h2 id="request-headers">
  Encabezados de solicitud
</h2>

Claude Code incluye estos encabezados en solicitudes de API. Los nombres de encabezados no distinguen mayúsculas de minúsculas en la red. Reenvíe `anthropic-version` y `anthropic-beta` sin cambios, más `anthropic-workspace-id` cuando el proveedor ascendente es la [Claude Platform en AWS](/docs/es/claude-platform-on-aws); el resto la puerta de enlace puede consumir para enrutamiento, atribución y seguimiento, y no necesita reenviar.

| Encabezado                      | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Authorization`, `x-api-key`    | La credencial de puerta de enlace del desarrollador, en uno o ambos encabezados dependiendo de cuál [variable de credencial](/docs/es/llm-gateway-connect#set-the-credential-variable) establezcan                                                                                                                                                                                                                                                                                                                             |
| `anthropic-version`             | Versión de API, actualmente `2023-06-01`. Las solicitudes con formato Amazon Bedrock y Agent Platform de Google Cloud también llevan el campo de cuerpo `anthropic_version`, cuyo valor es la cadena de dialecto del proveedor, no el valor de este encabezado                                                                                                                                                                                                                                                            |
| `anthropic-beta`                | Valores de capacidad separados por comas para la solicitud. Reenvíe el encabezado textualmente; no permita valores individuales, porque el conjunto cambia con las versiones de Claude Code. Cuando el desarrollador se autentica con un inicio de sesión de claude.ai, que es posible cuando `ANTHROPIC_BASE_URL` se establece sin una variable de credencial de puerta de enlace, este encabezado también lleva una capacidad OAuth que el proveedor ascendente requiere, y eliminarlo falla esas solicitudes con `401` |
| `x-claude-code-session-id`      | Un identificador único para la sesión actual de Claude Code. Úselo para agregar todas las solicitudes de una sesión sin analizar cuerpos de solicitud                                                                                                                                                                                                                                                                                                                                                                     |
| `x-claude-code-agent-id`        | Identificador del [subagente](/docs/es/sub-agents) que emitió la solicitud, presente solo en solicitudes de un agente que Claude Code generó dentro de la sesión. Úselo con el ID de sesión para atribuir costo a agentes paralelos                                                                                                                                                                                                                                                                                            |
| `x-claude-code-parent-agent-id` | Identificador del agente que generó el agente solicitante, presente solo para agentes anidados                                                                                                                                                                                                                                                                                                                                                                                                                            |

Los ID de subagente se generan nuevos para cada generación. Los agentes compañeros, los miembros nombrados de un [equipo de agentes](/docs/es/agent-teams), reutilizan un ID estable basado en nombres en reconexiones. En ambos casos, el ID identifica un agente, no una persona o un dispositivo, así que no trate el encabezado de ID de agente como un identificador de usuario.

Si sus desarrolladores establecen `ANTHROPIC_CUSTOM_HEADERS`, esos encabezados también aparecen en las solicitudes.

<h3 id="forward-as-open-lists">
  Reenviar como listas abiertas
</h3>

Trate los encabezados y campos de cuerpo como listas abiertas, no cerradas. Claude Code gana capacidades en las versiones, y llegan como nuevos valores `anthropic-beta`, nuevos campos de cuerpo de solicitud y ocasionalmente nuevos encabezados `anthropic-*` o `x-claude-code-*`.

Al reenviar a un proveedor ascendente con formato Anthropic, pase encabezados de solicitud `anthropic-*` y campos de cuerpo de solicitud sin cambios en lugar de permitir los que ve hoy. Una puerta de enlace fijada a una lista observada elimina el encabezado o campo de la siguiente capacidad y lo rompe en la versión que la introduce.

La excepción es un proveedor ascendente que no es Anthropic, como Amazon Bedrock o Agent Platform de Google Cloud, donde cerrar la diferencia de esquema es el trabajo de la puerta de enlace; consulte [paso de características](#feature-pass-through).

<h2 id="system-prompt-attribution-block">
  Bloque de atribución del mensaje del sistema
</h2>

Claude Code antepone un bloque de atribución corto al mensaje del sistema que contiene la versión del cliente y una huella digital derivada de la conversación. El punto final `api.anthropic.com` elimina el bloque antes de procesar cuando llega sin cambios como el primer bloque del sistema, por lo que no afecta el almacenamiento en caché de solicitudes de primera parte. Cualquier otro proveedor ascendente lo recibe como parte del mensaje.

La eliminación es posicional, por lo que solo funciona cuando la puerta de enlace reenvía la matriz `system` sin cambios. Para mantener el bloque fuera del mensaje sin perder otro contenido del sistema:

* Reenvíe la matriz `system` exactamente como se recibió, manteniendo el bloque primero: anteponer otro bloque del sistema, reordenar la matriz o convertirla en una sola cadena anula la eliminación, y el bloque luego llega al modelo y a la clave de caché de solicitud.
* Mantenga el bloque en su propia entrada de matriz: el punto final trata un bloque fusionado que comienza con el encabezado de atribución como atribución en su totalidad y descarta todo lo fusionado en él, incluido el resto del mensaje del sistema.
* Si su puerta de enlace debe remodelar el contenido del sistema, establezca [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/es/env-vars) para que Claude Code omita el bloque. Anthropic y los puntos finales de Claude de los proveedores de nube leen el bloque para atribución, así que omítalo en el cliente en lugar de eliminarlo o moverlo en la puerta de enlace.

Las solicitudes que llegan al punto final sin modificar no se ven afectadas.

Desde Claude Code v2.1.181, el bloque es estable para la vida útil de una conversación cuando las solicitudes se enrutan a través de una URL base personalizada, por lo que una caché de solicitud de puerta de enlace con clave en el cuerpo de solicitud completo funciona sin deshabilitarlo. Antes de v2.1.181, el bloque incluía un token por solicitud; en esas versiones, establezca `CLAUDE_CODE_ATTRIBUTION_HEADER=0` si su puerta de enlace implementa tal caché.

<h2 id="feature-pass-through">
  Paso de características
</h2>

Claude Code trata una puerta de enlace `ANTHROPIC_BASE_URL` como un punto final con formato Anthropic y le envía los encabezados beta y campos de cuerpo de solicitud que envía a `api.anthropic.com`, excepto un pequeño conjunto de diagnósticos y valores predeterminados reservados para conexiones directas, como el valor predeterminado de transmisión de herramientas de grano fino cubierto a continuación. Ese conjunto varía según la versión, así que no dependa de su contenido.

Las capacidades que agregan campos de cuerpo los emparejan con un encabezado beta, y el par viaja junto. Una puerta de enlace que elimina el encabezado mientras pasa el cuerpo, o reenvía un cuerpo con formato Anthropic a un proveedor ascendente con un esquema diferente, produce errores `400` duros; solo cuando ambas mitades están ausentes juntas la característica se apaga silenciosamente. Una puerta de enlace que reescribe o redacta cuerpos de solicitud para inspección de contenido rompe el emparejamiento de la misma manera que la eliminación, así que inspeccione sin modificar. La tabla señala dónde una característica se desvía del emparejamiento.

La transmisión de herramientas de grano fino es uno de los valores predeterminados de conexión directa: está desactivada de forma predeterminada siempre que las solicitudes se enruten a través de una URL base personalizada, y una puerta de enlace la recibe cuando los desarrolladores establecen [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/es/env-vars).

| Característica                                                                                                                                                                                                                                      | Encabezado y par de cuerpo                                                                                                                                                                                                 | Síntoma cuando se rompe                                                                                                                                  | Remediación                                                                                                                                              |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Razonamiento adaptativo](/docs/es/model-config#adjust-effort-level)                                                                                                                                                                                     | Sin encabezado beta. Claude Code envía `thinking: {"type": "adaptive"}` para Claude 4.6 y posterior, y trata nombres de modelo que no reconoce, como alias de puerta de enlace, como modelos actuales que reciben el campo | `400` nombrando el campo `thinking` o la etiqueta `adaptive` cuando la compilación del modelo ascendente no la acepta                                    | Actualice el proveedor ascendente. En Opus 4.6 y Sonnet 4.6, los desarrolladores pueden establecer `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` en su lugar |
| [Gestión de contexto](https://platform.claude.com/docs/en/build-with-claude/context-management)                                                                                                                                                     | El encabezado beta de gestión de contexto se empareja con el campo de cuerpo `context_management`                                                                                                                          | `400` con `Extra inputs are not permitted`. Común cuando una puerta de enlace acepta solicitudes con formato Anthropic pero las reenvía a Amazon Bedrock | Reenvíe ambos, o [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/es/env-vars)                                                                              |
| [Contexto extendido](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) y [pensamiento intercalado](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | Solo encabezados beta, sin campo de cuerpo                                                                                                                                                                                 | Silenciosamente no disponible cuando se elimina el encabezado; el proveedor ascendente nunca ve la solicitud de capacidad                                | Reenvíe `anthropic-beta` textualmente                                                                                                                    |
| Campos de [herramienta](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview) beta                                                                                                                                                | Los encabezados beta relacionados con herramientas se emparejan con campos de esquema de herramienta como `strict` y `defer_loading`                                                                                       | `400` nombrando el campo de esquema de herramienta no reconocido cuando el cuerpo pasa sin su encabezado                                                 | Reenvíe ambos, o `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`                                                                                              |
| [Esfuerzo](https://platform.claude.com/docs/en/build-with-claude/effort) y [salidas estructuradas](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                        | El campo de cuerpo `output_config` lleva esfuerzo, formato de salida estructurada y configuración de presupuesto de tarea; cada uno se empareja con su propio encabezado beta                                              | `400` nombrando `output_config`, a menudo `Extra inputs are not permitted`, en proveedores ascendentes Amazon Bedrock y Agent Platform                   | Reenvíe el campo y sus encabezados juntos                                                                                                                |
| [Conteo de tokens](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                            | Sin emparejamiento beta; utiliza el punto final `count_tokens`                                                                                                                                                             | Claude Code vuelve a estimar el uso de contexto localmente                                                                                               | Exponga el punto final si desea conteos exactos                                                                                                          |

Las [variables](/docs/es/model-config) `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` declaran capacidades de modelo solo en las configuraciones del proveedor: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, y [`CLAUDE_CODE_USE_MANTLE`](/docs/es/amazon-bedrock#use-the-mantle-endpoint). No tienen efecto detrás de una puerta de enlace `ANTHROPIC_BASE_URL`.

<h3 id="automatic-retry-and-error-forwarding">
  Reintento automático y reenvío de errores
</h3>

Claude Code reintenta automáticamente después de algunos rechazos ascendentes y deshabilita la capacidad rechazada para el resto de la conversación. Los rechazos del campo `thinking`, de [firmas de pensamiento](https://platform.claude.com/docs/en/build-with-claude/extended-thinking), y de mensajes del sistema a mitad de conversación se recuperan de esta manera. Los rechazos de gestión de contexto y campo de esquema de herramienta no reintentan; esos errores `400` llegan al desarrollador.

La lógica de reintento coincide con la redacción del error del proveedor ascendente, así que reenvíe cuerpos de respuesta de error sin modificar. Una puerta de enlace que envuelve errores ascendentes en su propio sobre rompe la ruta de recuperación incluso cuando preserva el código de estado.

<h3 id="disable-pre-release-capabilities">
  Deshabilitar capacidades de pre-lanzamiento
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` detiene Claude Code de enviar capacidades de pre-lanzamiento y sus campos de cuerpo en cada proveedor, incluida la gestión de contexto y los campos de herramienta beta. No afecta el razonamiento adaptativo, que se selecciona por modelo en lugar de por beta, y nunca suprime la capacidad OAuth que la autenticación de suscripción requiere.

El conjunto de capacidades que Claude Code envía crece en las versiones. Para cadenas de encabezado beta actuales, consulte la [referencia de encabezados beta](https://platform.claude.com/docs/en/api/beta-headers); pruebe su puerta de enlace contra nuevas versiones de Claude Code en lugar de fijar a una lista observada.

<h2 id="model-discovery">
  Descubrimiento de modelos
</h2>

Cuando `ANTHROPIC_BASE_URL` apunta a una puerta de enlace que expone el formato Anthropic Messages, Claude Code puede consultar el punto final `/v1/models` de la puerta de enlace al inicio y agregar los modelos devueltos al selector `/model`.

Los desarrolladores lo habilitan estableciendo [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/es/env-vars), en su propio entorno o a través de configuración administrada. El descubrimiento está desactivado de forma predeterminada para que las puertas de enlace respaldadas por una clave API compartida no expongan cada modelo que la clave puede acceder a cada usuario. Esto requiere Claude Code v2.1.129 o posterior.

<h3 id="when-discovery-runs">
  Cuándo se ejecuta el descubrimiento
</h3>

El descubrimiento se aplica solo al formato Anthropic Messages. No se ejecuta cuando:

* Se establece cualquier variable de proveedor `CLAUDE_CODE_USE_*`, incluso si `ANTHROPIC_BASE_URL` también se establece
* `ANTHROPIC_BASE_URL` no se establece o apunta a `api.anthropic.com`
* El tráfico no esencial está deshabilitado, a través de [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars) o política organizacional

<h3 id="request-and-response">
  Solicitud y respuesta
</h3>

La solicitud es `GET /v1/models?limit=1000` con un tiempo de espera de 3 segundos, y cualquier redirección se trata como fallo para que la credencial no pueda filtrarse a un destino de redirección. Una puerta de enlace que responde lentamente o redirige `/v1/models`, incluso `http` a `https`, falla el descubrimiento silenciosamente; sirva el punto final directamente en la URL base configurada.

La solicitud de descubrimiento envía exactamente un encabezado de credencial:

* `ANTHROPIC_AUTH_TOKEN` como un token portador, cuando se establece
* De lo contrario, la clave API resuelta, incluido un valor [`apiKeyHelper`](/docs/es/llm-gateway-connect#rotate-credentials-with-apikeyhelper), en el encabezado `x-api-key`

Esto difiere de las solicitudes de inferencia, que envían un valor auxiliar en ambos encabezados. Una puerta de enlace que autentica `/v1/models` debe aceptar `x-api-key` para implementaciones auxiliares. Cualquier encabezado de `ANTHROPIC_CUSTOM_HEADERS` también se incluye.

Claude Code lee `id` y el `display_name` opcional de cada entrada en la matriz `data` de la respuesta, e ignora entradas cuyo `id` no comienza con `claude` o `anthropic`:

```json theme={null}
{
  "data": [
    { "id": "claude-sonnet-4-6", "display_name": "Claude Sonnet 4.6" },
    { "id": "claude-opus-4-8" }
  ]
}
```

<h3 id="picker-entries-and-caching">
  Entradas del selector y almacenamiento en caché
</h3>

El selector es la lista de modelos interactiva que se abre cuando un desarrollador ejecuta `/model` en Claude Code. Cada entrada descubierta está etiquetada como "Desde puerta de enlace" y utiliza `display_name` cuando se proporciona. La [configuración administrada `availableModels`](/docs/es/settings#available-settings) limita lo que el descubrimiento puede agregar.

Una ID descubierta se omite cuando coincide exactamente con una fila ya en el selector, o cuando tanto la ID descubierta como la existente se resuelven en [Fable](/docs/es/model-config#work-with-fable-5). A partir de Claude Code v2.1.197, una ID explícita descubierta también se pliega en una entrada integrada cuando ambas se resuelven en el mismo modelo. Las filas integradas se clave en alias como `sonnet`, por lo que una ID explícita descubierta del modelo al que el alias se resuelve actualmente, como `claude-sonnet-5`, se colapsa en la fila `sonnet`, mientras que una ID a la que el alias no se resuelve, como `claude-sonnet-4-6`, aún agrega su propia fila "Desde puerta de enlace" junto a la entrada integrada.

Los resultados se almacenan en caché en `~/.claude/cache/gateway-models.json`, o `%USERPROFILE%\.claude\cache\gateway-models.json` en Windows, y se actualizan en cada inicio. Si la solicitud falla o la puerta de enlace no implementa `/v1/models`, el selector vuelve a la lista en caché del inicio anterior o a la lista de modelos integrada. Si su puerta de enlace sirve modelos de Claude bajo alias que no coinciden con el filtro de descubrimiento, los desarrolladores pueden agregar esos alias manualmente con las [variables de configuración de modelo](/docs/es/model-config).

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para el resto del conjunto de documentación de puerta de enlace y las referencias de API subyacentes:

* [Descripción general de puertas de enlace](/docs/es/gateways): qué es una puerta de enlace y cómo elegir entre la puerta de enlace de aplicaciones Claude y otro producto
* [Otras puertas de enlace LLM](/docs/es/llm-gateway): cómo implementar una puerta de enlace que su organización ejecuta y cómo interactúa con suscripciones de claude.ai
* [Implementar una puerta de enlace LLM para su organización](/docs/es/llm-gateway-rollout): la lista de verificación de administrador que utiliza este contrato
* [Conectar Claude Code a una puerta de enlace LLM](/docs/es/llm-gateway-connect): configuración por desarrollador y la tabla de solución de problemas
* [Referencia de encabezados beta](https://platform.claude.com/docs/en/api/beta-headers): el conjunto actual de valores `anthropic-beta`
* [API de mensajes](https://platform.claude.com/docs/en/api/messages): el formato de API que implementa una puerta de enlace con formato Anthropic
