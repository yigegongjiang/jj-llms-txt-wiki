> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Observabilidad con OpenTelemetry

> Exporte trazas, métricas y eventos del Agent SDK a su backend de observabilidad usando OpenTelemetry.

Cuando ejecuta agentes en producción, necesita visibilidad sobre lo que hicieron:

* qué herramientas llamaron
* cuánto tiempo tardó cada solicitud de modelo
* cuántos tokens se gastaron
* dónde ocurrieron los fallos

El Agent SDK puede exportar estos datos como trazas, métricas y eventos de registro de OpenTelemetry a cualquier backend que acepte el Protocolo OpenTelemetry (OTLP), como Honeycomb, Datadog, Grafana, Langfuse o un recopilador autohospedado.

Esta guía explica cómo el SDK emite telemetría, cómo configurar la exportación y cómo etiquetar y filtrar los datos una vez que llegan a su backend. Para leer el uso de tokens y el costo directamente desde el flujo de respuesta del SDK en lugar de exportar a un backend, consulte [Rastrear costo y uso](/docs/es/agent-sdk/cost-tracking).

<h2 id="how-telemetry-flows-from-the-sdk">
  Cómo fluye la telemetría desde el SDK
</h2>

El Agent SDK ejecuta la CLI de Claude Code como un proceso secundario y se comunica con ella a través de una tubería local. La CLI tiene instrumentación OpenTelemetry integrada: registra tramos alrededor de cada solicitud de modelo y ejecución de herramienta, emite métricas para contadores de tokens y costos, y emite eventos de registro estructurados para indicaciones y resultados de herramientas. El SDK no produce telemetría propia. En su lugar, pasa la configuración al proceso CLI, y la CLI exporta directamente a su recopilador.

La configuración se pasa como variables de entorno. De forma predeterminada, el proceso secundario hereda el entorno de su aplicación, por lo que puede configurar la telemetría en uno de dos lugares:

* **Entorno del proceso:** establezca las variables en su shell, contenedor u orquestador antes de que su aplicación se inicie. Cada llamada a `query()` las recoge automáticamente sin cambios de código. Este es el enfoque recomendado para implementaciones de producción.
* **Opciones por llamada:** establezca las variables en `ClaudeAgentOptions.env` (Python) u `options.env` (TypeScript). Úselo cuando diferentes agentes en el mismo proceso necesiten diferentes configuraciones de telemetría. En Python, `env` se fusiona sobre el entorno heredado. En TypeScript, `env` reemplaza completamente el entorno heredado, así que incluya `...process.env` en el objeto que pasa.

La CLI exporta tres señales OpenTelemetry independientes. Cada una tiene su propio interruptor de habilitación y su propio exportador, por lo que puede activar solo las que necesita.

| Señal      | Qué contiene                                                                                            | Habilitar con                                                      |
| ---------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| Metrics    | Contadores para tokens, costo, sesiones, líneas de código y decisiones de herramientas                  | `OTEL_METRICS_EXPORTER`                                            |
| Log events | Registros estructurados para cada indicación, solicitud de API, error de API y resultado de herramienta | `OTEL_LOGS_EXPORTER`                                               |
| Traces     | Tramos para cada interacción, solicitud de modelo, llamada de herramienta y hook (beta)                 | `OTEL_TRACES_EXPORTER` más `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` |

