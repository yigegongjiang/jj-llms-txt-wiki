> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ejecutar Claude Code mediante programación

> Utilice el Agent SDK para ejecutar Claude Code mediante programación desde la CLI, Python o TypeScript.

El [Agent SDK](/docs/es/agent-sdk/overview) le proporciona las mismas herramientas, bucle de agente y gestión de contexto que potencian Claude Code. Está disponible como CLI para scripts e CI/CD, o como paquetes de [Python](/docs/es/agent-sdk/python) y [TypeScript](/docs/es/agent-sdk/typescript) para control programático completo.

Para ejecutar Claude Code en modo no interactivo, pase `-p` con su indicación y las [opciones de CLI](/docs/es/cli-reference) que necesite:

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Esta página cubre el uso del Agent SDK a través de la CLI (`claude -p`). Para los paquetes SDK de Python y TypeScript con salidas estructuradas, devoluciones de llamada de aprobación de herramientas y objetos de mensaje nativos, consulte la [documentación completa del Agent SDK](/docs/es/agent-sdk/overview).

<h2 id="basic-usage">
  Uso básico
</h2>

Agregue la bandera `-p` (o `--print`) a cualquier comando `claude` para ejecutarlo de forma no interactiva. No todas las [opciones de CLI](/docs/es/cli-reference) se combinan con `-p`. Claude Code rechaza `--bg`, y rechaza `--cloud` con una descripción de tarea, con un error que nombra el conflicto; `--cloud` con un ID de sesión y `-p` en su lugar [pone en cola un mensaje en esa sesión en la nube](/docs/es/claude-code-on-the-web#send-follow-ups-from-the-cli) y sale. Las opciones que combinará con `-p` a menudo incluyen:

* `--continue` para [continuar conversaciones](#continue-conversations)
* `--allowedTools` para [aprobar herramientas automáticamente](#auto-approve-tools)
* `--output-format` para [obtener salida estructurada](#get-structured-output)

Este ejemplo le pregunta a Claude sobre su base de código e imprime la respuesta:

```bash theme={null}
claude -p "What does the auth module do?"
```

Claude Code sale con código 0 en caso de éxito y con un código distinto de cero cuando la ejecución falla, por lo que sus scripts pueden ramificarse según el estado de salida. Si pasa una bandera inválida, Claude Code reporta el error a stderr antes de que comience la ejecución. Cuando ocurre una falla dentro de la ejecución, como autenticación faltante, Claude Code imprime la falla como resultado en stdout.

<h3 id="start-faster-with-bare-mode">
  Comenzar más rápido con modo bare
</h3>

Agregue `--bare` para reducir el tiempo de inicio omitiendo el descubrimiento automático de hooks, skills, comandos personalizados, [subagentes](/docs/es/sub-agents), plugins instalados, servidores MCP, memoria automática y CLAUDE.md. Sin él, `claude -p` carga el mismo [contexto](/docs/es/how-claude-code-works#the-context-window) que una sesión interactiva, incluyendo cualquier cosa configurada en el directorio de trabajo o `~/.claude`.

El modo bare es útil para CI y scripts donde necesita el mismo resultado en cada máquina. Un hook en el `~/.claude` de un compañero de equipo o un servidor MCP en el `.mcp.json` del proyecto no se ejecutarán, porque el modo bare nunca los lee. Un directorio que nombre con `--add-dir` es una excepción parcial: el modo bare carga skills de su carpeta `.claude/skills/`, pero aún omite sus carpetas `.claude/commands/` y `.claude/agents/`. [Skills de directorios adicionales](/docs/es/skills#skills-from-additional-directories) cubre qué se carga y qué no.

Sin `--bare`, una sesión `-p` ejecuta los hooks en el `.claude/settings.json` de un proyecto y conecta los servidores en su `.mcp.json`, incluso en una carpeta que nunca ha confiado. Una sesión `-p` no muestra diálogo de confianza del espacio de trabajo ni solicitud de aprobación por servidor. [Lo que se ejecuta antes de confiar en una carpeta](/docs/es/permissions#what-runs-before-you-trust-a-folder) cubre cada tipo de contenido del repositorio bajo `-p` y cómo mantenerlo fuera.

Este ejemplo ejecuta una tarea de resumen única en modo bare y aprueba previamente la herramienta Read para que la llamada se complete sin una solicitud de permiso. Establezca `ANTHROPIC_API_KEY` antes de ejecutarlo, porque el modo bare no usa su inicio de sesión de suscripción:

```bash theme={null}
claude --bare -p "Summarize README.md" --allowedTools "Read"
```

En modo bare, Claude Code nunca lee credenciales OAuth ni el llavero del sistema. Para la API de Anthropic, establezca `ANTHROPIC_API_KEY` en el entorno, con una clave creada en la [Consola de Claude](https://platform.claude.com), o proporcione un `apiKeyHelper` en el JSON de `--settings`. Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry continúan leyendo sus credenciales de proveedor habituales como de costumbre.

En modo bare Claude tiene acceso a las herramientas Bash, lectura de archivos y edición de archivos. Pase cualquier contexto que necesite con una bandera:

| Para cargar                         | Utilice                                                 |
| ----------------------------------- | ------------------------------------------------------- |
| Adiciones de indicación del sistema | `--append-system-prompt`, `--append-system-prompt-file` |
| Configuración                       | `--settings <file-or-json>`                             |
| Servidores MCP                      | `--mcp-config <file-or-json>`                           |
| Agentes personalizados              | `--agents <json>`                                       |
| Un plugin                           | `--plugin-dir <path>`, `--plugin-url <url>`             |

<Note>
  `--bare` es el modo recomendado para llamadas con scripts y SDK, y se convertirá en el predeterminado para `-p` en una versión futura.
</Note>

<h3 id="background-tasks-at-exit">
  Tareas en segundo plano al salir
</h3>

Si Claude inicia una [tarea Bash en segundo plano](/docs/es/tools-reference#bash-tool-behavior) durante una ejecución de `claude -p`, por ejemplo un servidor de desarrollo o una compilación de vigilancia, ese shell se termina aproximadamente cinco segundos después de que Claude haya devuelto su resultado final y stdin se haya cerrado. El período de gracia permite que una tarea que finaliza justo después del resultado aún entregue su salida.

Si Claude inicia un [subagente](/docs/es/sub-agents) en segundo plano o flujo de trabajo, `claude -p` en su lugar permanece abierto hasta que ese trabajo se complete, porque su resultado es parte de la salida final.

De forma predeterminada, la espera termina después de 10 minutos de espera inactiva continua, por lo que un subagente o flujo de trabajo atascado no puede mantener el proceso abierto indefinidamente. En ese punto, Claude Code detiene lo que aún se está ejecutando y descarta su resultado parcial. Para cambiar el límite, establezca [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/es/env-vars), o establézcalo en `0` para esperar sin uno.

Si Claude inicia una vigilancia de [Monitor](/docs/es/tools-reference#monitor-tool) durante una ejecución de `claude -p`, Claude Code espera la vigilancia hasta que se agote el tiempo de espera o el límite de diez minutos termine la espera, lo que ocurra primero. Mientras espera, Claude continúa respondiendo a lo que la vigilancia reporta. De forma predeterminada, una vigilancia se agota cinco minutos después de que Claude la inicia.

<h3 id="stop-a-run-with-sigterm">
  Detener una ejecución con SIGTERM
</h3>

Si detiene una ejecución de `claude -p` con SIGTERM, por ejemplo con `kill` o desde un supervisor de procesos, Claude Code sale con código 143. Claude Code deja el turno que estaba en progreso sin terminar y no registra ningún resultado para él. Para terminar el turno en su lugar, envíe SIGINT, o llame a `interrupt()` del Agent SDK, antes de detener el proceso.

En SIGTERM, Claude Code termina el árbol de procesos de cualquier comando Bash que aún se esté ejecutando. Claude Code luego ejecuta los hooks [`SessionEnd`](/docs/es/hooks#sessionend) y sale. Al salir, Claude Code no inicia ninguna nueva llamada de herramienta, no envía ninguna nueva solicitud de modelo y no ejecuta ningún hook que no sea `SessionEnd`. Si la ejecución estaba en medio de un comando o esperando una respuesta a una solicitud de permiso cuando llegó la señal, Claude Code maneja ese paso de la siguiente manera:

* **Ejecutando un comando**: Claude Code registra el comando como eliminado en la sesión.
* **Esperando una respuesta a una solicitud de permiso**: si envía SIGTERM al proceso, Claude Code deja la solicitud sin responder. Si su programa cierra la sesión a través del Agent SDK, el SDK termina la entrada de Claude Code antes de enviar cualquier señal, y Claude Code cancela la solicitud tan pronto como la entrada termina.

Cuando [reanuda la sesión](#continue-conversations), Claude Code continúa el turno que SIGTERM dejó sin terminar.

<h2 id="examples">
  Ejemplos
</h2>

Estos ejemplos destacan patrones comunes de CLI. Donde un comando nombra un archivo como `auth.py` o `build-error.txt`, sustituya un archivo de su propio proyecto. En CI u otros entornos con scripts, agregue [`--bare`](#start-faster-with-bare-mode) para que Claude Code se inicie sin cargar los hooks del host, plugins, memoria automática o `CLAUDE.md`.

<h3 id="pipe-data-through-claude">
  Canalizar datos a través de Claude
</h3>

El modo no interactivo lee stdin, por lo que puede canalizar datos y redirigir la respuesta como cualquier otra herramienta de línea de comandos.

Este ejemplo canaliza un registro de compilación a Claude y escribe la explicación en un archivo:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Con `--output-format json`, la carga útil de respuesta incluye `total_cost_usd` y un desglose de costos por modelo, por lo que los llamadores con scripts pueden rastrear el gasto sin consultar el [panel de uso](/docs/es/costs). Cuando continúa una conversación anterior con `--continue` o `--resume`, la ejecución informa el total completo de la conversación, [gastos de ejecuciones anteriores incluidos](/docs/es/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Ambas cifras son [estimaciones del lado del cliente](/docs/es/agent-sdk/cost-tracking) y pueden diferir de su factura real.

<Note>
  El stdin canalizado está limitado a 10MB. Si excede el límite, Claude Code sale con un error claro y un estado distinto de cero. Para trabajar con entradas más grandes, escriba el contenido en un archivo y haga referencia a la ruta del archivo en su indicador en lugar de canalizarlo.
</Note>

Si Claude Code no puede leer stdin, por ejemplo porque el proceso que lo inició desconectó su extremo, Claude Code imprime una advertencia en stderr y continúa con el indicador de la línea de comandos. Antes de v2.1.211, un stdin no legible en Windows bloqueaba la sesión o hacía que saliera silenciosamente sin salida.

<h3 id="add-claude-to-a-build-script">
  Agregar Claude a un script de compilación
</h3>

Puede envolver una llamada no interactiva en un script para usar Claude como un linter o revisor específico del proyecto.

Este script `package.json` canaliza el diff contra `main` a Claude y le pide que informe sobre errores tipográficos. Canalizar el diff significa que Claude no necesita permiso de Bash para leerlo, y las comillas dobles escapadas mantienen el script portátil a Windows:

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

Ejecute con `npm run lint:claude`.

<h3 id="get-structured-output">
  Obtener salida estructurada
</h3>

Utilice `--output-format` para controlar cómo se devuelven las respuestas:

* `text` (predeterminado): salida de texto sin formato
* `json`: JSON estructurado con resultado, ID de sesión y metadatos
* `stream-json`: JSON delimitado por saltos de línea para transmisión en tiempo real

Este ejemplo devuelve un resumen del proyecto como JSON con metadatos de sesión, con el resultado de texto en el campo `result`:

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

Para obtener una salida que se ajuste a un esquema específico, utilice `--output-format json` con `--json-schema` y una definición de [JSON Schema](https://json-schema.org/). La respuesta incluye metadatos sobre la solicitud (ID de sesión, uso, etc.) con la salida estructurada en el campo `structured_output`.

Este ejemplo extrae nombres de funciones y los devuelve como una matriz de cadenas:

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

Si el valor no es un JSON Schema válido, `claude` sale con `Error: --json-schema is not a valid JSON Schema` seguido del diagnóstico del validador. Claude Code acepta esquemas que utilizan la palabra clave `format`, como `"format": "email"`, pero trata `format` como una anotación y no la aplica. Antes de v2.1.205, Claude Code ignoraba silenciosamente un esquema inválido y devolvía texto no estructurado, y trataba cualquier esquema que contenía `format` como inválido.

<Tip>
  Utilice una herramienta como [jq](https://jqlang.org/) para analizar la respuesta y extraer campos específicos:

  ```bash theme={null}
  # Extract the text result
  claude -p "Summarize this project" --output-format json | jq -r '.result'

  # Extract structured output
  claude -p "Extract function names from auth.py" \
    --output-format json \
    --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
    | jq '.structured_output'
  ```
</Tip>

<h3 id="stream-responses">
  Transmitir respuestas
</h3>

Utilice `--output-format stream-json` con `--verbose` e `--include-partial-messages` para recibir tokens a medida que se generan. Cada línea es un objeto JSON que representa un evento:

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

La última línea de la transmisión es un mensaje `result` con el texto de respuesta final, el costo y los metadatos de la sesión.

Si su consumidor lee la transmisión lentamente, Claude Code espera a que se drene la salida en cola antes de salir, escalando la espera con cuánto aún está en cola, limitado a 30 segundos. Antes de v2.1.214, la espera de salida estaba limitada a aproximadamente dos segundos, lo que podría cortar el final de una respuesta grande.

El siguiente ejemplo utiliza [jq](https://jqlang.org/) para filtrar deltas de texto y mostrar solo el texto transmitido. La bandera `-r` genera cadenas sin formato (sin comillas) y `-j` se une sin saltos de línea para que los tokens se transmitan continuamente:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Para transmisión programática con devoluciones de llamada y objetos de mensaje, consulte [Transmitir respuestas en tiempo real](/docs/es/agent-sdk/streaming-output) en la documentación del Agent SDK.

<h4 id="follow-subagent-messages">
  Seguir mensajes de subagentes
</h4>

Los mensajes de [subagentes](/docs/es/sub-agents) aparecen en la transmisión como mensajes `assistant` y `user` cuyo campo `parent_tool_use_id` es el ID de la llamada de herramienta que generó el subagente. Los mensajes de la conversación principal llevan `null` en ese campo.

El primer mensaje de un subagente que se ejecuta en [primer plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background) es un mensaje `user` que lleva el indicador que lo impulsa. Después de ese primer mensaje, Claude Code emite:

* **De forma predeterminada**: los bloques `tool_use` y `tool_result` del subagente.
* **Con [`--forward-subagent-text`](/docs/es/cli-reference#cli-flags) o [`CLAUDE_CODE_FORWARD_SUBAGENT_TEXT`](/docs/es/env-vars)**: también los bloques de texto y pensamiento del subagente, para que pueda reconstruir la transcripción de cada subagente. Esto requiere Claude Code v2.1.211 o posterior.

Cuando habilita cualquiera de las opciones, Claude Code reenvía mensajes de [subagentes en cada profundidad de anidamiento](/docs/es/sub-agents#let-subagents-spawn-their-own-subagents), ya sea que cada uno fue generado con la herramienta Agent o iniciado como una [skill bifurcada](/docs/es/skills#run-skills-in-a-subagent). Los mensajes de subagentes que una skill bifurcada genera, y de skills bifurcadas iniciadas dentro de un subagente u otra skill bifurcada, requieren Claude Code v2.1.275 o posterior. En `parent_tool_use_id`, los mensajes del subagente anidado llevan el ID de la llamada de herramienta Agent o Skill que lo inició, para que pueda reconstruir el árbol de anidamiento completo siguiendo esos IDs. Antes de v2.1.219, los mensajes de subagentes anidados no aparecían en la transmisión.

Las skills que [se ejecutan en un subagente](/docs/es/skills#run-skills-in-a-subagent) aparecen en la transmisión de la misma manera: el primer mensaje de la skill bifurcada es un mensaje `user` que lleva el contenido de la skill que impulsa la ejecución. Si habilita cualquiera de las opciones, la transmisión también lleva los bloques de texto y pensamiento de la skill bifurcada. Antes de v2.1.265, solo los bloques `tool_use` y `tool_result` de una skill bifurcada aparecían en la transmisión.

<h4 id="handle-api-retries">
  Manejar reintentos de API
</h4>

Cuando una solicitud de API falla con un error reintentable, Claude Code emite un evento `system/api_retry` antes de reintentar. En v2.1.246 o posterior, cuando un `401` o `403` rechaza una credencial [`apiKeyHelper`](/docs/es/settings-reference#apikeyhelper), Claude Code realiza los primeros dos reintentos silenciosamente sin evento, luego emite el evento como de costumbre desde el tercer reintento consecutivo en adelante. Los reintentos silenciosos aún cuentan hacia `attempt`. Puede usar el evento para mostrar el progreso del reintento en su propia interfaz.

| Campo            | Tipo             | Descripción                                                                                                                                                                                                                                                                                                                                                                                               |
| ---------------- | ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`       | tipo de mensaje                                                                                                                                                                                                                                                                                                                                                                                           |
| `subtype`        | `"api_retry"`    | identifica esto como un evento de reintento                                                                                                                                                                                                                                                                                                                                                               |
| `attempt`        | entero           | número de intento actual, comenzando en 1                                                                                                                                                                                                                                                                                                                                                                 |
| `max_retries`    | entero           | reintentos totales permitidos para la causa de este fallo, que pueden ser menos que el presupuesto de toda la sesión                                                                                                                                                                                                                                                                                      |
| `retry_delay_ms` | entero           | milisegundos hasta el siguiente intento                                                                                                                                                                                                                                                                                                                                                                   |
| `error_status`   | entero o nulo    | código de estado HTTP del intento fallido, o `null` cuando el intento no obtuvo respuesta HTTP de la API                                                                                                                                                                                                                                                                                                  |
| `no_response`    | objeto, opcional | presente solo cuando el intento fallido no obtuvo [encabezados de respuesta a tiempo](/docs/es/errors#no-response-from-api). `waited_ms` es cuánto tiempo esperó ese intento y `retry_wait_ms` es cuánto tiempo esperará el reintento. En estos eventos, `max_retries` refleja el reintento que normalmente obtiene esta causa, no el presupuesto de toda la sesión. Requiere Claude Code v2.1.261 o posterior |
| `error`          | cadena           | categoría de error: `authentication_failed`, `oauth_org_not_allowed`, `account_on_hold`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, `cloud_credential_error`, o `unknown`                                                                                                                                                    |
| `uuid`           | cadena           | identificador único del evento                                                                                                                                                                                                                                                                                                                                                                            |
| `session_id`     | cadena           | sesión a la que pertenece el evento                                                                                                                                                                                                                                                                                                                                                                       |

<h4 id="read-session-metadata">
  Leer metadatos de sesión
</h4>

El evento `system/init` informa metadatos de sesión incluyendo el modelo, herramientas, servidores MCP y plugins cargados. Es el primer evento en la transmisión a menos que eventos de inicio lo precedan:

* eventos `plugin_install`, cuando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/es/env-vars) está configurado.
* [eventos `hook_started`, `hook_progress` y `hook_response`](/docs/es/agent-sdk/typescript#sdkhookstartedmessage), mientras se ejecuta un hook [`SessionStart`](/docs/es/hooks#sessionstart) o [`Setup`](/docs/es/hooks#setup) configurado. Estos se transmiten a medida que el hook los produce. Claude Code v2.1.169 a v2.1.203 los entregó en un lote después de que el hook se completó, aún antes de `system/init`; v2.1.204 restauró la entrega en vivo.

El evento también lleva una matriz `capabilities` opcional de cadenas que nombran los comportamientos del protocolo que esta versión de Claude Code implementa, como `interrupt_receipt_v1` o `interrupt_cancel_queued_v1`. Verifíquelo para detectar características en lugar de comparar cadenas de versión, e ignore valores que no reconozca. El campo requiere Claude Code v2.1.205 o posterior y está ausente en versiones anteriores. Consulte [`SDKSystemMessage`](/docs/es/agent-sdk/typescript#sdksystemmessage) para la lista de capacidades.

<h4 id="fail-ci-when-a-plugin-or-mcp-server-doesn’t-load">
  Fallar CI cuando un plugin o servidor MCP no se carga
</h4>

Utilice los campos de plugin en el evento `system/init` para detectar un plugin que no se cargó:

| Campo           | Tipo   | Descripción                                                                                                                                                                                                                                                                                                             |
| --------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | matriz | plugins que se cargaron exitosamente, cada uno con `name` y `path`                                                                                                                                                                                                                                                      |
| `plugin_errors` | matriz | errores de tiempo de carga de plugin, cada uno con `plugin`, `type` y `message`. Incluye versiones de dependencia insatisfechas y fallos de carga de `--plugin-dir` como una ruta faltante o archivo inválido. Los plugins afectados se degradan y están ausentes de `plugins`. La clave se omite cuando no hay errores |

Utilice los campos del servidor MCP de la misma manera. Cuando pasa [`--mcp-config`](/docs/es/cli-reference#cli-flags) con `-p`, Claude Code espera a que los servidores aún pendientes se completen antes de ejecutar el primer turno, hasta el tiempo de espera de inicio [`MCP_TIMEOUT`](/docs/es/env-vars), 30 segundos de forma predeterminada. Un servidor remoto con una [lista de herramientas en caché](/docs/es/agent-sdk/mcp#connection-timing) omite la espera, muestra `pending` en `system/init` y se conecta en su primera llamada de herramienta. La espera requiere Claude Code v2.1.221 o posterior.

Claude Code valida cada entrada `--mcp-config` al inicio y omite las entradas que fallan la validación, por ejemplo una entrada `url` sin `type`. La ejecución continúa y sale limpiamente, por lo que verifique estos campos para detectar un servidor que nunca se cargó:

| Campo               | Tipo   | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_servers`       | matriz | servidores MCP en la sesión, cada uno con `name` y `status`                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `mcp_server_errors` | matriz | entradas `--mcp-config` omitidas por validación de configuración, cada una con `name`, `type` y `message`. `type` es una categoría de omisión como `unknown_type`, `url_missing_type`, `invalid_config` o `reserved_name`; trate valores que no reconozca como una omisión genérica. Los servidores afectados están ausentes de `mcp_servers`. La clave se omite cuando no hay errores, por lo que una puerta de CI puede fallar en una matriz no vacía. Requiere Claude Code v2.1.219 o posterior |

Cuando ejecuta el comando a mano en una terminal, Claude Code también imprime una advertencia de inicio en stderr, como `Warning: 1 MCP server skipped due to invalid config:`, seguida de la razón para cada entrada omitida. Cuando redirige stderr, o cuando un programa como un ejecutor de CI o un host SDK lo captura, Claude Code no imprime advertencia e informa las entradas omitidas solo en el campo `mcp_server_errors`. La advertencia requiere Claude Code v2.1.219 o posterior.

<h4 id="track-plugin-installs">
  Rastrear instalaciones de plugins
</h4>

Cuando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/es/env-vars) está configurado, Claude Code emite eventos `system/plugin_install` mientras los plugins del marketplace se instalan antes del primer turno. Use estos para mostrar el progreso de instalación en su propia interfaz de usuario.

| Campo        | Tipo                                                    | Descripción                                                                                                    |
| ------------ | ------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `type`       | `"system"`                                              | tipo de mensaje                                                                                                |
| `subtype`    | `"plugin_install"`                                      | identifica esto como un evento de instalación de plugin                                                        |
| `status`     | `"started"`, `"installed"`, `"failed"`, o `"completed"` | `started` y `completed` enmarcan la instalación general; `installed` y `failed` reportan mercados individuales |
| `name`       | cadena, opcional                                        | nombre del marketplace, presente en `installed` y `failed`                                                     |
| `error`      | cadena, opcional                                        | mensaje de fallo, presente en `failed`                                                                         |
| `uuid`       | cadena                                                  | identificador único del evento                                                                                 |
| `session_id` | cadena                                                  | sesión a la que pertenece el evento                                                                            |

<h3 id="auto-approve-tools">
  Aprobar herramientas automáticamente
</h3>

Utilice `--allowedTools` para permitir que Claude use ciertas herramientas sin solicitar confirmación. Este ejemplo ejecuta un conjunto de pruebas y corrige fallos, permitiendo que Claude ejecute comandos Bash y lea/edite archivos sin pedir permiso:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Para establecer una línea base para toda la sesión en lugar de enumerar herramientas individuales, pase un [modo de permiso](/docs/es/permission-modes). Para `-p`, el [modo de permiso de inicio integrado](/docs/es/permission-modes#which-mode-a-session-starts-in) es Manual en cada plan, por lo que pase el modo de permiso que desee:

* **`auto`**: pase `--permission-mode auto` para que un clasificador revise la mayoría de las acciones en lugar de usted
* **`dontAsk`**: Claude Code deniega cualquier llamada que de otro modo solicitaría, lo que es útil para ejecuciones de CI bloqueadas. Las acciones que no necesitan aprobación en modo Manual aún se ejecutan, como lecturas de archivos en sus directorios de trabajo y el [conjunto de comandos de solo lectura](/docs/es/permissions#read-only-commands), y también lo hacen las acciones que sus entradas `--allowedTools` o reglas `permissions.allow` cubren. `AskUserQuestion`, herramientas de conector [que su organización configuró para `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) se deniegan incluso cuando una regla de permiso coincide
* **`acceptEdits`**: Claude escribe archivos sin solicitar, y Claude Code aprueba automáticamente comandos comunes del sistema de archivos como `mkdir`, `touch`, `mv` y `cp`. Las [acciones que ningún modo aprueba automáticamente](/docs/es/permission-modes#actions-no-mode-auto-approves) aún se aplican. Aparte del conjunto de comandos de solo lectura, otros comandos de shell y solicitudes de red aún necesitan una entrada `--allowedTools` o una regla `permissions.allow`. Consulte [qué `acceptEdits` aprueba automáticamente](/docs/es/permission-modes#auto-approve-file-edits-with-acceptedits-mode) para la lista completa

Este ejemplo aplica correcciones de lint con `acceptEdits` como línea base:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="turn-off-permission-prompts-in-unattended-runs">
  Desactivar indicadores de permiso en ejecuciones desatendidas
</h3>

Pase `--permission-prompts none` cuando nadie esté disponible para responder indicadores de permiso, por ejemplo en un trabajo programado. La bandera es más importante cuando su ejecución tiene un host de permiso: una aplicación Agent SDK con una devolución de llamada [`canUseTool`](/docs/es/agent-sdk/user-input), o una herramienta MCP que pasa con [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags). Sin la bandera, su ejecución espera a que ese host responda cada solicitud de permiso.

Con la bandera, su ejecución no consulta al host ni espera en él. Cualquier cosa que solicitaría se deniega a menos que un hook `PermissionRequest` lo permita, se le dice a Claude que nadie puede aprobar la solicitud y que no la reintente, y la ejecución continúa. En una ejecución `-p` sin host, estas solicitudes se deniegan de cualquier forma, y la bandera también le dice a Claude que no las reintente. Las reglas de permiso, [hooks `PermissionRequest`](/docs/es/hooks#permissionrequest), y el modo de permiso que establezca aún deciden cada llamada primero; Claude Code deniega solo las solicitudes que nada más resuelve.

Este ejemplo ejecuta una tarea desatendida en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode). El clasificador revisa cada acción como de costumbre, y Claude Code deniega cualquier cosa que habría recurrido a un indicador:

```bash theme={null}
claude -p "Update the dependency pins and run the tests" --permission-mode auto --permission-prompts none
```

Con `--permission-prompts none`, Claude Code elimina las herramientas que necesitan una respuesta de una persona, como [`AskUserQuestion`](/docs/es/tools-reference#askuserquestion-tool-behavior), por lo que Claude no puede llamarlas. Cualquier [solicitud de elicitación MCP](/docs/es/mcp#respond-to-mcp-elicitation-requests) que ningún hook [`Elicitation`](/docs/es/hooks#elicitation) responda se cancela.

Con `--output-format stream-json`, las denegaciones aparecen como mensajes del sistema `permission_denied`, y el mensaje de resultado final las enumera en `permission_denials`.

<Note>
  La bandera `--permission-prompts` requiere Claude Code v2.1.259 o posterior. Las versiones anteriores la rechazan con un error de opción desconocida.
</Note>

<h3 id="create-a-commit">
  Crear una confirmación
</h3>

Este ejemplo revisa los cambios preparados y crea una confirmación con un mensaje apropiado:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

La bandera `--allowedTools` utiliza [sintaxis de regla de permiso](/docs/es/settings-reference#permission-rule-syntax). El ` *` final habilita la coincidencia de prefijo, por lo que `Bash(git diff *)` permite cualquier comando que comience con `git diff`. El espacio antes de `*` es importante: sin él, `Bash(git diff*)` también coincidiría con `git diff-index`.

<Note>
  La compatibilidad de comandos difiere en modo `-p`:

  * Las [skills](/docs/es/skills) invocadas por el usuario y los comandos personalizados funcionan. Incluya `/skill-name` en la cadena de indicador y Claude Code lo expande antes de ejecutar.
  * Los comandos integrados que solo se ejecutan en la interfaz de terminal, como `/login`, no están disponibles.
  * `/model`, `/effort`, `/fast`, `/color` y `/rename` aceptan el valor como argumento, por ejemplo `/model sonnet`, y `/mcp` sin argumento imprime un resumen de texto del estado del servidor. Estas formas requieren Claude Code v2.1.205 o posterior y siguen las [notas de disponibilidad](/docs/es/commands#all-commands) de cada comando.
  * Para cambiar una configuración, pase `key=value` a `/config`, por ejemplo `/config thinking=false`.
  * `/output-style <style>` cambia [estilos de salida](/docs/es/output-styles) y `/output-style` solo los enumera. Requiere Claude Code v2.1.269 o posterior.
</Note>

<h3 id="customize-the-system-prompt">
  Personalizar el indicador del sistema
</h3>

Utilice `--append-system-prompt` para agregar instrucciones mientras mantiene el comportamiento predeterminado de Claude Code. Este ejemplo canaliza un diff de PR a Claude e le indica que revise las vulnerabilidades de seguridad. Guárdelo como un script de shell, por ejemplo `review.sh`:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

En el script, `"$1"` representa el primer argumento que pasa en la línea de comandos. Ejecute `bash review.sh 123` y el shell reemplaza `"$1"` con `123`, por lo que el script obtiene el diff para PR 123. Claude Code imprime la revisión como JSON, con el texto en el campo `result`.

Consulte [banderas de indicador del sistema](/docs/es/cli-reference#system-prompt-flags) para más opciones incluyendo `--system-prompt` para reemplazar completamente el indicador predeterminado.

<h3 id="continue-conversations">
  Continuar conversaciones
</h3>

Utilice `--continue` para continuar la conversación más reciente, o `--resume` con un ID de sesión para continuar una conversación específica. En Claude Code v2.1.257 o posterior, cuando pasa `--continue`, Claude Code abre una [sesión en segundo plano](/docs/es/sessions#resume-a-session) que ha terminado, pero no una que aún se está ejecutando. Este ejemplo ejecuta una revisión y luego envía indicaciones de seguimiento:

```bash theme={null}
# First request
claude -p "Review this codebase for performance issues"

# Continue the most recent conversation
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

Si está ejecutando múltiples conversaciones, capture el ID de sesión para reanudar una específica:

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

Puede ejecutar los dos comandos desde diferentes directorios: Claude Code [encuentra la sesión por su ID](/docs/es/sessions#resume-a-session) en cualquier proyecto en esta máquina. Antes de v2.1.223, Claude Code buscaba el ID solo en el directorio del proyecto actual y sus git worktrees, por lo que tenía que ejecutar ambos comandos desde el mismo directorio.

En lugar del ID de sesión, puede pasar a `--resume` la ruta absoluta al archivo de [transcripción](/docs/es/sessions#where-transcripts-are-stored) `.jsonl` de una sesión, y Claude Code continúa la conversación almacenada en ese archivo.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Inicio rápido del Agent SDK](/docs/es/agent-sdk/quickstart): construya su primer agente con Python o TypeScript
* [Referencia de CLI](/docs/es/cli-reference): todas las banderas y opciones de CLI
* [GitHub Actions](/docs/es/github-actions): utilice el Agent SDK en flujos de trabajo de GitHub
* [GitLab CI/CD](/docs/es/gitlab-ci-cd): utilice el Agent SDK en canalizaciones de GitLab
