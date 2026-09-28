> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Guía de compatibilidad de Claude Code gateway

> Mantenga un gateway LLM compatible con Claude Code: los endpoints que llama, los encabezados y campos de cuerpo a reenviar, y qué se rompe cuando se eliminan.

Esta página documenta las solicitudes que Claude Code envía a un gateway, incluidos los endpoints que llama, los encabezados y campos de cuerpo que el gateway debe reenviar, y qué características dejan de funcionar cuando no lo hace. Está escrita para operadores que configuran un producto gateway para trabajar con Claude Code.

El [Claude apps gateway](/docs/es/claude-apps-gateway), el gateway autohospedado de Anthropic, sirve su propia referencia de endpoint en `GET /protocol`, cubriendo los endpoints de inicio de sesión, inferencia, configuración administrada, descubrimiento de modelos y telemetría de ese gateway. Es un documento separado de esta guía.

<Note>
  * Para implementar un gateway existente o de terceros en su organización, consulte [Implementar un gateway LLM](/docs/es/llm-gateway-rollout)
  * Si es un desarrollador individual que autentica Claude Code en un gateway con una credencial que le proporcionaron, consulte [Conectar Claude Code a un gateway LLM](/docs/es/llm-gateway-connect)
</Note>

Esta página cubre:

* [Formatos de API](#api-formats) y los endpoints a servir para cada uno
* [Comportamiento del cliente por método de conexión](#how-the-connection-method-changes-client-behavior): cómo los ID de modelo, valores de `anthropic-beta`, campos de solicitud y valores predeterminados difieren entre los formatos y un inicio de sesión de Claude apps gateway
* [Encabezados de solicitud](#request-headers): cuáles deben llegar al upstream y cuáles su gateway puede consumir
* [Encabezados de respuesta](#response-headers): qué devolver para que la detección de estancamiento, reintentos y la visualización del límite de uso funcionen
* El [bloque de atribución del prompt del sistema](#system-prompt-attribution-block) y cómo interactúa con el almacenamiento en caché de prompts
* [Paso a través de características](#feature-pass-through): qué se rompe cuando se eliminan encabezados o campos de cuerpo
* [Descubrimiento de modelos](#model-discovery)

Esta página utiliza dos términos para lo que su gateway hace con cada encabezado y campo de cuerpo:

* **Reenviar sin cambios**: pasarlo al upstream byte por byte
* **Consumir**: el gateway puede leerlo para enrutamiento, atribución o rastreo y no necesita reenviarlo

Cualquier cosa no marcada como reenviar sin cambios es suya para consumir o ignorar.

<h2 id="api-formats">
  Formatos de API
</h2>

Una puerta de enlace debe exponer al menos uno de los siguientes formatos de API a los clientes de Claude Code. Un cliente elige un formato y apunta Claude Code a su puerta de enlace con las variables en la columna Seleccionado por de la tabla siguiente.

Google Cloud's Agent Platform es el punto de conexión de Claude de Google Cloud, anteriormente Vertex AI; sus nombres de variables mantienen la ortografía `VERTEX`.

| Formato                                  | Seleccionado por                                             | Puntos de conexión                                                                                              | Reenviar sin cambios                                                                                                   |
| :--------------------------------------- | :----------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------- |
| Anthropic Messages                       | `ANTHROPIC_BASE_URL`                                         | `/v1/messages`, `/v1/messages/count_tokens` (opcional)                                                          | encabezados de solicitud `anthropic-beta` y `anthropic-version`                                                        |
| Amazon Bedrock InvokeModel               | `ANTHROPIC_BEDROCK_BASE_URL` con `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`, `/model/{model}/count-tokens` (opcional) | campos de cuerpo de solicitud `anthropic_beta` y `anthropic_version`                                                   |
| Google Cloud's Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` con `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (opcional)                                        | encabezados de solicitud `anthropic-beta` y `anthropic-version`, y el campo de cuerpo de solicitud `anthropic_version` |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry y Claude Platform en AWS
</h3>

Microsoft Foundry y la [Claude Platform en AWS](/docs/es/claude-platform-on-aws) implementan el formato Anthropic Messages. Claude Code se enruta a través de sus propias variables, `ANTHROPIC_FOUNDRY_BASE_URL` y `ANTHROPIC_AWS_BASE_URL`, pero una puerta de enlace que las frontal implementa la fila Anthropic Messages anterior. Una puerta de enlace que frontal la Claude Platform en AWS también debe reenviar el encabezado `anthropic-workspace-id`, que [esa plataforma requiere en cada solicitud](/docs/es/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  Puntos de conexión opcionales y tráfico de inicio
</h3>

Los puntos de conexión de conteo de tokens son los únicos opcionales: cuando están ausentes, Claude Code recurre a una estimación basada en caracteres del uso del contexto.

Coincida con la ruta, no con la URL completa:

* Las solicitudes de inferencia se publican en `/v1/messages?beta=true`
* El método de Google Cloud's Agent Platform sufijos se adjuntan a la ruta del modelo del editor, como en `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`

Una puerta de enlace también ve tráfico de inicio de mejor esfuerzo que puede rechazar sin romper nada. Una puerta de enlace en formato Anthropic Messages recibe una sonda de calentamiento de conexión `HEAD /api/hello`, que Claude Code omite cuando se configura un proxy HTTP o certificado de cliente. Una puerta de enlace en formato Amazon Bedrock recibe una solicitud `GET /inference-profiles?type=SYSTEM_DEFINED` y, cuando el modelo configurado es un perfil de inferencia, búsquedas `GET /inference-profiles/{profile}`.

La verificación de disponibilidad de [modo rápido](/docs/es/fast-mode) nunca aparece en los registros de la puerta de enlace: llama a `api.anthropic.com` directamente en lugar de seguir `ANTHROPIC_BASE_URL`, por lo que en una red que bloquea la salida directa a `api.anthropic.com`, el modo rápido puede informar un error de conectividad mientras que la inferencia a través de la puerta de enlace sigue funcionando. La [verificación de seguridad del dominio WebFetch](/docs/es/data-usage#webfetch-domain-safety-check) también llama a `api.anthropic.com` directamente. [Usar modo rápido detrás de proxies y puertas de enlace LLM](/docs/es/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) cubre las variables que lo restauran.

<h3 id="streaming">
  Streaming
</h3>

Transmita respuestas de inferencia. Claude Code lee la transmisión a medida que llega, por lo que si su puerta de enlace almacena en búfer respuestas completas antes de retransmitirlas, Claude Code se detiene.

Cuando el cliente habla el formato Amazon Bedrock, retransmita el cuerpo de respuesta `InvokeModelWithResponseStream` y su encabezado `Content-Type: application/vnd.amazon.eventstream` sin modificar, y no convierta la transmisión a eventos enviados por el servidor. Consulte [Errores de streaming detrás de una puerta de enlace o proxy](/docs/es/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

Reenvíe también los pings de mantenimiento de conexión. En conexiones a través de `ANTHROPIC_BASE_URL` o `ANTHROPIC_AWS_BASE_URL`, Claude Code cuenta cada byte que su puerta de enlace retransmite, incluidos los eventos SSE `ping` y líneas de comentario, e interrumpe una transmisión que se queda en silencio durante 300 segundos de forma predeterminada. Los pings del proveedor ascendente son el único tráfico durante pausas de pensamiento prolongado, por lo que si su puerta de enlace los elimina o los almacena en búfer, Claude Code interrumpe la transmisión durante esas pausas; [Reintentos automáticos](/docs/es/errors#automatic-retries) cubre lo que una transmisión interrumpida informa según cuán lejos había progresado la respuesta. Un proveedor ascendente que no envía pings en absoluto, como el flujo de eventos binarios de Amazon Bedrock, deja esas pausas sin nada que reenviar. Al traducir desde tal proveedor ascendente, emita sus propios eventos `ping` durante brechas silenciosas. Las puertas de enlace alcanzadas a través de `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, o `ANTHROPIC_FOUNDRY_BASE_URL` no están envueltas por este perro guardián a nivel de bytes, incluso cuando retransmiten el formato Anthropic Messages; allí, un [tiempo de espera de inactividad de 5 minutos](/docs/es/env-vars) interrumpe una transmisión silenciosa en su lugar, y en conexiones `ANTHROPIC_BEDROCK_BASE_URL` puede agregar el perro guardián de bytes con [`CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK`](/docs/es/env-vars).

<h3 id="format-mismatch-with-the-upstream">
  Desajuste de formato con el proveedor ascendente
</h3>

El formato que habla el cliente determina qué recibe su puerta de enlace. El modo de fallo común es un desajuste entre el formato que el cliente envía a su puerta de enlace y el formato que el proveedor ascendente detrás de él acepta.

* Cuando el cliente habla el formato Amazon Bedrock o Google Cloud's Agent Platform, Claude Code envía solo el subconjunto de su conjunto de capacidades completo que esos proveedores aceptan
* Cuando el cliente habla el formato Anthropic Messages, Claude Code envía el conjunto completo, incluso si su puerta de enlace se reenvía a un proveedor ascendente Amazon Bedrock o Google Cloud's Agent Platform

Hacer puente de esa diferencia es el trabajo de su puerta de enlace. [Paso de características](#feature-pass-through) describe qué se rompe cuando no lo hace.

Si su proveedor ascendente es Amazon Bedrock o Google Cloud's Agent Platform, puede evitar el puente exponiendo el formato de ese proveedor en su lugar. [Enrutar a un proveedor de nube a través de una puerta de enlace](/docs/es/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) muestra la configuración del cliente para ese formato.

<h2 id="how-the-connection-method-changes-client-behavior">
  Cómo el método de conexión cambia el comportamiento del cliente
</h2>

La forma en que un desarrollador se conecta a su gateway determina qué IDs de modelo, valores de `anthropic-beta` y campos de solicitud envía Claude Code, y qué valores predeterminados aplica. Su gateway ve uno de tres comportamientos del cliente:

* **Formato de Amazon Bedrock o Agent Platform**: el desarrollador establece `CLAUDE_CODE_USE_BEDROCK=1` con `ANTHROPIC_BEDROCK_BASE_URL`, o `CLAUDE_CODE_USE_VERTEX=1` con `ANTHROPIC_VERTEX_BASE_URL`, apuntando a su gateway. Claude Code utiliza los IDs de modelo, campos de solicitud y valores predeterminados de ese proveedor.
* **Formato de Anthropic Messages**: el desarrollador establece `ANTHROPIC_BASE_URL` en su gateway. Claude Code trata el gateway como la API de Claude y no puede determinar a cuál upstream lo reenvía.
* **Inicio de sesión en el gateway de aplicaciones Claude**: el desarrollador inicia sesión en un [gateway de aplicaciones Claude](/docs/es/claude-apps-gateway). Ese gateway habla el formato de Anthropic Messages pero puede enrutar a cualquier upstream, por lo que Claude Code envía solo los valores de `anthropic-beta` y las suposiciones de capacidad de modelo que Amazon Bedrock y Agent Platform también aceptan.

<h3 id="requests-and-defaults-by-connection-method">
  Solicitudes y valores predeterminados por método de conexión
</h3>

La tabla siguiente compara los tres métodos de conexión, un comportamiento por fila. Omite Microsoft Foundry y Claude Platform en AWS, que también utilizan el formato de Anthropic Messages pero a los que Claude Code accede a través de sus propias variables. Para esos, consulte las páginas de [Microsoft Foundry](/docs/es/microsoft-foundry) y [Claude Platform en AWS](/docs/es/claude-platform-on-aws).

| Comportamiento                                                                                                               | Formato de Amazon Bedrock o Agent Platform                                                                                                                                                                                                  | Formato de Anthropic Messages                                                                                                                                                                                      | Inicio de sesión en el gateway de aplicaciones Claude                                                                                         |
| :--------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| IDs de modelo en solicitudes de forma predeterminada                                                                         | La forma del proveedor, como `us.anthropic.claude-opus-4-8` en Amazon Bedrock                                                                                                                                                               | IDs de Anthropic, como `claude-opus-4-8`                                                                                                                                                                           | IDs de Anthropic                                                                                                                              |
| Valores de `anthropic-beta` enviados                                                                                         | El subconjunto que Amazon Bedrock y Agent Platform aceptan                                                                                                                                                                                  | El conjunto completo descrito en [paso de características](#feature-pass-through), a menos que el desarrollador establezca [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](#disable-pre-release-capabilities)           | El subconjunto que Amazon Bedrock y Agent Platform aceptan                                                                                    |
| Campos de solicitud para un ID de modelo que Claude Code no reconoce, como un alias de gateway                               | Pensamiento con un presupuesto fijo en lugar de razonamiento adaptativo, y sin campos de gestión de esfuerzo o contexto                                                                                                                     | Todo lo que los modelos Claude actuales aceptan en la API de Claude, incluido el razonamiento adaptativo, el esfuerzo y la gestión del contexto, que un upstream de Amazon Bedrock o Agent Platform puede rechazar | Igual que el formato de Amazon Bedrock o Agent Platform                                                                                       |
| [TTL de caché de prompt](/docs/es/prompt-caching#choose-the-ttl-yourself) de una hora cuando un desarrollador opta por participar | Solicitado a través del campo `ttl` en `cache_control`, sin valor beta                                                                                                                                                                      | Solicitado a través del campo `ttl` más un valor `extended-cache-ttl` en `anthropic-beta`, que debe reenviar                                                                                                       | Consulte la tabla de [disponibilidad y limitaciones](/docs/es/claude-apps-gateway#availability-and-limitations) del gateway de aplicaciones Claude |
| Modelo para [tareas en segundo plano](/docs/es/costs#background-token-usage) a menos que `ANTHROPIC_DEFAULT_HAIKU_MODEL` fije uno | El modelo Sonnet predeterminado, o el modelo principal una vez que se selecciona uno, como describen las páginas de [Amazon Bedrock](/docs/es/amazon-bedrock#4-pin-model-versions) y [Agent Platform](/docs/es/google-vertex-ai#5-pin-model-versions) | El modelo principal, o el modelo Haiku predeterminado cuando `ANTHROPIC_API_KEY` o `apiKeyHelper` proporciona una clave de Anthropic Console y `ANTHROPIC_AUTH_TOKEN` no está establecido                          | El modelo principal                                                                                                                           |

Para las características que cada conexión admite y la telemetría que envía a Anthropic de forma predeterminada, consulte [Disponibilidad de características](/docs/es/feature-availability#availability-by-model-provider) y [Comportamientos predeterminados por proveedor de API](/docs/es/data-usage#default-behaviors-by-api-provider).

<h3 id="settings-for-unrecognized-model-ids">
  Configuración para IDs de modelo no reconocidos
</h3>

Dos configuraciones del lado del cliente cambian lo que Claude Code asume para un ID de modelo que no reconoce, independientemente del método de conexión que utilice el desarrollador:

* **Ventana de contexto**: Claude Code asume 200K, o 1M cuando el ID lleva `[1m]`. Para declarar la ventana real, consulte [Corrija la ventana para un gateway o ID de modelo personalizado](/docs/es/model-config#correct-the-window-for-a-gateway-or-custom-model-id)
* **Capacidades**: para dar a un alias de gateway las capacidades del modelo detrás de él, asigne el ID de Anthropic de ese modelo a su alias con una entrada [`modelOverrides`](/docs/es/errors#unrecognized-model-id-on-a-request) en la configuración que distribuya. Para saber dónde se aplican las variables `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES`, consulte [paso de características](#feature-pass-through)

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

<h3 id="gateway-hint-headers">
  Encabezados de sugerencia de puerta de enlace
</h3>

Claude Code también puede enviar sugerencias de enrutamiento: hechos por solicitud que una puerta de enlace o enrutador puede usar para programar, almacenar en caché o atribuir una solicitud. Requiere Claude Code v2.1.273 o posterior.

Si una solicitud los lleva depende de dónde Claude Code los envía:

* Conexión directa a la API de Anthropic: enviado por defecto
* URL base personalizada: desactivado por defecto, porque un proxy que rechaza encabezados desconocidos fallaría la solicitud. Para recibirlos, establezca [`CLAUDE_CODE_GATEWAY_HINT_HEADERS=1`](/docs/es/env-vars) para sus desarrolladores, por ejemplo en el bloque `env` de [configuración administrada](/docs/es/managed-settings)
* Cualquier otro backend, incluidos Amazon Bedrock, Agent Platform de Google Cloud, Microsoft Foundry y Claude Platform en AWS: enviado solo cuando `CLAUDE_CODE_GATEWAY_HINT_HEADERS=1` está establecido

Establecer `CLAUDE_CODE_GATEWAY_HINT_HEADERS` a `0` detiene los encabezados en cada conexión.

Los encabezados llevan solo lo que las filas a continuación enumeran: vocabularios fijos, nombres de herramientas y duraciones, nunca texto de solicitud o contenidos de archivo. Cada valor es ASCII imprimible.

| Encabezado                          | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `x-claude-code-request-class`       | Qué tipo de solicitud es esta: `main` para un turno de la conversación principal, `subagent` para un turno de un [subagente](/docs/es/sub-agents), `workflow` para un agente ejecutándose dentro de un flujo de trabajo, `compaction` para la solicitud de resumen que compacta una conversación, o `auxiliary` para solicitudes secundarias como títulos de sesión, clasificadores y resúmenes. Enviado en cada solicitud                                                                                                                                                                        |
| `x-claude-code-agent-type`          | El tipo de subagente que emitió la solicitud: un nombre de tipo de agente integrado como `Explore`, `Plan` o `general-purpose`, o `custom` para un agente definido por el usuario, `teammate` para un miembro del [equipo de agentes](/docs/es/agent-teams) ejecutándose en el proceso del líder, o `fork` para un [fork](/docs/es/sub-agents#fork-the-current-conversation). Presente solo en los turnos propios de un subagente; la compactación o solicitudes secundarias de un subagente mantienen el ID del agente pero no llevan tipo. Un nombre de agente elegido por el usuario nunca se envía |
| `x-claude-code-compaction`          | Presente en la solicitud que resume la conversación durante una [compactación](/docs/es/prompt-caching#compacting-the-conversation). El valor dice qué la activó: `auto` cuando la ventana de contexto se acercaba a la capacidad, `manual` para `/compact`, o `reactive` cuando la API rechazó una solicitud como demasiado larga. Ausente en todas las otras solicitudes                                                                                                                                                                                                                        |
| `x-claude-code-context-compacted`   | Presente una vez, en la primera solicitud de conversación principal después de una compactación, con los mismos valores que `x-claude-code-compaction`. El prefijo de conversación antes de esta solicitud ya no se usa, así que un caché codificado en él puede descartarse                                                                                                                                                                                                                                                                                                                 |
| `x-claude-code-prev-tool-durations` | Tiempo de ejecución medido de las llamadas de herramienta cuyos resultados lleva esta solicitud, como `<name>=<ms>;<name>=<ms>`, por ejemplo `Bash=742;Read=9`. Enviado en la siguiente solicitud de la misma conversación después de un lote de llamadas de herramienta, desde la sesión principal o un subagente                                                                                                                                                                                                                                                                           |

Antes de analizar `x-claude-code-prev-tool-durations`, verifique cómo Claude Code construye el valor y qué deja fuera:

* Entradas: una por llamada de herramienta que se ejecutó, en el orden en que se recopiló su resultado, en milisegundos completos
* Límite: Claude Code envía como máximo 32 entradas y 4 KB, manteniendo las primeras entradas
* Codificación: los nombres de herramientas están codificados en porcentaje, cubriendo `%`, `;`, `=`, coma, espacio y cualquier carácter fuera de ASCII imprimible
* Análisis: dividir en `;`, luego en `=`, y decodificar cada nombre
* Ausencia: las llamadas de compactación, solicitudes secundarias y la primera solicitud de un nuevo mensaje nunca la llevan. No lea un encabezado faltante como un turno que no ejecutó herramientas
* Tiempos: cada uno excluye solicitudes de permiso y hooks, y las llamadas de herramienta paralelas cada una reporta su propio tiempo, así que las entradas no suman la brecha entre solicitudes

<h3 id="forward-as-open-lists">
  Reenviar como listas abiertas
</h3>

Trate los encabezados y campos de cuerpo como listas abiertas, no cerradas. Claude Code gana capacidades en las versiones, y llegan como nuevos valores `anthropic-beta`, nuevos campos de cuerpo de solicitud y ocasionalmente nuevos encabezados `anthropic-*` o `x-claude-code-*`.

Al reenviar a un proveedor ascendente con formato Anthropic, pase encabezados de solicitud `anthropic-*` y campos de cuerpo de solicitud sin cambios en lugar de permitir los que ve hoy. Una puerta de enlace fijada a una lista observada elimina el encabezado o campo de la siguiente capacidad y lo rompe en la versión que la introduce.

La excepción es un proveedor ascendente que no es Anthropic, como Amazon Bedrock o Agent Platform de Google Cloud, donde cerrar la diferencia de esquema es el trabajo de la puerta de enlace; consulte [paso de características](#feature-pass-through).

<h2 id="response-headers">
  Encabezados de respuesta
</h2>

Claude Code lee estos encabezados de respuesta para detectar flujos estancados, para decidir si y cuándo reintentar, y para mostrar límites de uso. La tabla enumera qué devolver para cada uno. También reenvíe los cuerpos de respuesta de error sin modificar, para que la [recuperación de rechazo de capacidad](#automatic-retry-and-error-forwarding) de Claude Code pueda coincidir con la redacción del error ascendente.

| Encabezado                      | Qué devolver y por qué                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content-type`                  | Devuelva `text/event-stream` en respuestas de formato Anthropic Messages transmitidas, y `application/vnd.amazon.eventstream`, sin modificar, en respuestas de formato Amazon Bedrock, donde [un tipo diferente falla la solicitud](/docs/es/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy). [Streaming](#streaming) enumera qué conexiones ejecutan detección de estancamiento en estos flujos |
| `retry-after`                   | Devuelva segundos enteros en lugar de una fecha HTTP. Claude Code espera al menos ese tiempo antes del siguiente [reintento automático](/docs/es/errors#automatic-retries), y fuera de sesiones [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/es/env-vars) un valor superior a 60 detiene los reintentos y muestra el error de inmediato                                                                                  |
| `x-should-retry`                | Pase el valor ascendente sin cambios. Claude Code lee este encabezado como una entrada cuando decide si reintentar una solicitud fallida: `true` marca la respuesta como reintentable y `false` la marca como no reintentable. Para recuentos de reintentos, retroceso y qué fallos reintenta Claude Code, consulte [reintentos automáticos](/docs/es/errors#automatic-retries)                             |
| `anthropic-ratelimit-unified-*` | Reenvíe los valores ascendentes sin cambios en cada respuesta. Claude Code los lee en respuestas exitosas para mostrar el uso contra los límites del plan a los desarrolladores que iniciaron sesión con claude.ai, y en un `429` para distinguir un límite de plan o límite de gasto de un acelerador temporal; consulte [límites de uso](/docs/es/errors#usage-limits)                                    |

<h2 id="system-prompt-attribution-block">
  Bloque de atribución del mensaje del sistema
</h2>

Claude Code antepone un bloque de atribución corto al mensaje del sistema que contiene la versión del cliente y una huella digital derivada de la conversación. El punto final `api.anthropic.com` elimina el bloque antes de procesar cuando llega sin cambios como el primer bloque del sistema, por lo que no afecta el almacenamiento en caché de solicitudes de primera parte. Cualquier otro proveedor ascendente lo recibe como parte del mensaje.

La eliminación es posicional, por lo que solo funciona cuando la puerta de enlace reenvía la matriz `system` sin cambios. Para mantener el bloque fuera del mensaje sin perder otro contenido del sistema:

* Reenvíe la matriz `system` exactamente como se recibió, manteniendo el bloque primero: anteponer otro bloque del sistema, reordenar la matriz o convertirla en una sola cadena anula la eliminación, y el bloque luego llega al modelo y a la clave de caché de solicitud.
* Mantenga el bloque en su propia entrada de matriz: el punto final trata un bloque fusionado que comienza con el encabezado de atribución como atribución en su totalidad y descarta todo lo fusionado en él, incluido el resto del mensaje del sistema.
* Si su puerta de enlace debe remodelar el contenido del sistema, establezca [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/es/env-vars) para que Claude Code omita el bloque. Anthropic y los puntos finales de Claude de los proveedores de nube leen el bloque para atribución, así que omítalo en el cliente en lugar de eliminarlo o moverlo en la puerta de enlace.

La variable existe para compatibilidad con almacenamiento en caché de puertas de enlace y terceros, no como control de privacidad: en una conexión directa la solicitud completa ya va a la API de Anthropic de cualquier forma. Cuando se cumplen ambas condiciones, Claude Code mantiene el bloque en solicitudes del clasificador de [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) incluso cuando establece la variable en `0`:

* Las solicitudes van a `api.anthropic.com`, con `ANTHROPIC_BASE_URL` sin establecer o nombrando ese host y sin proveedor de terceros seleccionado.
* La credencial activa no es una [credencial de perfil de Anthropic o federación](/docs/es/authentication#anthropic-profiles-and-federation-credentials).

Las solicitudes del clasificador omiten el resto del mensaje del sistema de Claude Code, por lo que en esas solicitudes el bloque es el único marcador en el cuerpo de la solicitud que las identifica como tráfico de Claude Code. Cuando falla cualquiera de las condiciones, a través de una puerta de enlace LLM, en un proveedor de terceros, o con una credencial de perfil o federación activa, establecer `0` elimina el bloque de las solicitudes del clasificador también. Antes de v2.1.229, esta excepción no existía: establecer `0` eliminaba el bloque de esas solicitudes del clasificador, y cuando la API rechazaba las solicitudes no identificadas, el modo automático fallaba en cada acción que enviaba al clasificador.

Desde Claude Code v2.1.181, el bloque es estable para la vida útil de una conversación cuando las solicitudes se enrutan a través de una URL base personalizada, por lo que una caché de solicitud de puerta de enlace con clave en el cuerpo de solicitud completo funciona sin deshabilitarlo, y cualquier proveedor al que su puerta de enlace reenvíe recibe un prefijo de solicitud estable. Antes de v2.1.181, el bloque incluía un token por solicitud que cambiaba el inicio del mensaje del sistema en cada solicitud. En esas versiones, establezca `CLAUDE_CODE_ATTRIBUTION_HEADER=0` cuando su puerta de enlace haga cualquiera de estas cosas:

* Implementa una caché de solicitud con clave en el cuerpo de la solicitud.
* Reenvía solicitudes a un proveedor de terceros como Amazon Bedrock, Microsoft Foundry o la Plataforma de Agentes de Google Cloud, en el formato de Mensajes de Anthropic o el del proveedor, donde el prefijo cambiante reduce la reutilización de caché de solicitud en ese proveedor.

<h2 id="feature-pass-through">
  Paso de características
</h2>

Claude Code trata una puerta de enlace `ANTHROPIC_BASE_URL` como un punto final con formato Anthropic y le envía los encabezados beta y campos de cuerpo de solicitud que envía a `api.anthropic.com`, excepto un pequeño conjunto de diagnósticos y valores predeterminados reservados para conexiones directas, como el valor predeterminado de transmisión de herramientas de grano fino cubierto a continuación. Ese conjunto varía según la versión, así que no dependa de su contenido.

Las capacidades que agregan campos de cuerpo los emparejan con un encabezado beta, y el par viaja junto. Una puerta de enlace que elimina el encabezado mientras pasa el cuerpo, o reenvía un cuerpo con formato Anthropic a un proveedor ascendente con un esquema diferente, produce errores `400` duros; solo cuando ambas mitades están ausentes juntas la característica se apaga silenciosamente. Una puerta de enlace que reescribe o redacta cuerpos de solicitud para inspección de contenido rompe el emparejamiento de la misma manera que la eliminación, así que inspeccione sin modificar. La tabla señala dónde una característica se desvía del emparejamiento.

La transmisión de herramientas de grano fino es uno de los valores predeterminados de conexión directa: está desactivada de forma predeterminada siempre que las solicitudes se enruten a través de una URL base personalizada, y una puerta de enlace la recibe cuando los desarrolladores establecen [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/es/env-vars).

| Característica                                                                                                                                                                                                                                      | Encabezado y par de cuerpo                                                                                                                                                                                                 | Síntoma cuando se rompe                                                                                                                                       | Remediación                                                                                                                                              |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Razonamiento adaptativo](/docs/es/model-config#adjust-effort-level)                                                                                                                                                                                     | Sin encabezado beta. Claude Code envía `thinking: {"type": "adaptive"}` para Claude 4.6 y posterior, y trata nombres de modelo que no reconoce, como alias de puerta de enlace, como modelos actuales que reciben el campo | `400` nombrando el campo `thinking` o la etiqueta `adaptive` cuando la compilación del modelo ascendente no la acepta                                         | Actualice el proveedor ascendente. En Opus 4.6 y Sonnet 4.6, los desarrolladores pueden establecer `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1` en su lugar |
| [Gestión de contexto](https://platform.claude.com/docs/en/build-with-claude/context-editing)                                                                                                                                                        | El encabezado beta de gestión de contexto se empareja con el campo de cuerpo `context_management`                                                                                                                          | `400` con `Extra inputs are not permitted`. Común cuando una puerta de enlace acepta solicitudes con formato Anthropic pero las reenvía a Amazon Bedrock      | Reenvíe ambos, o [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/es/env-vars)                                                                              |
| [Contexto extendido](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) y [pensamiento intercalado](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | Solo encabezados beta, sin campo de cuerpo                                                                                                                                                                                 | Silenciosamente no disponible cuando se elimina el encabezado; el proveedor ascendente nunca ve la solicitud de capacidad                                     | Reenvíe `anthropic-beta` textualmente                                                                                                                    |
| Campos de [herramienta](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview) beta                                                                                                                                                | Los encabezados beta relacionados con herramientas se emparejan con campos de esquema de herramienta como `strict` y `defer_loading`                                                                                       | `400` nombrando el campo de esquema de herramienta no reconocido cuando el cuerpo pasa sin su encabezado                                                      | Reenvíe ambos, o [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities)                                                         |
| [Esfuerzo](https://platform.claude.com/docs/en/build-with-claude/effort) y [salidas estructuradas](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                        | El campo de cuerpo `output_config` lleva esfuerzo, formato de salida estructurada y configuración de presupuesto de tarea; cada uno se empareja con su propio encabezado beta                                              | `400` nombrando `output_config`, a menudo `Extra inputs are not permitted`, en proveedores ascendentes Amazon Bedrock y Agent Platform                        | Reenvíe el campo y sus encabezados juntos                                                                                                                |
| [Almacenamiento en caché de solicitudes](/docs/es/prompt-caching)                                                                                                                                                                                        | Sin emparejamiento beta. Claude Code adjunta marcadores `cache_control` a bloques `system` y a entradas `messages`, incluidas entradas `role: "system"` añadidas a mitad de conversación                                   | Sin error: la conversación se factura como entrada sin caché en cada turno, visible como `input_tokens` alto con poca o ninguna actividad de caché en `usage` | Reenvíe `cache_control` sin cambios dondequiera que aparezca, y no convierta bloques de forma `system` o contenido de mensaje a cadenas simples          |
| [Conteo de tokens](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                                            | Sin emparejamiento beta; utiliza el punto final `count_tokens`                                                                                                                                                             | Sin error: Claude Code vuelve a un estimado basado en caracteres, así que `/context` muestra conteos aproximados                                              | Exponga el punto final para conteos de tokens exactos                                                                                                    |

Las [variables](/docs/es/model-config) `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` declaran capacidades de modelo solo en las configuraciones del proveedor: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, y [`CLAUDE_CODE_USE_MANTLE`](/docs/es/amazon-bedrock#use-the-mantle-endpoint). No tienen efecto detrás de una puerta de enlace `ANTHROPIC_BASE_URL`.

<h3 id="automatic-retry-and-error-forwarding">
  Reintento automático y reenvío de errores
</h3>

Lo que Claude Code hace después de un rechazo ascendente depende de lo que fue rechazado:

* Cuando el proveedor ascendente rechaza el campo `thinking`, un mensaje del sistema a mitad de conversación, o el marcador `cache_control` en tal mensaje, Claude Code reintenta la solicitud y deshabilita la capacidad rechazada para el resto de la conversación
* Cuando el proveedor ascendente rechaza una [firma de pensamiento](https://platform.claude.com/docs/en/build-with-claude/extended-thinking), incluido con un `400` cuyo mensaje dice que el bloque está `bound to a different conversation`, Claude Code elimina bloques de pensamiento anteriores de la solicitud, reintenta, y los mantiene fuera de cada solicitud posterior. Las nuevas respuestas aún incluyen pensamiento
* Cuando la puerta de enlace o su proveedor ascendente rechaza la entrada de [herramienta asesor](/docs/es/advisor) en `tools` como un tipo de herramienta no reconocido, Claude Code reintenta la solicitud una vez sin esa entrada y su valor `anthropic-beta`. Las solicitudes posteriores a esa URL base dejan el asesor fuera hasta que Claude Code se cierre, y `/advisor` no está disponible para el desarrollador durante ese tiempo. Claude Code reconoce este rechazo por una respuesta `400` o `422` cuyo mensaje nombra el tipo de herramienta después de `Input tag`, como `Input tag 'advisor_20260301'`. Antes de v2.1.280, Claude Code no reintentaba este rechazo
* Claude Code no reintenta rechazos de gestión de contexto o campos de esquema de herramienta, así que esos errores `400` llegan al desarrollador

El rechazo `bound to a different conversation` proviene de la verificación de [pensamiento preservado](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking) de la API, que falla cuando el contenido de `system`, `tools`, o `messages` anteriores difiere de la solicitud que produjo el pensamiento. Una puerta de enlace que reescribe cualquiera de ese contenido puede causar el rechazo en sí; [Bibliotecas, proxies y puertas de enlace](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking#libraries-proxies-gateways) cubre qué pasar sin cambios.

La lógica de reintento coincide con la redacción del error del proveedor ascendente, así que reenvíe cuerpos de respuesta de error sin modificar. Una puerta de enlace que envuelve errores ascendentes en su propio sobre rompe la ruta de recuperación, incluso cuando preserva el código de estado, a menos que el mensaje del sobre lleve un token `capability_rejected:` estable. [La puerta de enlace de aplicaciones Claude sustituye esos tokens por la redacción de error de los proveedores de nube](/docs/es/claude-apps-gateway-config#upstream-error-messages), por ejemplo `capability_rejected: prompt_too_long`.

<h3 id="disable-pre-release-capabilities">
  Deshabilitar capacidades de pre-lanzamiento
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` detiene Claude Code de enviar capacidades de pre-lanzamiento y sus campos de cuerpo en cada proveedor, incluida la gestión de contexto y los campos de herramienta beta. La variable no afecta el razonamiento adaptativo, que se selecciona por modelo en lugar de por beta. Nunca suprime la capacidad OAuth que la autenticación de suscripción requiere.

En Claude Code v2.1.227 o posterior, su organización puede mantener [búsqueda de herramientas MCP](/docs/es/mcp#scale-with-mcp-tool-search) activada bajo esta variable a través de [configuración administrada](/docs/es/managed-settings). Lo que Claude Code envía con esa anulación en su lugar depende de cómo se conecte:

* En una conexión directa, o a través de una puerta de enlace configurada con `ANTHROPIC_BASE_URL`, Claude Code sigue enviando el encabezado beta de búsqueda de herramientas, campos de herramienta `defer_loading`, y bloques `tool_reference`, y elimina el resto
* En un proveedor de nube, o iniciando sesión a través de una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), la anulación no tiene efecto

El conjunto de capacidades que Claude Code envía crece en las versiones. Para cadenas de encabezado beta actuales, consulte la [referencia de encabezados beta](https://platform.claude.com/docs/en/api/beta-headers); pruebe su puerta de enlace contra nuevas versiones de Claude Code en lugar de fijar a una lista observada.

<h2 id="model-discovery">
  Descubrimiento de modelos
</h2>

Cuando `ANTHROPIC_BASE_URL` apunta a una puerta de enlace que expone el formato Anthropic Messages, Claude Code puede consultar el punto final `/v1/models` de la puerta de enlace al inicio y agregar los modelos devueltos al selector `/model`. Si usted o su administrador establecen `replaceBuiltInOptions` en una alineación [`modelPicker`](/docs/es/settings-reference#modelpicker), Claude Code oculta los modelos descubiertos del selector.

Los desarrolladores lo habilitan estableciendo [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/es/env-vars), en su propio entorno o a través de configuración administrada. El descubrimiento está desactivado de forma predeterminada para que las puertas de enlace respaldadas por una clave API compartida no expongan cada modelo que la clave puede acceder a cada usuario.

<h3 id="when-discovery-runs">
  Cuándo se ejecuta el descubrimiento
</h3>

El descubrimiento se aplica solo al formato Anthropic Messages. No se ejecuta cuando:

* Se establece cualquier variable de proveedor `CLAUDE_CODE_USE_*`, incluso si `ANTHROPIC_BASE_URL` también se establece
* `ANTHROPIC_BASE_URL` no se establece o apunta a `api.anthropic.com`

El descubrimiento aún se ejecuta cuando [el tráfico no esencial está desactivado](/docs/es/llm-gateway-connect#turn-off-traffic-outside-the-gateway-path), porque la solicitud va solo a su puerta de enlace. Antes de v2.1.257, el descubrimiento no se ejecutaba mientras el tráfico no esencial estaba desactivado.

<h3 id="request-and-response">
  Solicitud y respuesta
</h3>

La solicitud es `GET /v1/models?limit=1000` con un tiempo de espera de 3 segundos de forma predeterminada, y cualquier redirección se trata como fallo para que la credencial no pueda filtrarse a un destino de redirección. Una puerta de enlace que responde más lentamente que el tiempo de espera, o una que redirige `/v1/models`, incluso `http` a `https`, falla el descubrimiento silenciosamente; sirva el punto final directamente en la URL base configurada.

Para dar a una puerta de enlace lenta más tiempo, establezca [`CLAUDE_CODE_GATEWAY_MODEL_DISCOVERY_TIMEOUT_MS`](/docs/es/env-vars#variables). La variable requiere Claude Code v2.1.269 o posterior.

Claude Code envía la solicitud de descubrimiento con ambos encabezados de credencial a continuación y omite un encabezado cuyo valor no se resuelve. Enviar ambos encabezados requiere Claude Code v2.1.248 o posterior. Las versiones anteriores envían solo `Authorization` cuando `ANTHROPIC_AUTH_TOKEN` se establece y solo `x-api-key` de lo contrario.

* `Authorization`: `ANTHROPIC_AUTH_TOKEN` como un token portador, de lo contrario el valor [`apiKeyHelper`](/docs/es/llm-gateway-connect#rotate-credentials-with-apikeyhelper) como un token portador. En ese caso, Claude Code espera a que el asistente regrese antes de enviar la solicitud.
* `x-api-key`: la clave API que Claude Code resolvió, como `ANTHROPIC_API_KEY`. Cuando un valor de asistente es la única credencial, este encabezado también la lleva, por lo que el valor llega en ambos encabezados.

Claude Code también envía cualquier encabezado de `ANTHROPIC_CUSTOM_HEADERS`. Cuando un encabezado personalizado tiene un valor no vacío, Claude Code lo envía en lugar de un encabezado integrado del mismo nombre, haciendo coincidir los nombres sin distinción de mayúsculas y minúsculas.

Cuando ningún valor del encabezado de credencial se resuelve, Claude Code omite el descubrimiento y escribe una línea `[gatewayDiscovery] skipped` en el registro de depuración de una sesión `claude --debug`. Si proporciona una credencial solo a través de `ANTHROPIC_CUSTOM_HEADERS`, Claude Code aún omite el descubrimiento.

Claude Code lee `id`, el `display_name` opcional y la `description` opcional de cada entrada en la matriz `data` de la respuesta:

```json theme={null}
{
  "data": [
    {
      "id": "claude-sonnet-4-6",
      "display_name": "Claude Sonnet 4.6",
      "description": "Default model for everyday coding tasks"
    },
    { "id": "claude-opus-4-8" }
  ]
}
```

Claude Code mantiene una entrada cuando su `id` contiene `claude` o `anthropic` en cualquier lugar de la cadena, coincidiendo sin distinción de mayúsculas y minúsculas, e ignora el resto. Los ID con prefijo de proveedor como `vertex_ai/claude-sonnet-4-6` o `bedrock/anthropic.claude-sonnet-4-5` pasan el filtro; un ID que no contiene ninguna subcadena no lo hace. Antes de v2.1.223, Claude Code mantenía una entrada solo cuando su `id` comenzaba con `claude` o `anthropic`, lo que ocultaba los ID con prefijo de proveedor.

<h3 id="picker-entries-and-caching">
  Entradas del selector y almacenamiento en caché
</h3>

El selector es la lista de modelos interactiva que se abre cuando un desarrollador ejecuta `/model` en Claude Code. Cada entrada descubierta utiliza `display_name` como su nombre cuando la puerta de enlace envía uno que difiere del `id`. De lo contrario, la entrada muestra el nombre del modelo cuando Claude Code [reconoce el `id`](/docs/es/model-config#customize-pinned-model-display-and-capabilities), y el `id` cuando no lo hace. Por ejemplo, una entrada con el `id` `my-gateway-claude-sonnet-4-6` y sin `display_name` aparece como `Sonnet 4.6`.

El descubrimiento agrega solo modelos que la [configuración administrada `availableModels`](/docs/es/settings-reference#availablemodels) permite.

Cada entrada también muestra la `description` del modelo, contraída a una línea. Una entrada sin una `description` lee "From gateway" en su lugar. Antes de v2.1.257, cada entrada descubierta leía "From gateway".

Una ID descubierta no obtiene su propia fila cuando coincide con una fila ya en el selector:

* ID igual: la ID descubierta coincide exactamente con el ID de una fila existente, o los dos ID son ortografías de la misma versión de [Fable](/docs/es/model-config#work-with-fable).
* Mismo modelo que un alias integrado: cuando una ID explícita descubierta nombra el modelo al que un alias integrado se resuelve actualmente, el selector muestra solo la fila de alias. Por ejemplo, mientras `sonnet` se resuelve a `claude-sonnet-5`, un `claude-sonnet-5` descubierto se colapsa en la fila `sonnet`, y un `claude-sonnet-4-6` descubierto aún obtiene su propia fila. Antes de v2.1.197, Claude Code no plegaba estos ID en filas integradas, por lo que `claude-sonnet-5` también obtenía su propia fila "From gateway".

Los resultados se almacenan en caché en `~/.claude/cache/gateway-models.json`, o `%USERPROFILE%\.claude\cache\gateway-models.json` en Windows, y se actualizan en cada inicio. Si establece [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars), el caché se encuentra bajo ese directorio en su lugar. Si la solicitud falla o la puerta de enlace no implementa `/v1/models`, el selector vuelve a la lista en caché del inicio anterior o a la lista de modelos integrada. Si su puerta de enlace sirve modelos de Claude bajo alias que no coinciden con el filtro de descubrimiento, los desarrolladores pueden agregar esos alias manualmente con las [variables de configuración de modelo](/docs/es/model-config).

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para el resto del conjunto de documentación de puerta de enlace y las referencias de API subyacentes:

* [Descripción general de puertas de enlace](/docs/es/gateways): qué es una puerta de enlace y cómo elegir entre la puerta de enlace de aplicaciones Claude y otro producto
* [Otras puertas de enlace LLM](/docs/es/llm-gateway): cómo implementar una puerta de enlace que su organización ejecuta y cómo interactúa con suscripciones de claude.ai
* [Implementar una puerta de enlace LLM para su organización](/docs/es/llm-gateway-rollout): la lista de verificación de administrador que utiliza esta guía
* [Conectar Claude Code a una puerta de enlace LLM](/docs/es/llm-gateway-connect): configuración por desarrollador y la tabla de solución de problemas
* [Referencia de encabezados beta](https://platform.claude.com/docs/en/api/beta-headers): el conjunto actual de valores `anthropic-beta`
* [API de mensajes](https://platform.claude.com/docs/en/api/messages): el formato de API que implementa una puerta de enlace con formato Anthropic