Para la lista completa de nombres de métricas, nombres de eventos y atributos, consulte la referencia de Monitoreo de Claude Code [Monitoring](/docs/es/monitoring-usage). El Agent SDK emite los mismos datos porque ejecuta la misma CLI. Los nombres de tramos se enumeran en [Leer trazas de agentes](#read-agent-traces) a continuación.

<h2 id="enable-telemetry-export">
  Habilitar la exportación de telemetría
</h2>

La telemetría está desactivada hasta que establezca `CLAUDE_CODE_ENABLE_TELEMETRY=1` y elija al menos un exportador. La configuración más común envía las tres señales sobre OTLP HTTP a un recopilador.

El siguiente ejemplo establece las variables en un diccionario y las pasa a través de `options.env`. El agente ejecuta una única tarea, y la CLI exporta tramos, métricas y eventos al recopilador en `collector.example.com` mientras el bucle consume el flujo de respuesta:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions

  OTEL_ENV = {
      "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
      # Required for traces, which are in beta. Metrics and log events do not need this.
      "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA": "1",
      # Choose an exporter per signal. Use otlp for the SDK; see the Note below.
      "OTEL_TRACES_EXPORTER": "otlp",
      "OTEL_METRICS_EXPORTER": "otlp",
      "OTEL_LOGS_EXPORTER": "otlp",
      # Standard OTLP transport configuration.
      "OTEL_EXPORTER_OTLP_PROTOCOL": "http/protobuf",
      "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4318",
      "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-token",
  }


  async def main():
      options = ClaudeAgentOptions(env=OTEL_ENV)
      async for message in query(
          prompt="List the files in this directory", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const otelEnv = {
    CLAUDE_CODE_ENABLE_TELEMETRY: "1",
    // Required for traces, which are in beta. Metrics and log events do not need this.
    CLAUDE_CODE_ENHANCED_TELEMETRY_BETA: "1",
    // Choose an exporter per signal. Use otlp for the SDK; see the Note below.
    OTEL_TRACES_EXPORTER: "otlp",
    OTEL_METRICS_EXPORTER: "otlp",
    OTEL_LOGS_EXPORTER: "otlp",
    // Standard OTLP transport configuration.
    OTEL_EXPORTER_OTLP_PROTOCOL: "http/protobuf",
    OTEL_EXPORTER_OTLP_ENDPOINT: "http://collector.example.com:4318",
    OTEL_EXPORTER_OTLP_HEADERS: "Authorization=Bearer your-token",
  };

  for await (const message of query({
    prompt: "List the files in this directory",
    // env replaces the inherited environment in TypeScript, so spread
    // process.env first to keep PATH, ANTHROPIC_API_KEY, and other variables.
    options: { env: { ...process.env, ...otelEnv } },
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Debido a que el proceso secundario hereda el entorno de su aplicación de forma predeterminada, puede lograr el mismo resultado exportando estas variables en un Dockerfile, manifiesto de Kubernetes o perfil de shell y omitiendo `options.env` completamente.

<Note>
  El exportador `console` escribe telemetría en la salida estándar, que el SDK utiliza como su canal de mensajes. No establezca `console` como un valor de exportador cuando se ejecute a través del SDK. Para inspeccionar telemetría localmente, apunte `OTEL_EXPORTER_OTLP_ENDPOINT` a un recopilador local o un contenedor Jaeger todo en uno en su lugar.
</Note>

<h3 id="flush-telemetry-from-short-lived-calls">
  Vaciar telemetría de llamadas de corta duración
</h3>

La CLI agrupa telemetría y exporta en un intervalo. En una salida limpia del proceso, intenta vaciar datos pendientes, pero el vaciado está limitado por un tiempo de espera corto, por lo que los tramos aún pueden descartarse si el recopilador es lento para responder. Si su proceso se mata antes de que la CLI se apague, cualquier cosa aún en el búfer de lotes se pierde. Reducir los intervalos de exportación reduce ambas ventanas.

De forma predeterminada, las métricas se exportan cada 60 segundos y los trazos y registros se exportan cada 5 segundos. El siguiente ejemplo acorta los tres intervalos para que los datos lleguen al recopilador mientras una tarea corta aún se está ejecutando:

<CodeGroup>
  ```python Python theme={null}
  OTEL_ENV = {
      # ... exporter configuration from the previous example ...
      "OTEL_METRIC_EXPORT_INTERVAL": "1000",
      "OTEL_LOGS_EXPORT_INTERVAL": "1000",
      "OTEL_TRACES_EXPORT_INTERVAL": "1000",
  }
  ```

  ```typescript TypeScript theme={null}
  const otelEnv = {
    // ... exporter configuration from the previous example ...
    OTEL_METRIC_EXPORT_INTERVAL: "1000",
    OTEL_LOGS_EXPORT_INTERVAL: "1000",
    OTEL_TRACES_EXPORT_INTERVAL: "1000",
  };
  ```
</CodeGroup>

<h2 id="read-agent-traces">
  Leer trazas de agentes
</h2>

Los trazos le dan la vista más detallada de una ejecución de agente. Con `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` establecido, cada paso del bucle del agente se convierte en un tramo que puede inspeccionar en su backend de rastreo:

* **`claude_code.interaction`:** envuelve un único turno del bucle del agente, desde recibir una indicación hasta producir una respuesta.
* **`claude_code.llm_request`:** envuelve cada llamada a la API de Claude, con nombre de modelo, latencia y conteos de tokens como atributos.
* **`claude_code.tool`:** envuelve cada invocación de herramienta, con tramos secundarios para la espera de permiso (`claude_code.tool.blocked_on_user`) y la ejecución en sí (`claude_code.tool.execution`).
* **`claude_code.hook`:** envuelve cada ejecución de [hook](/docs/es/agent-sdk/hooks). Requiere rastreo beta detallado (`ENABLE_BETA_TRACING_DETAILED=1` y `BETA_TRACING_ENDPOINT`) además de las variables anteriores.

Los tramos `llm_request`, `tool` y `hook` son secundarios del tramo `claude_code.interaction` envolvente. Cuando el agente genera un subagente a través de la herramienta Task, los tramos `llm_request` y `tool` del subagente se anidan bajo el tramo `claude_code.tool` del agente padre, por lo que la cadena de delegación completa aparece como un único trazo.

Los tramos llevan un atributo `session.id` de forma predeterminada. Cuando realiza varias llamadas a `query()` contra la misma [sesión](/docs/es/agent-sdk/sessions), filtre en `session.id` en su backend para verlas como una línea de tiempo. El atributo se omite si `OTEL_METRICS_INCLUDE_SESSION_ID` se establece en un valor falso.

<Note>
  El rastreo está en beta. Los nombres de tramos y atributos pueden cambiar entre versiones. Consulte
  [Traces (beta)](/docs/es/monitoring-usage#traces-beta) en la referencia de Monitoreo
  para las variables de configuración del exportador de trazas.
</Note>

<h2 id="link-traces-to-your-application">
  Vincular trazas a su aplicación
</h2>

El SDK propaga automáticamente el contexto de trazo W3C en el subproceso CLI. Cuando llama a `query()` mientras un tramo OpenTelemetry está activo en su aplicación, el SDK inyecta `TRACEPARENT` y `TRACESTATE` en el entorno del proceso secundario, y la CLI los lee para que su tramo `claude_code.interaction` se convierta en un secundario de su tramo. La ejecución del agente aparece entonces dentro del trazo de su aplicación en lugar de como una raíz desconectada.

Cuando la propagación del contexto de trazo está habilitada, la CLI también reenvía `TRACEPARENT` a cada comando Bash y PowerShell que ejecuta. Si un comando lanzado a través de la herramienta Bash emite sus propios trazos OpenTelemetry, esos trazos se anidan bajo el tramo `claude_code.tool.execution` que envuelve el comando.

La inyección automática se omite cuando establece `TRACEPARENT` explícitamente en `options.env`, por lo que puede fijar un contexto padre específico si es necesario. Las sesiones interactivas de CLI ignoran completamente `TRACEPARENT` entrante; solo las ejecuciones de Agent SDK y `claude -p` lo honran. Consulte [Traces (beta)](/docs/es/monitoring-usage#traces-beta) en la referencia de Monitoreo para la referencia completa de tramos y atributos.

<h2 id="tag-telemetry-from-your-agent">
  Etiquetar telemetría de su agente
</h2>

De forma predeterminada, la CLI informa `service.name` como `claude-code`. Si ejecuta varios agentes o ejecuta el SDK junto con otros servicios que exportan al mismo recopilador, anule el nombre del servicio y agregue atributos de recursos para poder filtrar por agente en su backend.

El siguiente ejemplo cambia el nombre del servicio y adjunta metadatos de implementación. Estos valores se aplican como atributos de recursos OpenTelemetry en cada tramo, métrica y evento que emite el agente:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_SERVICE_NAME": "support-triage-agent",
          "OTEL_RESOURCE_ATTRIBUTES": "service.version=1.4.0,deployment.environment=production",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_SERVICE_NAME: "support-triage-agent",
      OTEL_RESOURCE_ATTRIBUTES":
        "service.version=1.4.0,deployment.environment=production",
    },
  };
  ```
</CodeGroup>

<h2 id="attribute-actions-to-your-end-users">
  Atribuir acciones a sus usuarios finales
</h2>

La CLI adjunta [atributos de identidad](/docs/es/monitoring-usage#standard-attributes) a cada evento basado en la credencial que utiliza para llamar a Anthropic. Cuando construye una aplicación que sirve a muchos usuarios finales desde una implementación, estos atributos identifican la credencial de su servicio, no el usuario final en cuyo nombre actuó el agente.

Para hacer que las llamadas de herramientas y la actividad de MCP sean atribuibles a los usuarios finales de su aplicación, inyecte la identidad del usuario final como atributos de recursos en cada llamada a `query()`. Codifique porcentualmente los valores antes de interpolarlos, ya que `OTEL_RESOURCE_ATTRIBUTES` [reserva comas, espacios y signos de igualdad](/docs/es/monitoring-usage#multi-team-organization-support). El siguiente ejemplo adjunta el usuario solicitante y el inquilino a cada tramo y evento de una solicitud. Asume un objeto `request` de su marco web que lleva los IDs de usuario e inquilino:

<CodeGroup>
  ```python Python theme={null}
  from urllib.parse import quote

  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_RESOURCE_ATTRIBUTES": f"enduser.id={quote(request.user_id)},tenant.id={quote(request.tenant_id)}",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_RESOURCE_ATTRIBUTES: `enduser.id=${encodeURIComponent(request.userId)},tenant.id=${encodeURIComponent(request.tenantId)}`,
    },
  };
  ```
</CodeGroup>

Con la identidad del usuario final adjunta, los eventos `tool_decision`, `tool_result`, `mcp_server_connection` y `permission_mode_changed`, que se exportan como registros de registro nombrados con un prefijo `claude_code.`, se convierten en un registro de auditoría por usuario que puede reenviar a una plataforma de Gestión de Información y Eventos de Seguridad (SIEM). Consulte [Auditar eventos de seguridad](/docs/es/monitoring-usage#audit-security-events) en la referencia de Monitoreo para la lista completa de eventos relevantes para la seguridad y los atributos que lleva cada uno.

<h2 id="control-sensitive-data-in-exports">
  Controlar datos sensibles en exportaciones
</h2>

La telemetría es estructural de forma predeterminada. Las duraciones, nombres de modelos y nombres de herramientas se registran en cada tramo; los conteos de tokens se registran cuando la solicitud de API subyacente devuelve datos de uso, por lo que los tramos para solicitudes fallidas o abortadas pueden omitirlos. El contenido que su agente lee y escribe no se registra de forma predeterminada. Estas variables de opción adicional agregan contenido a los datos exportados:

| Variable                  | Agrega                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `OTEL_LOG_USER_PROMPTS=1` | Texto de indicación en eventos `claude_code.user_prompt` y en el tramo `claude_code.interaction`                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `OTEL_LOG_TOOL_DETAILS=1` | Argumentos de entrada de herramienta (rutas de archivo, comandos de shell, patrones de búsqueda) en eventos `claude_code.tool_result`                                                                                                                                                                                                                                                                                                                                                                                                   |
| `OTEL_LOG_TOOL_CONTENT=1` | Cuerpos de entrada y salida de herramienta completos como eventos de tramo en `claude_code.tool`, truncados en 60 KB. Requiere que el [rastreo](#read-agent-traces) esté habilitado                                                                                                                                                                                                                                                                                                                                                     |
| `OTEL_LOG_RAW_API_BODIES` | Solicitud y respuesta JSON completas de la API de Mensajes de Anthropic como eventos de registro `claude_code.api_request_body` y `claude_code.api_response_body`. Establezca en `1` para cuerpos en línea truncados en 60 KB, o `file:<dir>` para cuerpos sin truncar en disco con una ruta `body_ref` en el evento. Los cuerpos incluyen el historial de conversación completo y tienen contenido de pensamiento extendido redactado. Habilitar esto implica consentimiento para todo lo que las tres variables anteriores revelarían |

Deje estos sin establecer a menos que su canalización de observabilidad esté aprobada para almacenar los datos que maneja su agente. Consulte [Seguridad y privacidad](/docs/es/monitoring-usage#security-and-privacy) en la referencia de Monitoreo para la lista completa de atributos y comportamiento de redacción.

<h2 id="related-documentation">
  Documentación relacionada
</h2>

Estas guías cubren temas adyacentes para monitoreo e implementación de agentes:

* [Rastrear costo y uso](/docs/es/agent-sdk/cost-tracking): lea datos de tokens y costos del flujo de mensajes sin un backend externo.
* [Hosting del Agent SDK](/docs/es/agent-sdk/hosting): implemente agentes en contenedores donde puede establecer variables OpenTelemetry a nivel de entorno.
* [Monitoring](/docs/es/monitoring-usage): la referencia completa para cada variable de entorno, métrica y evento que emite la CLI.
