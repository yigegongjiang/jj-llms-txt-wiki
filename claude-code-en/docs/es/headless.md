> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ejecutar Claude Code mediante programación

> Utilice el Agent SDK para ejecutar Claude Code mediante programación desde la CLI, Python o TypeScript.

El [Agent SDK](/docs/es/agent-sdk/overview) le proporciona las mismas herramientas, bucle de agente y gestión de contexto que potencian Claude Code. Está disponible como CLI para scripts e CI/CD, o como paquetes de [Python](/docs/es/agent-sdk/python) y [TypeScript](/docs/es/agent-sdk/typescript) para control programático completo.

Para ejecutar Claude Code en modo no interactivo, pase `-p` con su indicación y cualquier [opción de CLI](/docs/es/cli-reference):

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

Esta página cubre el uso del Agent SDK a través de la CLI (`claude -p`). Para los paquetes SDK de Python y TypeScript con salidas estructuradas, devoluciones de llamada de aprobación de herramientas y objetos de mensaje nativos, consulte la [documentación completa del Agent SDK](/docs/es/agent-sdk/overview).

<h2 id="basic-usage">
  Uso básico
</h2>

Agregue la bandera `-p` (o `--print`) a cualquier comando `claude` para ejecutarlo de forma no interactiva. Todas las [opciones de CLI](/docs/es/cli-reference) funcionan con `-p`, incluyendo:

