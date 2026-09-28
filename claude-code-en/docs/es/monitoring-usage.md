> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Monitoreo

> Aprende cómo habilitar y configurar OpenTelemetry para Claude Code.

Rastrea el uso de Claude Code, costos y actividad de herramientas en toda tu organización exportando datos de telemetría a través de OpenTelemetry (OTel). Claude Code exporta métricas como datos de series temporales a través del protocolo estándar de métricas, eventos a través del protocolo de registros/eventos, y opcionalmente trazas distribuidas a través del [protocolo de trazas](#traces-beta).

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

# 5. Para depuración: reducir intervalos de exportación, y restablecerlos para uso en producción
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10 segundos (predeterminado: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5 segundos (predeterminado: 5000ms)

# 6. Ejecutar Claude Code
claude
```

Para verificar una configuración que exporta métricas, comprueba tu backend para la métrica `claude_code.session.count`, que Claude Code emite cuando se inicia una sesión. Para verificar una configuración solo de registros, envía un mensaje y comprueba el evento `claude_code.user_prompt`.

Si nada llega, ejecuta `claude --debug` y comprueba el registro de depuración. Claude Code reporta fallos de los exportadores que configures como errores `[3P telemetry]`, donde 3P significa terceros. Las líneas con prefijo `[Anthropic telemetry]` describen la [telemetría operativa separada de Anthropic](/docs/es/data-usage#telemetry-services) y no indican un problema con tu configuración.

Para opciones de configuración completas, consulta la [especificación de OpenTelemetry](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options).

<h2 id="administrator-configuration">
  Configuración del administrador
</h2>

Los administradores pueden configurar los ajustes de OpenTelemetry para todos los usuarios a través del [archivo de configuración administrada](/docs/es/managed-settings#delivery-mechanisms). Consulta la [precedencia de configuración](/docs/es/settings#settings-precedence) para obtener más información sobre cómo se aplican los ajustes.

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

Claude Code ignora las [variables del exportador de OpenTelemetry](/docs/es/settings-reference#variables-claude-code-ignores-in-env) en el `.claude/settings.json` y `.claude/settings.local.json` de un repositorio, por lo que un repositorio no puede usarlas para activar la telemetría, elegir dónde va, o capturar contenido. Establécelas en los ajustes administrados, o haz que cada desarrollador las establezca en su shell o `~/.claude/settings.json`. Un repositorio aún puede desactivar una señal estableciendo su selector de exportador, como `OTEL_LOGS_EXPORTER`, en `none`, a menos que los ajustes administrados, un archivo `--settings`, o el entorno desde el que inicias Claude Code establezca esa variable.

Claude Code no pasa variables de entorno `OTEL_*` a los subprocesos que genera, incluyendo la herramienta Bash, hooks, servidores MCP, y servidores de lenguaje. Una aplicación instrumentada con OpenTelemetry que ejecutes a través de la herramienta Bash no hereda el punto final del exportador de Claude Code ni los encabezados, así que establece esas variables directamente en el comando si esa aplicación necesita exportar su propia telemetría.

<h3 id="how-managed-settings-lock-the-otlp-destination">
  Cómo los ajustes administrados bloquean el destino OTLP
</h3>

Cuando estableces una variable `OTEL_EXPORTER_OTLP_*` en los ajustes administrados, Claude Code elimina las variables conflictivas establecidas por el desarrollador al inicio y registra una advertencia que puedes ver con `claude --debug`. Lo que elimina depende de qué variable establezca:

* **Puntos finales**: cuando estableces `OTEL_EXPORTER_OTLP_ENDPOINT`, Claude Code elimina todos los puntos finales por señal establecidos por el desarrollador. Los desarrolladores no pueden apuntar una señal a un recopilador diferente, así que no necesitas establecer también las variables de punto final por señal en los ajustes administrados.
* **Protocolos**: cuando estableces `OTEL_EXPORTER_OTLP_PROTOCOL`, Claude Code elimina todos los protocolos por señal establecidos por el desarrollador.
* **Credenciales**: cuando estableces `OTEL_EXPORTER_OTLP_HEADERS`, `OTEL_EXPORTER_OTLP_CLIENT_KEY`, u `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, Claude Code elimina las versiones por señal establecidas por el desarrollador de esa variable, más todas las variables de punto final establecidas por el desarrollador, genéricas o por señal, ya que esas credenciales de otro modo llegarían a un recopilador que los ajustes administrados no eligieron.
* **Selectores de exportador**: `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER`, y el `OTEL_TRACES_EXPORTER` beta siguen la precedencia normal por clave. Un ajuste del desarrollador aún puede desactivar una señal o cambiarla al exportador de consola, así que establece también los selectores en los ajustes administrados si necesitas que estén bloqueados. En [fuentes de administrador](/docs/es/managed-settings#precedence-within-the-managed-tier), `OTEL_LOGS_EXPORTER` sigue la [unidad de telemetría](/docs/es/server-managed-settings#per-key-exceptions-across-managed-sources) mientras que los otros dos selectores se fusionan por clave. Requiere Claude Code v2.1.223 o posterior.
* **Puntos finales de rastreo beta**: con [rastreo beta detallado](#traces-beta) activo, Claude Code exporta registros y rastreos a `BETA_TRACING_ENDPOINT` en lugar de a través de los exportadores de registros y rastreos. Claude Code por lo tanto elimina un `BETA_TRACING_ENDPOINT` establecido por el desarrollador siempre que cualquiera de estos ajustes administrados decida el destino de cualquiera de las señales:

  * Un punto final genérico o de registros/rastreos o credencial
  * Un [`otelHeadersHelper`](/docs/es/settings-reference#otelheadershelper)
  * Un selector de exportador de registros o rastreos establecido en `none`, `console`, o vacío, valores que mantienen la señal fuera de un recopilador
  * `CLAUDE_CODE_ENABLE_TELEMETRY` desactivado

  Un punto final o credencial solo de métricas no lo elimina. Antes de v2.1.251, un `BETA_TRACING_ENDPOINT` establecido por el desarrollador redirigía los registros y rastreos que el rastreo beta detallado exporta incluso cuando los ajustes administrados fijaban el recopilador.

Claude Code no elimina variables por señal que estableces en los ajustes administrados en sí, así que puedes enrutar una señal a un recopilador diferente estableciendo su variable allí, como hace el [ejemplo SIEM](#send-events-to-a-siem). Si estableces una credencial por señal allí, Claude Code elimina el punto final establecido por el desarrollador para esa señal.

Este comportamiento de eliminación cambia dónde se entrega la telemetría, no lo que Claude Code recopila.

Antes de v2.1.217, cada variable seguía la precedencia de ajustes por clave de forma independiente, así que un punto final específico de señal establecido en los ajustes del usuario o el shell redirigía esa señal lejos del recopilador administrado.

Cuando la aplicación de escritorio o un ejecutor de [entorno autohospedado](/docs/es/self-hosted-environments) inicia Claude Code y nombra un punto final OTLP en el entorno que proporciona, Claude Code fija el destino de la misma manera: las variables de telemetría del iniciador eliminan variables establecidas por el desarrollador exactamente como lo hacen los ajustes administrados. Claude Code no elimina variables que el iniciador en sí estableció. Requiere Claude Code v2.1.251 o posterior.

<h2 id="configuration-details">
  Detalles de configuración
</h2>

<h3 id="common-configuration-variables">
  Variables de configuración comunes
</h3>

Estas variables configuran exportadores, puntos finales y comportamiento de exportación para todas las implementaciones.

Si estableces una variable de punto final o protocolo por señal, como `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`, Claude Code la utiliza en lugar de la variable genérica para esa señal. Si estableces una variable de encabezados por señal, como `OTEL_EXPORTER_OTLP_METRICS_HEADERS`, Claude Code la fusiona con la genérica `OTEL_EXPORTER_OTLP_HEADERS` para esa señal.

En máquinas con configuración administrada, consulta [Cómo la configuración administrada bloquea el destino OTLP](#how-managed-settings-lock-the-otlp-destination) para ver qué elimina Claude Code.

| Variable de Entorno                                 | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Valores de Ejemplo                                                                                                                                                         |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | Habilita la recopilación de telemetría (requerido)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `1`                                                                                                                                                                        |
| `OTEL_METRICS_EXPORTER`                             | Tipos de exportador de métricas, separados por comas. Usa `none` para deshabilitar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `console`, `otlp`, `prometheus`, `none`                                                                                                                                    |
| `OTEL_LOGS_EXPORTER`                                | Tipos de exportador de registros/eventos, separados por comas. Usa `none` para deshabilitar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `console`, `otlp`, `none`                                                                                                                                                  |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | Protocolo para exportador OTLP, se aplica a todas las señales. Claude Code no tiene protocolo predeterminado, así que establece esto o la variable de protocolo específica de la señal para cada exportador `otlp` que habilites                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `grpc`, `http/json`, `http/protobuf`                                                                                                                                       |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | Punto final del recopilador OTLP para todas las señales                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `http://localhost:4317`                                                                                                                                                    |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | Protocolo para métricas, anula la configuración general                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `grpc`, `http/json`, `http/protobuf`                                                                                                                                       |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | Punto final de métricas OTLP, anula la configuración general                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `http://localhost:4318/v1/metrics`                                                                                                                                         |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | Protocolo para registros, anula la configuración general                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `grpc`, `http/json`, `http/protobuf`                                                                                                                                       |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | Punto final de registros OTLP, anula la configuración general                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `http://localhost:4318/v1/logs`                                                                                                                                            |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | Encabezados de autenticación para OTLP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `Authorization=Bearer token`                                                                                                                                               |
| `OTEL_EXPORTER_OTLP_METRICS_HEADERS`                | Encabezados de autenticación para métricas, fusionados con los encabezados generales                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `Authorization=Bearer token`                                                                                                                                               |
| `OTEL_EXPORTER_OTLP_LOGS_HEADERS`                   | Encabezados de autenticación para registros, fusionados con los encabezados generales                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `Authorization=Bearer token`                                                                                                                                               |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | Intervalo de exportación en milisegundos (predeterminado: 60000)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `5000`, `60000`                                                                                                                                                            |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | Intervalo de exportación de registros en milisegundos (predeterminado: 5000)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `1000`, `10000`                                                                                                                                                            |
| `OTEL_LOG_USER_PROMPTS`                             | Habilitar registro del contenido del mensaje del usuario (predeterminado: deshabilitado)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `1` para habilitar                                                                                                                                                         |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | Habilitar registro del texto de respuesta del asistente en eventos `assistant_response` (predeterminado: deshabilitado). Cuando no está establecido, recurre al valor de `OTEL_LOG_USER_PROMPTS`. Requiere Claude Code v2.1.193 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `1` para habilitar, `0` para mantener redactado                                                                                                                            |
| `OTEL_LOG_TOOL_DETAILS`                             | Habilitar registro de parámetros de herramientas e argumentos de entrada en eventos de herramientas y atributos de span de traza: comandos Bash, nombres de servidor MCP y herramienta, nombres de habilidades, nombres de flujo de trabajo creados por el usuario, e entrada de herramienta. También habilita nombres de comandos personalizados, de plugin y MCP en eventos `user_prompt` (predeterminado: deshabilitado). Para los servidores integrados de Claude Desktop, en sesiones que Claude Desktop posee, `mcp_server_name`/`mcp_tool_name` se emiten en `tool_decision`/`tool_result` incluso con la bandera desactivada. La excepción requiere Claude Code v2.1.214 o posterior                                                                                                                                                | `1` para habilitar                                                                                                                                                         |
| `OTEL_LOG_TOOL_CONTENT`                             | Habilitar registro de contenido de herramientas en el evento de span [`tool.output`](#tool-output-span-event) (predeterminado: deshabilitado). Los atributos de span llevan contenido de herramientas bajo [sus propias puertas](#new-context-gates). Requiere [trazas](#traces-beta). El contenido se trunca en el límite de contenido (60 KB por defecto)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `1` para habilitar                                                                                                                                                         |
| `OTEL_LOG_MANAGED_SETTINGS`                         | Agregar la configuración administrada redactada, y un resumen SHA-256 de la configuración antes de la redacción, a eventos [managed settings resolved](#managed-settings-resolved-event) (predeterminado: deshabilitado). Un valor en configuración de proyecto o local no lo activa. Requiere Claude Code v2.1.274 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `1` para habilitar                                                                                                                                                         |
| `OTEL_LOG_RAW_API_BODIES`                           | Emitir el cuerpo completo de solicitud y respuesta JSON de la API de Mensajes de Anthropic como eventos de registro `api_request_body` / `api_response_body` (predeterminado: deshabilitado). Los cuerpos incluyen el historial de conversación completo. Habilitar esto implica consentimiento a todo lo que `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS`, y `OTEL_LOG_TOOL_CONTENT` revelarían                                                                                                                                                                                                                                                                                                                                                                                                                                        | `1` para cuerpos en línea truncados en el límite de contenido (60 KB por defecto), o `file:<dir>` para cuerpos sin truncar en disco con un puntero `body_ref` en el evento |
| `CLAUDE_CODE_OTEL_CONTENT_MAX_LENGTH`               | Límite de contenido: la longitud máxima de atributos que contienen contenido como respuestas de modelo, contenido de herramientas, mensajes del sistema, y cuerpos de API sin procesar, incluido el marcador de truncamiento, en unidades de código UTF-16 (predeterminado: 61440, es decir, 60 KB). El predeterminado está dimensionado para backends que limitan valores de atributos a 64 KB; auméntalo solo si tu backend acepta valores más grandes, o redúcelo para cortar el volumen de telemetría. Cuando un límite de atributo del SDK de OpenTelemetry, `OTEL_ATTRIBUTE_VALUE_LENGTH_LIMIT` o uno de sus variantes de logrecord y span, se establece más bajo, Claude Code trunca en ese valor más pequeño para que el marcador `[TRUNCATED ...]` permanezca dentro del límite del SDK. Requiere Claude Code v2.1.214 o posterior | `262144`                                                                                                                                                                   |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | Preferencia de temporalidad de métricas (predeterminado: `delta`). Establece en `cumulative` si tu backend espera temporalidad acumulativa                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `delta`, `cumulative`                                                                                                                                                      |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | Intervalo para actualizar encabezados dinámicos (predeterminado: 1740000ms / 29 minutos)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `900000`                                                                                                                                                                   |

Para los protocolos `http/protobuf` e `http/json`, Claude Code envía cada solicitud de exportación con un encabezado `Content-Length`. Antes de v2.1.212, las versiones de Claude Code desde v2.1.191 en adelante enviaban estas solicitudes con codificación de transferencia fragmentada; Azure Monitor y otros puntos finales que requieren una longitud declarada las rechazaban con errores `411 Length Required` o `400`.

<h3 id="mtls-authentication">
  Autenticación mTLS
</h3>

Cómo configures certificados de cliente para el exportador OTLP depende del protocolo OTLP en uso para esa señal, establecido a través de `OTEL_EXPORTER_OTLP_PROTOCOL` o la anulación por señal. La misma configuración se aplica a métricas, registros y trazas.

| Protocolo                    | Variables de certificado de cliente                                                                                                                                                            | Confiar en la CA del recopilador con |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY`, y opcionalmente `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. Consulta [Configuración de red](/docs/es/network-config#mtls-authentication)              | `NODE_EXTRA_CA_CERTS`                |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` y `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, o las variantes por señal como `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY` para usar un certificado diferente por señal | `OTEL_EXPORTER_OTLP_CERTIFICATE`     |

Para `grpc`, el SDK de OpenTelemetry lee las variables OTLP estándar directamente, por lo que las configuraciones existentes que establecen las variables de métricas por señal continúan funcionando. En máquinas con configuración administrada, Claude Code [puede eliminar credenciales y puntos finales por señal establecidos por desarrolladores](#how-managed-settings-lock-the-otlp-destination) al inicio.

<h3 id="metrics-cardinality-control">
  Control de cardinalidad de métricas
</h3>

Las siguientes variables de entorno controlan qué atributos se incluyen en las métricas para gestionar la cardinalidad:

| Variable de Entorno                        | Descripción                                                                                                                                      | Valor Predeterminado | Ejemplo para Deshabilitar |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------- | ------------------------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | Incluir atributo session.id en métricas                                                                                                          | `true`               | `false`                   |
| `OTEL_METRICS_INCLUDE_VERSION`             | Incluir atributo app.version en métricas                                                                                                         | `false`              | `true`                    |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | Incluir atributos user.account\_uuid y user.account\_id en métricas                                                                              | `true`               | `false`                   |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | Incluir atributo app.entrypoint en métricas                                                                                                      | `false`              | `true`                    |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | Incluir claves de `OTEL_RESOURCE_ATTRIBUTES` como atributos en puntos de datos de métricas                                                       | `true`               | `false`                   |
| `OTEL_METRICS_INCLUDE_REPOSITORY`          | Incluir [atributos de identidad de repositorio](#repository-attributes) `vcs.*` en métricas y eventos. Requiere Claude Code v2.1.269 o posterior | `false`              | `true`                    |

Una cardinalidad más baja generalmente significa mejor rendimiento y costos de almacenamiento más bajos, pero datos menos granulares para el análisis.

<h3 id="traces-beta">
  Trazas (beta)
</h3>

Las trazas distribuidas exportan spans que vinculan cada mensaje del usuario a las solicitudes de API y ejecuciones de herramientas que desencadena, para que puedas ver una solicitud completa como una única traza en tu backend de trazas.

Las trazas están deshabilitadas por defecto. Para habilitarlas, establece tanto `CLAUDE_CODE_ENABLE_TELEMETRY=1` como `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`, luego establece `OTEL_TRACES_EXPORTER` para elegir dónde se envían los spans. Las trazas reutilizan la [configuración OTLP común](#common-configuration-variables) para punto final, protocolo, encabezados, y [mTLS](#mtls-authentication). En máquinas con configuración administrada, Claude Code [puede eliminar credenciales y puntos finales por señal establecidos por desarrolladores](#how-managed-settings-lock-the-otlp-destination) al inicio.

| Variable de Entorno                   | Descripción                                                                              | Valores de Ejemplo                   |
| ------------------------------------- | ---------------------------------------------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | Habilitar trazas de span (requerido). `ENABLE_ENHANCED_TELEMETRY_BETA` también se acepta | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | Tipos de exportador de trazas, separados por comas. Usa `none` para deshabilitar         | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | Protocolo para trazas, anula `OTEL_EXPORTER_OTLP_PROTOCOL`                               | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | Punto final de trazas OTLP, anula `OTEL_EXPORTER_OTLP_ENDPOINT`                          | `http://localhost:4318/v1/traces`    |
| `OTEL_EXPORTER_OTLP_TRACES_HEADERS`   | Encabezados de autenticación para trazas, fusionados con `OTEL_EXPORTER_OTLP_HEADERS`    | `Authorization=Bearer token`         |
| `OTEL_TRACES_EXPORT_INTERVAL`         | Intervalo de exportación de lote de span en milisegundos (predeterminado: 5000)          | `1000`, `10000`                      |

Los spans redactan el texto del mensaje del usuario, los detalles de entrada de herramientas y el contenido de herramientas por defecto. Establece `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1`, y `OTEL_LOG_TOOL_CONTENT=1` para incluirlos.

Cuando el trazado está activo, los subprocesos de Bash y PowerShell heredan automáticamente una variable de entorno `TRACEPARENT` que contiene el contexto de traza W3C del span de ejecución de herramienta activo. Esto permite que cualquier subproceso que lea `TRACEPARENT` padre sus propios spans bajo la misma traza, habilitando trazado distribuido de extremo a extremo a través de scripts y comandos que Claude ejecuta.

Cuando el trazado está activo y Claude Code está conectado directamente a la API de Anthropic, cada solicitud de modelo lleva un encabezado W3C `traceparent` establecido en el contexto del span `claude_code.llm_request`, y el encabezado `traceresponse` de la API se registra como un enlace de span. Juntos, estos conectan los spans del lado del cliente de Claude Code a la traza del lado del servidor a través de cualquier intermediario compatible. Las solicitudes HTTP MCP salientes llevan `traceparent` de la misma manera. El encabezado no se envía a proveedores de terceros.

Por defecto, el encabezado `traceparent` en solicitudes de modelo y HTTP MCP se envía solo cuando `ANTHROPIC_BASE_URL` no está establecido o apunta a la API de Anthropic, ya que algunos proxies rechazan encabezados no reconocidos. La variable `TRACEPARENT` del subproceso se controla por el mismo interruptor por consistencia. Si ejecutas Claude Code a través de un proxy `ANTHROPIC_BASE_URL` personalizado y deseas que se propague el contexto de traza, establece `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`.

En sesiones del SDK de Agent y no interactivas iniciadas con `-p`, Claude Code también lee `TRACEPARENT` y `TRACESTATE` de su propio entorno cuando inicia cada span de interacción. Esto permite que un proceso de incrustación pase su contexto de traza W3C activo al subproceso para que los spans de Claude Code aparezcan como hijos de la traza distribuida del llamador. Las sesiones interactivas ignoran `TRACEPARENT` entrante para evitar heredar accidentalmente valores ambientes de entornos de CI o contenedor.

El contexto de traza entrante también se aplica a [eventos](#events). En sesiones del SDK de Agent y `-p` con `TRACEPARENT` establecido, cada registro de evento de registro OTLP lleva valores `trace_id` y `span_id` que lo unen a la traza de tu aplicación, incluso cuando el exportador de trazas no está configurado, para que tu backend de registro pueda correlacionar eventos con el resto de la traza.

Un registro emitido mientras una interacción está activa lleva los IDs del span de interacción, incluso cuando Claude Code lo emite fuera del contexto asincrónico del span, como en una devolución de llamada de solicitud de permiso o para un registro almacenado en búfer durante el inicio y exportado más tarde. Un registro emitido sin span de interacción activo lleva los IDs de `TRACEPARENT` entrante directamente. Antes de v2.1.214, los registros emitidos fuera del contexto asincrónico del span llevaban los IDs de `TRACEPARENT` entrante en lugar de los IDs del span. Antes de v2.1.212, los registros de eventos emitidos fuera de un span activo no llevaban `trace_id` o `span_id`.

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

Cuando un hook `PreToolUse` [difiere una llamada de herramienta](/docs/es/hooks#defer-a-tool-call-for-later), Claude Code guarda el contexto de traza del turno que lo difirió. Cuando reanudas la sesión y la herramienta se ejecuta nuevamente, los spans de la herramienta se unen a la traza de ese turno anterior como hijos del span `claude_code.interaction` del turno.

<h4 id="span-attributes">
  Atributos de spans
</h4>

Cada span lleva los [atributos estándar](#standard-attributes) más un atributo `span.type` que coincide con su nombre. Las tablas a continuación enumeran los atributos adicionales establecidos en cada span. Los spans `llm_request`, `tool.execution`, y `hook` establecen el estado de OpenTelemetry `ERROR` cuando registran una falla; los otros spans siempre terminan con estado `UNSET`.

**`claude_code.interaction`**

| Atributo                  | Descripción                                                                                                                                                                      | Controlado Por          |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | Texto del mensaje. El valor es `<REDACTED>` a menos que la puerta esté establecida                                                                                               | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | Longitud del mensaje en caracteres                                                                                                                                               |                         |
| `interaction.sequence`    | Contador basado en 1 de interacciones, contado por proceso de Claude Code en lugar de por sesión, como se describe para [`event.sequence`](#event-correlation-attributes)        |                         |
| `parent.source`           | Cómo el span obtuvo su padre de traza: `env` cuando se emparentó bajo un `TRACEPARENT` entrante, `none` cuando inició su propia traza. Requiere Claude Code v2.1.268 o posterior |                         |
| `interaction.duration_ms` | Duración de pared del turno                                                                                                                                                      |                         |

**`claude_code.llm_request`**

| Atributo                         | Descripción                                                                                                                                                                                                                                                                                                            | Controlado Por                 |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| `model`                          | Identificador de modelo                                                                                                                                                                                                                                                                                                |                                |
| `gen_ai.system`                  | Siempre `anthropic`. Convención semántica de GenAI de OpenTelemetry                                                                                                                                                                                                                                                    |                                |
| `gen_ai.request.model`           | Mismo valor que `model`. Convención semántica de GenAI de OpenTelemetry                                                                                                                                                                                                                                                |                                |
| `query_source`                   | Subsistema que emitió la solicitud, como `repl_main_thread` o un nombre de subagente                                                                                                                                                                                                                                   | `ENABLE_BETA_TRACING_DETAILED` |
| `query_source_safe`              | Forma acotada de `query_source`, emitida independientemente de si el trazado beta detallado está activo, con valores como `repl_main_thread` o `agent.builtin.general-purpose`. `:` se convierte en `.` y los agentes nombrados por el usuario aparecen como `agent.custom`. Requiere Claude Code v2.1.268 o posterior |                                |
| `agent_id`                       | Identificador del subagente o compañero que emitió la solicitud. Ausente en la sesión principal                                                                                                                                                                                                                        |                                |
| `parent_agent_id`                | Identificador del agente que generó este. Ausente para la sesión principal y para agentes generados directamente desde ella                                                                                                                                                                                            |                                |
| `workflow.run_id`                | Identificador de ejecución de la ejecución de la herramienta [Workflow](/docs/es/workflows), prefijado con `wf_`. Ausente para agentes no generados por un workflow                                                                                                                                                         |                                |
| `workflow.name`                  | Nombre del workflow que generó este agente. Los nombres creados por el usuario se reemplazan con `custom` a menos que la puerta esté establecida                                                                                                                                                                       | `OTEL_LOG_TOOL_DETAILS`        |
| `speed`                          | `fast` o `normal`                                                                                                                                                                                                                                                                                                      |                                |
| `effort`                         | [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud: `low`, `medium`, `high`, `xhigh`, o `max`. Ausente cuando Claude Code no envía nivel de esfuerzo, por ejemplo en un modelo que no lo soporta. Requiere Claude Code v2.1.274 o posterior                                             |                                |
| `llm_request.context`            | `interaction`, `tool`, o `standalone` dependiendo del span padre                                                                                                                                                                                                                                                       |                                |
| `duration_ms`                    | Duración de pared incluyendo reintentos                                                                                                                                                                                                                                                                                |                                |
| `ttft_ms`                        | Tiempo al primer token en milisegundos                                                                                                                                                                                                                                                                                 |                                |
| `first_content_ms`               | Tiempo desde el inicio de la solicitud hasta el primer bloque de contenido del intento exitoso, en milisegundos. Ausente en solicitudes que retrocedieron a la ruta sin transmisión. Requiere Claude Code v2.1.268 o posterior                                                                                         |                                |
| `input_tokens`                   | Recuento de tokens de entrada del bloque de uso de API                                                                                                                                                                                                                                                                 |                                |
| `output_tokens`                  | Recuento de tokens de salida                                                                                                                                                                                                                                                                                           |                                |
| `cache_read_tokens`              | Tokens leídos del caché de mensaje                                                                                                                                                                                                                                                                                     |                                |
| `cache_creation_tokens`          | Tokens escritos en el caché de mensaje                                                                                                                                                                                                                                                                                 |                                |
| `request_id`                     | ID de solicitud de API de Anthropic del encabezado de respuesta `request-id`                                                                                                                                                                                                                                           |                                |
| `gen_ai.response.id`             | Mismo valor que `request_id`. Convención semántica de GenAI de OpenTelemetry                                                                                                                                                                                                                                           |                                |
| `client_request_id`              | `x-client-request-id` generado por cliente del intento final                                                                                                                                                                                                                                                           |                                |
| `attempt`                        | Intentos totales realizados para esta solicitud                                                                                                                                                                                                                                                                        |                                |
| `success`                        | `true` o `false`                                                                                                                                                                                                                                                                                                       |                                |
| `status_code`                    | Código de estado HTTP cuando la solicitud falló                                                                                                                                                                                                                                                                        |                                |
| `error`                          | Mensaje de error cuando la solicitud falló                                                                                                                                                                                                                                                                             |                                |
| `error_class`                    | Token de clase de error corto cuando la solicitud falló, como `api_timeout` o `server_overload`. Requiere Claude Code v2.1.268 o posterior                                                                                                                                                                             |                                |
| `response.has_tool_call`         | `true` cuando la respuesta contenía bloques de uso de herramientas                                                                                                                                                                                                                                                     |                                |
| `stop_reason`                    | Respuesta de API `stop_reason`, como `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn`, o `refusal`                                                                                                                                                                                                  |                                |
| `gen_ai.response.finish_reasons` | Mismo valor que `stop_reason`, envuelto en una matriz de cadena. Convención semántica de GenAI de OpenTelemetry                                                                                                                                                                                                        |                                |

Cada intento de reintento también se registra como un evento de span `gen_ai.request.attempt` con atributos `attempt` e `client_request_id`.

**`claude_code.tool`**

| Atributo              | Descripción                                                                                                                                                                                                                                                                                                                                                                          | Controlado Por          |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------- |
| `tool_name`           | Nombre de la herramienta                                                                                                                                                                                                                                                                                                                                                             |                         |
| `tool_name_safe`      | Forma de `tool_name` que no lleva nombres elegidos por el usuario. Los nombres de herramientas integradas pasan verbatim. Los nombres de herramientas MCP aparecen como `mcp_other`, excepto los nombres de herramientas que coinciden con algunas formas fijas, como herramientas `playwright` nombradas `browser_*`, que pasan verbatim. Requiere Claude Code v2.1.268 o posterior |                         |
| `bash_command_class`  | Para la herramienta Bash: categoría del primer programa del comando de una lista fija, como `vcs` o `package_manager`. `other` para un programa fuera de la lista, `unparsed` cuando la línea no se puede analizar. Requiere Claude Code v2.1.268 o posterior                                                                                                                        |                         |
| `bash_argv0`          | Para la herramienta Bash: el primer programa del comando cuando está en la misma lista fija, como `git` o `npm`. `other` para cualquier programa fuera de la lista. Requiere Claude Code v2.1.268 o posterior                                                                                                                                                                        |                         |
| `duration_ms`         | Duración de pared incluyendo espera de permiso y ejecución                                                                                                                                                                                                                                                                                                                           |                         |
| `result_tokens`       | Tamaño aproximado de token del resultado de la herramienta                                                                                                                                                                                                                                                                                                                           |                         |
| `agent_id`            | Identificador del subagente o compañero que ejecutó la herramienta. Ausente en la sesión principal                                                                                                                                                                                                                                                                                   |                         |
| `parent_agent_id`     | Identificador del agente que generó este. Ausente para la sesión principal y para agentes generados directamente desde ella                                                                                                                                                                                                                                                          |                         |
| `workflow.run_id`     | Identificador de ejecución de la ejecución de la herramienta Workflow que generó este agente, prefijado con `wf_`. Ausente para agentes no generados por un workflow                                                                                                                                                                                                                 |                         |
| `workflow.name`       | Nombre del workflow que generó este agente. Los nombres creados por el usuario se reemplazan con `custom` a menos que la puerta esté establecida                                                                                                                                                                                                                                     | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | El ID del bloque `tool_use` del modelo para esta llamada. Coincide con el `tool_use_id` en los eventos [tool\_result](#tool-result-event) y [tool\_decision](#tool-decision-event) y en cargas útiles de hooks, para que puedas unir el span a esos registros                                                                                                                        |                         |
| `gen_ai.tool.call.id` | Mismo valor que `tool_use_id`. Convención semántica de GenAI de OpenTelemetry                                                                                                                                                                                                                                                                                                        |                         |
| `file_path`           | Ruta de archivo de destino para herramientas Read, Edit, y Write                                                                                                                                                                                                                                                                                                                     | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Cadena de comando para la herramienta Bash                                                                                                                                                                                                                                                                                                                                           | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Nombre de habilidad para la herramienta Skill                                                                                                                                                                                                                                                                                                                                        | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Tipo de subagente para la herramienta Agent o herramienta Task heredada                                                                                                                                                                                                                                                                                                              | `OTEL_LOG_TOOL_DETAILS` |

<span id="tool-output-span-event" />**`tool.output` span event on `claude_code.tool`**

Si estableces `OTEL_LOG_TOOL_CONTENT=1`, las llamadas Read y Bash pueden registrar un evento de span `tool.output` en el span `claude_code.tool`. Las llamadas Edit y Write registran uno solo cuando también estableces `OTEL_LOG_TOOL_DETAILS=1`. Esa variable no está limitada a esas dos herramientas, así que verifica su [fila en la tabla de configuración](#common-configuration-variables) para los argumentos que agrega en otros lugares.

Claude Code escribe este evento desde el retorno exitoso de una llamada de herramienta, por lo que una llamada que genera un error no registra nada, sea cual sea la herramienta. Entre las llamadas que sí retornan, no registra un evento `tool.output` para:

* Una llamada a cualquier herramienta que no sea Read, Edit, Write, y Bash, incluyendo herramientas MCP y WebFetch
* Un Read que retorna algo que no sea texto de archivo, como una imagen, un PDF, o una relectura de un archivo cuyo contenido no ha cambiado
* Una llamada Edit o Write, a menos que también establezca `OTEL_LOG_TOOL_DETAILS=1`

El evento lleva estos atributos, cada uno truncado en el límite de contenido (60 KB por defecto). `Controlado Por` nombra la variable que un atributo necesita además de `OTEL_LOG_TOOL_CONTENT=1`, y para Edit y Write esa variable controla el evento en sí en lugar del atributo.

| Atributo       | Descripción                                                                                                           | Controlado Por                                    |
| -------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| `content`      | Texto que la herramienta Read retornó, o el texto que una llamada Write fue solicitada a escribir                     | `OTEL_LOG_TOOL_DETAILS` para la herramienta Write |
| `output`       | Salida combinada de un comando Bash, con stderr intercalado en stdout                                                 |                                                   |
| `diff`         | Parche estructurado que la herramienta Edit aplicó                                                                    | `OTEL_LOG_TOOL_DETAILS`                           |
| `file_path`    | Ruta de archivo de destino para las herramientas Read, Edit, y Write, repitiendo el atributo de span del mismo nombre | `OTEL_LOG_TOOL_DETAILS`                           |
| `bash_command` | Cadena de comando para la herramienta Bash                                                                            | `OTEL_LOG_TOOL_DETAILS`                           |

El atributo `tool_name` del span padre te dice de qué herramienta vino un evento. Un atributo cortado en el límite de contenido va acompañado de `<attribute>_truncated` y `<attribute>_original_length`.

**`claude_code.tool.blocked_on_user`**

| Atributo      | Descripción                                                                                | Controlado Por |
| ------------- | ------------------------------------------------------------------------------------------ | -------------- |
| `duration_ms` | Tiempo dedicado a esperar la decisión de permiso                                           |                |
| `decision`    | `accept` o `reject`                                                                        |                |
| `source`      | Fuente de decisión, coincidiendo con el evento [Tool decision event](#tool-decision-event) |                |

**`claude_code.tool.execution`**

| Atributo              | Descripción                                                                                                                                                                                                                                                                       | Controlado Por          |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | Tiempo dedicado a ejecutar el cuerpo de la herramienta                                                                                                                                                                                                                            |                         |
| `tool_use_id`         | Mismo valor que en el span padre `claude_code.tool`                                                                                                                                                                                                                               |                         |
| `gen_ai.tool.call.id` | Mismo valor que `tool_use_id`. Convención semántica de GenAI de OpenTelemetry                                                                                                                                                                                                     |                         |
| `success`             | `true` o `false`                                                                                                                                                                                                                                                                  |                         |
| `error`               | Cadena de categoría de error cuando la ejecución falló, como `Error:ENOENT` o `ShellError`. Contiene el mensaje de error completo en su lugar cuando la puerta está establecida                                                                                                   | `OTEL_LOG_TOOL_DETAILS` |
| `error_class`         | La categoría de error en forma de identificador, con caracteres fuera de letras, dígitos y guiones bajos reemplazados por `_`, como `Error_ENOENT` o `ShellError`. Lleva la categoría incluso cuando `error` lleva el mensaje completo. Requiere Claude Code v2.1.268 o posterior |                         |

**`claude_code.hook`**

Este span aparece solo cuando el trazado beta detallado está activo, lo que requiere `ENABLE_BETA_TRACING_DETAILED=1` y `BETA_TRACING_ENDPOINT`, un par que también [cambia dónde van tus registros y trazas](/docs/es/env-vars#variables). Establece el par en tu shell, configuración de usuario, o configuración administrada; ambas variables se ignoran en [configuración de proyecto y local](/docs/es/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` solo no lo produce.

En sesiones de CLI interactivas, el trazado beta detallado también requiere que tu organización esté en la lista de permitidos para la característica. Las sesiones del SDK de Agent y no interactivas `-p` no requieren lista de permitidos.

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

<span id="new-context-gates" />

<Note>
  Atributos adicionales que contienen contenido como `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input`, y `response.model_output` se emiten solo cuando el trazado beta detallado está activo. No son parte del esquema de span estable.

  La puerta en `new_context` depende de qué span lo lleva, y cada copia se trunca en el límite de contenido (60 KB por defecto). En el span `claude_code.tool` lleva el resultado de esa llamada de herramienta, sea cual sea la herramienta, y requiere `OTEL_LOG_TOOL_CONTENT=1`. En el span `claude_code.interaction` lleva el mensaje del usuario, y en el span `claude_code.llm_request` los nuevos mensajes del usuario y resultados de herramientas de esa solicitud. Ambos requieren `OTEL_LOG_USER_PROMPTS=1`.

  `user_system_prompt` además requiere `OTEL_LOG_USER_PROMPTS=1`. Lleva solo el texto del mensaje del sistema que proporcionas a través de la opción `systemPrompt` del SDK o las banderas `--system-prompt` y `--append-system-prompt`, truncado en el límite de contenido (60 KB por defecto), y se emite una vez por sesión en lugar de por solicitud.
</Note>

<h3 id="dynamic-headers">
  Encabezados dinámicos
</h3>

Para entornos empresariales que requieren autenticación dinámica, puedes configurar un script para generar encabezados dinámicamente. Los encabezados dinámicos se aplican solo a los protocolos `http/protobuf` e `http/json`. Con el protocolo `grpc`, Claude Code usa solo las variables de encabezados estáticos, `OTEL_EXPORTER_OTLP_HEADERS` y sus variantes por señal.

<h4 id="settings-configuration">
  Configuración de ajustes
</h4>

Agrega a tu `.claude/settings.json`, reemplazando la ruta con la tuya propia:

```json theme={null}
{
  "otelHeadersHelper": "/path/to/generate-otel-headers.sh"
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

Si el auxiliar falla o imprime salida que no cumple con estos requisitos, las exportaciones fallan y tu backend de telemetría no recibe nada de la sesión hasta que el auxiliar funcione nuevamente. Claude Code reporta el error en:

* Una notificación de advertencia en sesiones interactivas, [`otelHeadersHelper failed; telemetry is not being exported`](/docs/es/errors#otelheadershelper-failed), mostrada una vez por sesión cuando el auxiliar falla por primera vez
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

Claude Code adjunta estos valores como atributos en cada punto de datos de métrica y registro de evento, además de enviarlos en el bloque de recursos OTLP. Debido a que la mayoría de los backends de métricas exponen atributos de punto de datos como etiquetas consultables, puedes agrupar y filtrar métricas por tus claves personalizadas directamente. Excepto por los [atributos de repositorio](#repository-attributes) `vcs.*`, las claves personalizadas nunca anulan los [atributos estándar](#standard-attributes) como `user.id` o `session.id`: cuando una clave colisiona, Claude Code mantiene el valor integrado.

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

Establece estas variables de entorno antes de ejecutar `claude`. Cada escenario a continuación muestra una configuración completa, y cada variable se describe bajo [Variables de configuración comunes](#common-configuration-variables). Para confirmar que una configuración tuvo efecto, verifica tu backend para la métrica `claude_code.session.count` después de iniciar una sesión; la [Guía de inicio rápido](#quick-start) cubre verificación solo de registros y qué verificar cuando nada llega.

Para depuración de consola con intervalo de exportación de 1 segundo:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000
```

Para OTLP sobre gRPC:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Para Prometheus, raspado desde `http://localhost:9464/metrics`:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus
```

En un [entorno autohospedado](/docs/es/self-hosted-environments-reference#pass-through-session-child-metrics), la sesión vincula el puerto 9464 solo en la capacidad predeterminada del ejecutor de uno. A mayor capacidad, el ejecutor reexpone contadores y medidores de sesión en su propio punto final `/metrics` en su lugar.

Para enviar métricas a múltiples exportadores:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json
```

Para enviar métricas y registros a diferentes puntos finales o backends:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317
```

Para exportar solo métricas, sin eventos o registros:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Para exportar solo eventos y registros, sin métricas:

```bash theme={null}
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

| Atributo                                                                                | Descripción                                                                                                                                                                                                                                                     | Controlado por                                                                                       |
| --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `session.id`                                                                            | Identificador único de sesión                                                                                                                                                                                                                                   | `OTEL_METRICS_INCLUDE_SESSION_ID` (predeterminado: true)                                             |
| `app.version`                                                                           | Versión actual de Claude Code                                                                                                                                                                                                                                   | `OTEL_METRICS_INCLUDE_VERSION` (predeterminado: false)                                               |
| `app.entrypoint`                                                                        | Cómo se inició la sesión, como `cli`, `sdk-cli`, `sdk-ts`, `sdk-py`, o `claude-vscode`                                                                                                                                                                          | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (predeterminado: false)                                            |
| `organization.id`                                                                       | UUID de la organización (cuando está autenticado)                                                                                                                                                                                                               | Siempre se incluye cuando está disponible                                                            |
| `user.account_uuid`                                                                     | UUID de la cuenta (cuando está autenticado)                                                                                                                                                                                                                     | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (predeterminado: true)                                           |
| `user.account_id`                                                                       | ID de cuenta en formato etiquetado que coincide con las API de administración de Anthropic (cuando está autenticado), como `user_01BWBeN28...`                                                                                                                  | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (predeterminado: true)                                           |
| `user.id`                                                                               | Identificador anónimo aleatorio generado en la primera ejecución y persistido en `~/.claude.json`. No contiene información personal y no se deriva de su cuenta de Claude. Eliminar el archivo produce un nuevo valor no relacionado en la siguiente ejecución. | Siempre se incluye                                                                                   |
| `user.email`                                                                            | Dirección de correo electrónico del usuario, de su inicio de sesión o, en una [sesión en la nube](/docs/es/claude-code-on-the-web), de las credenciales de la propia sesión                                                                                          | Siempre se incluye cuando está disponible                                                            |
| `terminal.type`                                                                         | Tipo de terminal, como `iTerm.app`, `vscode`, `cursor`, o `tmux`                                                                                                                                                                                                | Siempre se incluye cuando se detecta                                                                 |
| Claves de `OTEL_RESOURCE_ATTRIBUTES`                                                    | Atributos personalizados que establece, como `department` o `team.id`. Consulte [Soporte de organización multiequipo](#multi-team-organization-support)                                                                                                         | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (predeterminado: true)                                    |
| `vcs.repository.url.full`, `vcs.owner.name`, `vcs.repository.name`, `vcs.provider.name` | La identidad del repositorio de la sesión, derivada de su remoto `origin`. Consulte [Atributos del repositorio](#repository-attributes)                                                                                                                         | `OTEL_METRICS_INCLUDE_REPOSITORY` (predeterminado: false). Requiere Claude Code v2.1.269 o posterior |

Cuando Claude Code está autenticado en una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway), la CLI marca las exportaciones con la identidad autenticada de la sesión de la puerta de enlace: `user.id` es el asunto del IdP en lugar de un identificador de instalación anónimo, `user.email` es el correo electrónico con sesión iniciada, y `user.groups` lleva la pertenencia al grupo del IdP como una cadena separada por comas. Cada exportación también lleva `identity.source: gateway-oidc`. La identidad de la puerta de enlace se aplica en último lugar, por lo que las claves `user.*` e `identity.*` establecidas a través de `OTEL_RESOURCE_ATTRIBUTES` se ignoran en sesiones de puerta de enlace.

Los eventos incluyen además los siguientes atributos. Estos nunca se adjuntan a las métricas porque causarían cardinalidad ilimitada:

* `prompt.id`: UUID que correlaciona un mensaje del usuario con todos los eventos posteriores hasta el siguiente mensaje. Consulte [Atributos de correlación de eventos](#event-correlation-attributes).
* `workspace.host_paths`: directorios del espacio de trabajo del host seleccionados en la aplicación de escritorio, como una matriz de cadenas
* `workflow.run_id`: identificador de ejecución, con prefijo `wf_`, en los eventos de API y herramienta emitidos por agentes que pertenecen a una ejecución de herramienta [Workflow](/docs/es/workflows). Filtrar eventos por un `workflow.run_id` reconstruye las solicitudes de API y resultados de herramientas de esa ejecución. El identificador cubre los agentes que el script del flujo de trabajo genera y cualquier agente que esos generen a su vez, como invocaciones de habilidades. Coincide con el identificador de ejecución reportado en el resultado de la herramienta Workflow. Ausente en todos los demás eventos. Requiere Claude Code v2.1.202 o posterior
* `workflow.name`: nombre del flujo de trabajo, el `meta.name` de su script, emitido junto con `workflow.run_id`. Los nombres de flujo de trabajo integrados aparecen literalmente cuando la ejecución ejecuta el script integrado sin modificar. Los nombres creados por el usuario, incluidas las copias editadas de scripts integrados, se reemplazan con `custom` a menos que se establezca `OTEL_LOG_TOOL_DETAILS=1`. Requiere Claude Code v2.1.202 o posterior

<h4 id="repository-attributes">
  Atributos del repositorio
</h4>

Establezca `OTEL_METRICS_INCLUDE_REPOSITORY=true` para etiquetar métricas y eventos con la identidad del repositorio de la sesión, de modo que un recopilador compartido pueda atribuir el uso por repositorio. Requiere Claude Code v2.1.269 o posterior.

Claude Code deriva estos atributos una vez por sesión desde el remoto `origin` del repositorio. Los remotos HTTPS y SSH de un repositorio producen valores idénticos:

| Atributo                  | Valor                                                                                                                                                         |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `vcs.repository.url.full` | La URL del navegador del repositorio sin `.git`, como `https://github.com/example-org/example-repo`                                                           |
| `vcs.owner.name`          | La ruta del propietario o grupo, como `example-org`; omitido cuando la ruta remota tiene un solo segmento                                                     |
| `vcs.repository.name`     | El nombre del repositorio desnudo, como `example-repo`                                                                                                        |
| `vcs.provider.name`       | `github`, `gitlab`, `bitbucket`, o `gitea` cuando Claude Code reconoce el host remoto o la forma de URL como uno de esos proveedores; omitido de lo contrario |

Los valores se convierten a minúsculas, y las credenciales, cadenas de consulta y fragmentos de la URL remota nunca aparecen en ellos. Los atributos se omiten cuando la sesión no tiene un remoto `origin`, cuando el remoto no tiene forma de URL, o cuando el único repositorio envolvente es su directorio de inicio.

Una clave `vcs.*` que declare en [`OTEL_RESOURCE_ATTRIBUTES`](#multi-team-organization-support) reemplaza el valor derivado para esa clave. Si declara `vcs.repository.url.full`, Claude Code nunca lee el remoto e informa solo las claves que declara.

Los atributos fluyen solo a sus propios exportadores; la telemetría de Anthropic descarta cada clave `vcs.*`.

<h3 id="metrics">
  Métricas
</h3>

Claude Code exporta las siguientes métricas. La columna Unit muestra la cadena de unidad de OpenTelemetry adjunta a cada métrica; las métricas de conteo no llevan ninguna.

| Nombre de métrica                     | Descripción                                                         | Unidad  |
| ------------------------------------- | ------------------------------------------------------------------- | ------- |
| `claude_code.session.count`           | Conteo de sesiones de CLI iniciadas                                 | ninguna |
| `claude_code.lines_of_code.count`     | Conteo de líneas de código modificadas                              | ninguna |
| `claude_code.pull_request.count`      | Número de solicitudes de extracción creadas                         | ninguna |
| `claude_code.commit.count`            | Número de confirmaciones de git creadas                             | ninguna |
| `claude_code.cost.usage`              | Costo de la sesión de Claude Code                                   | USD     |
| `claude_code.token.usage`             | Número de tokens utilizados                                         | tokens  |
| `claude_code.code_edit_tool.decision` | Conteo de decisiones de permiso de herramienta de edición de código | ninguna |
| `claude_code.active_time.total`       | Tiempo activo total                                                 | s       |

Cuando `prometheus` es el único exportador listado en `OTEL_METRICS_EXPORTER`, Claude Code omite las unidades `USD`, `tokens`, y `s` de las métricas exportadas para que el raspado permanezca en formato de texto Prometheus válido. Los nombres de las métricas no cambian, y las configuraciones que combinan exportadores, como `otlp,prometheus`, mantienen las unidades. Antes de v2.1.216, el raspado de Prometheus incluía líneas `# UNIT` solo de OpenMetrics que algunos raspadores rechazaban.

<h3 id="metric-details">
  Detalles de métricas
</h3>

Cada métrica incluye los atributos estándar listados arriba. Las métricas con atributos adicionales específicos del contexto se notan a continuación.

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
* `speed`: `"fast"` cuando la solicitud utilizó modo rápido. Ausente de lo contrario
* `effort`: [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud: `"low"`, `"medium"`, `"high"`, `"xhigh"`, o `"max"`. Ausente cuando Claude Code no envía un nivel de esfuerzo, por ejemplo en un modelo que no admite esfuerzo.
* `agent.name`: Tipo de subagente que emitió la solicitud. Los nombres de agentes integrados y los agentes de plugins del mercado oficial aparecen literalmente. Otros nombres de agentes definidos por el usuario se reemplazan con `"custom"` a menos que se establezca `OTEL_LOG_TOOL_DETAILS=1`. Ausente cuando la solicitud no fue emitida por un tipo de subagente nombrado.
* `skill.name`: Habilidad activa para la solicitud, establecida por la herramienta Skill, un comando `/`, o heredada por un subagente generado. Los nombres de habilidades integradas, agrupadas, definidas por el usuario y del plugin del mercado oficial aparecen literalmente. Los nombres de habilidades de plugins de terceros se reemplazan con `"third-party"` a menos que se establezca `OTEL_LOG_TOOL_DETAILS=1`. Ausente cuando no hay habilidad activa.
* `plugin.name`: Plugin propietario cuando la habilidad activa o subagente es proporcionado por un plugin. Los nombres de plugins del mercado oficial aparecen literalmente. Los nombres de plugins de terceros se reemplazan con `"third-party"` a menos que se establezca `OTEL_LOG_TOOL_DETAILS=1`. Ausente cuando ni la habilidad ni el subagente tienen un plugin propietario.
* `marketplace.name`: Mercado desde el cual se instaló el plugin propietario. Solo se emite para plugins del mercado oficial. Ausente de lo contrario.
* `mcp_server.name`: Servidor MCP cuyo resultado de herramienta consumió esta solicitud. Los nombres de servidores integrados, proxificados por claude.ai, y del registro oficial aparecen literalmente. Los nombres de servidores configurados por el usuario se reemplazan con `"custom"` a menos que se establezca `OTEL_LOG_TOOL_DETAILS=1`. Ausente cuando la solicitud no consumió ningún resultado de herramienta MCP. Antes de v2.1.222, Claude Code establecía este atributo en cada solicitud después de una llamada de herramienta MCP, no solo en solicitudes que consumieron un resultado de herramienta, por lo que los paneles de control que lo agregan muestran una caída después de actualizar.
* `mcp_tool.name`: Herramienta MCP cuyo resultado consumió esta solicitud, con el mismo comportamiento de redacción y versión que `mcp_server.name`. Ausente cuando la solicitud no consumió ningún resultado de herramienta MCP.

<h4 id="token-counter">
  Contador de tokens
</h4>

Se incrementa después de cada solicitud de API.

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `type`: (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model`: Identificador de modelo (por ejemplo, "claude-sonnet-5")
* `query_source`: Categoría del subsistema que emitió la solicitud. Uno de `"main"`, `"subagent"`, o `"auxiliary"`
* `speed`: `"fast"` cuando la solicitud utilizó modo rápido. Ausente de lo contrario
* `effort`: [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud. Consulte [Contador de costo](#cost-counter) para obtener detalles.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribución de habilidad, plugin, agente y MCP para la solicitud. Consulte [Contador de costo](#cost-counter) para obtener definiciones y comportamiento de redacción.

<h4 id="code-edit-tool-decision-counter">
  Contador de decisión de herramienta de edición de código
</h4>

Se incrementa cuando el usuario acepta o rechaza el uso de la herramienta Edit, Write, o NotebookEdit.

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `tool_name`: Nombre de herramienta (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: Decisión del usuario (`"accept"`, `"reject"`)
* `source`: De dónde vino la decisión. Uno de `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"`, o `"user_reject"`. Consulte el [evento de decisión de herramienta](#tool-decision-event) para ver qué significa cada valor.
* `language`: Lenguaje de programación del archivo editado, como `"TypeScript"`, `"Python"`, `"JavaScript"`, o `"Markdown"`. Devuelve `"unknown"` para extensiones de archivo no reconocidas.

<h4 id="active-time-counter">
  Contador de tiempo activo
</h4>

Rastrea el tiempo real dedicado a usar activamente Claude Code, excluyendo el tiempo inactivo. Esta métrica se incrementa durante las interacciones del usuario, como escribir y leer respuestas, y durante el procesamiento de CLI, como la ejecución de herramientas y la generación de respuestas de IA.

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

| Atributo            | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt.id`         | Identificador UUID v4 que vincula todos los eventos producidos mientras se procesa un único mensaje del usuario                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `event.sequence`    | Contador basado en 0 para ordenar eventos, contado por proceso de Claude Code en lugar de por sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `message.uuid`      | UUID del mensaje tal como se persiste en la transcripción de la sesión, los archivos `~/.claude/projects/*/*.jsonl`. Presente en `assistant_response`, en `api_response_body`, y en `user_prompt` excepto para envíos de comandos, que pueden producir cero o muchos mensajes. En `assistant_response` y `api_response_body`, esta es la entrada de transcripción final de la respuesta, desde la cual el `parentUuid` del siguiente turno se encadena. Requiere Claude Code v2.1.214 o posterior, o v2.1.274 o posterior en `api_response_body`                              |
| `client_request_id` | UUID generado por el cliente enviado como encabezado de solicitud `x-client-request-id`. Presente en `api_request` y `api_error` en conexiones de API de primera parte; ausente en backends de proveedores de terceros y cuando la solicitud se reintentó a través de la alternativa sin transmisión. Empareja una solicitud con su respuesta y permanece disponible para fallos como tiempos de espera que nunca produjeron un `request_id` del servidor. Coincide con el mismo atributo en el intervalo de rastreo `llm_request`. Requiere Claude Code v2.1.214 o posterior |

Para rastrear toda la actividad desencadenada por un único mensaje, filtre sus eventos por un valor específico de `prompt.id`. Esto devuelve el evento user\_prompt, cualquier evento api\_request, y cualquier evento tool\_result que ocurrió mientras se procesaba ese mensaje.

`event.sequence` comienza en 0 cada vez que se inicia un proceso de Claude Code y cuenta hacia arriba durante la vida de ese proceso. Continúa contando a través de `/clear`, que asigna un nuevo `session.id`. Si [reanuda una sesión sin bifurcar](/docs/es/how-claude-code-works#resume-or-fork-sessions), la sesión mantiene su `session.id` pero toma sus valores de `event.sequence` del proceso que la reanudó, por lo que dentro de una sesión un evento posterior puede llevar un valor más bajo que uno anterior, o repetir uno. Para ordenar los eventos de una sesión, ordene por `event.timestamp` y use `event.sequence` para ordenar eventos que comparten una marca de tiempo.

Para la reconstrucción a nivel de mensaje, cada clase de evento lleva una clave que coincide con un campo en la transcripción de la sesión. El formato de entrada de transcripción es [interno a Claude Code](/docs/es/sessions#where-transcripts-are-stored) y cambia entre versiones, por lo que un pipeline que se une en estos campos puede romperse en cualquier versión; trate las uniones como específicas de la versión en lugar de un contrato estable:

* `message.uuid` en `user_prompt`, `assistant_response`, y `api_response_body`
* `request_id` en los eventos de API, persistido como `requestId` en las entradas del asistente de la transcripción
* `tool_use_id` en eventos `tool_result` y `tool_decision`

<h4 id="user-prompt-event">
  Evento de mensaje del usuario
</h4>

Se registra cuando un usuario envía un mensaje.

**Nombre del evento**: `claude_code.user_prompt`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `prompt_length`: Longitud del mensaje
* `prompt`: Contenido del mensaje. Redactado por defecto. Establezca `OTEL_LOG_USER_PROMPTS=1` para incluirlo
* `message.uuid`: UUID del mensaje del usuario resultante, coincidiendo con la entrada de transcripción persistida. Ausente en envíos de comandos, que pueden producir cero o muchos mensajes. Requiere Claude Code v2.1.214 o posterior
* `command_name`: Nombre del comando cuando el mensaje invoca uno. Los nombres de comandos integrados y agrupados como `compact` o `debug` se emiten tal cual; los alias como `reset` se emiten tal como se escribieron en lugar del nombre canónico. Los nombres de comandos personalizados, de plugin y MCP se contraen a `custom` o `mcp` a menos que se establezca `OTEL_LOG_TOOL_DETAILS=1`
* `command_source`: Origen del comando cuando está presente: `builtin`, `custom`, o `mcp`. Los comandos proporcionados por plugins reportan como `custom`

<h4 id="assistant-response-event">
  Evento de respuesta del asistente
</h4>

Se registra después de cada solicitud de API que devuelve contenido de texto del modelo. Solo se incluyen los bloques de texto de la respuesta; los bloques de pensamiento y los bloques de uso de herramientas se excluyen. Requiere Claude Code v2.1.193 o posterior.

**Nombre del evento**: `claude_code.assistant_response`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"assistant_response"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `response_length`: Longitud del texto de respuesta en caracteres
* `response`: Texto de respuesta, truncado en el límite de contenido (60 KB por defecto). Redactado a `<REDACTED>` por defecto. Establezca `OTEL_LOG_ASSISTANT_RESPONSES=1` para incluirlo. Cuando `OTEL_LOG_ASSISTANT_RESPONSES` no está establecido, `OTEL_LOG_USER_PROMPTS` lo controla en su lugar, por lo que establezca `OTEL_LOG_ASSISTANT_RESPONSES=0` para mantener las respuestas redactadas mientras el registro de mensajes está activado
* `model`: Identificador de modelo (por ejemplo, "claude-sonnet-5")
* `request_id`: ID de solicitud de API de Anthropic del encabezado `request-id` de la respuesta. Presente solo cuando la API devuelve uno
* `message.uuid`: UUID de la entrada de transcripción final de la respuesta. Una respuesta de API se persiste como una entrada de transcripción por bloque de contenido; esta es la última, desde la cual el `parentUuid` del siguiente turno se encadena. Requiere Claude Code v2.1.214 o posterior
* `query_source`: Subsistema que emitió la solicitud, como `"repl_main_thread"`, `"compact"`, o un nombre de subagente

<h4 id="tool-result-event">
  Evento de resultado de herramienta
</h4>

Se registra cuando una herramienta completa la ejecución. No se emite si la llamada de herramienta fue rechazada; consulte el [evento de decisión de herramienta](#tool-decision-event) para rechazos.

**Nombre del evento**: `claude_code.tool_result`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `tool_name`: Nombre de la herramienta
* `tool_use_id`: Identificador único para esta invocación de herramienta. Coincide con el `tool_use_id` pasado a hooks, permitiendo correlación entre eventos OTel y datos capturados por hooks.
* `success`: `"true"` o `"false"`
* `duration_ms`: Tiempo de ejecución en milisegundos
* `error_type`: Cadena de categoría de error cuando la herramienta falló, como `"Error:ENOENT"` o `"ShellError"`
* `error` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Mensaje de error completo cuando la herramienta falló
* `decision_type`: Siempre `"accept"`, ya que este evento solo se emite después de que la herramienta se ejecuta. Las llamadas rechazadas no producen un resultado de herramienta
* `decision_source`: De dónde vino la decisión de permiso. Uno de `"config"`, `"hook"`, `"user_permanent"`, o `"user_temporary"`. Consulte el [evento de decisión de herramienta](#tool-decision-event) para ver qué significa cada valor. Las fuentes solo de rechazo `"user_abort"` y `"user_reject"` nunca aparecen en este evento.
* `tool_input_size_bytes`: Tamaño de la entrada de herramienta serializada en JSON en bytes
* `tool_result_size_bytes`: Tamaño del resultado de la herramienta en bytes
* `mcp_server_scope`: Identificador de alcance del servidor MCP (para herramientas MCP)
* `vcs.ref.head.revision`, `vcs.ref.head.name`, `vcs.ref.head.type` (cuando `OTEL_LOG_TOOL_DETAILS=1`): la identidad de confirmación de una ejecución exitosa de `git commit` ejecutada por la herramienta Bash o PowerShell. `vcs.ref.head.revision` es el SHA de confirmación, `vcs.ref.head.name` es la rama en la que se confirmó, y `vcs.ref.head.type` es `branch`. El nombre y tipo se omiten cuando la confirmación se realizó en un HEAD desconectado. Requiere Claude Code v2.1.269 o posterior
* `tool_parameters` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Cadena JSON que contiene parámetros específicos de la herramienta. Para los servidores integrados de Claude Desktop, en sesiones que Claude Desktop posee, el par `mcp_server_name`/`mcp_tool_name` se incluye incluso con la bandera desactivada, la misma excepción creada por el host que el [evento de decisión de herramienta](#tool-decision-event), requiriendo Claude Code v2.1.214 o posterior. Los parámetros varían según la herramienta:
  * Para herramienta Bash: incluye `bash_command`, `full_command`, `timeout`, `description`, y `dangerouslyDisableSandbox`, más `git_commit_id` y `git_branch` cuando un comando `git commit` tiene éxito. `git_commit_id` es el SHA de confirmación completo cuando la confirmación es el HEAD del directorio de trabajo de la sesión, y el SHA abreviado de git de lo contrario. `git_branch` es la rama en la que se confirmó, omitida en un HEAD desconectado
  * Para la herramienta Bash del espacio de trabajo de la aplicación de escritorio, que también reporta `tool_name` como `Bash`: incluye solo `bash_command`, `full_command`, y `timeout`
  * Para herramientas MCP: incluye `mcp_server_name`, `mcp_tool_name`
  * Para herramienta Skill: incluye `skill_name`
  * Para herramienta Agent o herramienta Task heredada: incluye `subagent_type`
* `tool_input` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Argumentos de herramienta serializados en JSON. Los valores individuales superiores a 512 caracteres se truncan, y la carga útil completa se limita a aproximadamente 4 K caracteres. Se aplica a todas las herramientas, incluidas las herramientas MCP.

<h4 id="api-request-event">
  Evento de solicitud de API
</h4>

Se registra para cada solicitud de API a Claude.

**Nombre del evento**: `claude_code.api_request`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `model`: Modelo utilizado (por ejemplo, "claude-sonnet-5")
* `cost_usd`: Costo estimado en USD
* `cost_usd_micros`: Costo estimado en millonésimas de dólar estadounidense, emitido como un entero
* `duration_ms`: Duración de la solicitud en milisegundos
* `input_tokens`: Número de tokens de entrada
* `output_tokens`: Número de tokens de salida
* `cache_read_tokens`: Número de tokens leídos de la caché
* `cache_creation_tokens`: Número de tokens utilizados para la creación de caché
* `request_id`: ID de solicitud de API de Anthropic del encabezado `request-id` de la respuesta, como `"req_011..."`. Presente solo cuando la API devuelve uno.
* `client_request_id`: UUID generado por el cliente enviado como encabezado de solicitud `x-client-request-id`; consulte la tabla [atributos de correlación de eventos](#event-correlation-attributes) para saber cuándo está presente. Requiere Claude Code v2.1.214 o posterior
* `speed`: `"fast"` o `"normal"`, indicando si el modo rápido estaba activo
* `query_source`: Subsistema que emitió la solicitud, como `"repl_main_thread"`, `"compact"`, o un nombre de subagente
* `effort`: [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud: `"low"`, `"medium"`, `"high"`, `"xhigh"`, o `"max"`. Ausente cuando Claude Code no envía un nivel de esfuerzo, por ejemplo en un modelo que no admite esfuerzo.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribución de habilidad, plugin, agente y MCP para la solicitud. Consulte [Contador de costo](#cost-counter) para obtener definiciones y comportamiento de redacción.

<h4 id="api-error-event">
  Evento de error de API
</h4>

Se registra cuando una solicitud de API a Claude falla.

**Nombre del evento**: `claude_code.api_error`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `model`: Modelo utilizado (por ejemplo, "claude-sonnet-5")
* `error`: Mensaje de error
* `status_code`: Código de estado HTTP como número. Ausente para errores no HTTP como fallos de conexión.
* `duration_ms`: Duración de la solicitud en milisegundos
* `attempt`: Número total de intentos realizados, incluida la solicitud inicial (`1` significa que no ocurrieron reintentos)
* `request_id`: ID de solicitud de API de Anthropic del encabezado `request-id` de la respuesta, como `"req_011..."`. Presente solo cuando la API devuelve uno.
* `client_request_id`: UUID generado por el cliente enviado como encabezado de solicitud `x-client-request-id`. Disponible incluso cuando un fallo como un tiempo de espera o error de conexión nunca produjo un `request_id` del servidor; consulte la tabla [atributos de correlación de eventos](#event-correlation-attributes) para saber cuándo está presente. Requiere Claude Code v2.1.214 o posterior
* `speed`: `"fast"` o `"normal"`, indicando si el modo rápido estaba activo
* `query_source`: Subsistema que emitió la solicitud, como `"repl_main_thread"`, `"compact"`, o un nombre de subagente
* `effort`: [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud. Ausente cuando Claude Code no envía un nivel de esfuerzo, por ejemplo en un modelo que no admite esfuerzo.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribución de habilidad, plugin, agente y MCP para la solicitud. Consulte [Contador de costo](#cost-counter) para obtener definiciones y comportamiento de redacción.

<h4 id="api-refusal-event">
  Evento de rechazo de API
</h4>

Se registra cuando una solicitud de API devuelve `stop_reason: "refusal"`. Los rechazos llegan en una transmisión de respuesta exitosa en lugar de como un error HTTP, por lo que el evento `api_error` no se activa para ellos. Este evento le permite rastrear la frecuencia de rechazo y agrupar rechazos por los mismos atributos que `api_request` y `api_error`.

**Nombre del evento**: `claude_code.api_refusal`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `model`: Identificador de modelo de la solicitud
* `request_id`: ID de solicitud de API de Anthropic del encabezado `request-id` de la respuesta, como `"req_011..."`. Presente solo cuando la API devuelve uno.
* `query_source`: Subsistema que emitió la solicitud, como `"repl_main_thread"`, `"compact"`, o un nombre de subagente. Consulte [`api_request`](#api-request-event) para obtener definiciones.
* `speed`: Ya sea `"fast"` cuando [Modo rápido](/docs/es/fast-mode) está activo, o `"normal"`
* `attempt`: Número de intento de reintento. El primer intento es `1`.
* `effort`: [Nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) aplicado a la solicitud. Ausente cuando Claude Code no envía un nivel de esfuerzo, por ejemplo en un modelo que no admite esfuerzo.
* `server_fallback_hop`: `true` cuando la alternativa de modelo del lado del servidor de la API ya reintentó este rechazo en un modelo diferente, por lo que el usuario no vio este rechazo en particular. `false` cuando la solicitud terminó en un rechazo. Un único turno puede emitir tanto un evento de salto `true` como un evento final `false` posterior cuando el modelo de alternativa también rechaza.
* `has_category`: `true` cuando la respuesta de la API llevaba un `stop_details.category` de `"cyber"`, `"bio"`, `"frontier_llm"`, o `"reasoning_extraction"`. `false` cuando la respuesta no llevaba categoría o un valor fuera de ese conjunto. Ausente cuando `server_fallback_hop` es `true`, porque los saltos no llevan `stop_details`.
* `has_explanation`: `true` cuando la respuesta de la API llevaba un `stop_details.explanation`, de lo contrario `false`. Ausente cuando `server_fallback_hop` es `true`.
* `category`: El valor `stop_details.category` de la respuesta de la API. Uno de `"cyber"`, `"bio"`, `"frontier_llm"`, o `"reasoning_extraction"`. Solo presente cuando `OTEL_LOG_TOOL_DETAILS=1` está establecido y `has_category` es `true`.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: Atribución de habilidad, plugin, agente y MCP para la solicitud. Consulte [Contador de costo](#cost-counter) para obtener definiciones y comportamiento de redacción.

<h4 id="api-request-body-event">
  Evento de cuerpo de solicitud de API
</h4>

Se registra para cada intento de solicitud de API cuando `OTEL_LOG_RAW_API_BODIES` está establecido. Se emite un evento por intento, por lo que los reintentos con parámetros ajustados producen cada uno su propio evento.

**Nombre del evento**: `claude_code.api_request_body`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `body`: Parámetros de solicitud de API de Messages serializados en JSON, como el mensaje del sistema, mensajes y herramientas, truncados en el límite de contenido (60 KB por defecto). El contenido de pensamiento extendido en turnos anteriores del asistente se redacta. Se emite solo en modo en línea (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Ruta absoluta a un archivo `<dir>/<uuid>.request.json` que contiene el cuerpo sin truncar. Se emite solo en modo de archivo (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Longitud del cuerpo sin truncar. Bytes UTF-8 cuando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, o unidades de código UTF-16 cuando `=1`
* `body_truncated`: `"true"` cuando ocurrió truncamiento en línea. Ausente en modo de archivo y cuando no ocurrió truncamiento.
* `model`: Identificador de modelo de los parámetros de solicitud
* `query_source`: Subsistema que emitió la solicitud (por ejemplo, `"compact"`)
* `request_body_id`: UUID que identifica el cuerpo de solicitud de este intento. El evento [`api_response_body`](#api-response-body-event) para el intento que tiene éxito lleva el mismo valor, por lo que puede emparejar una respuesta con la solicitud exacta que la produjo. Requiere Claude Code v2.1.274 o posterior

<h4 id="api-response-body-event">
  Evento de cuerpo de respuesta de API
</h4>

Se registra para cada respuesta de API exitosa cuando `OTEL_LOG_RAW_API_BODIES` está establecido.

En modo de archivo (`OTEL_LOG_RAW_API_BODIES=file:<dir>`), Claude Code también añade una línea JSON a `<dir>/index.jsonl` para cada respuesta exitosa, con los campos `timestamp`, `session_id`, `query_source`, `model`, `request_id`, `message_id`, `message_uuid`, `request_file`, y `response_file`. Léalo para encontrar los archivos de solicitud y respuesta detrás de un mensaje de transcripción dado sin consultar su backend de telemetría. El archivo de índice requiere Claude Code v2.1.274 o posterior.

**Nombre del evento**: `claude_code.api_response_body`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `body`: Respuesta de API de Messages serializada en JSON, incluido el id, bloques de contenido, uso y razón de parada, truncada en el límite de contenido (60 KB por defecto). El contenido de pensamiento extendido se redacta. Se emite solo en modo en línea (`OTEL_LOG_RAW_API_BODIES=1`).
* `body_ref`: Ruta absoluta a un archivo `<dir>/<request_id>.response.json` que contiene el cuerpo sin truncar. Se emite solo en modo de archivo (`OTEL_LOG_RAW_API_BODIES=file:<dir>`).
* `body_length`: Longitud del cuerpo sin truncar. Bytes UTF-8 cuando `OTEL_LOG_RAW_API_BODIES=file:<dir>`, o unidades de código UTF-16 cuando `=1`
* `body_truncated`: `"true"` cuando ocurrió truncamiento en línea. Ausente en modo de archivo y cuando no ocurrió truncamiento.
* `model`: Identificador de modelo
* `query_source`: Subsistema que emitió la solicitud
* `request_id`: ID de solicitud de API de Anthropic del encabezado `request-id` de la respuesta, como `"req_011..."`. Presente solo cuando la API devuelve uno.
* `request_body_id`: El `request_body_id` del evento [`api_request_body`](#api-request-body-event) que esta respuesta responde. Requiere Claude Code v2.1.274 o posterior
* `message.id`: ID de mensaje que la API asignó a la respuesta, el campo `id` del cuerpo de respuesta. Requiere Claude Code v2.1.274 o posterior
* `message.uuid`: UUID de la entrada de transcripción final de la respuesta. Junto con `request_body_id`, vincula un mensaje de transcripción a los cuerpos de solicitud y respuesta detrás de él. Requiere Claude Code v2.1.274 o posterior

<h4 id="tool-decision-event">
  Evento de decisión de herramienta
</h4>

Se registra cuando se toma una decisión de permiso de herramienta (aceptar/rechazar).

**Nombre del evento**: `claude_code.tool_decision`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `tool_name`: Nombre de la herramienta (por ejemplo, "Read", "Edit", "Write", "NotebookEdit")
* `tool_use_id`: Identificador único para esta invocación de herramienta. Coincide con el `tool_use_id` pasado a hooks, permitiendo correlación entre eventos OTel y datos capturados por hooks.
* `decision`: Ya sea `"accept"` o `"reject"`
* `tool_source`: Siempre presente. La procedencia de la herramienta, como un conjunto cerrado de valores creados por CLI. Requiere Claude Code v2.1.214 o posterior
  * `"builtin"`: las herramientas propias de la CLI
  * `"mcp"`: servidores MCP en general
  * `"sdk_host_builtin_mcp"`: un servidor en proceso integrado en Claude Desktop mismo, en una sesión que Claude Desktop posee. Claude Desktop posee una sesión que inició desde uno de sus propios puntos de entrada, `claude-desktop`, `claude-desktop-3p`, o `local-agent`, cuando esa sesión no es un hijo anidado; las sesiones anidadas, incluidas las sesiones que Claude Code mismo genera, reportan estos servidores como `"mcp"`
* `source`: De dónde vino la decisión:
  * `"config"`: Decidido automáticamente sin preguntar, basado en la configuración del proyecto, reglas de permitir o denegar en la configuración personal del usuario, política gestionada empresarial, banderas `--allowedTools` o `--disallowedTools`, el modo de permiso activo, una concesión con alcance de sesión de un mensaje anterior en la misma sesión de CLI interactiva, o porque la herramienta es inherentemente segura. El evento no indica cuál de estas fuentes coincidió. Claude Code también reporta `"config"` cuando la solicitud de mensaje de permiso en sí falla, por ejemplo cuando la devolución de llamada [`canUseTool`](/docs/es/agent-sdk/typescript#canusetool) del SDK de Agent o la herramienta [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags) devuelve un resultado inválido, o cuando el flujo de entrada se cierra mientras la solicitud está pendiente. Antes de v2.1.216, Claude Code reportaba estos fallos como `"user_reject"`.
  * `"hook"`: Un hook `PreToolUse` o `PermissionRequest` devolvió la decisión.
  * `"user_permanent"`: Se emite cuando el usuario eligió "Sí, y no preguntar de nuevo para ..." en un mensaje de permiso, que guarda una regla de permitir en su configuración personal. En la CLI interactiva esto se emite solo para esa opción en sí; las llamadas posteriores que coinciden con la regla guardada emiten `"config"` en su lugar. En sesiones de SDK de Agent o no interactivas `-p`, tanto la opción inicial como las coincidencias de reglas posteriores emiten `"user_permanent"`. Se trata como una aceptación.
  * `"user_temporary"`: Se emite cuando el usuario eligió "Sí" en un mensaje de permiso para una aprobación única, u optó por una opción que otorga acceso para el resto de la sesión en un mensaje de edición o lectura de archivo. En la CLI interactiva esto se emite solo para la opción en sí; las llamadas posteriores permitidas por esa concesión con alcance de sesión emiten `"config"` en su lugar. En sesiones de SDK de Agent o no interactivas `-p`, tanto la opción como las coincidencias posteriores emiten `"user_temporary"`. Se trata como una aceptación.
  * `"user_abort"`: Se emite cuando el usuario descartó el mensaje de permiso sin responder. En sesiones de SDK de Agent y no interactivas `-p`, esto incluye interrumpir el turno mientras una solicitud de permiso `canUseTool` o `--permission-prompt-tool` está pendiente; antes de v2.1.216, Claude Code reportaba esa interrupción como `"user_reject"`. Se trata como un rechazo.
  * `"user_reject"`: Se emite cuando el usuario eligió "No" cuando se le preguntó. En la CLI interactiva esto se emite solo para esa opción en sí; las llamadas que coinciden con una regla de denegar en la configuración personal del usuario emiten `"config"` en su lugar. En sesiones de SDK de Agent o no interactivas `-p`, las llamadas que coinciden con una regla de denegar en la configuración personal emiten `"user_reject"`. Se trata como un rechazo.
* `tool_parameters` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Cadena JSON que contiene parámetros específicos de la herramienta. Misma forma que el [evento de resultado de herramienta](#tool-result-event), menos campos posteriores a la ejecución como `git_commit_id`. Los valores pueden diferir de `tool_result` para una llamada aceptada si la decisión de permiso reescribe la entrada de herramienta a través de `updatedInput`. Use este atributo para ver qué comando fue rechazado cuando `decision` es `"reject"`.
  * Para herramientas `"sdk_host_builtin_mcp"`: `mcp_server_name` y `mcp_tool_name` se incluyen incluso cuando `OTEL_LOG_TOOL_DETAILS` está desactivado, porque la aplicación host define estos nombres; sin ellos, una llamada rechazada a uno de estos servidores integrados sería inattribuible en la transmisión predeterminada. Para servidores MCP configurados por el usuario, el `tool_name` del evento es siempre el literal `"mcp_tool"`, y los nombres del servidor y herramienta aparecen solo en `tool_parameters` con la bandera activada; el contenido del argumento requiere la bandera en todas partes. Requiere Claude Code v2.1.214 o posterior
  * Para herramienta Bash: incluye `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`. La herramienta bash del espacio de trabajo de la aplicación de escritorio también reporta `tool_name` como `Bash`, pero incluye solo `bash_command`, `full_command`, y `timeout`
  * Para herramientas MCP: incluye `mcp_server_name`, `mcp_tool_name`
  * Para herramienta Skill: incluye `skill_name`
  * Para herramienta Agent o herramienta Task heredada: incluye `subagent_type`

<h4 id="permission-mode-changed-event">
  Evento de cambio de modo de permiso
</h4>

Se registra cuando el modo de permiso cambia, por ejemplo al ciclar con Shift+Tab, salir del modo de plan, o una verificación de puerta automática.

**Nombre del evento**: `claude_code.permission_mode_changed`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `from_mode`: El modo de permiso anterior, por ejemplo `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, o `"bypassPermissions"`
* `to_mode`: El nuevo modo de permiso
* `trigger`: Qué causó el cambio. Uno de `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"`, o `"auto_opt_in"`. Ausente cuando la transición se origina del SDK o puente

<h4 id="auth-event">
  Evento de autenticación
</h4>

Se registra cuando `/login` o `/logout` se completa.

**Nombre del evento**: `claude_code.auth`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"auth"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `action`: `"login"` o `"logout"`
* `success`: `"true"` o `"false"`
* `auth_method`: Método de autenticación, como `"oauth"`
* `error_category`: Tipo de error categórico cuando la acción falló. El mensaje de error sin procesar nunca se incluye
* `status_code`: Código de estado HTTP como cadena cuando la acción falló con un error HTTP

<h4 id="mcp-server-connection-event">
  Evento de conexión del servidor MCP
</h4>

Se registra cuando un servidor MCP se conecta, desconecta o falla al conectarse.

**Nombre del evento**: `claude_code.mcp_server_connection`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `status`: `"connected"`, `"failed"`, o `"disconnected"`
* `transport_type`: Transporte del servidor, como `"stdio"`, `"sse"`, o `"http"`
* `server_scope`: Alcance en el que está configurado el servidor, como `"user"`, `"project"`, o `"local"`
* `duration_ms`: Duración del intento de conexión en milisegundos
* `error_code`: Código de error cuando la conexión falló
* `is_plugin`: `true` cuando el servidor es proporcionado por un plugin, `false` de lo contrario
* `plugin_id_hash` (cuando `is_plugin` es `true`): Hash estable del nombre del plugin y mercado, para agrupar eventos por plugin sin exponer el nombre. Claude Code lo calcula como se describe en el [evento de plugin cargado](#plugin-loaded-event)
* `plugin.name` (cuando `is_plugin` es `true`): Nombre del plugin que proporciona el servidor. Para plugins de terceros este es el literal `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`; esto protege los nombres de plugins de terceros de aparecer en registros por defecto. Los plugins de fuentes oficiales de Anthropic siempre se identifican por nombre. Los atributos `plugin_id_hash` y `plugin.name` fluyen a su propio backend de monitoreo y no se envían a Anthropic
* `server_name` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Nombre del servidor configurado
* `error` (cuando `OTEL_LOG_TOOL_DETAILS=1`): Mensaje de error completo cuando la conexión falló

<h4 id="internal-error-event">
  Evento de error interno
</h4>

Se registra cuando Claude Code detecta un error interno inesperado. Solo se registran el nombre de la clase de error y un código de estilo errno. El mensaje de error y el rastreo de pila nunca se incluyen. Este evento no se emite cuando se ejecuta contra Amazon Bedrock, la plataforma de agentes de Google Cloud, Microsoft Foundry, o cuando `DISABLE_ERROR_REPORTING` está establecido.

**Nombre del evento**: `claude_code.internal_error`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `error_name`: Nombre de la clase de error, como `"TypeError"` o `"SyntaxError"`
* `error_code`: Código errno de Node.js como `"ENOENT"` cuando está presente en el error

<h4 id="plugin-installed-event">
  Evento de plugin instalado
</h4>

Se registra cuando un plugin termina de instalarse, tanto desde el comando CLI `claude plugin install` como desde la interfaz de usuario interactiva `/plugin`.

**Nombre del evento**: `claude_code.plugin_installed`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `marketplace.is_official`: `"true"` si el mercado es un mercado oficial de Anthropic, `"false"` de lo contrario
* `install.trigger`: `"cli"` o `"ui"`
* `plugin.name`: Nombre del plugin instalado. Para mercados de terceros esto se incluye solo cuando `OTEL_LOG_TOOL_DETAILS=1`
* `plugin.version`: Versión del plugin cuando se declara en la entrada del mercado. Para mercados de terceros esto se incluye solo cuando `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: Mercado desde el cual se instaló el plugin. Para mercados de terceros esto se incluye solo cuando `OTEL_LOG_TOOL_DETAILS=1`

<h4 id="plugin-loaded-event">
  Evento de plugin cargado
</h4>

Se registra una vez por plugin habilitado al inicio de la sesión. Use este evento para inventariar qué plugins están activos en su flota, como complemento a `plugin_installed` que registra la acción de instalación en sí.

**Nombre del evento**: `claude_code.plugin_loaded`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `plugin.name`: nombre del plugin. Para plugins fuera del mercado oficial y paquete integrado el valor es `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `marketplace.name`: mercado desde el cual se instaló el plugin, cuando se conoce. Redactado a `"third-party"` bajo la misma condición que `plugin.name`
* `plugin.version`: versión del manifiesto del plugin. Se incluye solo cuando el nombre no está redactado y el manifiesto declara una versión
* `plugin.scope`: categoría de procedencia del plugin: `"official"`, `"community"`, `"org"`, `"user-local"`, o `"default-bundle"`
* `enabled_via`: cómo el plugin llegó a estar habilitado: `"default-enable"`, `"org-policy"`, `"admin-install"`, `"seed-mount"`, o `"user-install"`. El valor `"admin-install"` significa que el plugin está configurado como requerido o instalación automática para su organización en [**Configuración de la organización > Plugins y habilidades**](https://claude.ai/admin-settings/skills?tab=inventory). Antes de v2.1.246, Claude Code reportaba estos plugins como `"user-install"` o `"seed-mount"`
* `plugin_id_hash`: hash determinista del nombre del plugin y mercado, enviado solo a su exportador configurado. Le permite contar los plugins de terceros distintos cargados en su flota sin registrar sus nombres. Para [plugins sincronizados desde claude.ai](/docs/es/plugins/loading#synced-plugins), Claude Code codifica el nombre del plugin con el nombre del mercado que claude.ai reporta para el plugin, o con `synced` de lo contrario. Antes de v2.1.246, Claude Code no usaba el nombre del mercado que claude.ai reporta en el hash
* `has_hooks`: si el plugin contribuye hooks
* `has_mcp`: si el plugin contribuye servidores MCP
* `host_owned_mcp`: `true` cuando el host del SDK gestiona las conexiones MCP de este plugin y Claude Code omitió leer la configuración del servidor MCP del plugin, `false` de lo contrario. Requiere Claude Code v2.1.172 o posterior
* `skill_path_count`: número de directorios de habilidades que declara el plugin
* `command_path_count`: número de directorios de comandos que declara el plugin
* `agent_path_count`: número de directorios de agentes que declara el plugin
* `safe_mode`: `"true"` cuando la sesión se inició con [`--safe-mode`](/docs/es/cli-reference), `"false"` de lo contrario. En modo seguro este evento reporta solo el inventario configurado; los comandos, habilidades, hooks y servidores MCP del plugin no se cargan. Requiere Claude Code v2.1.169 o posterior

<h4 id="skill-activated-event">
  Evento de habilidad activada
</h4>

Se registra cuando se invoca una habilidad, ya sea que Claude la llame a través de la herramienta Skill o que la ejecute como un comando `/`.

**Nombre del evento**: `claude_code.skill_activated`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `skill.name`: Nombre de la habilidad. Para habilidades definidas por el usuario y de plugins de terceros el valor es el marcador de posición `"custom_skill"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `invocation_trigger`: Cómo se activó la habilidad (`"user-slash"`, `"claude-proactive"`, o `"nested-skill"`)
* `skill.source`: De dónde se cargó la habilidad (por ejemplo, `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: `"workflow"` cuando la habilidad es una habilidad de flujo de trabajo. Ausente de lo contrario
* `plugin.name` (cuando `OTEL_LOG_TOOL_DETAILS=1` o el plugin es de un mercado oficial): Nombre del plugin propietario cuando la habilidad es proporcionada por un plugin
* `marketplace.name` (cuando `OTEL_LOG_TOOL_DETAILS=1` o el plugin es de un mercado oficial): Mercado desde el cual se instaló el plugin propietario, cuando la habilidad es proporcionada por un plugin

<h4 id="at-mention-event">
  Evento de mención @
</h4>

Se registra cuando Claude Code resuelve una mención `@` en un mensaje. No todas las menciones emiten un evento: las rutas de salida temprana como denegaciones de permiso, archivos de tamaño excesivo, adjuntos de referencia PDF y fallos de listado de directorios se devuelven sin registrar.

**Nombre del evento**: `claude_code.at_mention`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `mention_type`: Tipo de mención (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`, `"peer"`). El valor `"peer"` significa que mencionó [una de sus otras sesiones de Claude Code](/docs/es/cross-session-messaging). Requiere Claude Code v2.1.232 o posterior
* `success`: Si la mención se resolvió exitosamente (`"true"` o `"false"`)

<h4 id="api-retries-exhausted-event">
  Evento de reintentos de API agotados
</h4>

Se registra una vez cuando una solicitud de API falla después de más de un intento. Se emite junto con el evento `api_error` final.

**Nombre del evento**: `claude_code.api_retries_exhausted`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `model`: Modelo utilizado
* `error`: Mensaje de error final
* `status_code`: Código de estado HTTP como número. Ausente para errores no HTTP.
* `total_attempts`: Número total de intentos realizados
* `total_retry_duration_ms`: Tiempo total de reloj de pared en todos los intentos
* `speed`: `"fast"` o `"normal"`

<h4 id="hook-registered-event">
  Evento de hook registrado
</h4>

Se registra una vez por hook configurado al inicio de la sesión. Use este evento para inventariar qué hooks están activos en su flota, como complemento a los eventos por ejecución `hook_execution_start` y `hook_execution_complete`.

**Nombre del evento**: `claude_code.hook_registered`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `hook_event`: tipo de evento de hook, como `"PreToolUse"` o `"PostToolUse"`
* `hook_type`: tipo de implementación de hook: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"`, o `"agent"`
* `hook_source`: dónde se define el hook: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"`, o `"pluginHook"`
* `safe_mode`: `"true"` cuando la sesión se inició con [`--safe-mode`](/docs/es/cli-reference), `"false"` de lo contrario. Requiere Claude Code v2.1.169 o posterior
* `hook_matcher` (cuando `OTEL_LOG_TOOL_DETAILS=1`): la cadena de coincidencia de la configuración del hook, cuando se establece
* `plugin.name` (cuando `hook_source` es `"pluginHook"`): nombre del plugin contribuyente. Para plugins fuera del mercado oficial y paquete integrado el valor es `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1`
* `plugin_id_hash` (cuando `hook_source` es `"pluginHook"`): hash determinista del nombre del plugin y mercado, enviado solo a su exportador configurado. Le permite contar plugins contribuyentes distintos sin registrar sus nombres. Claude Code lo calcula como se describe en el [evento de plugin cargado](#plugin-loaded-event)

<h4 id="hook-execution-start-event">
  Evento de inicio de ejecución de hook
</h4>

Se registra cuando uno o más hooks comienzan a ejecutarse para un evento de hook.

**Nombre del evento**: `claude_code.hook_execution_start`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `hook_event`: Tipo de evento de hook, como `"PreToolUse"` o `"PostToolUse"`
* `hook_name`: Nombre completo del hook incluida la coincidencia, como `"PreToolUse:Write"`
* `num_hooks`: Número de comandos de hook coincidentes
* `managed_only`: `"true"` cuando solo se permiten hooks de política gestionada
* `hook_source`: `"policySettings"` o `"merged"`
* `safe_mode`: `"true"` cuando la sesión se inició con [`--safe-mode`](/docs/es/cli-reference), `"false"` de lo contrario. Requiere Claude Code v2.1.169 o posterior
* `hook_definitions`: Configuración de hook serializada en JSON. Se incluye solo cuando tanto el rastreo beta detallado como `OTEL_LOG_TOOL_DETAILS=1` están habilitados

<h4 id="hook-execution-complete-event">
  Evento de finalización de ejecución de hook
</h4>

Se registra cuando todos los hooks para un evento de hook han terminado.

**Nombre del evento**: `claude_code.hook_execution_complete`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `hook_event`: Tipo de evento de hook
* `hook_name`: Nombre completo del hook incluida la coincidencia
* `num_hooks`: Número de comandos de hook coincidentes
* `num_success`: Conteo que se completó exitosamente
* `num_blocking`: Conteo que devolvió una decisión de bloqueo
* `num_non_blocking_error`: Conteo que falló sin bloquear
* `num_cancelled`: Conteo cancelado antes de completarse
* `total_duration_ms`: Duración de reloj de pared de todos los hooks coincidentes
* `stdout_chars`: Total de caracteres de stdout en todos los hooks coincidentes que tuvieron éxito. Requiere Claude Code v2.1.280 o posterior
* `additional_context_chars`: Total de caracteres de `additionalContext` devueltos por los hooks coincidentes. Requiere Claude Code v2.1.280 o posterior
* `system_message_chars`: Total de caracteres de `systemMessage` devueltos por los hooks coincidentes. Requiere Claude Code v2.1.280 o posterior
* `initial_user_message_chars`: Total de caracteres de `initialUserMessage` devueltos por los hooks coincidentes. Requiere Claude Code v2.1.280 o posterior
* `num_outputs_persisted`: Número de salidas de hook sobre el [límite de 10,000 caracteres](/docs/es/hooks#json-output) que Claude Code guardó en un archivo. Requiere Claude Code v2.1.280 o posterior
* `managed_only`: `"true"` cuando solo se permiten hooks de política gestionada
* `hook_source`: `"policySettings"` o `"merged"`
* `safe_mode`: `"true"` cuando la sesión se inició con [`--safe-mode`](/docs/es/cli-reference), `"false"` de lo contrario. Requiere Claude Code v2.1.169 o posterior
* `hook_definitions`: Configuración de hook serializada en JSON. Se incluye solo cuando tanto el rastreo beta detallado como `OTEL_LOG_TOOL_DETAILS=1` están habilitados

<h4 id="hook-plugin-metrics-event">
  Evento de métricas de plugin de hook
</h4>

Se registra cuando un hook de plugin del mercado oficial emite métricas por invocación. Solo los plugins instalados desde un mercado oficial de Anthropic pueden emitir estos. Los plugins de mercado de terceros y los hooks configurados por el usuario no emiten a este evento. Use este evento para monitorear el comportamiento del plugin como tasas de búsqueda, costos y duraciones desde su propia pila de observabilidad.

**Nombre del evento**: `claude_code.hook_plugin_metrics`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `plugin_id`: identificador del plugin en forma `<name>@<marketplace>`
* `hook_event`: tipo de evento de hook que emitió las métricas
* Hasta 20 claves de métrica emitidas por el plugin. Los nombres coinciden con `^[a-z][a-z0-9_]{0,39}$`. Los valores son booleanos o números.

<h4 id="compaction-event">
  Evento de compactación
</h4>

Se registra cuando la compactación de conversación se completa.

**Nombre del evento**: `claude_code.compaction`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"compaction"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `trigger`: `"auto"` o `"manual"`
* `success`: `"true"` o `"false"`
* `duration_ms`: Duración de la compactación
* `pre_tokens`: Conteo aproximado de tokens antes de la compactación
* `post_tokens`: Conteo aproximado de tokens después de la compactación
* `error`: Mensaje de error cuando la compactación falló
* `precompute_reuse`: Solo se establece cuando `trigger` es `"manual"`. La compactación automática puede preparar un resumen en el fondo antes de que la ventana de contexto se llene, y este atributo registra si `/compact` reutilizó ese resumen preparado. `"hit"` significa que se reutilizó; `"miss_custom_instructions"`, `"miss_hook"`, y `"miss_not_ready"` dan la razón por la que se calculó un resumen fresco en su lugar. Requiere Claude Code v2.1.153 o posterior

<h4 id="subagent-completed-event">
  Evento de subagente completado
</h4>

Se registra cuando un [subagente](/docs/es/sub-agents) termina y devuelve su resultado a la conversación que lo inició. Úselo para acumular el uso de herramientas y tiempo de ejecución por tipo de subagente; para acumulaciones de tokens o costo, use el [contador de tokens](#token-counter) y [contador de costo](#cost-counter) filtrados a `query_source` `"subagent"`, ya que el `total_tokens` de este evento cubre solo la solicitud final. La categoría `"subagent"` también cuenta solicitudes de hooks basados en agentes, que no emiten evento de subagente.

**Nombre del evento**: `claude_code.subagent_completed`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"subagent_completed"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `agent_type`: El tipo de subagente. Los nombres de agentes integrados y los agentes de plugins del mercado oficial aparecen literalmente; otros nombres de agentes se reemplazan con `"custom"` a menos que `OTEL_LOG_TOOL_DETAILS=1` esté establecido
* `agent.source`: De dónde vino la definición del agente: `built-in`, `plugin`, o la fuente de configuración que definió un agente personalizado, como `userSettings` o `projectSettings`
* `is_built_in`: Si el subagente es un tipo de agente integrado
* `is_async`: Si el subagente se ejecutó en [segundo plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background)
* `total_tokens`: La huella de tokens de la solicitud de API final del subagente: esa solicitud única de entrada, creación de caché, lectura de caché y tokens de salida, aproximadamente el tamaño de contexto del subagente al completarse. No es una suma en toda la ejecución
* `total_tool_uses`: Número de llamadas de herramienta que realizó el subagente en toda la ejecución
* `duration_ms`: Tiempo de ejecución en milisegundos
* `model`: El modelo en el que se resolvió el subagente para ejecutarse
* `final_model`: El modelo que produjo la respuesta final del subagente, que difiere de `model` después de un cambio a mitad de ejecución como una alternativa. Requiere Claude Code v2.1.212 o posterior
* `model_swapped`: Si más de un modelo sirvió las solicitudes del subagente. Requiere Claude Code v2.1.212 o posterior
* `plugin_id_hash`, `plugin.name`: Presente para agentes proporcionados por plugins. Los nombres de plugins del mercado oficial aparecen literalmente; otros nombres de plugins se reemplazan con `"third-party"` a menos que `OTEL_LOG_TOOL_DETAILS=1` esté establecido

<h4 id="feedback-survey-event">
  Evento de encuesta de retroalimentación
</h4>

Se registra cuando se muestra o se responde una encuesta de calidad de sesión. Consulte [Encuestas de calidad de sesión](/docs/es/data-usage#session-quality-surveys) para ver qué recopilan las encuestas y cómo controlarlas.

**Nombre del evento**: `claude_code.feedback_survey`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `event_type`: Evento del ciclo de vida de la encuesta, por ejemplo `"appeared"`, `"responded"`, o `"transcript_prompt_appeared"`
* `appearance_id`: ID único que vincula los eventos emitidos para una instancia de encuesta
* `survey_type`: Qué encuesta produjo el evento. `"session"` es el mensaje de calificación "¿Cómo está Claude?"
* `response`: La selección del usuario en eventos `responded`
* `enabled_via_override`: `true` cuando [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/es/env-vars) está establecido. Se emite como booleano, no como cadena. Presente en eventos de encuesta `session`. Filtre en este atributo para confirmar que la anulación se aplica en toda una flota

<h4 id="retention-sweep-event">
  Evento de barrido de retención
</h4>

Se registra una vez por ejecución del barrido de limpieza de retención, que elimina [transcripciones de sesión y otros datos de aplicación](/docs/es/claude-directory#cleaned-up-automatically) más antiguos que la configuración [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays). Claude Code ejecuta el barrido en el fondo como máximo una vez por sesión, y una ejecución que no elimina nada aún emite el evento. Si Claude Code ejecutó el barrido en cualquier sesión en la misma máquina en las últimas 24 horas, retrasa el barrido de esta sesión al menos 10 minutos, por lo que una sesión que sale antes no emite nada. Cuando ejecuta `claude -p` con `--bare`, Claude Code no ejecuta el barrido y no emite nada.

Como todos los eventos OTel en esta página, solo va al backend de telemetría que configura. Requiere Claude Code v2.1.227 o posterior.

Cuando Claude Code no puede determinar de forma segura el período de retención, pausa el barrido y emite el evento con `result` establecido en `"skipped"` y un `skip_reason`. Cuando [configuración gestionada](/docs/es/server-managed-settings) establece `cleanupPeriodDays`, el valor gestionado fija el período de retención y el barrido se ejecuta incluso cuando un archivo de configuración en un alcance de prioridad más baja está roto o es inválido. Cuando `managed-settings.json` en sí no se puede leer, Claude Code aún pausa el barrido a menos que el [nivel gestionado](/docs/es/managed-settings#how-claude-code-combines-managed-sources) suministre `cleanupPeriodDays` desde otro lugar, como configuración gestionada por servidor o un drop-in `managed-settings.d/` junto al archivo roto. Los atributos del contador de eliminación están presentes solo cuando `result` es `"complete"`.

**Nombre del evento**: `claude_code.retention_sweep`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"retention_sweep"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `result`: `"complete"` cuando el barrido se ejecutó, `"skipped"` cuando Claude Code lo pausó
* `period_days`: El valor `cleanupPeriodDays` de la configuración fusionada, en días, o `30` cuando ninguna fuente lo establece. En eventos omitidos, el valor que el barrido habría utilizado, calculado a partir de las fuentes de configuración que Claude Code pudo leer
* `used_default`: `"true"` cuando ninguna fuente de configuración legible establece `cleanupPeriodDays`, `"false"` de lo contrario. En eventos completos, `"true"` significa que se aplicó el predeterminado de 30 días
* `skip_reason`: Por qué Claude Code pausó el barrido. Presente solo cuando `result` es `"skipped"`:
  * `"user_source_disabled"`: La configuración del usuario está excluida, por ejemplo por la bandera [`--setting-sources`](/docs/es/cli-reference#cli-flags) o la opción [`settingSources`](/docs/es/agent-sdk/typescript#options) del SDK, y ninguna fuente habilitada proporciona `cleanupPeriodDays`
  * `"settings_unknowable"`: Un archivo de configuración no se pudo leer o analizar, por lo que `cleanupPeriodDays` o `desktopSessionCleanupPeriodDays` pueden estar establecidos en un valor que Claude Code no puede ver
  * `"settings_invalid_key_set"`: La configuración tiene errores de validación y `cleanupPeriodDays` o `desktopSessionCleanupPeriodDays` está explícitamente establecido, por lo que recurrir al predeterminado podría eliminar o mantener archivos contra esa configuración
* `transcripts_deleted`: Número de transcripciones de sesión, los archivos de nivel superior `~/.claude/projects/*/*.jsonl`, que el barrido eliminó
* `transcripts_exempted_desktop`: Número de transcripciones pasadas el período de retención que el barrido mantuvo bajo la [regla de Claude Desktop y Cowork](/docs/es/claude-directory#cleaned-up-automatically). Estos no cuentan hacia `files_past_cutoff`. Requiere Claude Code v2.1.248 o posterior
* `session_files_deleted`: Número de artefactos que el barrido de archivos de sesión eliminó: transcripciones más archivos complementarios por sesión como barras laterales, grabaciones y resultados de herramientas
* `artifacts_deleted`: Total de elementos que el barrido eliminó en todos los directorios de datos que cubre, incluidos los archivos de sesión. Algunos barridos cuentan un árbol de directorios completo eliminado como un elemento y algunos pases de limpieza no contribuyen al contador, por lo que trate el valor como un piso en lugar de un conteo exacto de archivos
* `files_retained_fresh`: Archivos inspeccionados y dejados en su lugar porque aún están dentro del período de retención. Solo los barridos por archivo cuentan estos, por lo que el valor es un piso; un valor distinto de cero es el estado estable normal
* `files_past_cutoff`: Archivos más antiguos que el período de retención que el barrido no pudo eliminar, por ejemplo debido a un error de permiso o un archivo mantenido abierto. Un valor superior a cero significa que los archivos sobrevivieron al período de retención configurado; cero no es prueba de que ninguno lo hizo, porque una eliminación fallida de un directorio completo cuenta hacia `error_count` en su lugar
* `error_count`: Número de errores que el barrido encontró mientras listaba o eliminaba archivos

<h4 id="managed-settings-resolved-event">
  Evento de configuración gestionada resuelta
</h4>

Se registra con la [configuración gestionada](/docs/es/managed-settings) que una sesión resolvió: una vez al inicio de la sesión, nuevamente cuando la configuración gestionada o el [asistente de política](/docs/es/managed-settings#compute-the-policy-with-a-helper-program) cambia de estado durante la sesión, y cuando Claude Code se niega a iniciar o termina la sesión por una de las razones que el atributo `error.type` lista.
Use este evento para encontrar máquinas ejecutándose en una fuente gestionada inesperada, máquinas cuyo asistente de política está fallando, y la razón por la que una máquina se negó a iniciar.
Requiere Claude Code v2.1.274 o posterior.

Por defecto, el evento lleva las fuentes gestionadas y el estado del asistente de política pero no la configuración en sí. Para agregar el atributo `managed_settings.settings` redactado y el resumen `managed_settings.resolved_sha256`, establezca `OTEL_LOG_MANAGED_SETTINGS=1`:

* Establézcalo en el bloque `env` de configuración gestionada, configuración del usuario, o `--settings`, o en el entorno con el que inicia Claude Code. Un valor en configuración de proyecto o local no lo activa, porque un repositorio clonado puede escribirlos.
* La configuración gestionada por servidor puede establecerlo sin mostrar el [diálogo de aprobación de seguridad](/docs/es/server-managed-settings#security-approval-dialogs), porque la variable solo agrega su propia política redactada de la organización a un evento que su organización ya recibe.

En una sesión interactiva en una carpeta que no ha [confiado](/docs/es/permissions#what-runs-before-you-trust-a-folder), Claude Code no exporta el evento de rechazo.

**Nombre del evento**: `claude_code.managed_settings_resolved`

**Atributos**:

* Todos los [atributos estándar](#standard-attributes)
* `event.name`: `"managed_settings_resolved"`
* `event.timestamp`: Marca de tiempo ISO 8601
* `event.sequence`: contador por proceso para ordenar eventos, descrito en [Atributos de correlación de eventos](#event-correlation-attributes)
* `managed_settings.trigger`: `"startup"` para el evento de inicio de sesión, `"change"` cuando la configuración gestionada o el estado del asistente de política cambió más tarde en la sesión, o `"refused"` cuando una política de configuración gestionada detuvo la sesión. Claude Code envía un evento `change` solo cuando un atributo difiere del último evento que envió, y un valor de configuración cambiado cuenta incluso cuando `OTEL_LOG_MANAGED_SETTINGS` está desactivado
* `error.type`: por qué Claude Code detuvo la sesión. Presente solo en eventos `refused`:
  * `"helper_failed"`: una [ejecución del asistente de política falló](/docs/es/settings-reference#helper-failures)
  * `"policy_invalid"`: la configuración gestionada contiene un error que detiene a Claude Code de iniciar, o una fuente de administrador no se pudo cargar, por lo que Claude Code no puede verificar la aplicación de inicio de sesión de la organización
  * `"consent_rejected"`: el usuario rechazó el [diálogo de aprobación de seguridad](/docs/es/server-managed-settings#security-approval-dialogs) para configuración gestionada por servidor
  * `"force_refresh_failed"`: la búsqueda de configuración que [`forceRemoteSettingsRefresh`](/docs/es/settings-reference#forceremotesettingsrefresh) requiere falló
  * `"gateway_rejected"`: una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) respondió a la carga de configuración gestionada con HTTP 403
  * `"version_below_minimum"`: esta versión de Claude Code está por debajo de [`requiredMinimumVersion`](/docs/es/settings-reference#requiredminimumversion) o por encima de [`requiredMaximumVersion`](/docs/es/settings-reference#requiredmaximumversion)
  * `"_OTHER"`: la carga de configuración gestionada de la puerta de enlace de aplicaciones Claude falló por otra razón
* `managed_settings.sources`: cada fuente gestionada que entrega al menos una [clave de política](/docs/es/managed-settings#how-claude-code-combines-managed-sources), prioridad más alta primero, incluidas fuentes cuyas claves no tienen efecto bajo `first-wins`. Los valores son `"remote"`, `"plist"` o `"hklm"` para la política de MDM o nivel de SO, `"file"` para archivos de configuración gestionada y drop-ins, `"parent"` cuando un [host de incrustación](/docs/es/managed-settings#let-an-embedding-host-add-policy) suministra configuración, y `"hkcu"` para el [valor del registro HKCU de Windows](/docs/es/managed-settings#where-each-mechanism-stores-the-policy) cuando Claude Code lo [lee](/docs/es/managed-settings#how-claude-code-combines-managed-sources). Una fuente que lleva solo claves de control, o que Claude Code no pudo leer, no está listada. Se emite como una matriz de cadenas, vacía cuando ninguna fuente gestionada entrega una clave de política
* `managed_settings.source_behavior`: el valor [`managedSourcesBehavior`](/docs/es/settings-reference#managedsourcesbehavior) que Claude Code leyó, `"first-wins"` o `"merge"`. `"first-wins"` cuando ninguna fuente establece la clave
* `managed_settings.helper.state`: estado del asistente de política que la fuente de MDM o archivo seleccionada configura:
  * `"ok"`: la salida del asistente sirve como la configuración gestionada
  * `"bad_path"`, `"not_a_file"`, `"exit_nonzero"`, `"timed_out"`, `"oversize"`, `"parse_failed"`, `"envelope_invalid"`, o `"schema_rejected"`: la última ejecución del asistente falló. [Fallos del asistente](/docs/es/settings-reference#helper-failures) describe los casos
  * `"none"`: ningún asistente está configurado, o la fuente que lo configura no es una política de MDM o archivo de configuración gestionada
* `managed_settings.helper.applied`: `"output"` mientras la salida propia del asistente sirve como la configuración gestionada, `"none"` cuando no lo hace
* `managed_settings.helper.entry`: `"policyHelper"` cuando Claude Code seleccionó un [`policyHelper`](/docs/es/settings-reference#policyhelper). Ausente cuando no seleccionó ningún asistente.
* `managed_settings.helper.path`: la [`path`](/docs/es/settings-reference#policyhelper-path) configurada del asistente. Presente siempre que Claude Code seleccionó un asistente, independientemente de si `OTEL_LOG_MANAGED_SETTINGS` está establecido
* `managed_settings.resolved_sha256` (cuando `OTEL_LOG_MANAGED_SETTINGS=1`): SHA-256 de la configuración gestionada resuelta antes de la redacción, serializada como JSON con claves ordenadas recursivamente y sin espacios en blanco. Las máquinas con el mismo resumen ejecutan la misma política. Claude Code envía el resumen solo con la opción de participación porque una política corta se puede recuperar codificando adivinanzas. Ausente cuando no se resolvió configuración gestionada, y en eventos `refused`
* `managed_settings.settings` (cuando `OTEL_LOG_MANAGED_SETTINGS=1`): los nombres y forma de la configuración gestionada resuelta como una cadena JSON, con los valores redactados. Ausente en eventos `refused`. Claude Code lo construye a partir de su esquema de configuración:

  * Un nombre de configuración que el esquema declara se exporta, y una clave que no declara se deja fuera
  * Booleanos, números y valores de cadena que el esquema restringe a un conjunto fijo de opciones, como `permissions.defaultMode`, se exportan tal cual. `sandbox.network.httpProxyPort` y `sandbox.network.socksProxyPort` se exportan como `"[REDACTED]"`
  * Cada otra cadena, como `model`, `apiKeyHelper`, cada valor `env`, cada URL, y cada comando, se exporta como `"[REDACTED]"`
  * Los nombres de entrada de mapas, como nombres de variables `env` e IDs de plugins, se exportan tal cual. Una configuración cuyas entradas el esquema no escribe, como `vimInsertModeRemaps`, se exporta como un único `"[REDACTED]"`, y `sandbox.ignoreViolations` se exporta como una lista de sus listas de rutas sin los patrones de comando
  * Una lista mantiene su longitud, con cada entrada redactada por las mismas reglas
  * Una regla `permissions.allow`, `permissions.deny`, o `permissions.ask` se exporta como su nombre de herramienta con el contenido redactado, como `Read([REDACTED])`, cuando la herramienta está integrada en esta versión de Claude Code o es una referencia `mcp__` como `mcp__jira__create_issue`. Cualquier otra regla se exporta como `"[REDACTED]"`
  * Los hooks siguen las mismas reglas, por lo que los campos de opción fija y numéricos como `type` y `timeout` se muestran, mientras que cada comando, URL, `matcher`, y condición `if` se exporta como `"[REDACTED]"`

  Por ejemplo, la configuración gestionada con `apiKeyHelper`, dos variables `env`, y una regla de denegar se exporta como `{"apiKeyHelper":"[REDACTED]","env":{"HTTPS_PROXY":"[REDACTED]","CLAUDE_CODE_ENABLE_TELEMETRY":"[REDACTED]"},"permissions":{"deny":["Read([REDACTED])"]}}`.

  Claude Code corta el valor en 8 KB de UTF-8, y el valor cortado no es JSON válido
* `managed_settings.settings_truncated` (cuando `managed_settings.settings` está presente): `true` cuando Claude Code cortó `managed_settings.settings` en 8 KB, `false` de lo contrario. Se emite como booleano, no como cadena

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

Claude Code cuenta cada respuesta de transmisión hacia las métricas de costo y token exactamente una vez, incluyendo cuando una puerta de enlace o proxy detrás de `ANTHROPIC_BASE_URL` transmite el uso progresivamente a través de múltiples fotogramas. Antes de v2.1.214, las transmisiones que llevaban uso en más de un fotograma inflaban `claude_code.cost.usage` y `claude_code.token.usage` aproximadamente una solicitud completa adicional por fotograma adicional.

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

El atributo `attempt` en el evento registra el número total de intentos. `CLAUDE_CODE_MAX_RETRIES` tiene un valor predeterminado de 10 y está limitado a 15. A partir de v2.1.199, puede establecer `CLAUDE_CODE_RETRY_WATCHDOG` para aumentar el valor predeterminado y eliminar el límite.

Cuando la solicitud agota todos los reintentos en un error transitorio, `attempt` es igual a uno más que ese límite efectivo: 11 por defecto, y nunca más de 16 a menos que el vigilante esté configurado. Un valor más bajo indica un error no reintentable como una respuesta `400`, o una causa con su propio presupuesto de reintento más pequeño. Por ejemplo, Claude Code reintenta una falla al cargar credenciales de AWS o Google Cloud como máximo dos veces.

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

Los [atributos estándar](#standard-attributes) en cada evento incluyen la identidad del usuario autenticado: `user.email`, `user.account_uuid`, `user.account_id`, y `organization.id` cuando se inicia sesión con una cuenta de Claude o, en una [sesión en la nube](/docs/es/claude-code-on-the-web), cuando las credenciales propias de la sesión las llevan, más `user.id` y el `session.id` por sesión. `user.id` es un identificador con alcance de instalación, excepto en sesiones de [Claude apps gateway](/docs/es/claude-apps-gateway), donde es el asunto del IdP del token emitido por la puerta de enlace.

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

* `tool_result`: mantiene `mcp_server_scope` y un `tool_name` redactado al literal `"mcp_tool"` para servidores configurados por el usuario, omite contenido de argumentos. Para los servidores integrados de Claude Desktop, en sesiones que Claude Desktop posee, también mantiene el par `mcp_server_name`/`mcp_tool_name` dentro de `tool_parameters`, la misma excepción autorizada por el host que `tool_decision`, requiere Claude Code v2.1.214 o posterior
* `tool_decision`: mantiene `tool_source` y un `tool_name` redactado al literal `"mcp_tool"` para servidores configurados por el usuario, omite contenido de argumentos. Para los servidores integrados de Claude Desktop, en sesiones que Claude Desktop posee, también mantiene el par `mcp_server_name`/`mcp_tool_name` dentro de `tool_parameters`; `tool_source` y el par de nombres requieren Claude Code v2.1.214 o posterior
* `mcp_server_connection`: omite `server_name` y el mensaje de error, pero mantiene `is_plugin`, `plugin_id_hash`, y `plugin.name`, con nombres de plugins que no son de Anthropic redactados al literal `"third-party"`, por lo que los servidores proporcionados por plugins siguen siendo distinguibles sin registro detallado

<h3 id="map-security-questions-to-events">
  Mapear preguntas de seguridad a eventos
</h3>

Al construir reglas de detección, busque la señal que desea monitorear y consulte su backend para el evento correspondiente y atributos:

| Señal                                                                                                                                         | Evento                                                                                | Atributos Clave                                                                                                                                                                                                                               |
| --------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Llamada de herramienta permitida o denegada, y por qué                                                                                        | `tool_decision`                                                                       | `decision`, `source`, `tool_name`, `tool_parameters`                                                                                                                                                                                          |
| Escalada de modo de permiso                                                                                                                   | `permission_mode_changed`                                                             | `from_mode`, `to_mode`, `trigger`                                                                                                                                                                                                             |
| Hook de política bloqueó una acción                                                                                                           | `hook_execution_complete`                                                             | `hook_event`, `num_blocking`                                                                                                                                                                                                                  |
| Inicio de sesión, cierre de sesión y fallo de autenticación                                                                                   | `auth`                                                                                | `action`, `success`, `error_category`                                                                                                                                                                                                         |
| Conexión del servidor MCP o fallo                                                                                                             | `mcp_server_connection`                                                               | `status`, `server_name`, `is_plugin`, `error_code`                                                                                                                                                                                            |
| Plugin instalado y su origen                                                                                                                  | `plugin_installed`                                                                    | `plugin.name`, `marketplace.name`, `marketplace.is_official`                                                                                                                                                                                  |
| Comandos ejecutados y archivos tocados                                                                                                        | `tool_result` (ejecutado) o `tool_decision` (rechazado) con `OTEL_LOG_TOOL_DETAILS=1` | `tool_parameters`; `tool_input` (`tool_result` solo)                                                                                                                                                                                          |
| Qué fuentes de configuración administrada ejecuta una máquina, si su asistente de política es saludable y por qué una máquina rechazó iniciar | `managed_settings_resolved`                                                           | `managed_settings.trigger`, `managed_settings.sources`, `managed_settings.source_behavior`, `managed_settings.helper.state`, `error.type`; `managed_settings.settings` y `managed_settings.resolved_sha256` con `OTEL_LOG_MANAGED_SETTINGS=1` |

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

Para confirmar que los eventos llegan, envíe un mensaje en una sesión que se ejecute bajo esta configuración y verifique su SIEM para el evento `claude_code.user_prompt`. Si nada llega, ejecute `claude --debug` y verifique el registro de depuración para errores de exportación `[3P telemetry]`.

<h2 id="backend-considerations">
  Consideraciones de backend
</h2>

Tu elección de backends de métricas, registros y trazas determina los tipos de análisis que puedes realizar:

<h3 id="for-metrics">
  Para métricas
</h3>

* **Bases de datos de series temporales**: Cálculos de tasa, métricas agregadas
* **Almacenes columnares**: Consultas complejas, análisis de usuario único
* **Plataformas de observabilidad completas**: Consultas avanzadas, visualización, alertas

<h3 id="for-events/logs">
  Para eventos/registros
</h3>

* **Sistemas de agregación de registros**: Búsqueda de texto completo, análisis de registros
* **Almacenes columnares**: Análisis de eventos estructurados
* **Plataformas de observabilidad completas**: Correlación entre métricas y eventos

<h3 id="for-traces">
  Para trazas
</h3>

Elige un backend que admita almacenamiento de trazas distribuidas y correlación de spans:

* **Sistemas de trazas distribuidas**: Visualización de spans, cascadas de solicitudes, análisis de latencia
* **Plataformas de observabilidad completas**: Búsqueda de trazas y correlación con métricas y registros

Para organizaciones que requieren métricas de Usuarios Activos Diarios/Semanales/Mensuales (DAU/WAU/MAU), considera backends que admitan consultas de valores únicos eficientes.

<h2 id="service-information">
  Información del servicio
</h2>

Todas las métricas y eventos se exportan con los siguientes atributos de recurso:

* `service.name`: `claude-code` para sesiones de terminal, `claude-code-desktop` para sesiones iniciadas desde la pestaña Code en la [aplicación Claude Desktop](/docs/es/desktop)
* `service.version`: Versión actual de Claude Code, o la versión de la aplicación Desktop para sesiones de la pestaña Code
* `os.type`: Tipo de sistema operativo (por ejemplo, `linux`, `darwin`, `windows`)
* `os.version`: Cadena de versión del sistema operativo
* `host.arch`: Arquitectura del host (por ejemplo, `amd64`, `arm64`)
* `wsl.version`: Número de versión de WSL (solo presente cuando se ejecuta en Windows Subsystem for Linux)
* Nombre del Medidor: `com.anthropic.claude_code`

Si sus canalizaciones de recopilador o paneles filtran por `service.name = claude-code`, agregue `claude-code-desktop` al filtro para capturar también la telemetría de sesiones de la pestaña Code.

<h2 id="roi-measurement-resources">
  Recursos de medición de ROI
</h2>

Para una guía completa sobre cómo medir el retorno de inversión para Claude Code, incluyendo configuración de telemetría, análisis de costos, métricas de productividad e informes automatizados, consulta la [Guía de Medición de ROI de Claude Code](https://github.com/anthropics/claude-code-monitoring-guide). Este repositorio proporciona configuraciones de Docker Compose listas para usar, configuraciones de Prometheus y OpenTelemetry, y plantillas para generar informes de productividad integrados con herramientas como Linear.

<h2 id="security-and-privacy">
  Seguridad y privacidad
</h2>

* La exportación de OpenTelemetry a tu backend es opcional y requiere configuración explícita. Para la telemetría operativa separada de Anthropic y cómo deshabilitarla, consulta [Uso de datos](/docs/es/data-usage#telemetry-services)
* Los contenidos de archivos sin procesar y fragmentos de código no se incluyen en métricas o eventos. Las trazas de span son una ruta de datos separada: consulta la viñeta `OTEL_LOG_TOOL_CONTENT` a continuación
* Cuando está autenticado a través de OAuth, `user.email` se incluye en atributos de telemetría, enviados solo al punto final de OTel que configures, nunca a Anthropic. Si esto es una preocupación para tu organización, trabaja con tu backend de telemetría para filtrar o redactar este campo
* El contenido del mensaje del usuario no se recopila por defecto. Solo se registra la longitud del mensaje. Para incluir contenido del mensaje, establece `OTEL_LOG_USER_PROMPTS=1`. Bajo el rastreo beta detallado, esta variable llega más lejos que el texto del mensaje: también controla el [atributo de span `new_context`](#new-context-gates), que lleva resultados de herramientas en el span `claude_code.llm_request`
* El texto de respuesta del asistente no se recopila por defecto. Solo se registra la longitud de la respuesta. Para incluir texto de respuesta, establece `OTEL_LOG_ASSISTANT_RESPONSES=1`. Como todos los datos de OpenTelemetry de Claude Code, el texto de respuesta se envía solo al punto final de OTel que configures, nunca a Anthropic. Cuando esta variable no está establecida, `OTEL_LOG_USER_PROMPTS` se utiliza como alternativa, así que establece `OTEL_LOG_ASSISTANT_RESPONSES=0` si deseas contenido de mensaje sin contenido de respuesta
* Los argumentos de entrada de herramientas y parámetros no se registran por defecto. Para incluirlos, establece `OTEL_LOG_TOOL_DETAILS=1`. Para los servidores integrados de Claude Desktop, en sesiones que Claude Desktop posee, `tool_decision` y `tool_result` llevan el par `mcp_server_name`/`mcp_tool_name`, nombres creados por el host en lugar de contenido de argumentos, incluso con la bandera desactivada. La excepción requiere Claude Code v2.1.214 o posterior. Estos datos se envían solo al punto final de OTEL que configures, nunca a Anthropic. Los argumentos aún pueden contener valores sensibles, así que configura tu backend de telemetría para filtrar o redactar estos atributos según sea necesario. Cuando está habilitado:
  * Los eventos `tool_result` y `tool_decision` incluyen un atributo `tool_parameters` con comandos Bash, nombres de servidor MCP y herramienta, y nombres de skills. Los campos como `full_command` se emiten sin truncar
  * Los eventos `tool_result` además incluyen un atributo `tool_input` con rutas de archivo, URLs, patrones de búsqueda y otros argumentos. Los valores individuales superiores a 512 caracteres se truncan y el total está limitado a aproximadamente 4 K caracteres
  * Los eventos `user_prompt` incluyen el `command_name` verbatim para comandos personalizados, de plugin y MCP
  * Los spans de traza incluyen el mismo atributo `tool_input` e atributos derivados de entrada como `file_path`, con el mismo truncamiento que `tool_input`
* El contenido de herramientas no se registra en spans de trazas por defecto. Para incluirlo, establece `OTEL_LOG_TOOL_CONTENT=1`. El span `claude_code.tool` entonces lleva un [evento de span `tool.output`](#tool-output-span-event) con contenidos de archivo sin procesar y salida de comandos Bash, truncado en el límite de contenido (60 KB por defecto) por atributo. El contenido de herramientas también llega a los spans a través de [`new_context`, cuya puerta difiere por span](#new-context-gates). Configura tu backend de telemetría para filtrar o redactar estos atributos según sea necesario
* Los cuerpos de solicitud y respuesta de la API de Mensajes de Anthropic sin procesar no se registran por defecto. Para incluirlos, establece `OTEL_LOG_RAW_API_BODIES` en tu shell, configuración de usuario o configuración administrada. Se ignora en [configuración de proyecto y local](/docs/es/settings-reference#variables-claude-code-ignores-in-env). Los cuerpos contienen el historial de conversación completo, incluido el mensaje del sistema, cada turno anterior de usuario y asistente, y resultados de herramientas, así que habilitar esto implica consentimiento a todo lo que las otras banderas de contenido `OTEL_LOG_*` revelarían. Claude Code siempre redacta el contenido de pensamiento extendido de Claude de estos cuerpos, independientemente de otras configuraciones. El valor que estableces determina cómo Claude Code entrega los cuerpos:
  * Con `=1`, Claude Code emite eventos de registro `api_request_body` y `api_response_body` para cada llamada de API. El atributo `body` de los eventos lleva la carga útil serializada en JSON, truncada en el límite de contenido (60 KB por defecto)
  * Con `=file:<dir>`, Claude Code escribe cuerpos sin truncar en archivos `.request.json` y `.response.json` bajo ese directorio, y los eventos llevan una ruta `body_ref` en su lugar del cuerpo en línea. Envía el directorio con un recopilador de registros o sidecar en lugar de a través del flujo de telemetría.

    Para cada respuesta exitosa, Claude Code también añade una línea a `index.jsonl` en ese directorio, vinculando el archivo de respuesta al archivo de solicitud que lo produjo y al mensaje de transcripción en el que se convirtió. Cada línea no contiene contenido de mensaje, y la sección [evento de cuerpo de respuesta de API](#api-response-body-event) enumera sus campos. El archivo de índice requiere Claude Code v2.1.274 o posterior

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Monitorear Claude Code en Amazon Bedrock
</h2>

Para orientación detallada sobre monitoreo de uso de Claude Code para Amazon Bedrock, consulta [Implementación de Monitoreo de Claude Code (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md).
