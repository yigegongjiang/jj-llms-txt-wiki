> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Monitoreo

> Aprende cómo habilitar y configurar OpenTelemetry para Claude Code.

Rastrea el uso de Claude Code, costos y actividad de herramientas en toda tu organización exportando datos de telemetría a través de OpenTelemetry (OTel). Claude Code exporta métricas como datos de series temporales a través del protocolo estándar de métricas, eventos a través del protocolo de registros/eventos, y opcionalmente trazas distribuidas a través del [protocolo de trazas](#traces-beta). Configura tus backends de métricas, registros y trazas para que coincidan con tus requisitos de monitoreo.

<h2 id="quick-start">
  Inicio rápido
</h2>

Configura OpenTelemetry usando variables de entorno:

```bash theme={null}
# 1. Habilitar telemetría
export CLAUDE_CODE_ENABLE_TELEMETRY=1

# 2. Elegir exportadores (ambos son opcionales - configura solo lo que necesites)
export OTEL_METRICS_EXPORTER=otlp       # Opciones: otlp, prometheus, console, none
export OTEL_LOGS_EXPORTER=otlp          # Opciones: otlp, console, none

# 3. Configurar punto final OTLP (para exportador OTLP)
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# 4. Establecer autenticación (si es requerida)
export OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer your-token"

# 5. Para depuración: reducir intervalos de exportación
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 segundos (predeterminado: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 segundos (predeterminado: 5000ms)

# 6. Ejecutar Claude Code
claude
```

<Note>
  Los intervalos de exportación predeterminados son 60 segundos para métricas y 5 segundos para registros. Durante la configuración, es posible que desees usar intervalos más cortos para propósitos de depuración. Recuerda restablecer estos valores para uso en producción.
</Note>

Para opciones de configuración completas, consulta la [especificación de OpenTelemetry](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Configuración del administrador
</h2>

Los administradores pueden configurar los ajustes de OpenTelemetry para todos los usuarios a través del [archivo de configuración administrada](/docs/es/settings#settings-files). Esto permite el control centralizado de los ajustes de telemetría en toda una organización. Consulta la [precedencia de configuración](/docs/es/settings#settings-precedence) para obtener más información sobre cómo se aplican los ajustes.

Ejemplo de configuración de ajustes administrados:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_EXPORTER_OTLP_PROTOCOL": "grpc",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4317",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer example-token"
  }
}
```

<Note>
  Los ajustes administrados pueden distribuirse a través de MDM (Mobile Device Management) u otras soluciones de gestión de dispositivos. Las variables de entorno definidas en el archivo de configuración administrada tienen alta precedencia y no pueden ser anuladas por los usuarios.
</Note>

Claude Code no pasa variables de entorno `OTEL_*` a los subprocesos que genera, incluyendo la herramienta Bash, hooks, servidores MCP, y servidores de lenguaje. Una aplicación instrumentada con OpenTelemetry que ejecutes a través de la herramienta Bash no hereda el punto final del exportador de Claude Code ni los encabezados, así que establece esas variables directamente en el comando si esa aplicación necesita exportar su propia telemetría.

<h2 id="configuration-details">
  Detalles de configuración
</h2>

<h3 id="common-configuration-variables">
  Variables de configuración comunes
</h3>

| Variable de Entorno                                 | Descripción                                                                                                                                                                                                                                                                                                                                                                                          | Valores de Ejemplo                                                                                                                    |
| --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Habilita la recopilación de telemetría (requerido)                                                                                                                                                                                                                                                                                                                                                   | `1`                                                                                                                                   |
| `OTEL_METRICS_EXPORTER`                             | Tipos de exportador de métricas, separados por comas. Usa `none` para deshabilitar                                                                                                                                                                                                                                                                                                                   | `console`, `otlp`, `prometheus`, `none`                                                                                               |
| `OTEL_LOGS_EXPORTER`                                | Tipos de exportador de registros/eventos, separados por comas. Usa `none` para deshabilitar                                                                                                                                                                                                                                                                                                          | `console`, `otlp`, `none`                                                                                                             |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protocolo para exportador OTLP, se aplica a todas las señales                                                                                                                                                                                                                                                                                                                                        | `grpc`, `http/json`, `http/protobuf`                                                                                                  |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | Punto final del recopilador OTLP para todas las señales                                                                                                                                                                                                                                                                                                                                              | `http://localhost:4317`                                                                                                               |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protocolo para métricas, anula la configuración general                                                                                                                                                                                                                                                                                                                                              | `grpc`, `http/json`, `http/protobuf`                                                                                                  |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | Punto final de métricas OTLP, anula la configuración general                                                                                                                                                                                                                                                                                                                                         | `http://localhost:4318/v1/metrics`                                                                                                    |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protocolo para registros, anula la configuración general                                                                                                                                                                                                                                                                                                                                             | `grpc`, `http/json`, `http/protobuf`                                                                                                  |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | Punto final de registros OTLP, anula la configuración general                                                                                                                                                                                                                                                                                                                                        | `http://localhost:4318/v1/logs`                                                                                                       |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | Encabezados de autenticación para OTLP                                                                                                                                                                                                                                                                                                                                                               | `Authorization=Bearer token`                                                                                                          |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Intervalo de exportación en milisegundos (predeterminado: 60000)                                                                                                                                                                                                                                                                                                                                     | `5000`, `60000`                                                                                                                       |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Intervalo de exportación de registros en milisegundos (predeterminado: 5000)                                                                                                                                                                                                                                                                                                                         | `1000`, `10000`                                                                                                                       |
| `OTEL_LOG_USER_PROMPTS`                             | Habilitar registro del contenido del mensaje del usuario (predeterminado: deshabilitado)                                                                                                                                                                                                                                                                                                             | `1` para habilitar                                                                                                                    |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Habilitar registro del texto de respuesta del asistente en eventos `assistant_response` (predeterminado: deshabilitado). Cuando no está establecido, recurre al valor de `OTEL_LOG_USER_PROMPTS`. Requiere Claude Code v2.1.193 o posterior                                                                                                                                                          | `1` para habilitar, `0` para mantener redactado                                                                                       |
| `OTEL_LOG_TOOL_DETAILS`                             | Habilitar registro de parámetros de herramientas e argumentos de entrada en eventos de herramientas y atributos de span de traza: comandos Bash, nombres de servidor MCP y herramienta, nombres de habilidades, e entrada de herramienta. También habilita nombres de comandos personalizados, de plugin y MCP en eventos `user_prompt` (predeterminado: deshabilitado)                              | `1` para habilitar                                                                                                                    |
| `OTEL_LOG_TOOL_CONTENT`                             | Habilitar registro de contenido de entrada y salida de herramientas en eventos de span (predeterminado: deshabilitado). Requiere [trazas](#traces-beta). El contenido se trunca en 60 KB                                                                                                                                                                                                             | `1` para habilitar                                                                                                                    |
| `OTEL_LOG_RAW_API_BODIES`                           | Emitir el cuerpo completo de solicitud y respuesta JSON de la API de Mensajes de Anthropic como eventos de registro `api_request_body` / `api_response_body` (predeterminado: deshabilitado). Los cuerpos incluyen el historial de conversación completo. Habilitar esto implica consentimiento a todo lo que `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS`, y `OTEL_LOG_TOOL_CONTENT` revelarían | `1` para cuerpos en línea truncados en 60 KB, o `file:<dir>` para cuerpos sin truncar en disco con un puntero `body_ref` en el evento |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Preferencia de temporalidad de métricas (predeterminado: `delta`). Establece en `cumulative` si tu backend espera temporalidad acumulativa                                                                                                                                                                                                                                                           | `delta`, `cumulative`                                                                                                                 |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Intervalo para actualizar encabezados dinámicos (predeterminado: 1740000ms / 29 minutos)                                                                                                                                                                                                                                                                                                             | `900000`                                                                                                                              |

<h3 id="mtls-authentication">
  Autenticación mTLS
</h3>

Cómo configures certificados de cliente para el exportador OTLP depende del protocolo OTLP en uso para esa señal, establecido a través de `OTEL_EXPORTER_OTLP_PROTOCOL` o la anulación por señal. La misma configuración se aplica a métricas, registros y trazas.

| Protocolo                    | Variables de certificado de cliente                                                                                                                                                            | Confiar en la CA del recopilador con |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY`, y opcionalmente `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Consulta [Configuración de red](/docs/es/network-config#mtls-authentication)              | `NODE_EXTRA_CA_CERTS`                |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` y `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, o las variantes por señal como `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` para usar un certificado diferente por señal | `OTEL_EXPORTER_OTLP_CERTIFICATE`     |

Para `grpc`, el SDK de OpenTelemetry lee las variables OTLP estándar directamente, por lo que las configuraciones existentes que establecen las variables de métricas por señal continúan funcionando.

<h3 id="metrics-cardinality-control">
  Control de cardinalidad de métricas
</h3>

Las siguientes variables de entorno controlan qué atributos se incluyen en las métricas para gestionar la cardinalidad:

| Variable de Entorno                        | Descripción                                                                                | Valor Predeterminado | Ejemplo para Deshabilitar |
| ------------------------------------------ | ------------------------------------------------------------------------------------------ | -------------------- | ------------------------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Incluir atributo session.id en métricas                                                    | `true`               | `false`                   |
| `OTEL_METRICS_INCLUDE_VERSION`             | Incluir atributo app.version en métricas                                                   | `false`              | `true`                    |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Incluir atributos user.account\_uuid y user.account\_id en métricas                        | `true`               | `false`                   |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Incluir atributo app.entrypoint en métricas                                                | `false`              | `true`                    |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Incluir claves de `OTEL_RESOURCE_ATTRIBUTES` como atributos en puntos de datos de métricas | `true`               | `false`                   |

Estas variables ayudan a controlar la cardinalidad de las métricas, lo que afecta los requisitos de almacenamiento y el rendimiento de las consultas en tu backend de métricas. Una cardinalidad más baja generalmente significa mejor rendimiento y costos de almacenamiento más bajos, pero datos menos granulares para el análisis.

<h3 id="traces-beta">
  Trazas (beta)
</h3>

Las trazas distribuidas exportan spans que vinculan cada mensaje del usuario a las solicitudes de API y ejecuciones de herramientas que desencadena, para que puedas ver una solicitud completa como una única traza en tu backend de trazas.

Las trazas están deshabilitadas por defecto. Para habilitarlas, establece tanto `CLAUDE_CODE_ENABLE_TELEMETRY=1` como `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, luego establece `OTEL_TRACES_EXPORTER` para elegir dónde se envían los spans. Las trazas reutilizan la [configuración OTLP común](#common-configuration-variables) para punto final, protocolo, encabezados, y [mTLS](#mtls-authentication).

| Variable de Entorno                   | Descripción                                                                              | Valores de Ejemplo                   |
| ------------------------------------- | ---------------------------------------------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Habilitar trazas de span (requerido). `ENABLE_ENHANCED_TELEMETRY_BETA` también se acepta | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Tipos de exportador de trazas, separados por comas. Usa `none` para deshabilitar         | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protocolo para trazas, anula `OTEL_EXPORTER_OTLP_PROTOCOL`                               | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | Punto final de trazas OTLP, anula `OTEL_EXPORTER_OTLP_ENDPOINT`                          | `http://localhost:4318/v1/traces`    |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Intervalo de exportación de lote de span en milisegundos (predeterminado: 5000)          | `1000`, `10000`                      |

Los spans redactan el texto del mensaje del usuario, los detalles de entrada de herramientas y el contenido de herramientas por defecto. Establece `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1`, y `OTEL_LOG_TOOL_CONTENT=1` para incluirlos.

Cuando el trazado está activo, los subprocesos de Bash y PowerShell heredan automáticamente una variable de entorno `TRACEPARENT` que contiene el contexto de traza W3C del span de ejecución de herramienta activo. Esto permite que cualquier subproceso que lea `TRACEPARENT` padre sus propios spans bajo la misma traza, habilitando trazado distribuido de extremo a extremo a través de scripts y comandos que Claude ejecuta.

Cuando el trazado está activo y Claude Code está conectado directamente a la API de Anthropic, cada solicitud de modelo lleva un encabezado W3C `traceparent` establecido en el contexto del span `claude_code.llm_request`, y el encabezado `traceresponse` de la API se registra como un enlace de span. Juntos, estos conectan los spans del lado del cliente de Claude Code a la traza del lado del servidor a través de cualquier intermediario compatible. Las solicitudes HTTP MCP salientes llevan `traceparent` de la misma manera. El encabezado no se envía a proveedores de terceros.

Por defecto, el encabezado `traceparent` en solicitudes de modelo y HTTP MCP se envía solo cuando `ANTHROPIC_BASE_URL` no está establecido o apunta a la API de Anthropic, ya que algunos proxies rechazan encabezados no reconocidos. La variable `TRACEPARENT` del subproceso se controla por el mismo interruptor por consistencia. Si ejecutas Claude Code a través de un proxy `ANTHROPIC_BASE_URL` personalizado y deseas que se propague el contexto de traza, establece `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

En sesiones del SDK de Agent y no interactivas iniciadas con `-p`, Claude Code también lee `TRACEPARENT` y `TRACESTATE` de su propio entorno cuando inicia cada span de interacción. Esto permite que un proceso de incrustación pase su contexto de traza W3C activo al subproceso para que los spans de Claude Code aparezcan como hijos de la traza distribuida del llamador. Las sesiones interactivas ignoran `TRACEPARENT` entrante para evitar heredar accidentalmente valores ambientes de entornos de CI o contenedor.

<h4 id="span-hierarchy">
  Jerarquía de spans
</h4>

Cada mensaje del usuario inicia un span raíz `claude_code.interaction`. Las llamadas de API, llamadas de herramientas y ejecuciones de hooks se registran como sus hijos. Los spans de herramientas tienen dos spans hijos propios: uno para el tiempo dedicado a esperar una decisión de permiso y otro para la ejecución en sí. Cuando la herramienta Agent o la herramienta Task heredada genera un subagente, los spans de API y herramienta del subagente se anidan bajo el span `claude_code.tool` del padre.

```text theme={null}
claude_code.interaction
├── claude_code.llm_request
├── claude_code.hook                    (requiere trazado beta detallado)
└── claude_code.tool
    ├── claude_code.tool.blocked_on_user
    ├── claude_code.tool.execution
    └── (herramienta Agent) spans de claude_code.llm_request / claude_code.tool del subagente
```

En sesiones del SDK de Agent y `claude -p`, `claude_code.interaction` en sí se convierte en un hijo del span del llamador cuando `TRACEPARENT` se establece en el entorno.

<h4 id="span-attributes">
  Atributos de spans
</h4>

Cada span lleva los [atributos estándar](#standard-attributes) más un atributo `span.type` que coincide con su nombre. Las tablas a continuación enumeran los atributos adicionales establecidos en cada span. Los spans `llm_request`, `tool.execution`, y `hook` establecen el estado de OpenTelemetry `ERROR` cuando registran una falla; los otros spans siempre terminan con estado `UNSET`.

**`claude_code.interaction`**

| Atributo                  | Descripción                                                                        | Controlado Por          |
| ------------------------- | ---------------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | Texto del mensaje. El valor es `<REDACTED>` a menos que la puerta esté establecida | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Longitud del mensaje en caracteres                                                 |                         |
| `interaction.sequence`    | Contador basado en 1 de interacciones en esta sesión                               |                         |
| `interaction.duration_ms` | Duración de pared del turno                                                        |                         |

**`claude_code.llm_request`**

| Atributo                         | Descripción                                                                                                                                                    | Controlado Por          |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `model`                          | Identificador de modelo                                                                                                                                        |                         |
| `gen_ai.system`                  | Siempre `anthropic`. Convención semántica de GenAI de OpenTelemetry                                                                                            |                         |
| `gen_ai.request.model`           | Mismo valor que `model`. Convención semántica de GenAI de OpenTelemetry                                                                                        |                         |
| `query_source`                   | Subsistema que emitió la solicitud, como `repl_main_thread` o un nombre de subagente                                                                           |                         |
| `agent_id`                       | Identificador del subagente o compañero que emitió la solicitud. Ausente en la sesión principal                                                                |                         |
| `parent_agent_id`                | Identificador del agente que generó este. Ausente para la sesión principal y para agentes generados directamente desde ella                                    |                         |
| `workflow.run_id`                | Identificador de ejecución de la ejecución de la herramienta [Workflow](/docs/es/workflows), prefijado con `wf_`. Ausente para agentes no generados por un workflow |                         |
| `workflow.name`                  | Nombre del workflow que generó este agente. Los nombres creados por el usuario se reemplazan con `custom` a menos que la puerta esté establecida               | `OTEL_LOG_TOOL_DETAILS` |
| `speed`                          | `fast` o `normal`                                                                                                                                              |                         |
| `llm_request.context`            | `interaction`, `tool`, o `standalone` dependiendo del span padre                                                                                               |                         |
| `duration_ms`                    | Duración de pared incluyendo reintentos                                                                                                                        |                         |
| `ttft_ms`                        | Tiempo al primer token en milisegundos                                                                                                                         |                         |
| `input_tokens`                   | Recuento de tokens de entrada del bloque de uso de API                                                                                                         |                         |
| `output_tokens`                  | Recuento de tokens de salida                                                                                                                                   |                         |
| `cache_read_tokens`              | Tokens leídos del caché de mensaje                                                                                                                             |                         |
| `cache_creation_tokens`          | Tokens escritos en el caché de mensaje                                                                                                                         |                         |
| `request_id`                     | ID de solicitud de API de Anthropic del encabezado de respuesta `request-id`                                                                                   |                         |
| `gen_ai.response.id`             | Mismo valor que `request_id`. Convención semántica de GenAI de OpenTelemetry                                                                                   |                         |
| `client_request_id`              | `x-client-request-id` generado por cliente del intento final                                                                                                   |                         |
| `attempt`                        | Intentos totales realizados para esta solicitud                                                                                                                |                         |
| `success`                        | `true` o `false`                                                                                                                                               |                         |
| `status_code`                    | Código de estado HTTP cuando la solicitud falló                                                                                                                |                         |
| `error`                          | Mensaje de error cuando la solicitud falló                                                                                                                     |                         |
| `response.has_tool_call`         | `true` cuando la respuesta contenía bloques de uso de herramientas                                                                                             |                         |
| `stop_reason`                    | Respuesta de API `stop_reason`, como `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn`, o `refusal`                                          |                         |
| `gen_ai.response.finish_reasons` | Mismo valor que `stop_reason`, envuelto en una matriz de cadena. Convención semántica de GenAI de OpenTelemetry                                                |                         |

Cada intento de reintento también se registra como un evento de span `gen_ai.request.attempt` con atributos `attempt` e `client_request_id`.

**`claude_code.tool`**

| Atributo              | Descripción                                                                                                                                                                                                                                                   | Controlado Por          |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | Nombre de la herramienta                                                                                                                                                                                                                                      |                         |
| `duration_ms`         | Duración de pared incluyendo espera de permiso y ejecución                                                                                                                                                                                                    |                         |
| `result_tokens`       | Tamaño aproximado de token del resultado de la herramienta                                                                                                                                                                                                    |                         |
| `agent_id`            | Identificador del subagente o compañero que ejecutó la herramienta. Ausente en la sesión principal                                                                                                                                                            |                         |
| `parent_agent_id`     | Identificador del agente que generó este. Ausente para la sesión principal y para agentes generados directamente desde ella                                                                                                                                   |                         |
| `workflow.run_id`     | Identificador de ejecución de la ejecución de la herramienta Workflow que generó este agente, prefijado con `wf_`. Ausente para agentes no generados por un workflow                                                                                          |                         |
| `workflow.name`       | Nombre del workflow que generó este agente. Los nombres creados por el usuario se reemplazan con `custom` a menos que la puerta esté establecida                                                                                                              | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | El ID del bloque `tool_use` del modelo para esta llamada. Coincide con el `tool_use_id` en los eventos [tool\_result](#tool-result-event) y [tool\_decision](#tool-decision-event) y en cargas útiles de hooks, para que puedas unir el span a esos registros |                         |
| `gen_ai.tool.call.id` | Mismo valor que `tool_use_id`. Convención semántica de GenAI de OpenTelemetry                                                                                                                                                                                 |                         |
| `file_path`           | Ruta de archivo de destino para herramientas Read, Edit, y Write                                                                                                                                                                                              | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Cadena de comando para la herramienta Bash                                                                                                                                                                                                                    | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Nombre de habilidad para la herramienta Skill                                                                                                                                                                                                                 | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Tipo de subagente para la herramienta Agent o herramienta Task heredada                                                                                                                                                                                       | `OTEL_LOG_TOOL_DETAILS` |

Cuando `OTEL_LOG_TOOL_CONTENT=1`, este span también registra un evento de span `tool.output` cuyos atributos contienen los cuerpos de entrada y salida de la herramienta, truncados en 60 KB por atributo.

**`claude_code.tool.blocked_on_user`**

| Atributo      | Descripción                                                                                | Controlado Por |
| ------------- | ------------------------------------------------------------------------------------------ | -------------- |
| `duration_ms` | Tiempo dedicado a esperar la decisión de permiso                                           |                |
| `decision`    | `accept` o `reject`                                                                        |                |
| `source`      | Fuente de decisión, coincidiendo con el evento [Tool decision event](#tool-decision-event) |                |

**`claude_code.tool.execution`**

| Atributo              | Descripción                                                                                                                                                                     | Controlado Por          |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | Tiempo dedicado a ejecutar el cuerpo de la herramienta                                                                                                                          |                         |
| `tool_use_id`         | Mismo valor que en el span padre `claude_code.tool`                                                                                                                             |                         |
| `gen_ai.tool.call.id` | Mismo valor que `tool_use_id`. Convención semántica de GenAI de OpenTelemetry                                                                                                   |                         |
| `success`             | `true` o `false`                                                                                                                                                                |                         |
| `error`               | Cadena de categoría de error cuando la ejecución falló, como `Error:ENOENT` o `ShellError`. Contiene el mensaje de error completo en su lugar cuando la puerta está establecida | `OTEL_LOG_TOOL_DETAILS` |

**`claude_code.hook`**

Este span se emite solo cuando el trazado beta detallado está activo, lo que requiere `ENABLE_BETA_TRACING_DETAILED=1` y `BETA_TRACING_ENDPOINT` además de la configuración del exportador de trazas anterior. En sesiones de CLI interactivas, esto también requiere que tu organización esté en la lista de permitidos para la característica. Las sesiones del SDK de Agent y no interactivas `-p` no están controladas. No se emite cuando solo `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` está establecido.

| Atributo                 | Descripción                                               | Controlado Por          |
| ------------------------ | --------------------------------------------------------- | ----------------------- |
| `hook_event`             | Tipo de evento de hook, como `PreToolUse`                 |                         |
| `hook_name`              | Nombre completo del hook, como `PreToolUse:Write`         |                         |
| `num_hooks`              | Número de comandos de hook coincidentes ejecutados        |                         |
| `hook_definitions`       | Configuración de hook serializada en JSON                 | `OTEL_LOG_TOOL_DETAILS` |
| `duration_ms`            | Duración de pared de todos los hooks coincidentes         |                         |
| `num_success`            | Recuento de hooks que se completaron exitosamente         |                         |
| `num_blocking`           | Recuento de hooks que devolvieron una decisión de bloqueo |                         |
| `num_non_blocking_error` | Recuento de hooks que fallaron sin bloquear               |                         |
| `num_cancelled`          | Recuento de hooks cancelados antes de completarse         |                         |

<Note>
  Atributos adicionales que contienen contenido como `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input`, y `response.model_output` se emiten solo cuando el trazado beta detallado está activo. No son parte del esquema de span estable. `user_system_prompt` además requiere `OTEL_LOG_USER_PROMPTS=1`. Lleva solo el texto del mensaje del sistema que proporcionas a través de la opción `systemPrompt` del SDK o las banderas `--system-prompt` y `--append-system-prompt`, truncado en 60 KB, y se emite una vez por sesión en lugar de por solicitud.
</Note>

<h3 id="dynamic-headers">
  Encabezados dinámicos
</h3>

Para entornos empresariales que requieren autenticación dinámica, puedes configurar un script para generar encabezados dinámicamente. Los encabezados dinámicos se aplican solo a los protocolos `http/protobuf` e `http/json`. El exportador `grpc` usa solo el valor estático `OTEL_EXPORTER_OTLP_HEADERS`.

<h4 id="settings-configuration">
  Configuración de ajustes
</h4>

Agrega a tu `.claude/settings.json`:

```json theme={null}
{
  "otelHeadersHelper": "/bin/generate_opentelemetry_headers.sh"
}
```

El valor puede ser la ruta a un archivo ejecutable, incluyendo una ruta que contiene espacios, o una línea de comando de shell con argumentos. En Windows, el valor siempre se ejecuta a través del shell, así que entrecomilla una ruta que contiene espacios dentro del valor JSON.

<h4 id="script-requirements">
  Requisitos del script
</h4>

El script debe generar JSON válido con pares clave-valor de cadena que representen encabezados HTTP:

```bash theme={null}
#!/bin/bash
# Ejemplo: Múltiples encabezados
echo "{\"Authorization\": \"Bearer $(get-token.sh)\", \"X-API-Key\": \"$(get-api-key.sh)\"}"
```

Si el auxiliar falla o imprime salida que no cumple con estos requisitos, Claude Code reporta el error en:

* Salida de `/status`
* El registro de depuración, cuando se ejecuta con [`--debug`](/docs/es/cli-reference#cli-flags) o después de ejecutar `/debug` en la sesión
* stderr, en sesiones no interactivas iniciadas con `-p`

<h4 id="refresh-behavior">
  Comportamiento de actualización
</h4>

El script auxiliar de encabezados se ejecuta al inicio y periódicamente después para admitir la actualización de tokens. Por defecto, el script se ejecuta cada 29 minutos. Personaliza el intervalo con la variable de entorno `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`.

<h3 id="multi-team-organization-support">
  Soporte de organización multi-equipo
</h3>

Las organizaciones con múltiples equipos o departamentos pueden agregar atributos personalizados para distinguir entre diferentes grupos usando la variable de entorno `OTEL_RESOURCE_ATTRIBUTES`:

```bash theme={null}
# Agregar atributos personalizados para identificación de equipo
export OTEL_RESOURCE_ATTRIBUTES="department=engineering,team.id=platform,cost_center=eng-123"
```

Estos atributos personalizados se incluirán en todas las métricas y eventos, permitiéndote:

* Filtrar métricas por equipo o departamento
* Rastrear costos por centro de costos
* Crear paneles específicos del equipo
* Configurar alertas para equipos específicos

Claude Code adjunta estos valores como atributos en cada punto de datos de métrica y registro de evento, además de enviarlos en el bloque de recursos OTLP. Debido a que la mayoría de los backends de métricas exponen atributos de punto de datos como etiquetas consultables, puedes agrupar y filtrar métricas por tus claves personalizadas directamente. Las claves personalizadas nunca anulan los [atributos estándar](#standard-attributes) como `user.id` o `session.id`: cuando una clave colisiona, Claude Code mantiene el valor integrado.

Cada clave personalizada se convierte en una etiqueta en cada serie de métricas, por lo que los valores de alta cardinalidad aumentan el costo de almacenamiento en tu backend de métricas. Para enviar atributos personalizados solo en el bloque de recursos y omitirlos de las etiquetas de punto de datos, establece `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES=false`. Consulta [Control de cardinalidad de métricas](#metrics-cardinality-control).

<Warning>
  La variable de entorno `OTEL_RESOURCE_ATTRIBUTES` utiliza pares clave=valor separados por comas con requisitos de formato estrictos:

  * **No se permiten espacios**: Los valores no pueden contener espacios. Por ejemplo, `user.organizationName=My Company` es inválido
  * **Formato**: Debe ser pares clave=valor separados por comas: `key1=value1,key2=value2`
  * **Caracteres permitidos**: Solo caracteres US-ASCII excluyendo caracteres de control, espacios en blanco, comillas dobles, comas, puntos y comas, y barras invertidas
  * **Caracteres especiales**: Los caracteres fuera del rango permitido deben estar codificados en porcentaje

  Para un valor que necesitaría un espacio, usa guiones bajos o camelCase en su lugar. Los siguientes ejemplos establecen `org.name` con cada forma:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=Johns_Organization"
  export OTEL_RESOURCE_ATTRIBUTES="org.name=JohnsOrganization"
  ```

  Puedes codificar en porcentaje cualquier carácter, no solo los excluidos. Este ejemplo codifica tanto el espacio como el apóstrofo:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=John%27s%20Organization"
  ```

  Envolver valores entre comillas no escapa espacios. Por ejemplo, `org.name="My Company"` resulta en el valor literal `"My Company"` con las comillas incluidas, no `My Company`.
</Warning>

<h3 id="example-configurations">
  Configuraciones de ejemplo
</h3>

Establece estas variables de entorno antes de ejecutar `claude`. Cada bloque muestra una configuración completa para un exportador diferente o escenario de implementación:

```bash theme={null}
# Depuración de consola (intervalos de 1 segundo)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000

# OTLP/gRPC
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Prometheus
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus

# Múltiples exportadores
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json

# Diferentes puntos finales/backends para métricas y registros
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317

# Solo métricas (sin eventos/registros)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Solo eventos/registros (sin métricas)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

<h2 id="available-metrics-and-events">
  Métricas y eventos disponibles
</h2>

<h3 id="standard-attributes">
  Atributos estándar
</h3>

Todas las métricas y eventos comparten estos atributos estándar:

| Atributo                             | Descripción                                                                                                                                                                                                                                                     | Controlado Por                                                    |
| ------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `session.id`                         | Identificador único de sesión                                                                                                                                                                                                                                   | `OTEL_METRICS_INCLUDE_SESSION_ID` (predeterminado: true)          |
| `app.version`                        | Versión actual de Claude Code                                                                                                                                                                                                                                   | `OTEL_METRICS_INCLUDE_VERSION` (predeterminado: false)            |
| `app.entrypoint`                     | Cómo se inició la sesión, como `cli`, `sdk-cli`, `sdk-ts`, `sdk-py`, o `claude-vscode`                                                                                                                                                                          | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (predeterminado: false)         |
| `organization.id`                    | UUID de organización (cuando está autenticado)                                                                                                                                                                                                                  | Siempre incluido cuando está disponible                           |
| `user.account_uuid`                  | UUID de cuenta (cuando está autenticado)                                                                                                                                                                                                                        | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (predeterminado: true)        |
| `user.account_id`                    | ID de cuenta en formato etiquetado que coincide con las API de administrador de Anthropic (cuando está autenticado), como `user_01BWBeN28...`                                                                                                                   | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (predeterminado: true)        |
| `user.id`                            | Identificador anónimo aleatorio generado en la primera ejecución y persistido en `~/.claude.json`. No contiene información personal y no se deriva de su cuenta de Claude. Eliminar el archivo produce un nuevo valor no relacionado en la siguiente ejecución. | Siempre incluido                                                  |
| `user.email`                         | Dirección de correo electrónico del usuario (cuando está autenticado a través de OAuth)                                                                                                                                                                         | Siempre incluido cuando está disponible                           |
| `terminal.type`                      | Tipo de terminal, como `iTerm.app`, `vscode`, `cursor`, o `tmux`                                                                                                                                                                                                | Siempre incluido cuando se detecta                                |
| Claves de `OTEL_RESOURCE_ATTRIBUTES` | Atributos personalizados que establece, como `department` o `team.id`. Consulte [Soporte de organización multiequipo](#multi-team-organization-support)                                                                                                         | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (predeterminado: true) |

Cuando Claude Code inicia sesión en una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), la CLI marca las exportaciones con la identidad autenticada de la sesión de la puerta de enlace: `user.id` es el asunto del IdP en lugar de un identificador de instalación anónimo, `user.email` es el correo electrónico con sesión iniciada, y `user.groups` lleva la pertenencia al grupo del IdP como una cadena separada por comas. Cada exportación también lleva `identity.source: gateway-oidc`. La identidad de la puerta de enlace se aplica al final, por lo que las claves `user.*` e `identity.*` establecidas a través de `OTEL_RESOURCE_ATTRIBUTES` se ignoran en sesiones de puerta de enlace.

Los eventos incluyen adicionalmente los siguientes atributos. Estos nunca se adjuntan a las métricas porque causarían cardinalidad ilimitada:

* `prompt.id`: UUID correlacionando un mensaje del usuario con todos los eventos posteriores hasta el siguiente mensaje. Consulte [Atributos de correlación de eventos](#event-correlation-attributes).
* `workspace.host_paths`: directorios de espacio de trabajo del host seleccionados en la aplicación de escritorio, como una matriz de cadenas
* `workflow.run_id`: identificador de ejecución, con prefijo `wf_`, en los eventos de API y herramienta emitidos por agentes que pertenecen a una ejecución de herramienta [Workflow](/docs/es/workflows). Filtrar eventos por un `workflow.run_id` reconstruye las solicitudes de API y resultados de herramientas de esa ejecución. El identificador cubre los agentes que genera el script de flujo de trabajo y cualquier agente que esos generen a su vez, como invocaciones de habilidades. Coincide con el identificador de ejecución reportado en el resultado de la herramienta Workflow. Ausente en todos los demás eventos. Requiere Claude Code v2.1.202 o posterior
* `workflow.name`: nombre del flujo de trabajo, el `meta.name` de su script, emitido junto con `workflow.run_id`. Los nombres de flujo de trabajo integrados aparecen tal cual cuando la ejecución ejecuta el script integrado sin modificar. Los nombres creados por el usuario, incluidas copias editadas de scripts integrados, se reemplazan con `custom` a menos que `OTEL_LOG_TOOL_DETAILS=1` esté establecido. Requiere Claude Code v2.1.202 o posterior

<h3 id="metrics">
  Métricas
</h3>

Claude Code exporta las siguientes métricas:

| Nombre de Métrica                     | Descripción                                                            | Unidad |
| ------------------------------------- | ---------------------------------------------------------------------- | ------ |
| `claude_code.session.count`           | Recuento de sesiones CLI iniciadas                                     | count  |
| `claude_code.lines_of_code.count`     | Recuento de líneas de código modificadas                               | count  |
| `claude_code.pull_request.count`      | Número de solicitudes de extracción creadas                            | count  |
| `claude_code.commit.count`            | Número de confirmaciones de git creadas                                | count  |
| `claude_code.cost.usage`              | Costo de la sesión de Claude Code                                      | USD    |
| `claude_code.token.usage`             | Número de tokens utilizados                                            | tokens |
| `claude_code.code_edit_tool.decision` | Recuento de decisiones de permisos de herramienta de edición de código | count  |
| `claude_code.active_time.total`       | Tiempo activo total en segundos                                        | s      |

<h3 id="metric-details">
  Detalles de métricas
</h3>

Cada métrica incluye los atributos estándar enumerados anteriormente. Las métricas con atributos adicionales específicos del contexto se indican a continuación.

<h4 id="session-counter">
  Contador de sesión
</h4>

Se incrementa al inicio de cada sesión.

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `start_type`: Cómo se inició la sesión. Uno de `"fresh"`, `"resume"`, `"continue"`, o `"agents_view"`. El valor `"agents_view"` identifica el proceso del panel de control `claude agents`, una interfaz de usuario local lanzada por el usuario en lugar de una sesión conversacional. Filtre en este valor para separar los lanzamientos de procesos de interfaz de usuario de las sesiones conversacionales en sus paneles de control.

<h4 id="lines-of-code-counter">
  Contador de líneas de código
</h4>

Se incrementa cuando se agrega o se elimina código.

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `type`: (`"added"`, `"removed"`)
* `model`: Identificador de modelo para el modelo que realizó el cambio (por ejemplo, "claude-sonnet-5")

<h4 id="pull-request-counter">
  Contador de solicitud de extracción
</h4>

Se incrementa cuando Claude Code crea una solicitud de extracción o solicitud de fusión a través de un comando de shell o una herramienta MCP.

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)

<h4 id="commit-counter">
  Contador de confirmación
</h4>

Se incrementa al crear confirmaciones de git a través de Claude Code.

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)

<h4 id="cost-counter">
  Contador de costo
</h4>

Se incrementa después de cada solicitud de API.

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `model`: Identificador de modelo (por ejemplo, "claude-sonnet-5")
* `query_source`: Categoría del subsistema que emitió la solicitud. Uno de `"main"`, `"subagent"`, o `"auxiliary"`
* `speed`: `"fast"` cuando la solicitud utilizó modo rápido. Ausente de otra manera
* `effort`: [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud: `"low"`, `"medium"`, `"high"`, `"xhigh"`, o `"max"`. Ausente cuando el modelo no admite esfuerzo.
* `agent.name`: Tipo de subagente que emitió la solicitud. Los nombres de agentes integrados y de plugins del mercado oficial aparecen tal cual. Otros nombres de agentes definidos por el usuario se reemplazan con `"custom"`. Ausente cuando la solicitud no fue emitida por un tipo de subagente nombrado.
* `skill.name`: Habilidad activa para la solicitud, establecida por la herramienta Skill, un comando `/`, o heredada por un subagente generado. Los nombres de habilidades integradas, agrupadas, definidas por el usuario y de plugins del mercado oficial aparecen tal cual. Los nombres de habilidades de plugins de terceros se reemplazan con `"third-party"`. Ausente cuando no hay habilidad activa.
* `plugin.name`: Plugin propietario cuando la habilidad activa o subagente es proporcionado por un plugin. Los nombres de plugins del mercado oficial aparecen tal cual. Los nombres de plugins de terceros se reemplazan con `"third-party"`. Ausente cuando ni la habilidad ni el subagente tienen un plugin propietario.
* `marketplace.name`: Mercado desde el que se instaló el plugin propietario. Solo se emite para plugins del mercado oficial. Ausente de otra manera.
* `mcp_server.name`: Servidor MCP cuya herramienta se ejecutó en el turno que produjo esta solicitud. Los nombres de servidor integrados, proxied por claude.ai, y del registro oficial aparecen tal cual. Los nombres de servidor configurados por el usuario se reemplazan con `"custom"`. Ausente cuando no se ejecutó herramienta MCP.
* `mcp_tool.name`: Herramienta MCP que se ejecutó en el turno que produjo esta solicitud, con la misma redacción que `mcp_server.name`. Ausente cuando no se ejecutó herramienta MCP.

<h4 id="token-counter">
  Contador de tokens
</h4>

Se incrementa después de cada solicitud de API.

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `type`: (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model`: Identificador de modelo (por ejemplo, "claude-sonnet-5")
* `query_source`: Categoría del subsistema que emitió la solicitud. Uno de `"main"`, `"subagent"`, o `"auxiliary"`
* `speed`: `"fast"` cuando la solicitud utilizó modo rápido. Ausente de otra manera
* `effort`: [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud. Consulte [Contador de costo](#cost-counter) para detalles.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribución de habilidad, plugin, agente y MCP para la solicitud. Consulte [Contador de costo](#cost-counter) para definiciones y comportamiento de redacción.

<h4 id="code-edit-tool-decision-counter">
  Contador de decisión de herramienta de edición de código
</h4>

Se incrementa cuando el usuario acepta o rechaza el uso de herramientas Edit, Write, o NotebookEdit.

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `tool_name`: Nombre de la herramienta (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: Decisión del usuario (`"accept"`, `"reject"`)
* `source`: Fuente de decisión. Uno de `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"`, o `"user_reject"`. Consulte el [Evento de decisión de herramienta](#tool-decision-event) para saber qué significa cada valor.
* `language`: Lenguaje de programación del archivo editado, como `"TypeScript"`, `"Python"`, `"JavaScript"`, o `"Markdown"`. Devuelve `"unknown"` para extensiones de archivo no reconocidas.

<h4 id="active-time-counter">
  Contador de tiempo activo
</h4>

Rastrea el tiempo real dedicado a usar activamente Claude Code, excluyendo tiempo inactivo. Esta métrica se incrementa durante interacciones del usuario (escribir, leer respuestas) y durante procesamiento de CLI (ejecución de herramientas, generación de respuestas de IA).

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `type`: `"user"` para interacciones de teclado, `"cli"` para ejecución de herramientas y respuestas de IA

<h3 id="events">
  Eventos
</h3>

Claude Code exporta los siguientes eventos a través de registros/eventos de OpenTelemetry (cuando `OTEL_LOGS_EXPORTER` está configurado):

<h4 id="event-correlation-attributes">
  Atributos de correlación de eventos
</h4>

Cuando un usuario envía un mensaje, Claude Code puede hacer múltiples llamadas de API y ejecutar varias herramientas. El atributo `prompt.id` le permite vincular todos esos eventos al único mensaje que los desencadenó.

| Atributo    | Descripción                                                                                                     |
| ----------- | --------------------------------------------------------------------------------------------------------------- |
| `prompt.id` | Identificador UUID v4 que vincula todos los eventos producidos mientras se procesa un único mensaje del usuario |

Para rastrear toda la actividad desencadenada por un único mensaje, filtre sus eventos por un valor específico de `prompt.id`. Esto devuelve el evento user\_prompt, cualquier evento api\_request, y cualquier evento tool\_result que ocurrió mientras se procesaba ese mensaje.

<Note>
  `prompt.id` se excluye intencionalmente de las métricas porque cada mensaje genera un ID único, lo que crearía un número siempre creciente de series temporales. Úselo solo para análisis a nivel de evento y auditoría.
</Note>

<h4 id="user-prompt-event">
  Evento de mensaje del usuario
</h4>

Se registra cuando un usuario envía un mensaje.

**Nombre del Evento**: `claude_code.user_prompt`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `prompt_length`: Longitud del mensaje
* `prompt`: Contenido del mensaje. Redactado por defecto. Establezca `OTEL_LOG_USER_PROMPTS=1` para incluirlo
* `command_name`: Nombre del comando cuando el mensaje invoca uno. Los nombres de comandos integrados y agrupados como `compact` o `debug` se emiten tal cual; los alias como `reset` se emiten como se escribieron en lugar del nombre canónico. Los nombres de comandos personalizados, de plugin y MCP se contraen a `custom` o `mcp` a menos que `OTEL_LOG_TOOL_DETAILS=1` esté establecido
* `command_source`: Origen del comando cuando está presente: `builtin`, `custom`, o `mcp`. Los comandos proporcionados por plugins reportan como `custom`

<h4 id="assistant-response-event">
  Evento de respuesta del asistente
</h4>

Se registra después de cada solicitud de API que devuelve contenido de texto del modelo. Solo se incluyen los bloques de texto de la respuesta; los bloques de pensamiento y los bloques de uso de herramientas se excluyen. Requiere Claude Code v2.1.193 o posterior.

**Nombre del Evento**: `claude_code.assistant_response`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"assistant_response"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `response_length`: Longitud del texto de respuesta en caracteres
* `response`: Texto de respuesta, truncado en 60 KB. Redactado a `<REDACTED>` por defecto. Establezca `OTEL_LOG_ASSISTANT_RESPONSES=1` para incluirlo. Cuando `OTEL_LOG_ASSISTANT_RESPONSES` no está establecido, `OTEL_LOG_USER_PROMPTS` lo controla en su lugar, así que establezca `OTEL_LOG_ASSISTANT_RESPONSES=0` para mantener las respuestas redactadas mientras el registro de mensajes está activado
* `model`: Identificador de modelo (por ejemplo, "claude-sonnet-5")
* `request_id`: ID de solicitud de API de Anthropic del encabezado `request-id` de la respuesta. Presente solo cuando la API devuelve uno
* `query_source`: Subsistema que emitió la solicitud, como `"repl_main_thread"`, `"compact"`, o un nombre de subagente

<h4 id="tool-result-event">
  Evento de resultado de herramienta
</h4>

Se registra cuando una herramienta completa la ejecución. No se emite si la llamada de herramienta fue rechazada; consulte el [Evento de decisión de herramienta](#tool-decision-event) para rechazos.

**Nombre del Evento**: `claude_code.tool_result`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `tool_name`: Nombre de la herramienta
* `tool_use_id`: Identificador único para esta invocación de herramienta. Coincide con el `tool_use_id` pasado a hooks, permitiendo correlación entre eventos OTel y datos capturados por hooks.
* `success`: `"true"` o `"false"`
* `duration_ms`: Tiempo de ejecución en milisegundos
* `error_type`: Cadena de categoría de error cuando la herramienta falló, como `"Error:ENOENT"` o `"ShellError"`
* `error` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Mensaje de error completo cuando la herramienta falló
* `decision_type`: Siempre `"accept"`, ya que este evento se emite solo después de que se ejecuta la herramienta. Las llamadas rechazadas no producen un resultado de herramienta
* `decision_source`: Fuente de decisión de permiso. Uno de `"config"`, `"hook"`, `"user_permanent"`, o `"user_temporary"`. Consulte el [Evento de decisión de herramienta](#tool-decision-event) para saber qué significa cada valor. Las fuentes solo de rechazo `"user_abort"` y `"user_reject"` nunca aparecen en este evento.
* `tool_input_size_bytes`: Tamaño de la entrada de herramienta serializada en JSON en bytes
* `tool_result_size_bytes`: Tamaño del resultado de la herramienta en bytes
* `mcp_server_scope`: Identificador de alcance del servidor MCP (para herramientas MCP)
* `tool_parameters` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Cadena JSON que contiene parámetros específicos de la herramienta:
  * Para herramienta Bash: incluye `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`, y `git_commit_id` (el SHA del commit, cuando un comando `git commit` tiene éxito)
  * Para herramienta WorkspaceBash: incluye `bash_command`, `full_command`, `timeout`
  * Para herramientas MCP: incluye `mcp_server_name`, `mcp_tool_name`
  * Para herramienta Skill: incluye `skill_name`
  * Para herramienta Agent o herramienta Task heredada: incluye `subagent_type`
* `tool_input` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Argumentos de herramienta serializados en JSON. Los valores individuales superiores a 512 caracteres se truncan, y la carga útil completa está limitada a aproximadamente 4 K caracteres. Se aplica a todas las herramientas, incluidas las herramientas MCP.

<h4 id="api-request-event">
  Evento de solicitud de API
</h4>

Se registra para cada solicitud de API a Claude.

**Nombre del Evento**: `claude_code.api_request`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `model`: Modelo utilizado (por ejemplo, "claude-sonnet-5")
* `cost_usd`: Costo estimado en USD
* `duration_ms`: Duración de la solicitud en milisegundos
* `input_tokens`: Número de tokens de entrada
* `output_tokens`: Número de tokens de salida
* `cache_read_tokens`: Número de tokens leídos del caché
* `cache_creation_tokens`: Número de tokens utilizados para la creación del caché
* `request_id`: ID de solicitud de API de Anthropic del encabezado `request-id` de la respuesta, como `"req_011..."`. Presente solo cuando la API devuelve uno.
* `speed`: `"fast"` o `"normal"`, indicando si el modo rápido estaba activo
* `query_source`: Subsistema que emitió la solicitud, como `"repl_main_thread"`, `"compact"`, o un nombre de subagente
* `effort`: [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud: `"low"`, `"medium"`, `"high"`, `"xhigh"`, o `"max"`. Ausente cuando el modelo no admite esfuerzo.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribución de habilidad, plugin, agente y MCP para la solicitud. Consulte [Contador de costo](#cost-counter) para definiciones y comportamiento de redacción.

<h4 id="api-error-event">
  Evento de error de API
</h4>

Se registra cuando una solicitud de API a Claude falla.

**Nombre del Evento**: `claude_code.api_error`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `model`: Modelo utilizado (por ejemplo, "claude-sonnet-5")
* `error`: Mensaje de error
* `status_code`: Código de estado HTTP como número. Ausente para errores no HTTP como fallos de conexión.
* `duration_ms`: Duración de la solicitud en milisegundos
* `attempt`: Número total de intentos realizados, incluyendo la solicitud inicial (`1` significa que no ocurrieron reintentos)
* `request_id`: ID de solicitud de API de Anthropic del encabezado `request-id` de la respuesta, como `"req_011..."`. Presente solo cuando la API devuelve uno.
* `speed`: `"fast"` o `"normal"`, indicando si el modo rápido estaba activo
* `query_source`: Subsistema que emitió la solicitud, como `"repl_main_thread"`, `"compact"`, o un nombre de subagente
* `effort`: [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud. Ausente cuando el modelo no admite esfuerzo.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribución de habilidad, plugin, agente y MCP para la solicitud. Consulte [Contador de costo](#cost-counter) para definiciones y comportamiento de redacción.

<h4 id="api-refusal-event">
  Evento de rechazo de API
</h4>

Se registra cuando una solicitud de API devuelve `stop_reason: "refusal"`. Los rechazos llegan en un flujo de respuesta exitoso en lugar de como un error HTTP, por lo que el evento `api_error` no se activa para ellos. Este evento le permite rastrear la frecuencia de rechazo y agrupar rechazos por los mismos atributos que `api_request` y `api_error`.

**Nombre del Evento**: `claude_code.api_refusal`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `model`: Identificador de modelo de la solicitud
* `request_id`: ID de solicitud de API de Anthropic del encabezado `request-id` de la respuesta, como `"req_011..."`. Presente solo cuando la API devuelve uno.
* `query_source`: Subsistema que emitió la solicitud, como `"repl_main_thread"`, `"compact"`, o un nombre de subagente. Consulte [`api_request`](#api-request-event) para definiciones.
* `speed`: Ya sea `"fast"` cuando [Modo rápido](/docs/es/fast-mode) está activo, o `"normal"`
* `attempt`: Número de intento de reintento. El primer intento es `1`.
* `effort`: [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud. Ausente cuando el modelo no admite esfuerzo.
* `server_fallback_hop`: `true` cuando el fallback de modelo del lado del servidor de la API ya reintentó este rechazo en un modelo diferente, por lo que el usuario no vio este rechazo en particular. `false` cuando la solicitud terminó en un rechazo. Un único turno puede emitir tanto un evento hop `true` como un evento final `false` posterior cuando el modelo de fallback también rechaza.
* `has_category`: `true` cuando la respuesta de la API llevaba una `stop_details.category` de `"cyber"`, `"bio"`, `"frontier_llm"`, o `"reasoning_extraction"`. `false` cuando la respuesta no llevaba categoría o un valor fuera de ese conjunto. Ausente cuando `server_fallback_hop` es `true`, porque los bloques hop no llevan `stop_details`.
* `has_explanation`: `true` cuando la respuesta de la API llevaba una `stop_details.explanation`, de otra manera `false`. Ausente cuando `server_fallback_hop` es `true`.
* `category`: El valor `stop_details.category` de la respuesta de la API. Uno de `"cyber"`, `"bio"`, `"frontier_llm"`, o `"reasoning_extraction"`. Solo presente cuando `OTEL_LOG_TOOL_DETAILS=1` está establecido y `has_category` es `true`.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribución de skill, plugin, agente y MCP para la solicitud. Consulte [Contador de costo](#cost-counter) para definiciones y comportamiento de redacción.

<h4 id="api-request-body-event">
  Evento de cuerpo de solicitud de API
</h4>

Se registra para cada intento de solicitud de API cuando `OTEL_LOG_RAW_API_BODIES` está establecido. Se emite un evento por intento, por lo que los reintentos con parámetros ajustados producen cada uno su propio evento.

**Nombre del Evento**: `claude_code.api_request_body`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `body`: Parámetros de solicitud de API de Mensajes serializados en JSON (mensaje del sistema, mensajes, herramientas, etc.), truncados en 60 KB. El contenido de pensamiento extendido en turnos de asistente anteriores se redacta. Se emite solo en modo en línea (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Ruta absoluta a un archivo `<dir>/<uuid>.request.json` que contiene el cuerpo sin truncar. Se emite solo en modo de archivo (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Longitud del cuerpo sin truncar. Bytes UTF-8 cuando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, o unidades de código UTF-16 cuando `=1`
* `body_truncated`: `"true"` cuando ocurrió truncamiento en línea. Ausente en modo de archivo y cuando no ocurrió truncamiento.
* `model`: Identificador de modelo de los parámetros de solicitud
* `query_source`: Subsistema que emitió la solicitud (por ejemplo, `"compact"`)

<h4 id="api-response-body-event">
  Evento de cuerpo de respuesta de API
</h4>

Se registra para cada respuesta de API exitosa cuando `OTEL_LOG_RAW_API_BODIES` está establecido.

**Nombre del Evento**: `claude_code.api_response_body`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `body`: Respuesta de API de Mensajes serializada en JSON (id, bloques de contenido, uso, razón de parada), truncada en 60 KB. El contenido de pensamiento extendido se redacta. Se emite solo en modo en línea (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Ruta absoluta a un archivo `<dir>/<request_id>.response.json` que contiene el cuerpo sin truncar. Se emite solo en modo de archivo (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Longitud del cuerpo sin truncar. Bytes UTF-8 cuando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, o unidades de código UTF-16 cuando `=1`
* `body_truncated`: `"true"` cuando ocurrió truncamiento en línea. Ausente en modo de archivo y cuando no ocurrió truncamiento.
* `model`: Identificador de modelo
* `query_source`: Subsistema que emitió la solicitud
* `request_id`: ID de solicitud de API de Anthropic del encabezado `request-id` de la respuesta, como `"req_011..."`. Presente solo cuando la API devuelve uno.

<h4 id="tool-decision-event">
  Evento de decisión de herramienta
</h4>

Se registra cuando se toma una decisión de permiso de herramienta (aceptar/rechazar).

**Nombre del Evento**: `claude_code.tool_decision`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `tool_name`: Nombre de la herramienta (por ejemplo, "Read", "Edit", "Write", "NotebookEdit")
* `tool_use_id`: Identificador único para esta invocación de herramienta. Coincide con el `tool_use_id` pasado a hooks, permitiendo correlación entre eventos OTel y datos capturados por hooks.
* `decision`: Ya sea `"accept"` o `"reject"`
* `source`: Fuente de decisión:
  * `"config"`: Decidido automáticamente sin solicitar, basado en configuración del proyecto, reglas de permiso en la configuración personal del usuario, política administrada empresarial, banderas `--allowedTools` o `--disallowedTools`, el modo de permiso activo, una concesión con alcance de sesión de un mensaje anterior en la misma sesión de CLI interactiva, o porque la herramienta es inherentemente segura. El evento no indica cuál de estas fuentes coincidió.
  * `"hook"`: Un hook `PreToolUse` o `PermissionRequest` devolvió la decisión.
  * `"user_permanent"`: Se emite cuando el usuario eligió "Sí, y no preguntes de nuevo para ..." en un mensaje de permiso, que guarda una regla de permiso en su configuración personal. En la CLI interactiva esto se emite solo para esa opción en sí; las llamadas posteriores que coincidan con la regla guardada emiten `"config"` en su lugar. En sesiones del SDK de Agent o no interactivas `-p`, tanto la opción inicial como las coincidencias de regla posteriores emiten `"user_permanent"`. Se trata como una aceptación.
  * `"user_temporary"`: Se emite cuando el usuario eligió "Sí" en un mensaje de permiso para una aprobación única, o eligió una de las opciones "... durante esta sesión" en un mensaje de edición o lectura de archivo. En la CLI interactiva esto se emite solo para la opción en sí; las llamadas posteriores permitidas por esa concesión con alcance de sesión emiten `"config"` en su lugar. En sesiones del SDK de Agent o no interactivas `-p`, tanto la opción como las coincidencias posteriores emiten `"user_temporary"`. Se trata como una aceptación.
  * `"user_abort"`: Se emite cuando el usuario descartó el mensaje de permiso sin responder. Se trata como un rechazo.
  * `"user_reject"`: Se emite cuando el usuario eligió "No" cuando se le solicitó. En la CLI interactiva esto se emite solo para esa opción en sí; las llamadas que coincidan con una regla de denegación en la configuración personal del usuario emiten `"config"` en su lugar. En sesiones del SDK de Agent o no interactivas `-p`, las llamadas que coincidan con una regla de denegación en la configuración personal emiten `"user_reject"`. Se trata como un rechazo.
* `tool_parameters` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Cadena JSON que contiene parámetros específicos de la herramienta. Misma forma que el [Evento de resultado de herramienta](#tool-result-event), menos campos posteriores a la ejecución como `git_commit_id`. Los valores pueden diferir de `tool_result` para una llamada aceptada si la decisión de permiso reescribe la entrada de herramienta a través de `updatedInput`. Use este atributo para ver qué comando fue rechazado cuando `decision` es `"reject"`.
  * Para herramienta Bash: incluye `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`
  * Para herramienta WorkspaceBash: incluye `bash_command`, `full_command`, `timeout`
  * Para herramientas MCP: incluye `mcp_server_name`, `mcp_tool_name`
  * Para herramienta Skill: incluye `skill_name`
  * Para herramienta Agent o herramienta Task heredada: incluye `subagent_type`

<h4 id="permission-mode-changed-event">
  Evento de cambio de modo de permiso
</h4>

Se registra cuando el modo de permiso cambia, por ejemplo al ciclar con Shift+Tab, salir del modo de plan, o una verificación de puerta de modo automático.

**Nombre del Evento**: `claude_code.permission_mode_changed`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `from_mode`: El modo de permiso anterior, por ejemplo `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, o `"bypassPermissions"`
* `to_mode`: El nuevo modo de permiso
* `trigger`: Qué causó el cambio. Uno de `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"`, o `"auto_opt_in"`. Ausente cuando la transición se origina del SDK o puente

<h4 id="auth-event">
  Evento de autenticación
</h4>

Se registra cuando `/login` o `/logout` se completa.

**Nombre del Evento**: `claude_code.auth`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"auth"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `action`: `"login"` o `"logout"`
* `success`: `"true"` o `"false"`
* `auth_method`: Método de autenticación, como `"oauth"`
* `error_category`: Tipo de error categórico cuando la acción falló. El mensaje de error sin procesar nunca se incluye
* `status_code`: Código de estado HTTP como cadena cuando la acción falló con un error HTTP

<h4 id="mcp-server-connection-event">
  Evento de conexión del servidor MCP
</h4>

Se registra cuando un servidor MCP se conecta, desconecta o falla al conectarse.

**Nombre del Evento**: `claude_code.mcp_server_connection`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `status`: `"connected"`, `"failed"`, o `"disconnected"`
* `transport_type`: Transporte del servidor, como `"stdio"`, `"sse"`, o `"http"`
* `server_scope`: Alcance en el que está configurado el servidor, como `"user"`, `"project"`, o `"local"`
* `duration_ms`: Duración del intento de conexión en milisegundos
* `error_code`: Código de error cuando la conexión falló
* `is_plugin`: `true` cuando el servidor es proporcionado por un plugin, `false` de otra manera
* `plugin_id_hash` (cuando `is_plugin` es `true`): Hash estable del nombre del plugin y mercado, para agrupar eventos por plugin sin exponer el nombre
* `plugin.name` (cuando `is_plugin` es `true`): Nombre del plugin que proporciona el servidor. Para plugins de terceros esto es la cadena literal `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`; esto protege los nombres de plugins de terceros de aparecer en registros por defecto. Los plugins de fuentes oficiales de Anthropic siempre se identifican por nombre. Los atributos `plugin_id_hash` y `plugin.name` fluyen a su propio backend de monitoreo y no se envían a Anthropic
* `server_name` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Nombre del servidor configurado
* `error` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Mensaje de error completo cuando la conexión falló

<h4 id="internal-error-event">
  Evento de error interno
</h4>

Se registra cuando Claude Code captura un error interno inesperado. Solo se registran el nombre de la clase de error y un código de estilo errno. El mensaje de error y el seguimiento de pila nunca se incluyen. Este evento no se emite cuando se ejecuta contra Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry, o cuando `DISABLE_ERROR_REPORTING` está establecido.

**Nombre del Evento**: `claude_code.internal_error`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `error_name`: Nombre de la clase de error, como `"TypeError"` o `"SyntaxError"`
* `error_code`: Código errno de Node.js como `"ENOENT"` cuando está presente en el error

<h4 id="plugin-installed-event">
  Evento de plugin instalado
</h4>

Se registra cuando un plugin termina de instalarse, tanto desde el comando CLI `claude plugin install` como desde la interfaz de usuario interactiva `/plugin`.

**Nombre del Evento**: `claude_code.plugin_installed`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `marketplace.is_official`: `"true"` si el mercado es un mercado oficial de Anthropic, `"false"` de otra manera
* `install.trigger`: `"cli"` o `"ui"`
* `plugin.name`: Nombre del plugin instalado. Para mercados de terceros esto se incluye solo cuando `OTEL_LOG_TOOL_DETAILS=1`
* `plugin.version`: Versión del plugin cuando se declara en la entrada del mercado. Para mercados de terceros esto se incluye solo cuando `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Mercado desde el que se instaló el plugin. Para mercados de terceros esto se incluye solo cuando `OTEL_LOG_TOOL_DETAILS=1`

<h4 id="plugin-loaded-event">
  Evento de plugin cargado
</h4>

Se registra una vez por plugin habilitado al inicio de la sesión. Use este evento para inventariar qué plugins están activos en su flota, como complemento a `plugin_installed` que registra la acción de instalación en sí.

**Nombre del Evento**: `claude_code.plugin_loaded`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `plugin.name`: nombre del plugin. Para plugins fuera del mercado oficial y paquete integrado el valor es `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: mercado desde el que se instaló el plugin, cuando se conoce. Redactado a `"third-party"` bajo la misma condición que `plugin.name`
* `plugin.version`: versión del manifiesto del plugin. Se incluye solo cuando el nombre no está redactado y el manifiesto declara una versión
* `plugin.scope`: categoría de procedencia para el plugin: `"official"`, `"org"`, `"user-local"`, o `"default-bundle"`
* `enabled_via`: cómo el plugin llegó a estar habilitado: `"default-enable"`, `"org-policy"`, `"seed-mount"`, o `"user-install"`
* `plugin_id_hash`: hash determinista del nombre del plugin y mercado, enviado solo a su exportador configurado. Le permite contar cuántos plugins de terceros distintos se cargan en su flota sin registrar sus nombres
* `has_hooks`: si el plugin contribuye hooks
* `has_mcp`: si el plugin contribuye servidores MCP
* `host_owned_mcp`: `true` cuando el host del SDK administra las conexiones MCP de este plugin y Claude Code omitió leer la configuración del servidor MCP del plugin, `false` de otra manera. Requiere Claude Code v2.1.172 o posterior
* `skill_path_count`: número de directorios de habilidades que declara el plugin
* `command_path_count`: número de directorios de comandos que declara el plugin
* `agent_path_count`: número de directorios de agentes que declara el plugin
* `safe_mode`: `"true"` cuando la sesión se inició con [`--safe-mode`](/docs/es/cli-reference), `"false"` de otra manera. En modo seguro este evento reporta solo inventario configurado; los comandos, habilidades, hooks y servidores MCP del plugin no se cargan. Requiere Claude Code v2.1.169 o posterior

<h4 id="skill-activated-event">
  Evento de habilidad activada
</h4>

Se registra cuando se invoca una habilidad, ya sea que Claude la llame a través de la herramienta Skill o que la ejecute como un comando `/`.

**Nombre del Evento**: `claude_code.skill_activated`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `skill.name`: Nombre de la habilidad. Para habilidades definidas por el usuario y de plugin de terceros el valor es el marcador de posición `"custom_skill"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger`: Cómo se activó la habilidad (`"user-slash"`, `"claude-proactive"`, o `"nested-skill"`)
* `skill.source`: De dónde se cargó la habilidad (por ejemplo, `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: `"workflow"` cuando la habilidad es una habilidad de flujo de trabajo. Ausente de otra manera
* `plugin.name` (cuando `OTEL_LOG_TOOL_DETAILS=1` o el plugin es de un mercado oficial): Nombre del plugin propietario cuando la habilidad es proporcionada por un plugin
* `marketplace.name` (cuando `OTEL_LOG_TOOL_DETAILS=1` o el plugin es de un mercado oficial): Mercado desde el que se instaló el plugin propietario, cuando la habilidad es proporcionada por un plugin

<h4 id="at-mention-event">
  Evento de mención @
</h4>

Se registra cuando Claude Code resuelve una mención `@` en un mensaje. No todas las menciones emiten un evento: las rutas de salida anticipada como denegaciones de permisos, archivos de tamaño excesivo, archivos adjuntos de referencia PDF, y fallos de listado de directorios se devuelven sin registrar.

**Nombre del Evento**: `claude_code.at_mention`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `mention_type`: Tipo de mención (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`)
* `success`: Si la mención se resolvió exitosamente (`"true"` o `"false"`)

<h4 id="api-retries-exhausted-event">
  Evento de reintentos de API agotados
</h4>

Se registra una vez cuando una solicitud de API falla después de más de un intento. Se emite junto con el evento `api_error` final.

**Nombre del Evento**: `claude_code.api_retries_exhausted`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `model`: Modelo utilizado
* `error`: Mensaje de error final
* `status_code`: Código de estado HTTP como número. Ausente para errores no HTTP.
* `total_attempts`: Número total de intentos realizados
* `total_retry_duration_ms`: Tiempo total de pared en todos los intentos
* `speed`: `"fast"` o `"normal"`

<h4 id="hook-registered-event">
  Evento de hook registrado
</h4>

Se registra una vez por hook configurado al inicio de la sesión. Use este evento para inventariar qué hooks están activos en su flota, como complemento a los eventos `hook_execution_start` y `hook_execution_complete` por ejecución.

**Nombre del Evento**: `claude_code.hook_registered`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `hook_event`: tipo de evento de hook, como `"PreToolUse"` o `"PostToolUse"`
* `hook_type`: tipo de implementación de hook: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"`, o `"agent"`
* `hook_source`: dónde se define el hook: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"`, o `"pluginHook"`
* `safe_mode`: `"true"` cuando la sesión se inició con [`--safe-mode`](/docs/es/cli-reference), `"false"` de otra manera. Requiere Claude Code v2.1.169 o posterior
* `hook_matcher` (cuando `OTEL_LOG_TOOL_DETAILS=1`): la cadena de coincidencia de la configuración del hook, cuando se establece una
* `plugin.name` (cuando `hook_source` es `"pluginHook"`): nombre del plugin contribuyente. Para plugins fuera del mercado oficial y paquete integrado el valor es `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (cuando `hook_source` es `"pluginHook"`): hash determinista del nombre del plugin y mercado, enviado solo a su exportador configurado. Le permite contar plugins contribuyentes distintos sin registrar sus nombres

<h4 id="hook-execution-start-event">
  Evento de inicio de ejecución de hook
</h4>

Se registra cuando uno o más hooks comienzan a ejecutarse para un evento de hook.

**Nombre del Evento**: `claude_code.hook_execution_start`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `hook_event`: Tipo de evento de hook, como `"PreToolUse"` o `"PostToolUse"`
* `hook_name`: Nombre completo del hook incluyendo coincidencia, como `"PreToolUse:Write"`
* `num_hooks`: Número de comandos de hook coincidentes
* `managed_only`: `"true"` cuando solo se permiten hooks de política administrada
* `hook_source`: `"policySettings"` o `"merged"`
* `safe_mode`: `"true"` cuando la sesión se inició con [`--safe-mode`](/docs/es/cli-reference), `"false"` de otra manera. Requiere Claude Code v2.1.169 o posterior
* `hook_definitions`: Configuración de hook serializada en JSON. Se incluye solo cuando tanto el trazado beta detallado como `OTEL_LOG_TOOL_DETAILS=1` están habilitados

<h4 id="hook-execution-complete-event">
  Evento de ejecución de hook completado
</h4>

Se registra cuando todos los hooks para un evento de hook han terminado.

**Nombre del Evento**: `claude_code.hook_execution_complete`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `hook_event`: Tipo de evento de hook
* `hook_name`: Nombre completo del hook incluyendo coincidencia
* `num_hooks`: Número de comandos de hook coincidentes
* `num_success`: Recuento que se completó exitosamente
* `num_blocking`: Recuento que devolvió una decisión de bloqueo
* `num_non_blocking_error`: Recuento que falló sin bloquear
* `num_cancelled`: Recuento cancelado antes de completarse
* `total_duration_ms`: Duración de pared de todos los hooks coincidentes
* `managed_only`: `"true"` cuando solo se permiten hooks de política administrada
* `hook_source`: `"policySettings"` o `"merged"`
* `safe_mode`: `"true"` cuando la sesión se inició con [`--safe-mode`](/docs/es/cli-reference), `"false"` de otra manera. Requiere Claude Code v2.1.169 o posterior
* `hook_definitions`: Configuración de hook serializada en JSON. Se incluye solo cuando tanto el trazado beta detallado como `OTEL_LOG_TOOL_DETAILS=1` están habilitados

<h4 id="hook-plugin-metrics-event">
  Evento de métricas de plugin de hook
</h4>

Se registra cuando un hook de plugin del mercado oficial emite métricas por invocación. Solo los plugins instalados desde un mercado oficial de Anthropic pueden emitir estos. Los plugins de mercado de terceros y los hooks configurados por el usuario no emiten a este evento. Use este evento para monitorear el comportamiento del plugin como tasas de búsqueda, costos y duraciones desde su propia pila de observabilidad.

**Nombre del Evento**: `claude_code.hook_plugin_metrics`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `plugin_id`: identificador del plugin en forma `<name>@<marketplace>`
* `hook_event`: tipo de evento de hook que emitió las métricas
* Hasta 20 claves de métricas emitidas por el plugin. Los nombres coinciden con `^[a-z][a-z0-9_]{0,39}$`. Los valores son booleanos o números.

<h4 id="compaction-event">
  Evento de compactación
</h4>

Se registra cuando la compactación de conversación se completa.

**Nombre del Evento**: `claude_code.compaction`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"compaction"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `trigger`: `"auto"` o `"manual"`
* `success`: `"true"` o `"false"`
* `duration_ms`: Duración de compactación
* `pre_tokens`: Recuento aproximado de tokens antes de compactación
* `post_tokens`: Recuento aproximado de tokens después de compactación
* `error`: Mensaje de error cuando la compactación falló
* `precompute_reuse`: Solo se establece cuando `trigger` es `"manual"`. La compactación automática puede preparar un resumen en segundo plano antes de que se llene la ventana de contexto, y este atributo registra si `/compact` reutilizó ese resumen preparado. `"hit"` significa que se reutilizó; `"miss_custom_instructions"`, `"miss_hook"`, y `"miss_not_ready"` dan la razón por la que se calculó un resumen fresco en su lugar. Requiere Claude Code v2.1.153 o posterior

<h4 id="feedback-survey-event">
  Evento de encuesta de retroalimentación
</h4>

Se registra cuando se muestra una encuesta de calidad de sesión o se responde. Consulte [Encuestas de calidad de sesión](/docs/es/data-usage#session-quality-surveys) para saber qué recopilan las encuestas y cómo controlarlas.

**Nombre del Evento**: `claude_code.feedback_survey`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: Contador monotónicamente creciente para ordenar eventos dentro de una sesión
* `event_type`: Evento del ciclo de vida de la encuesta, por ejemplo `"appeared"`, `"responded"`, o `"transcript_prompt_appeared"`
* `appearance_id`: ID único que vincula los eventos emitidos para una instancia de encuesta
* `survey_type`: Qué encuesta produjo el evento. `"session"` es el mensaje de calificación "¿Cómo está Claude?"
* `response`: La selección del usuario en eventos `responded`
* `enabled_via_override`: `true` cuando [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/es/env-vars) está establecido. Se emite como booleano, no como cadena. Presente en eventos de encuesta `session`. Filtre en este atributo para confirmar que la anulación se aplica en toda una flota

<h2 id="interpret-metrics-and-events-data">
  Interpretar datos de métricas y eventos
</h2>

Las métricas y eventos exportados admiten una variedad de análisis:

<h3 id="usage-monitoring">
  Monitoreo de uso
</h3>

| Métrica                                                       | Oportunidad de Análisis                                                                                     |
| ------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `claude_code.token.usage`                                     | Desglosar por `type` (entrada/salida), usuario, equipo, modelo, `skill.name`, `plugin.name`, o `agent.name` |
| `claude_code.session.count`                                   | Rastrear adopción y compromiso a lo largo del tiempo                                                        |
| `claude_code.lines_of_code.count`                             | Medir productividad rastreando adiciones y eliminaciones de código, desglosado por modelo                   |
| `claude_code.commit.count` & `claude_code.pull_request.count` | Entender el impacto en los flujos de trabajo de desarrollo                                                  |

<h3 id="cost-monitoring">
  Monitoreo de costos
</h3>

La métrica `claude_code.cost.usage` ayuda con:

* Rastrear tendencias de uso entre equipos o individuos
* Identificar sesiones de alto uso para optimización
* Atribuir gastos a habilidades, plugins, o tipos de subagente específicos a través de los atributos `skill.name`, `plugin.name`, y `agent.name`

<Note>
  Las métricas de costo son aproximaciones. Para datos de facturación oficiales, consulte su proveedor de API (Claude Console, Amazon Bedrock, o Google Cloud's Agent Platform).
</Note>

<h3 id="alerting-and-segmentation">
  Alertas y segmentación
</h3>

Alertas comunes a considerar:

* Picos de costo
* Consumo inusual de tokens
* Alto volumen de sesiones de usuarios específicos

Todas las métricas pueden segmentarse por los [atributos estándar](#standard-attributes). El atributo `model` está disponible en `claude_code.token.usage`, `claude_code.cost.usage`, y a partir de v2.1.172, `claude_code.lines_of_code.count`.

Los desgloses por modelo de confirmaciones solo pueden aproximarse uniéndose contra las métricas de token o costo en `session.id`, ya que una sesión puede abarcar múltiples modelos. Filtre el lado de token o costo a filas donde `query_source` es `"main"` para que las solicitudes auxiliares y de subagente no atribuyan los compromisos de la sesión a un modelo que no los realizó.

<h3 id="detect-retry-exhaustion">
  Detectar agotamiento de reintentos
</h3>

Claude Code reintenta solicitudes de API fallidas internamente y emite un único evento `claude_code.api_error` solo después de rendirse, por lo que el evento en sí es la señal terminal para esa solicitud. Los intentos de reintento intermedios no se registran como eventos separados.

El atributo `attempt` en el evento registra el número total de intentos. `CLAUDE_CODE_MAX_RETRIES` tiene un valor predeterminado de 10 y está limitado a 15; a partir de v2.1.199, `CLAUDE_CODE_RETRY_WATCHDOG` aumenta el valor predeterminado y elimina el límite. Cuando la solicitud agota todos los reintentos en un error transitorio, `attempt` es igual a uno más que ese límite efectivo: 11 por defecto, y nunca más de 16 a menos que el vigilante esté configurado. Un valor más bajo indica un error no reintentable como una respuesta `400`.

Para distinguir una sesión que se recuperó de una que se estancó, agrupe eventos por `session.id` y verifique si existe un evento `api_request` posterior después del error.

<h3 id="event-analysis">
  Análisis de eventos
</h3>

Los datos de eventos proporcionan información detallada sobre las interacciones de Claude Code:

**Patrones de Uso de Herramientas**: analice eventos de resultado de herramientas para identificar:

* Herramientas más utilizadas frecuentemente
* Tasas de éxito de herramientas
* Tiempos de ejecución promedio de herramientas
* Patrones de error por tipo de herramienta

**Monitoreo de Rendimiento**: rastree duraciones de solicitudes de API y tiempos de ejecución de herramientas para identificar cuellos de botella de rendimiento.

<h2 id="audit-security-events">
  Auditar eventos de seguridad
</h2>

Los eventos de OpenTelemetry son la fuente de datos de auditoría para la actividad de Claude Code. Cada evento lleva atributos de identidad que vinculan llamadas de herramientas, actividad MCP y decisiones de permisos al usuario que las desencadenó. El exportador de registros OTLP puede entregar estos eventos a cualquier plataforma de Gestión de Información y Eventos de Seguridad (SIEM) con un receptor OTLP, o a un Recopilador de OpenTelemetry que reenvíe a su SIEM.

<h3 id="attribute-actions-to-users">
  Atribuir acciones a usuarios
</h3>

Los [atributos estándar](#standard-attributes) en cada evento incluyen la identidad del usuario autenticado: `user.email`, `user.account_uuid`, `user.account_id`, y `organization.id` cuando se inicia sesión con una cuenta de Claude, más `user.id` y el `session.id` por sesión. `user.id` es un identificador con alcance de instalación, excepto en sesiones de [Claude apps gateway](/docs/es/claude-apps-gateway), donde es el asunto del IdP del token emitido por la puerta de enlace.

Las llamadas de herramientas MCP, comandos Bash y ediciones de archivos se atribuyen por lo tanto al desarrollador que inició la sesión. Claude Code no actúa bajo una cuenta de servicio separada; la identidad registrada en cada evento es la propia cuenta de Claude del desarrollador, o la identidad del IdP del desarrollador en una sesión de [Claude apps gateway](/docs/es/claude-apps-gateway).

Cuando Claude Code se autentica con una clave de API directa, o contra Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry, no hay cuenta de Claude en la sesión y solo `user.id` y `session.id` se rellenan. En estas implementaciones, adjunte la identidad del usuario usted mismo con `OTEL_RESOURCE_ATTRIBUTES`, establecido por usuario a través del archivo de [configuración administrada](#administrator-configuration) o un contenedor de lanzamiento. Las sesiones de Claude apps gateway no necesitan nada de esto: la CLI marca la identidad del IdP automáticamente, como se describe en [Atributos estándar](#standard-attributes).

```bash theme={null}
export OTEL_RESOURCE_ATTRIBUTES="enduser.id=jdoe@example.com,enduser.directory_id=S-1-5-21-..."
```

<h3 id="audit-mcp-activity">
  Auditar actividad MCP
</h3>

Para capturar la actividad del servidor MCP con detalle completo de llamadas, habilite el exportador de registros y establezca `OTEL_LOG_TOOL_DETAILS=1`. Cada operación MCP produce entonces eventos estructurados que llevan el nombre del servidor, nombre de la herramienta y argumentos de llamada junto con los atributos de identidad estándar:

| Evento                  | Qué registra para MCP                                                                                                                                                                                                         |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_server_connection` | Conexión del servidor, desconexión y fallo de conexión con `server_name`, `transport_type`, `server_scope`, y detalle de error                                                                                                |
| `tool_result`           | Cada llamada de herramienta MCP con `tool_name` y `mcp_server_scope`, una carga útil `tool_parameters` que contiene `mcp_server_name` y `mcp_tool_name`, y una carga útil `tool_input` que contiene los argumentos de llamada |
| `tool_decision`         | Si la llamada fue permitida o denegada, si la decisión provino de configuración, un hook, o el usuario, y una carga útil `tool_parameters` que contiene `mcp_server_name` y `mcp_tool_name`                                   |

Sin `OTEL_LOG_TOOL_DETAILS`, estos eventos descartan el detalle de identificación:

* `tool_result`: mantiene `tool_name` y `mcp_server_scope`, omite `mcp_server_name`, `mcp_tool_name`, y argumentos
* `tool_decision`: mantiene `tool_name`, omite `tool_parameters`
* `mcp_server_connection`: omite `server_name` y el mensaje de error, pero mantiene `is_plugin`, `plugin_id_hash`, y `plugin.name`, con nombres de plugins que no son de Anthropic redactados al literal `"third-party"`, por lo que los servidores proporcionados por plugins siguen siendo distinguibles sin registro detallado

<h3 id="map-security-questions-to-events">
  Mapear preguntas de seguridad a eventos
</h3>

Al construir reglas de detección, busque la señal que desea monitorear y consulte su backend para el evento correspondiente y atributos:

| Señal                                                       | Evento                                                                                | Atributos Clave                                              |
| ----------------------------------------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| Llamada de herramienta permitida o denegada, y por qué      | `tool_decision`                                                                       | `decision`, `source`, `tool_name`, `tool_parameters`         |
| Escalada de modo de permiso                                 | `permission_mode_changed`                                                             | `from_mode`, `to_mode`, `trigger`                            |
| Hook de política bloqueó una acción                         | `hook_execution_complete`                                                             | `hook_event`, `num_blocking`                                 |
| Inicio de sesión, cierre de sesión y fallo de autenticación | `auth`                                                                                | `action`, `success`, `error_category`                        |
| Conexión del servidor MCP o fallo                           | `mcp_server_connection`                                                               | `status`, `server_name`, `is_plugin`, `error_code`           |
| Plugin instalado y su origen                                | `plugin_installed`                                                                    | `plugin.name`, `marketplace.name`, `marketplace.is_official` |
| Comandos ejecutados y archivos tocados                      | `tool_result` (ejecutado) o `tool_decision` (rechazado) con `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters`; `tool_input` (`tool_result` solo)         |

Claude Code emite solo el flujo de eventos sin procesar. La detección de anomalías, establecimiento de línea base, correlación entre sesiones y alertas son responsabilidad de su SIEM o backend de observabilidad.

<h3 id="send-events-to-a-siem">
  Enviar eventos a un SIEM
</h3>

Apunte `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` al receptor OTLP de su SIEM, o a un Recopilador de OpenTelemetry que reenvíe a la API de ingesta nativa de su SIEM. El siguiente ejemplo de configuración de ajustes administrados exporta solo eventos, con detalle completo de herramientas habilitado para auditoría de MCP y Bash:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_LOG_TOOL_DETAILS": "1",
    "OTEL_EXPORTER_OTLP_LOGS_PROTOCOL": "http/protobuf",
    "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT": "https://siem.example.com:4318/v1/logs",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-siem-token"
  }
}
```

<h2 id="backend-considerations">
  Consideraciones de backend
</h2>

Tu elección de backends de métricas, registros y trazas determina los tipos de análisis que puedes realizar:

<h3 id="for-metrics">
  Para métricas
</h3>

* **Bases de datos de series temporales (por ejemplo, Prometheus)**: Cálculos de tasa, métricas agregadas
* **Almacenes columnares (por ejemplo, ClickHouse)**: Consultas complejas, análisis de usuario único
* **Plataformas de observabilidad completas (por ejemplo, Honeycomb, Datadog, Grafana Cloud)**: Consultas avanzadas, visualización, alertas

<h3 id="for-events/logs">
  Para eventos/registros
</h3>

* **Sistemas de agregación de registros (por ejemplo, Elasticsearch, Loki)**: Búsqueda de texto completo, análisis de registros
* **Almacenes columnares (por ejemplo, ClickHouse)**: Análisis de eventos estructurados
* **Plataformas de observabilidad completas (por ejemplo, Honeycomb, Datadog, Grafana Cloud)**: Correlación entre métricas y eventos

<h3 id="for-traces">
  Para trazas
</h3>

Elige un backend que admita almacenamiento de trazas distribuidas y correlación de spans:

* **Sistemas de trazas distribuidas (por ejemplo, Jaeger, Zipkin, Grafana Tempo)**: Visualización de spans, cascadas de solicitudes, análisis de latencia
* **Plataformas de observabilidad completas (por ejemplo, Honeycomb, Datadog, Grafana Cloud)**: Búsqueda de trazas y correlación con métricas y registros

Para organizaciones que requieren métricas de Usuarios Activos Diarios/Semanales/Mensuales (DAU/WAU/MAU), considera backends que admitan consultas de valores únicos eficientes.

<h2 id="service-information">
  Información del servicio
</h2>

Todas las métricas y eventos se exportan con los siguientes atributos de recurso:

* `service.name`: `claude-code`
* `service.version`: Versión actual de Claude Code
* `os.type`: Tipo de sistema operativo (por ejemplo, `linux`, `darwin`, `windows`)
* `os.version`: Cadena de versión del sistema operativo
* `host.arch`: Arquitectura del host (por ejemplo, `amd64`, `arm64`)
* `wsl.version`: Número de versión de WSL (solo presente cuando se ejecuta en Windows Subsystem for Linux)
* Nombre del Medidor: `com.anthropic.claude_code`

<h2 id="roi-measurement-resources">
  Recursos de medición de ROI
</h2>

Para una guía completa sobre cómo medir el retorno de inversión para Claude Code, incluyendo configuración de telemetría, análisis de costos, métricas de productividad e informes automatizados, consulta la [Guía de Medición de ROI de Claude Code](https://github.com/anthropics/claude-code-monitoring-guide). Este repositorio proporciona configuraciones de Docker Compose listas para usar, configuraciones de Prometheus y OpenTelemetry, y plantillas para generar informes de productividad integrados con herramientas como Linear.

<h2 id="security-and-privacy">
  Seguridad y privacidad
</h2>

* La exportación de OpenTelemetry a tu backend es opcional y requiere configuración explícita. Para la telemetría operativa separada de Anthropic y cómo deshabilitarla, consulta [Uso de datos](/docs/es/data-usage#telemetry-services)
* Los contenidos de archivos sin procesar y fragmentos de código no se incluyen en métricas o eventos. Las trazas de span son una ruta de datos separada: consulta la viñeta `OTEL_LOG_TOOL_CONTENT` a continuación
* Cuando está autenticado a través de OAuth, `user.email` se incluye en atributos de telemetría. Si esto es una preocupación para tu organización, trabaja con tu backend de telemetría para filtrar o redactar este campo
* El contenido del mensaje del usuario no se recopila por defecto. Solo se registra la longitud del mensaje. Para incluir contenido del mensaje, establece `OTEL_LOG_USER_PROMPTS=1`
* El texto de respuesta del asistente no se recopila por defecto. Solo se registra la longitud de la respuesta. Para incluir texto de respuesta, establece `OTEL_LOG_ASSISTANT_RESPONSES=1`. Como todos los datos de OpenTelemetry de Claude Code, el texto de respuesta se envía solo al punto final de OTel que configures, nunca a Anthropic. Cuando esta variable no está establecida, `OTEL_LOG_USER_PROMPTS` se utiliza como alternativa, así que establece `OTEL_LOG_ASSISTANT_RESPONSES=0` si deseas contenido de mensaje sin contenido de respuesta
* Los argumentos de entrada de herramientas y parámetros no se registran por defecto. Para incluirlos, establece `OTEL_LOG_TOOL_DETAILS=1`. Estos datos se envían solo al punto final de OTEL que configures, nunca a Anthropic. Los argumentos aún pueden contener valores sensibles, así que configura tu backend de telemetría para filtrar o redactar estos atributos según sea necesario. Cuando está habilitado:
  * Los eventos `tool_result` y `tool_decision` incluyen un atributo `tool_parameters` con comandos Bash, nombres de servidor MCP y herramienta, y nombres de skills. Los campos como `full_command` se emiten sin truncar
  * Los eventos `tool_result` además incluyen un atributo `tool_input` con rutas de archivo, URLs, patrones de búsqueda y otros argumentos. Los valores individuales superiores a 512 caracteres se truncan y el total está limitado a aproximadamente 4 K caracteres
  * Los eventos `user_prompt` incluyen el `command_name` verbatim para comandos personalizados, de plugin y MCP
  * Los spans de traza incluyen el mismo atributo `tool_input` e atributos derivados de entrada como `file_path`, con el mismo truncamiento que `tool_input`
* El contenido de entrada y salida de herramientas no se registra en spans de trazas por defecto. Para incluirlo, establece `OTEL_LOG_TOOL_CONTENT=1`. Cuando está habilitado, los eventos de span incluyen contenido completo de entrada y salida de herramientas truncado en 60 KB por span. Esto puede incluir contenidos de archivo sin procesar de resultados de herramienta Read y salida de comandos Bash. Configura tu backend de telemetría para filtrar o redactar estos atributos según sea necesario
* Los cuerpos de solicitud y respuesta de la API de Mensajes de Anthropic sin procesar no se registran por defecto. Para incluirlos, establece `OTEL_LOG_RAW_API_BODIES`. Con `=1`, cada llamada de API emite eventos de registro `api_request_body` y `api_response_body` cuyo atributo `body` es la carga útil serializada en JSON, truncada en 60 KB. Con `=file:<dir>`, los cuerpos sin truncar se escriben en archivos `.request.json` y `.response.json` bajo ese directorio y los eventos llevan una ruta `body_ref` en su lugar del cuerpo en línea. Envía el directorio con un recopilador de registros o sidecar en lugar de a través del flujo de telemetría. En ambos modos, los cuerpos contienen el historial de conversación completo (mensaje del sistema, cada turno anterior de usuario y asistente, resultados de herramientas), por lo que habilitar esto implica consentimiento a todo lo que las otras banderas de contenido `OTEL_LOG_*` revelarían. El contenido de pensamiento extendido de Claude siempre se redacta de estos cuerpos independientemente de otras configuraciones

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Monitorear Claude Code en Amazon Bedrock
</h2>

Para orientación detallada sobre monitoreo de uso de Claude Code para Amazon Bedrock, consulta [Implementación de Monitoreo de Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