* `--continue` para [continuar conversaciones](#continue-conversations)
* `--allowedTools` para [aprobar herramientas automáticamente](#auto-approve-tools)
* `--output-format` para [obtener salida estructurada](#get-structured-output)

Este ejemplo le pregunta a Claude sobre su base de código e imprime la respuesta:

```bash theme={null}
claude -p "What does the auth module do?"
```

<h3 id="start-faster-with-bare-mode">
  Comenzar más rápido con modo bare
</h3>

Agregue `--bare` para reducir el tiempo de inicio omitiendo el descubrimiento automático de hooks, skills, plugins, servidores MCP, memoria automática y CLAUDE.md. Sin él, `claude -p` carga el mismo [contexto](/docs/es/how-claude-code-works#the-context-window) que una sesión interactiva, incluyendo cualquier cosa configurada en el directorio de trabajo o `~/.claude`.

El modo bare es útil para CI y scripts donde necesita el mismo resultado en cada máquina. Un hook en el `~/.claude` de un compañero de equipo o un servidor MCP en el `.mcp.json` del proyecto no se ejecutarán, porque el modo bare nunca los lee. Solo las banderas que pasa explícitamente tienen efecto.

Este ejemplo ejecuta una tarea de resumen única en modo bare y aprueba previamente la herramienta Read para que la llamada se complete sin una solicitud de permiso:

```bash theme={null}
claude --bare -p "Summarize this file" --allowedTools "Read"
```

En modo bare Claude tiene acceso a las herramientas Bash, lectura de archivos y edición de archivos. Pase cualquier contexto que necesite con una bandera:

| Para cargar                         | Utilice                                                 |
| ----------------------------------- | ------------------------------------------------------- |
| Adiciones de indicación del sistema | `--append-system-prompt`, `--append-system-prompt-file` |
| Configuración                       | `--settings <file-or-json>`                             |
| Servidores MCP                      | `--mcp-config <file-or-json>`                           |
| Agentes personalizados              | `--agents <json>`                                       |
| Un plugin                           | `--plugin-dir <path>`, `--plugin-url <url>`             |

El modo bare omite lecturas de OAuth y llavero. La autenticación de Anthropic debe provenir de `ANTHROPIC_API_KEY` o un `apiKeyHelper` en el JSON pasado a `--settings`. Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry utilizan sus credenciales de proveedor habituales.

<Note>
  `--bare` es el modo recomendado para llamadas con scripts y SDK, y se convertirá en el predeterminado para `-p` en una versión futura.
</Note>

<h3 id="background-tasks-at-exit">
  Tareas en segundo plano al salir
</h3>

Si Claude inicia una [tarea Bash en segundo plano](/docs/es/tools-reference#bash-tool-behavior) durante una ejecución de `claude -p`, por ejemplo un servidor de desarrollo o una compilación de vigilancia, esa tarea se termina aproximadamente cinco segundos después de que Claude haya devuelto su resultado final y stdin se haya cerrado. El período de gracia permite que una tarea que finaliza justo después del resultado aún entregue su salida. Antes de v2.1.163, un proceso en segundo plano que nunca se cerraba mantendría la invocación de `claude -p` abierta indefinidamente.

Los [subagentes](/docs/es/sub-agents) y flujos de trabajo en segundo plano están exentos del período de gracia de cinco segundos porque su resultado es parte de la salida final, por lo que `claude -p` espera a que se completen. A partir de v2.1.182, esa espera se limita a diez minutos de forma predeterminada para que un agente en segundo plano atascado no pueda mantener el proceso abierto indefinidamente. Ajuste el límite con [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/es/env-vars), o establézcalo en `0` para esperar sin límite.

<h2 id="examples">
  Ejemplos
</h2>

Estos ejemplos destacan patrones comunes de CLI. Para CI y otras llamadas con scripts, agregue [`--bare`](#start-faster-with-bare-mode) para que no recojan lo que esté configurado localmente.

<h3 id="pipe-data-through-claude">
  Canalizar datos a través de Claude
</h3>

El modo no interactivo lee stdin, por lo que puede canalizar datos y redirigir la respuesta como cualquier otra herramienta de línea de comandos.

Este ejemplo canaliza un registro de compilación a Claude y escribe la explicación en un archivo:

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

Con `--output-format json`, la carga útil de respuesta incluye `total_cost_usd` y un desglose de costos por modelo, por lo que los llamadores con scripts pueden rastrear el gasto por invocación sin consultar el [panel de uso](/docs/es/costs).

<Note>
  A partir de Claude Code v2.1.128, stdin canalizado está limitado a 10MB. Si excede el límite, Claude Code sale con un error claro y un estado distinto de cero. Para trabajar con entradas más grandes, escriba el contenido en un archivo y haga referencia a la ruta del archivo en su indicador en lugar de canalizarlo.
</Note>

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
  Utilice una herramienta como [jq](https://jqlang.github.io/jq/) para analizar la respuesta y extraer campos específicos:

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

La última línea de la transmisión es un mensaje `result` con el texto de respuesta final, el costo y los metadatos de la sesión. Antes de v2.1.208, canalizar una respuesta grande podría truncar la línea final y omitir el mensaje `result`.

El siguiente ejemplo utiliza [jq](https://jqlang.github.io/jq/) para filtrar deltas de texto y mostrar solo el texto transmitido. La bandera `-r` genera cadenas sin formato (sin comillas) y `-j` se une sin saltos de línea para que los tokens se transmitan continuamente:

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

Cuando una solicitud de API falla con un error reintentable, Claude Code emite un evento `system/api_retry` antes de reintentar. Puede usar esto para mostrar el progreso del reintento o implementar lógica de retroceso personalizada.

| Campo            | Tipo          | Descripción                                                                                                                                                                                               |
| ---------------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`    | tipo de mensaje                                                                                                                                                                                           |
| `subtype`        | `"api_retry"` | identifica esto como un evento de reintento                                                                                                                                                               |
| `attempt`        | entero        | número de intento actual, comenzando en 1                                                                                                                                                                 |
| `max_retries`    | entero        | reintentos totales permitidos                                                                                                                                                                             |
| `retry_delay_ms` | entero        | milisegundos hasta el siguiente intento                                                                                                                                                                   |
| `error_status`   | entero o nulo | código de estado HTTP, o `null` para errores de conexión sin respuesta HTTP                                                                                                                               |
| `error`          | cadena        | categoría de error: `authentication_failed`, `oauth_org_not_allowed`, `billing_error`, `rate_limit`, `overloaded`, `invalid_request`, `model_not_found`, `server_error`, `max_output_tokens`, o `unknown` |
| `uuid`           | cadena        | identificador único del evento                                                                                                                                                                            |
| `session_id`     | cadena        | sesión a la que pertenece el evento                                                                                                                                                                       |

El evento `system/init` informa metadatos de sesión incluyendo el modelo, herramientas, servidores MCP y plugins cargados. Es el primer evento en la transmisión a menos que eventos de inicio lo precedan:

* eventos `plugin_install`, cuando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/es/env-vars) está configurado.
* [eventos `hook_started`, `hook_progress` y `hook_response`](/docs/es/agent-sdk/typescript#sdkhookstartedmessage), mientras se ejecuta un hook [`SessionStart`](/docs/es/hooks#sessionstart) o [`Setup`](/docs/es/hooks#setup) configurado. Estos se transmiten a medida que el hook los produce. Claude Code v2.1.169 a v2.1.203 los entregó en un lote después de que el hook se completó, aún antes de `system/init`; v2.1.204 restauró la entrega en vivo.

El evento también lleva una matriz `capabilities` opcional de cadenas que nombran los comportamientos del protocolo que esta versión de Claude Code implementa, como `interrupt_receipt_v1`. Verifíquelo para detectar características en lugar de comparar cadenas de versión, e ignore valores que no reconozca. El campo requiere Claude Code v2.1.205 o posterior y está ausente en versiones anteriores. Consulte [`SDKSystemMessage`](/docs/es/agent-sdk/typescript#sdksystemmessage) para la lista de capacidades.

Use los campos de plugin para fallar CI cuando un plugin no se cargó:

| Campo           | Tipo   | Descripción                                                                                                                                                                                                                                                                                                             |
| --------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | matriz | plugins que se cargaron exitosamente, cada uno con `name` y `path`                                                                                                                                                                                                                                                      |
| `plugin_errors` | matriz | errores de tiempo de carga de plugin, cada uno con `plugin`, `type` y `message`. Incluye versiones de dependencia insatisfechas y fallos de carga de `--plugin-dir` como una ruta faltante o archivo inválido. Los plugins afectados se degradan y están ausentes de `plugins`. La clave se omite cuando no hay errores |

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

Para transmisión programática con devoluciones de llamada y objetos de mensaje, consulte [Transmitir respuestas en tiempo real](/docs/es/agent-sdk/streaming-output) en la documentación del Agent SDK.

<h3 id="auto-approve-tools">
  Aprobar herramientas automáticamente
</h3>

Utilice `--allowedTools` para permitir que Claude use ciertas herramientas sin solicitar confirmación. Este ejemplo ejecuta un conjunto de pruebas y corrige fallos, permitiendo que Claude ejecute comandos Bash y lea/edite archivos sin pedir permiso:

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

Para establecer una línea base para toda la sesión en lugar de enumerar herramientas individuales, pase un [modo de permiso](/docs/es/permission-modes). `dontAsk` deniega cualquier cosa que no esté en sus reglas `permissions.allow` o el [conjunto de comandos de solo lectura](/docs/es/permissions#read-only-commands), que es útil para ejecuciones de CI bloqueadas. `AskUserQuestion`, herramientas de conector [que su organización configuró para `ask`](/docs/es/mcp#organization-controls-on-connector-tools), y herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) se deniegan incluso cuando una regla de permiso coincide.

`acceptEdits` permite que Claude escriba archivos sin solicitar y también aprueba automáticamente comandos comunes del sistema de archivos como `mkdir`, `touch`, `mv` y `cp`. Otros comandos de shell y solicitudes de red aún necesitan una entrada `--allowedTools` o una regla `permissions.allow`, de lo contrario la ejecución se aborta cuando se intenta uno:

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="create-a-commit">
  Crear una confirmación
</h3>

Este ejemplo revisa los cambios preparados y crea una confirmación con un mensaje apropiado:

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

La bandera `--allowedTools` utiliza [sintaxis de regla de permiso](/docs/es/settings#permission-rule-syntax). El ` *` final habilita la coincidencia de prefijo, por lo que `Bash(git diff *)` permite cualquier comando que comience con `git diff`. El espacio antes de `*` es importante: sin él, `Bash(git diff*)` también coincidiría con `git diff-index`.

<Note>
  Las skills invocadas por el usuario y los comandos personalizados funcionan en modo `-p`: incluya `/skill-name` en la cadena de indicador y Claude Code lo expande antes de ejecutar. Los comandos integrados que solo se ejecutan en la interfaz de terminal, como `/login`, no están disponibles en modo `-p`. `/model`, `/effort`, `/fast`, `/color` y `/rename` aceptan el valor como argumento, por ejemplo `/model sonnet`, y `/mcp` sin argumento imprime un resumen de texto del estado del servidor; estas formas requieren Claude Code v2.1.205 o posterior y siguen las [notas de disponibilidad](/docs/es/commands#all-commands) de cada comando. Para cambiar una configuración desde una invocación `-p`, pase `key=value` a `/config`, por ejemplo `/config thinking=false`.
</Note>

<h3 id="customize-the-system-prompt">
  Personalizar el indicador del sistema
</h3>

Utilice `--append-system-prompt` para agregar instrucciones mientras mantiene el comportamiento predeterminado de Claude Code. Este ejemplo canaliza un diff de PR a Claude e le indica que revise las vulnerabilidades de seguridad:

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

Consulte [banderas de indicador del sistema](/docs/es/cli-reference#system-prompt-flags) para más opciones incluyendo `--system-prompt` para reemplazar completamente el indicador predeterminado.

<h3 id="continue-conversations">
  Continuar conversaciones
</h3>

Utilice `--continue` para continuar la conversación más reciente, o `--resume` con un ID de sesión para continuar una conversación específica. Este ejemplo ejecuta una revisión y luego envía indicaciones de seguimiento:

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

Ejecute ambos comandos desde el mismo directorio: la búsqueda de ID de sesión está limitada al directorio del proyecto actual y sus git worktrees. Consulte [Reanudar una sesión](/docs/es/sessions#resume-a-session) para las reglas de alcance completo.

<h2 id="next-steps">
  Próximos pasos
</h2>

* [Inicio rápido del Agent SDK](/docs/es/agent-sdk/quickstart): construya su primer agente con Python o TypeScript
* [Referencia de CLI](/docs/es/cli-reference): todas las banderas y opciones de CLI
* [GitHub Actions](/docs/es/github-actions): utilice el Agent SDK en flujos de trabajo de GitHub
* [GitLab CI/CD](/docs/es/gitlab-ci-cd): utilice el Agent SDK en canalizaciones de GitLab
