> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia de errores

> Busque mensajes de error en tiempo de ejecución de Claude Code con lo que significa cada uno y cómo solucionarlo.

Esta página enumera los errores en tiempo de ejecución que Claude Code muestra y cómo recuperarse de cada uno, además de qué verificar cuando las respuestas parecen incorrectas sin un error. Para errores de instalación como `command not found` o fallos de TLS durante la configuración, consulte [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install).

Estos errores y comandos de recuperación se aplican en la CLI, la [aplicación de escritorio](/docs/es/desktop) y [Claude Code en la web](/docs/es/claude-code-on-the-web), ya que los tres envuelven la misma CLI de Claude Code. Para problemas específicos de la superficie, consulte la sección de solución de problemas en la página de esa superficie.

<Note>
  Claude Code llama a la API de Claude para obtener respuestas del modelo, por lo que la mayoría de los errores en tiempo de ejecución se asignan a un código de error de API subyacente. Esta página cubre lo que significa cada error dentro de Claude Code y cómo recuperarse. Para las definiciones de código de estado HTTP sin procesar, consulte la [referencia de errores de la plataforma Claude](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Encuentre su error
</h2>

Haga coincidir el mensaje que ve en su terminal con una sección a continuación.

| Mensaje                                                                                            | Sección                                                                                                                      |
| :------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `API Error: 500 Internal server error`                                                             | [Errores del servidor](#api-error-500-internal-server-error)                                                                 |
| `API Error: Repeated 529 Overloaded errors`                                                        | [Errores del servidor](#api-error-repeated-529-overloaded-errors)                                                            |
| `Request timed out`                                                                                | [Errores del servidor](#request-timed-out), o [Red](#unable-to-connect-to-api) si el mensaje menciona su conexión a Internet |
| `Server error mid-response. The response above may be incomplete.`                                 | [Errores del servidor](#the-response-above-may-be-incomplete)                                                                |
| `Connection closed mid-response` / `Response stalled mid-stream`                                   | [Errores del servidor](#the-response-above-may-be-incomplete)                                                                |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`               | [Errores del servidor](#auto-mode-cannot-determine-the-safety-of-an-action)                                                  |
| `Auto mode could not evaluate this action and is blocking it for safety`                           | [Errores del servidor](#auto-mode-cannot-determine-the-safety-of-an-action)                                                  |
| `Auto mode classifier transcript exceeded context window`                                          | [Errores del servidor](#auto-mode-cannot-determine-the-safety-of-an-action)                                                  |
| `Agent terminated early due to an API error`                                                       | [Errores del servidor](#agent-terminated-early-due-to-an-api-error)                                                          |
| `You've hit your session limit` / `You've hit your weekly limit`                                   | [Límites de uso](#youve-hit-your-session-limit)                                                                              |
| `Usage credits required for 1M context`                                                            | [Límites de uso](#usage-credits-required-for-1m-context)                                                                     |
| `Server is temporarily limiting requests`                                                          | [Límites de uso](#server-is-temporarily-limiting-requests)                                                                   |
| `Request rejected (429)`                                                                           | [Límites de uso](#request-rejected-429)                                                                                      |
| `Credit balance is too low`                                                                        | [Límites de uso](#credit-balance-is-too-low)                                                                                 |
| `Not logged in · Please run /login`                                                                | [Autenticación](#not-logged-in)                                                                                              |
| `Could not resolve authentication method`                                                          | [Autenticación](#could-not-resolve-authentication-method)                                                                    |
| `Invalid API key`                                                                                  | [Autenticación](#invalid-api-key)                                                                                            |
| `Your apiKeyHelper script is failing`                                                              | [Autenticación](#your-apikeyhelper-script-is-failing)                                                                        |
| `This organization has been disabled`                                                              | [Autenticación](#this-organization-has-been-disabled)                                                                        |
| `Your organization has disabled API key authentication`                                            | [Autenticación](#your-organization-has-disabled-api-key-authentication)                                                      |
| `Your organization has disabled Claude subscription access`                                        | [Autenticación](#your-organization-has-disabled-claude-subscription-access)                                                  |
| `Routines are disabled by your organization's policy`                                              | [Autenticación](#routines-are-disabled-by-your-organizations-policy)                                                         |
| `Remote Control is only available when using Claude via api.anthropic.com`                         | [Autenticación](#remote-control-requires-the-anthropic-api)                                                                  |
| `OAuth token revoked` / `OAuth token has expired`                                                  | [Autenticación](#oauth-token-revoked-or-expired)                                                                             |
| `Login expired · Please run /login`                                                                | [Autenticación](#login-expired)                                                                                              |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                         | [Autenticación](#login-expired)                                                                                              |
| `does not meet scope requirement user:profile`                                                     | [Autenticación](#oauth-scope-requirement)                                                                                    |
| `AWS credentials expired or invalid`                                                               | [Autenticación](#aws-credentials-expired-or-invalid)                                                                         |
| `AWS authentication failed`                                                                        | [Autenticación](#aws-authentication-failed)                                                                                  |
| `AWS default-chain credential resolve timed out`                                                   | [Autenticación](#aws-default-chain-credential-resolve-timed-out)                                                             |
| `Unable to connect to API`                                                                         | [Red](#unable-to-connect-to-api)                                                                                             |
| `Waiting for API response · will retry in`                                                         | [Reintentos automáticos](#automatic-retries), o [Red](#unable-to-connect-to-api) si persiste                                 |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"` | [Red](#bedrock-streaming-response-has-an-unexpected-content-type)                                                            |
| `SSL certificate verification failed`                                                              | [Red](#ssl-certificate-errors)                                                                                               |
| `SSL certificate error (...)` during login or startup                                              | [Red](#ssl-certificate-errors)                                                                                               |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                         | [Red](#host-not-allowed-in-a-cloud-session)                                                                                  |
| `Couldn't reconnect to your Remote Control session`                                                | [Red](#couldnt-reconnect-to-your-remote-control-session)                                                                     |
| `Prompt is too long`                                                                               | [Errores de solicitud](#prompt-is-too-long)                                                                                  |
| `Error during compaction: Conversation too long`                                                   | [Errores de solicitud](#error-during-compaction-conversation-too-long)                                                       |
| `Request too large`                                                                                | [Errores de solicitud](#request-too-large)                                                                                   |
| `Image was too large`                                                                              | [Errores de solicitud](#image-was-too-large)                                                                                 |
| `Unable to resize image`                                                                           | [Errores de solicitud](#unable-to-resize-image)                                                                              |
| `PDF too large` / `PDF is password protected`                                                      | [Errores de solicitud](#pdf-errors)                                                                                          |
| `Extra inputs are not permitted`                                                                   | [Errores de solicitud](#extra-inputs-are-not-permitted)                                                                      |
| `There's an issue with the selected model`                                                         | [Errores de solicitud](#theres-an-issue-with-the-selected-model)                                                             |
| `Model ... is not a recognized model id`                                                           | [Errores de solicitud](#model-is-not-a-recognized-model-id)                                                                  |
| `Claude Opus is not available with the Claude Pro plan`                                            | [Errores de solicitud](#claude-opus-is-not-available-with-the-claude-pro-plan)                                               |
| `Model ... is restricted by your organization's settings`                                          | [Errores de solicitud](#model-is-restricted-by-your-organizations-settings)                                                  |
| `thinking.type.enabled is not supported for this model`                                            | [Errores de solicitud](#thinking-type-enabled-is-not-supported-for-this-model)                                               |
| `max_tokens must be greater than thinking.budget_tokens`                                           | [Errores de solicitud](#thinking-budget-exceeds-output-limit)                                                                |
| `API Error: 400 due to tool use concurrency issues`                                                | [Errores de solicitud](#tool-use-or-thinking-block-mismatch)                                                                 |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`      | [Errores de solicitud](#usage-policy-refusal)                                                                                |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                  | [Errores de solicitud](#safety-measures-flagged-a-cybersecurity-topic)                                                       |
| `Installation was killed before it could finish (exit code 137)`                                   | [Errores de instalación](#installation-was-killed-before-it-could-finish)                                                    |
| `The connection dropped while downloading the update`                                              | [Errores de instalación](#the-connection-dropped-while-downloading-the-update)                                               |
| `Download timed out: exceeded the total deadline`                                                  | [Errores de instalación](#the-connection-dropped-while-downloading-the-update)                                               |
| `--bg and --print conflict`                                                                        | [Errores de línea de comandos](#command-line-errors)                                                                         |
| `Error: --json-schema is not a valid JSON Schema`                                                  | [Errores de línea de comandos](#command-line-errors)                                                                         |
| `Could not import <server>: <reason>`                                                              | [Errores de línea de comandos](#could-not-import-a-server-from-claude-desktop)                                               |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                           | [Errores de línea de comandos](#mcp-permission-prompt-tool-not-found)                                                        |
| `Marketplace "<name>" is registered from an untrusted source`                                      | [Errores de plugins](#marketplace-is-registered-from-an-untrusted-source)                                                    |
| `references ${user_config.*} in a shell-form command`                                              | [Errores de plugins](#plugin-command-references-user-config)                                                                 |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                 | [Errores de plugins](#plugin-command-references-user-config)                                                                 |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                | [Errores de plugins](#plugin-command-references-user-config)                                                                 |
| `would be spawned with zero tools — refusing`                                                      | [Errores de herramientas](#agent-would-be-spawned-with-zero-tools)                                                           |
| `File is covered by a Read deny rule in your permission settings`                                  | [Errores de herramientas](#file-is-covered-by-a-read-deny-rule)                                                              |
| `Can't open MCP settings in a background session`                                                  | [Errores de sesión en segundo plano](#commands-refused-in-a-background-session)                                              |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                        | [Errores de sesión en segundo plano](#claude_code_process_wrapper-launcher-errors)                                           |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                | [Advertencias de configuración](#workspace-has-not-been-trusted)                                                             |
| Responses seem lower quality than usual                                                            | [Calidad de respuesta](#responses-seem-lower-quality-than-usual)                                                             |

<h2 id="automatic-retries">
  Reintentos automáticos
</h2>

Claude Code reintenta fallos transitorios antes de mostrarle un error. Los errores del servidor, respuestas sobrecargadas, tiempos de espera de solicitud, aceleraciones 429 temporales y conexiones perdidas se reintentan hasta 10 veces con retroceso exponencial. A partir de v2.1.198, esto cubre conexiones que se cierran en medio de una respuesta antes de que se haya transmitido ninguna salida visible: Claude Code reemite la solicitud con el mismo retroceso y el turno continúa en lugar de detenerse con un error de conexión. A partir de v2.1.199, las aceleraciones 429 temporales que no llevan los encabezados de cuota de su plan también se reintentan cuando ha iniciado sesión con una suscripción de claude.ai; las versiones anteriores las reintentaban solo para autenticaciones de clave de API y Enterprise.

Algunas clases de fallo no se reintentan, porque un reintento no puede tener éxito:

* A partir de v2.1.199, una falla de validación de certificado TLS, como un proxy que inspecciona TLS, un paquete `NODE_EXTRA_CA_CERTS` faltante, o un certificado expirado, falla en el primer intento para que la corrección aparezca inmediatamente en lugar de después del presupuesto de reintento completo. Consulte [Errores de certificado SSL](#ssl-certificate-errors). Las condiciones TLS transitorias como un tiempo de espera de protocolo de enlace aún se reintentan.
* A partir de v2.1.199, un error del servidor que llega después de que Claude ya ha transmitido salida visible mantiene la respuesta parcial y agrega un [aviso de respuesta incompleta](#the-response-above-may-be-incomplete) en lugar de reintentar, ya que volver a ejecutar la solicitud podría ejecutar las mismas herramientas dos veces. Las versiones anteriores descartaban la salida parcial e informaban el turno como un error.
* Una [respuesta de streaming de Amazon Bedrock con un tipo de contenido inesperado](#bedrock-streaming-response-has-an-unexpected-content-type) falla en el primer intento, porque la puerta de enlace o proxy que reescribe la respuesta reescribiría el reintento de la misma manera. Requiere Claude Code v2.1.208 o posterior.

Mientras se reintenta, el spinner muestra una cuenta regresiva de `Retrying in Ns · attempt x/y` después de una etiqueta de error. La etiqueta nombra la razón específica del primer intento para fallos en los que puede actuar de inmediato: la red está caída, un protocolo de enlace TLS falló, o alcanzó un límite de velocidad. Para otros errores, dice `API error` al principio. A partir de v2.1.198, cambia a la razón específica del tercer intento, o en el intento final cuando `CLAUDE_CODE_MAX_RETRIES` permite menos de tres; las versiones anteriores cambian solo en el intento final.

A partir de v2.1.198, el consejo del spinner habitual se suprime durante los reintentos. Una vez que se revela la razón del error, si el fallo es una sobrecarga 529, la línea debajo de la cuenta regresiva también nombra dónde verificar el estado del servicio: `status.claude.com` en la API de Anthropic, o el host del proveedor o puerta de enlace nombrado en el mensaje en otras configuraciones.

Si no llegan datos en el flujo de respuesta durante 20 segundos mientras una solicitud aún está pendiente, el spinner muestra `Waiting for API response · will retry in … · check your network` antes de que comience cualquier reintento. La solicitud aún no ha fallado: la cuenta regresiva se ejecuta hasta el punto en que Claude Code interrumpe la conexión estancada y reintenta, por lo que el banner se borra por sí solo una vez que se reanuden los datos o el reintento tenga éxito. A partir de v2.1.185, el umbral es de 20 segundos; las versiones anteriores muestran el banner después de 10 segundos con una redacción diferente. Si reaparece en cada intento, trátelo como un [problema de red](#unable-to-connect-to-api).

Cuando ve uno de los errores en esta página, esos reintentos ya se han agotado, a menos que pertenezca a una clase que no se reintenta, como una falla de validación de certificado. Puede ajustar el comportamiento con estas variables de entorno:

| Variable                                     | Predeterminado | Efecto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------------------------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/es/env-vars)    | 10             | Número de intentos de reintento. Limitado a 15 a partir de v2.1.186; a partir de v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` aumenta el valor predeterminado y elimina el límite. Redúzcalo para que los fallos aparezcan más rápido en scripts.                                                                                                                                                                                                                                                                                                |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/es/env-vars) | sin establecer | Establézcalo en `1` en sesiones desatendidas como trabajos de CI para reintentar errores de capacidad `429` y `529` indefinidamente en lugar de fallar después de `CLAUDE_CODE_MAX_RETRIES` intentos. A partir de v2.1.199, también aumenta el recuento de reintento predeterminado para otros errores transitorios, como errores del servidor, tiempos de espera y conexiones perdidas, a 300, aproximadamente tres horas de retroceso, y elimina el límite de 15 en `CLAUDE_CODE_MAX_RETRIES` si establece esa variable explícitamente. |
| [`API_TIMEOUT_MS`](/docs/es/env-vars)             | 600000         | Tiempo de espera por solicitud en milisegundos. Auméntelo para redes lentas o proxies.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

<h2 id="server-errors">
  Errores del servidor
</h2>

Estos errores provienen del proveedor de inferencia en lugar de su cuenta o solicitud. En la API de Anthropic, eso significa la infraestructura de Anthropic. En Amazon Bedrock, la plataforma de agentes de Google Cloud, Microsoft Foundry o una puerta de enlace personalizada, significa la infraestructura de ese proveedor.

<h3 id="api-error-500-internal-server-error">
  Error de API: 500 Error interno del servidor
</h3>

Claude Code muestra el código de estado y el mensaje de error de la API para cualquier respuesta 5xx. El ejemplo a continuación muestra una respuesta 500 en la API de Anthropic:

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

La oración final indica dónde verificar el estado del servicio y varía según el proveedor. Las configuraciones de Amazon Bedrock, la plataforma de agentes de Google Cloud y Microsoft Foundry nombran el estado del servicio de ese proveedor. Una `ANTHROPIC_BASE_URL` personalizada nombra el host de la puerta de enlace.

Esto indica un fallo inesperado dentro de la API. No es causado por su prompt, configuración o cuenta.

**Qué hacer:**

* Verifique [status.claude.com](https://status.claude.com), o la página de estado del proveedor nombrada en el mensaje, para incidentes activos
* Espere un minuto y luego envíe su mensaje nuevamente. Su mensaje original sigue en la conversación, así que para un prompt largo puede escribir `try again` en lugar de pegar todo de nuevo.
* Si el error persiste sin incidente publicado, ejecute `/feedback` para que Anthropic pueda investigar con los detalles de su solicitud. Consulte [Reportar un error](#report-an-error) si `/feedback` no está disponible en su entorno.

<h3 id="api-error-repeated-529-overloaded-errors">
  Error de API: Errores 529 Overloaded repetidos
</h3>

La API está temporalmente a capacidad en todos los usuarios. Claude Code ya ha reintentado varias veces antes de mostrar este mensaje:

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

La oración final varía según el proveedor de la misma manera que el error 500 anterior.

Un 529 no es su límite de uso y no cuenta contra su cuota.

**Qué hacer:**

* Verifique [status.claude.com](https://status.claude.com), o la página de estado del proveedor nombrada en el mensaje, para avisos de capacidad
* Intente de nuevo en unos minutos
* Ejecute `/model` y cambie a un modelo diferente para continuar trabajando, ya que la capacidad se rastrea por modelo. Claude Code le solicita que haga esto cuando un modelo está bajo una carga particularmente alta, por ejemplo `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Solicitud agotada
</h3>

La API no respondió antes del plazo de conexión.

```text theme={null}
Request timed out
```

Esto puede suceder durante períodos de alta carga o cuando el modelo está generando una respuesta muy grande. El tiempo de espera de solicitud predeterminado es de 10 minutos.

**Qué hacer:**

* Reintente la solicitud
* Para tareas de larga duración, divida el trabajo en prompts más pequeños
* Si la causa es una red lenta o un proxy, aumente `API_TIMEOUT_MS` como se describe en [Reintentos automáticos](#automatic-retries)
* Si los tiempos de espera son frecuentes y su red es de otro modo saludable, consulte [Errores de red y conexión](#network-and-connection-errors) a continuación

<h3 id="the-response-above-may-be-incomplete">
  La respuesta anterior puede estar incompleta
</h3>

Una respuesta de transmisión falló después de que Claude ya había producido salida visible. Reenviar la solicitud podría ejecutar las mismas llamadas de herramienta dos veces, por lo que Claude Code mantiene lo que ya se transmitió y añade este aviso en lugar de descartar el turno. La variante que ve indica la causa:

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection closed mid-response. The response above may be incomplete.
API Error: Response stalled mid-stream. The response above may be incomplete.
```

* `Server error mid-response`: un error de servidor 5xx o sobrecargado a mitad de la transmisión. Esta variante requiere Claude Code v2.1.199 o posterior; antes de eso, ese caso descartaba la salida parcial e informaba todo el turno como un error.
* `Connection closed mid-response`: la conexión se interrumpió.
* `Response stalled mid-stream`: la transmisión dejó de enviar datos.

**Qué hacer:**

* Lea la respuesta que se transmitió. Nada se ha perdido, pero las oraciones finales o las llamadas de herramienta pueden faltar.
* Responda con `continue` para que Claude continúe donde se detuvo
* Si el mismo error aparece antes de cualquier salida visible, Claude Code reintenta la solicitud en lugar de finalizarla. Consulte [Reintentos automáticos](#automatic-retries).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  El modo automático no puede determinar la seguridad de una acción
</h3>

El modelo que [el modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) utiliza para clasificar acciones no pudo producir una decisión, por lo que el modo automático no aprobó la acción automáticamente. El mensaje que ve depende de por qué falló el clasificador.

Las lecturas, búsquedas y ediciones dentro de su directorio de trabajo omiten el clasificador, por lo que continúan funcionando en todos estos casos.

Cuando el modelo clasificador está sobrecargado:

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait briefly and then try this action again.
```

**Qué hacer:**

* Reintente después de unos segundos; Claude ve el mismo mensaje y generalmente reintenta por su cuenta
* Si los reintentos continúan fallando, continúe con tareas de solo lectura y vuelva a la acción bloqueada más tarde
* Esto es transitorio e independiente de la [elegibilidad del modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode); no necesita cambiar la configuración

Cuando el clasificador devolvió una respuesta no analizable:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Qué hacer:**

* Reintente la acción; esto generalmente tiene éxito en el siguiente intento
* Ejecute `claude --debug` y repita la acción para ver la respuesta del clasificador subyacente en el registro de depuración

Cuando una verificación de seguridad de API separada bloqueó la solicitud del clasificador debido al contenido de la conversación anterior:

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

**Qué hacer:**

* Esta no es una decisión sobre su acción. El contenido ya en su conversación activó un filtro de seguridad en la API cuando el modo automático envió la conversación al clasificador
* Reintentar no ayudará; el mismo contenido de conversación activará el filtro nuevamente
* Cambie a un [modo de permiso](/docs/es/permission-modes) diferente para que pueda aprobar la acción cuando se le solicite, o inicie una conversación nueva sin el contenido que activa el filtro

Cuando la conversación ha crecido más que la ventana de contexto del clasificador:

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

En una sesión interactiva, el modo automático vuelve a un aviso de permiso normal para esa acción para que pueda aprobar o denegar manualmente. En [modo no interactivo](/docs/es/headless) la ejecución se cancela porque la transcripción solo crece y reintentar no puede tener éxito.

**Qué hacer:**

* Apruebe o deniegue la acción en el aviso que aparece
* Ejecute `/compact` para reducir el tamaño de la conversación para que las acciones posteriores se ajusten nuevamente dentro de la ventana del clasificador

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agente terminado anticipadamente debido a un error de API
</h3>

La solicitud de API de un [subagente](/docs/es/sub-agents) falló terminalmente, por ejemplo porque se alcanzó un límite de uso o los reintentos de un error del servidor se agotaron, por lo que el subagente se detuvo antes de terminar su tarea. Este mensaje requiere Claude Code v2.1.199 o posterior; antes de eso, el texto de error de la API se devolvía a Claude como si fuera el resultado del subagente.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Qué hacer:**

* Haga coincidir el detalle del error después de los dos puntos con su propia sección en esta página, como [Límites de uso](#usage-limits) o [Errores del servidor](#server-errors), y siga los pasos de esa sección
* Una vez que el error subyacente se resuelva, pida a Claude que reintente la tarea o [reanude el subagente](/docs/es/sub-agents#resume-subagents)

Cuando un límite de velocidad, sobrecarga o error del servidor interrumpe un subagente en primer plano que ya produjo salida de texto, Claude recibe esa salida parcial marcada como incompleta en lugar de este error. Un subagente cuya única salida fueron llamadas de herramienta también obtiene este error; en v2.1.199 eso devolvía un resultado parcial vacío en su lugar. Consulte [Errores de API en subagentes](/docs/es/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Límites de uso
</h2>

Estos errores significan que se ha alcanzado una cuota vinculada a su cuenta o plan. Son distintos de los [errores del servidor](#server-errors), que afectan a todos.

<h3 id="youve-hit-your-session-limit">
  Ha alcanzado su límite de sesión
</h3>

Los planes de suscripción incluyen una asignación de uso continuo. Cuando se agota, verá uno de estos mensajes:

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
```

Claude Code bloquea las solicitudes adicionales hasta la hora de reinicio que se muestra en el mensaje. Los límites de sesión y semanales se comparten entre todos los modelos, por lo que cambiar de modelo no restaura el acceso. El límite de Opus se aplica solo a las solicitudes de Opus, por lo que cambiar a otro modelo con `/model` le permite seguir trabajando.

El uso se cuenta contra las asignaciones de sesión y semanales al mismo tiempo. Una única ráfaga de actividad intensa, como un gran fanout de flujo de trabajo, puede agotar la asignación semanal antes de que se reinicie la ventana de sesión.

**Qué hacer:**

* Espere a la hora de reinicio que se muestra en el error
* Para el límite de Opus, ejecute `/model` y cambie a otro modelo para seguir trabajando
* Ejecute `/usage` para ver los límites de su plan y cuándo se reinician
* Ejecute `/usage-credits` para comprar uso adicional en Pro y Max, o para solicitarlo a su administrador en Team y Enterprise. Consulte [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) para obtener información sobre cómo se factura esto.
* Para actualizar su plan a límites base más altos, consulte [claude.com/pricing](https://claude.com/pricing)

Para monitorear su asignación restante antes de alcanzar el límite, agregue los campos `rate_limits` a una [línea de estado personalizada](/docs/es/statusline#rate-limit-usage), o en la aplicación de escritorio haga clic en el [anillo de uso](/docs/es/desktop#check-usage) junto al selector de modelo.

<h3 id="usage-credits-required-for-1m-context">
  Se requieren créditos de uso para contexto de 1M
</h3>

El modelo seleccionado utiliza la ventana de contexto extendido de 1M tokens, y su plan solo lo incluye a través de créditos de uso.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on, or /model to switch to standard context
```

Esta es una verificación de derechos, no un agotamiento de cuota. Se activa incluso cuando sus asignaciones de sesión y semanales tienen capacidad restante. Consulte [Extended context](/docs/es/model-config#extended-context) para ver qué planes incluyen contexto de 1M directamente y cuáles requieren créditos de uso.

Cuando este error aparece a mitad de la conversación porque el contexto creció más allá de 200K tokens, Claude Code compacta automáticamente la conversación por debajo del límite de contexto estándar y mantiene la sesión en ese límite después, por lo que no se requiere ninguna acción. En versiones anteriores a v2.1.172, el error se repetía en cada solicitud posterior incluyendo `/compact`; ejecute `/clear` en esas versiones para recuperarse. Los pasos a continuación se aplican cuando seleccionó explícitamente un modelo `[1m]`.

**Qué hacer:**

* Ejecute `/model` y seleccione la variante sin el sufijo `[1m]` para volver a la ventana de contexto estándar
* Ejecute `/usage-credits` para activar la facturación medida para la variante de 1M en Pro y Max, o para solicitarla a su administrador en Team y Enterprise
* Si el error persiste después de `/model`, es posible que una ID de modelo de 1M esté configurada en otro lugar. Consulte [There's an issue with the selected model](#theres-an-issue-with-the-selected-model) para ver las ubicaciones de configuración a verificar en orden de prioridad.
* Para eliminar variantes de 1M del selector de modelo por completo, configure [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/es/env-vars)

<h3 id="server-is-temporarily-limiting-requests">
  El servidor está limitando temporalmente las solicitudes
</h3>

La API aplicó un acelerador de corta duración que no está relacionado con su cuota de plan.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code distingue estos de su límite de plan por la ausencia de los encabezados de cuota unificada que lleva una respuesta de límite real. A partir de v2.1.199, esto se [reintenta automáticamente](#automatic-retries) con retroceso antes de mostrarse, independientemente de cómo se autentique. En versiones anteriores, una sesión iniciada con una suscripción de claude.ai falló el turno en la primera ocurrencia; solo las claves API y los inicios de sesión de Enterprise lo reintentaron.

**Qué hacer:**

* Espere brevemente e intente de nuevo
* Consulte [status.claude.com](https://status.claude.com) si persiste

<h3 id="request-rejected-429">
  Solicitud rechazada (429)
</h3>

Ha alcanzado el límite de velocidad configurado para su clave API, proyecto de Amazon Bedrock o proyecto de Google Cloud.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

La oración final indica dónde verificar el estado del servicio y varía según el proveedor. Las configuraciones de Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry nombran el estado del servicio de ese proveedor en lugar de la página de estado de Anthropic. Una `ANTHROPIC_BASE_URL` personalizada nombra el host de la puerta de enlace.

**Qué hacer:**

* Ejecute `/status` y confirme que la credencial activa es la que espera. Una `ANTHROPIC_API_KEY` extraviada en su entorno puede enrutar solicitudes a través de una clave de nivel bajo en lugar de su suscripción.
* Consulte la consola de su proveedor para ver los límites activos y solicite un nivel más alto si es necesario
* Para claves API de Anthropic, consulte la [referencia de límites de velocidad](https://platform.claude.com/docs/en/api/rate-limits) para ver cómo funcionan los niveles y cómo establecer límites por espacio de trabajo
* Reduzca la concurrencia: reduzca [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/es/env-vars), evite ejecutar muchos subagentos paralelos, o cambie a un modelo más pequeño con `/model` para ejecuciones de alto volumen con scripts

<h3 id="credit-balance-is-too-low">
  El saldo de crédito es demasiado bajo
</h3>

Su organización de Console se ha quedado sin créditos prepagados.

```text theme={null}
Credit balance is too low
```

**Qué hacer:**

* Agregue créditos en [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), y considere habilitar la recarga automática allí para que el saldo se reabastezca antes de llegar a cero
* Cambie a autenticación de suscripción con `/login` si tiene un plan Pro, Max, Team o Enterprise
* Establezca límites de gasto por espacio de trabajo en la Console para evitar que un único proyecto agote el saldo de la organización. Consulte [Manage costs effectively](/docs/es/costs).

<h2 id="authentication-errors">
  Errores de autenticación
</h2>

Estos errores significan que Claude Code no puede probar quién es usted ante la API. Ejecute `/status` en cualquier momento para ver qué credencial está actualmente activa.

<h3 id="not-logged-in">
  No ha iniciado sesión
</h3>

No hay una credencial válida disponible para esta sesión.

```text theme={null}
Not logged in · Please run /login
```

**Qué hacer:**

* Ejecute `/login` para autenticarse con su suscripción de Claude o cuenta de Console
* Si esperaba que una variable de entorno lo autenticara, confirme que `ANTHROPIC_API_KEY` esté configurada y exportada en el shell donde lanzó `claude`
* Para CI o automatización donde el inicio de sesión interactivo no es posible, configure un script [`apiKeyHelper`](/docs/es/settings#available-settings) que obtenga una clave al iniciar
* Consulte [Precedencia de autenticación](/docs/es/authentication#authentication-precedence) para entender qué credencial usa Claude Code cuando hay varias presentes

Si se le solicita que inicie sesión repetidamente, consulte [No ha iniciado sesión o token expirado](/docs/es/troubleshoot-install#not-logged-in-or-token-expired) para obtener correcciones del reloj del sistema y Keychain de macOS.

<h3 id="could-not-resolve-authentication-method">
  No se pudo resolver el método de autenticación
</h3>

La sesión llegó al cliente de API sin ninguna credencial. Esto aparece en [sesiones en segundo plano](/docs/es/agent-view), sesiones en la nube y contextos del SDK de Agent donde la verificación de inicio de sesión interactivo no se ejecuta antes de la primera solicitud.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

Antes de v2.1.174, una sesión en segundo plano o en la nube asignada a un worker pre-inicializado inactivo podría fallar de esta manera incluso cuando las credenciales válidas estaban configuradas. Actualice para recuperarse. En las versiones actuales, el error significa que no había credencial disponible para el proceso del worker.

**Qué hacer:**

* Actualice a v2.1.174 o posterior si esto aparece en una sesión en segundo plano o en la nube y sus credenciales ya están configuradas
* Confirme que `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` o sus credenciales del proveedor de nube estén configuradas en el entorno que lanza el worker, no solo en su shell interactivo
* Para el SDK de Agent, consulte [configuración de autenticación](/docs/es/agent-sdk/overview#get-started)
* Ejecute `/status` en una sesión interactiva en el mismo entorno para confirmar qué fuente de credencial se resuelve

<h3 id="invalid-api-key">
  Clave de API no válida
</h3>

La variable de entorno `ANTHROPIC_API_KEY` o el script `apiKeyHelper` devolvió una clave que la API rechazó.

```text theme={null}
Invalid API key · Fix external API key
```

**Qué hacer:**

* Verifique si hay errores tipográficos y confirme que la clave no haya sido revocada en la [Console](https://platform.claude.com/settings/keys)
* Ejecute `env | grep ANTHROPIC` en el mismo shell. Herramientas como direnv, complementos de shell dotenv e IDE terminals pueden cargar una clave obsoleta de un archivo `.env` en su proyecto sin que la configure explícitamente.
* Desactive `ANTHROPIC_API_KEY` y ejecute `/login` para usar autenticación de suscripción en su lugar
* Si la clave proviene de un script [`apiKeyHelper`](/docs/es/settings#available-settings), ejecute el script directamente para confirmar que imprime una clave válida en stdout
* Ejecute `/status` para confirmar qué fuente de credencial está usando realmente Claude Code

<h3 id="your-apikeyhelper-script-is-failing">
  Su script apiKeyHelper está fallando
</h3>

El comando configurado en la configuración [`apiKeyHelper`](/docs/es/settings#available-settings) salió con un error, agotó el tiempo de espera o no imprimió nada en stdout. Sin una clave del script, la solicitud llega a la API con una credencial de marcador de posición, y la API la rechaza con `401`.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Claude Code vuelve a ejecutar el script y reintenta la solicitud hasta dos veces más antes de mostrar este mensaje, por lo que el fallo aparece dentro de tres intentos. Antes de v2.1.208, Claude Code gastaba el [presupuesto de reintentos](#automatic-retries) completo reenviando la solicitud con la credencial de marcador de posición y luego reportaba un error de autenticación `401` genérico en lugar del fallo del script.

Ejecutar `/login` no ayuda aquí: la salida del helper [tiene precedencia](/docs/es/authentication#authentication-precedence) sobre un inicio de sesión guardado mientras la configuración esté presente.

**Qué hacer:**

* Ejecute el comando configurado en `apiKeyHelper` directamente en su shell para reproducir el fallo
* Si el comando reporta una sesión expirada, vuelva a autenticarse con su proveedor de credenciales, por ejemplo iniciando sesión nuevamente en su SSO o bóveda de secretos
* Corrija el comando para que imprima la clave en stdout y salga con código 0. Consulte [rotar credenciales con apiKeyHelper](/docs/es/llm-gateway-connect#rotate-credentials-with-apikeyhelper) para una configuración funcional.
* Ejecute `/status` para confirmar que `apiKeyHelper` es la fuente de credencial activa. Cada vez que el comando falla, su código de salida y salida de error aparecen en un panel `Cloud authentication` en la terminal.

<h3 id="this-organization-has-been-disabled">
  Esta organización ha sido deshabilitada
</h3>

Una `ANTHROPIC_API_KEY` obsoleta de una organización de Console deshabilitada está anulando su inicio de sesión de suscripción.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your other credentials
API Error: 400 ... This organization has been disabled.
```

Las variables de entorno tienen precedencia sobre `/login`, por lo que una clave exportada en su perfil de shell o cargada desde un archivo `.env` se usa incluso cuando tiene una suscripción Pro o Max funcional. En modo no interactivo (`-p`), la clave siempre se usa cuando está presente.

**Qué hacer:**

* Desactive `ANTHROPIC_API_KEY` en el shell actual y elimínela de su perfil de shell, luego relance `claude`
* Ejecute `/status` después para confirmar que la credencial activa es su suscripción
* Si no hay variable de entorno configurada y el error persiste, la organización deshabilitada es la vinculada a su `/login`. Contacte con soporte o inicie sesión con una cuenta diferente.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Su organización ha deshabilitado la autenticación por clave de API
</h3>

Este mensaje requiere Claude Code v2.1.169 o posterior. El administrador de la organización de Console ha desactivado la autenticación por clave de API, por lo que la API rechaza la clave que Claude Code está enviando. La sugerencia de recuperación después del `·` varía según de dónde provenga la clave:

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Las variables de entorno y `apiKeyHelper` tienen precedencia sobre `/login`, por lo que ejecutar `/login` solo no ayuda mientras cualquiera de ellos siga suministrando una clave. Consulte [Precedencia de autenticación](/docs/es/authentication#authentication-precedence).

**Qué hacer:**

* Si el mensaje menciona `ANTHROPIC_API_KEY`, desactívela en el shell actual y elimínela de su perfil de shell o archivo `.env`, luego relance `claude`
* Si el mensaje menciona `apiKeyHelper`, elimine la configuración [`apiKeyHelper`](/docs/es/settings#available-settings) de su `settings.json`
* Ejecute `/login` para iniciar sesión con su cuenta de claude.ai
* Ejecute `/status` después para confirmar que la credencial activa es su suscripción en lugar de una clave de API
* Si necesita autenticación por clave de API para automatización, pida al administrador de su organización que la vuelva a habilitar en la Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Su organización ha deshabilitado el acceso a la suscripción de Claude
</h3>

Su organización de Claude no permite iniciar sesión en Claude Code con un inicio de sesión de suscripción. Ejecutar `/login` nuevamente con la misma cuenta devuelve el mismo error.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

Esta es una configuración de organización del lado del servidor, por lo que no se puede anular desde la configuración local, variables de entorno o banderas de CLI.

El SDK de Agent y el modo no interactivo `-p` presentan esto como el código de error `oauth_org_not_allowed`.

**Qué hacer:**

* Pida a su administrador que habilite el acceso a Claude Code para su organización
* Autentíquese con una clave de API de Console en lugar de su suscripción. Consulte [Autenticación de Claude Console](/docs/es/authentication#claude-console-authentication) para la configuración.
* Si usted es el administrador y no ve una opción para habilitar el acceso, contacte con [soporte de Anthropic](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Las rutinas están deshabilitadas por la política de su organización
</h3>

Un Propietario en su organización de Team o Enterprise ha desactivado las rutinas a nivel de organización. El error aparece cuando intenta crear o ejecutar una rutina, incluyendo desde `/schedule` y la interfaz de usuario de [Routines](/docs/es/routines) en claude.ai/code.

```text theme={null}
Routines are disabled by your organization's policy.
```

Esta es una configuración del lado del servidor, por lo que no se puede anular desde la configuración local, variables de entorno o banderas de CLI.

**Qué hacer:**

* Pida a un Propietario en su organización que habilite el interruptor **Routines** en [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Para trabajo programado único que no requiere rutinas a nivel de organización, consulte [tareas programadas](/docs/es/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control requiere la API de Anthropic
</h3>

La sesión no está hablando directamente con la API de Anthropic, por lo que no hay un backend de claude.ai para que [Remote Control](/docs/es/remote-control) se empareje.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com.
```

Esto aparece en Amazon Bedrock, Agent Platform de Google Cloud y Microsoft Foundry. A partir de v2.1.196, también aparece cuando [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) apunta a un host diferente de `api.anthropic.com`, como una [puerta de enlace LLM](/docs/es/llm-gateway) o proxy, incluso cuando inicia sesión con claude.ai.

**Qué hacer:**

* Desactive `ANTHROPIC_BASE_URL` y reinicie la sesión, o inicie Remote Control desde una sesión que hable directamente con la API de Anthropic
* Para este y los otros mensajes de inicio de Remote Control, consulte [Solucionar problemas de Remote Control](/docs/es/remote-control#troubleshooting)

<h3 id="oauth-token-revoked-or-expired">
  Token OAuth revocado o expirado
</h3>

Su inicio de sesión guardado ya no es válido. Un token revocado significa que cerró sesión en todas partes o un administrador eliminó el acceso; un token expirado significa que la actualización automática falló a mitad de la sesión.

Ambos mensajes reportan un rechazo que la API devolvió para una solicitud que Claude Code envió. Cuando el inicio de sesión guardado ya ha sido borrado después de una actualización fallida, verá [Login expirado](#login-expired) en su lugar.

```text theme={null}
OAuth token revoked · Please run /login
OAuth token has expired · Please run /login
API Error: 401 ... authentication_error
```

**Qué hacer:**

* Ejecute `/login` para iniciar sesión nuevamente
* Si el error regresa dentro de la misma sesión después de volver a autenticarse, ejecute `/logout` primero para borrar completamente el token almacenado, luego `/login`
* Para solicitudes repetidas de inicio de sesión entre lanzamientos, consulte las verificaciones del reloj del sistema y Keychain de macOS en [Solución de problemas](/docs/es/troubleshoot-install#not-logged-in-or-token-expired)
* Para otras fallas incluyendo `403 Forbidden` y problemas del navegador OAuth, consulte [Inicio de sesión y autenticación](/docs/es/troubleshoot-install#login-and-authentication)

<h3 id="login-expired">
  Login expirado
</h3>

Claude Code intentó renovar su inicio de sesión guardado de claude.ai o Claude Console y el servicio OAuth rechazó el token de actualización almacenado, por lo que Claude Code borró las credenciales guardadas. Después de eso, cada solicitud se detiene localmente antes de llegar a la API, porque solo `/login` puede crear nuevas credenciales. Antes de v2.1.206, Claude Code enviaba la solicitud de todas formas con cualquier credencial que permaneciera en el entorno, y luego cada modelo fallaba con [Hay un problema con el modelo seleccionado](#theres-an-issue-with-the-selected-model) o un 401 en lugar de un aviso para iniciar sesión.

```text theme={null}
Login expired · Please run /login
```

En [modo no interactivo](/docs/es/headless) (`-p`) y el [SDK de Agent](/docs/es/agent-sdk/overview), el mensaje se lee de la siguiente manera, y el código de error estructurado es `authentication_failed`:

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Este no es el mismo estado que [Token OAuth revocado o expirado](#oauth-token-revoked-or-expired). Esos mensajes reportan un 401 que la API devolvió. Claude Code mismo produce `Login expired` para un inicio de sesión que ya falló al renovar, por lo que no envía ninguna solicitud.

Las sesiones autenticadas con una clave de API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/es/env-vars) o un proveedor de terceros no usan el inicio de sesión guardado y nunca ven este mensaje.

**Qué hacer:**

* Ejecute `/login` para iniciar sesión nuevamente. Reintentar sin iniciar sesión muestra el mismo mensaje en cada solicitud.
* En modo no interactivo, ejecute `claude` en el mismo entorno, complete `/login`, luego reejecutar su comando. Para automatización que no puede iniciar sesión interactivamente, autentíquese con `ANTHROPIC_API_KEY` o [genere un token de larga duración con `claude setup-token`](/docs/es/authentication#generate-a-long-lived-token).
* Si iniciar sesión sigue fallando, consulte [Inicio de sesión y autenticación](/docs/es/troubleshoot-install#login-and-authentication)

<h3 id="oauth-scope-requirement">
  Requisito de alcance de OAuth
</h3>

El token almacenado es anterior a un alcance de permiso que una característica más nueva necesita. Verá esto más a menudo desde `/usage` y el indicador de uso de la línea de estado:

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**Qué hacer:**

* Ejecute `/login` para obtener un nuevo token con los alcances actuales. No necesita cerrar sesión primero.

<h3 id="aws-credentials-expired-or-invalid">
  Credenciales de AWS expiradas o no válidas
</h3>

Este mensaje requiere Claude Code v2.1.198 o posterior y solo aparece cuando [`awsAuthRefresh`](/docs/es/amazon-bedrock#advanced-credential-configuration) está configurado en su archivo de configuración. Su token de sesión de AWS expiró o fue rechazado, y la actualización automática que Claude Code ya ejecutó no produjo una credencial que la API acepte. Aparece en un 401 de [Claude Platform on AWS](/docs/es/claude-platform-on-aws) o el [punto final de Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint), que es cómo esos proveedores reportan un token de seguridad expirado.

La sugerencia de acción en el medio nombra el comando `awsAuthRefresh` de su configuración, por lo que varía. La parte estable es el `AWS credentials expired or invalid` inicial:

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Sin `awsAuthRefresh` configurado, el mismo 401 muestra el mensaje genérico `Please run /login` en su lugar, que no puede actualizar las credenciales de AWS.

**Qué hacer:**

* Ejecute el comando `awsAuthRefresh` nombrado en el mensaje, como `aws sso login --profile myprofile`, en otra terminal y complete el inicio de sesión del navegador, luego reintente
* En una sesión interactiva, ejecute `/login`, elija **plataforma de terceros**, luego seleccione **Claude Platform on AWS · refresh credentials** bajo **Usando plataformas de terceros** para ejecutar el mismo comando sin reiniciar Claude Code. Consulte [Configurar credenciales de AWS](/docs/es/claude-platform-on-aws#1-configure-aws-credentials)
* Si el error se repite después de que el comando de actualización tenga éxito, confirme que la identidad es válida fuera de Claude Code con `aws sts get-caller-identity` en el mismo shell y perfil

<h3 id="aws-authentication-failed">
  Falló la autenticación de AWS
</h3>

Este mensaje requiere Claude Code v2.1.198 o posterior y solo aparece cuando [`awsAuthRefresh`](/docs/es/amazon-bedrock#advanced-credential-configuration) está configurado en su archivo de configuración. Su proveedor de AWS devolvió un 403, o [Amazon Bedrock](/docs/es/amazon-bedrock) devolvió un 401.

Claude Code no puede decir cuál es la causa que encontró. Amazon Bedrock reporta un token de seguridad expirado como un 403, pero un 403 también es cómo reporta una denegación de autorización, como un `AccessDeniedException` de un permiso de IAM faltante o un modelo que no está habilitado para su cuenta.

Un 401 de Amazon Bedrock también llega aquí en lugar de bajo [Credenciales de AWS expiradas o no válidas](#aws-credentials-expired-or-invalid), porque Amazon Bedrock no reporta un token expirado como un 401. Un 401 de ese punto final típicamente proviene de algo más en la ruta de solicitud, como un proxy corporativo.

Una actualización de credencial corrige un token expirado y no puede corregir las otras causas, por lo que el mensaje ofrece ambas:

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

La sugerencia de acción en el medio nombra el comando `awsAuthRefresh` de su configuración, por lo que varía. La parte estable es el `AWS authentication failed` inicial.

**Qué hacer:**

* Ejecute el comando `awsAuthRefresh` nombrado en el mensaje, o `aws sso login`, en caso de que una credencial expirada sea la causa
* Si sus credenciales son actuales, confirme que los permisos de IAM en [Configuración de IAM](/docs/es/amazon-bedrock#iam-configuration) estén adjuntos a la identidad que está usando y que el modelo seleccionado esté habilitado para su cuenta y región
* Ejecute `aws sts get-caller-identity` para confirmar qué identidad usan sus solicitudes; un `AWS_PROFILE` obsoleto o perfil predeterminado es una causa común de una falta de coincidencia de permisos

<h3 id="aws-default-chain-credential-resolve-timed-out">
  El tiempo de resolución de credenciales de la cadena predeterminada de AWS se agotó
</h3>

El proveedor de credenciales de cadena predeterminada de AWS no produjo credenciales dentro de 60 segundos, por lo que Claude Code detuvo la resolución y falló la solicitud. El fallo es resolución de credenciales local: la solicitud nunca llegó a [Amazon Bedrock](/docs/es/amazon-bedrock), [Claude Platform on AWS](/docs/es/claude-platform-on-aws) o el [punto final de Mantle](/docs/es/amazon-bedrock#use-the-mantle-endpoint). Claude Code borra su [caché de credenciales](/docs/es/amazon-bedrock#credential-caching-and-resolution-timeout) y reintenta antes de que este error aparezca, por lo que en el momento en que lo ve la cadena se ha estancado en intentos repetidos.

```text theme={null}
API Error: AWS default-chain credential resolve timed out
```

Las causas comunes son un comando `credential_process` en su perfil de AWS que espera entrada que no puede recibir, y un contenedor o VM cuyo servicio de metadatos de instancia (IMDS) nunca responde a la prueba de la cadena. Antes de v2.1.207, una cadena estancada dejaba la solicitud esperando indefinidamente en lugar de fallar con este mensaje.

**Qué hacer:**

* Ejecute `aws sts get-caller-identity` en el mismo shell con el mismo `AWS_PROFILE`. Si también se cuelga, corrija el perfil; un comando `credential_process` que solicita interactivamente es una causa común.
* Complete el paso de inicio de sesión antes de iniciar Claude Code, por ejemplo `aws sso login --profile myprofile`, para que la cadena se resuelva desde el caché de SSO local en lugar de esperar un flujo del navegador
* Si su cadena ejecuta un inicio de sesión interactivo que legítimamente necesita más de 60 segundos, como SSO con MFA a través de un contenedor como `aws-vault`, aumente el límite en milisegundos con [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/es/env-vars)

<h2 id="network-and-connection-errors">
  Errores de red y conexión
</h2>

Estos errores significan que una solicitud de red desde Claude Code no pudo llegar a su destino, o algo entre Claude Code y la API alteró la respuesta en el camino de regreso. Generalmente se originan en su red local, proxy o firewall, o en la política de red del entorno en la nube.

<h3 id="unable-to-connect-to-api">
  No se puede conectar a la API
</h3>

La conexión TCP a la API falló o nunca se completó.

```text theme={null}
Unable to connect to API. Check your internet connection
Unable to connect to API (ECONNREFUSED)
Unable to connect to API (ECONNRESET)
Unable to connect to API (ETIMEDOUT)
fetch failed
Request timed out. Check your internet connection and proxy settings
```

Las causas comunes incluyen la falta de acceso a Internet, una VPN que bloquea `api.anthropic.com`, o un proxy corporativo requerido que no está configurado.

**Qué hacer:**

* Confirme que puede alcanzar el host de la API desde el mismo shell ejecutando `curl -I https://api.anthropic.com`. En Windows PowerShell use `curl.exe -I https://api.anthropic.com` para que no se use el alias `Invoke-WebRequest` integrado.
* Si está detrás de un proxy corporativo, establezca `HTTPS_PROXY` antes de lanzar Claude Code y consulte [Configuración de red](/docs/es/network-config)
* Si enruta a través de una puerta de enlace LLM o relé, establezca [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) en su dirección. Consulte [Conectar Claude Code a una puerta de enlace LLM](/docs/es/llm-gateway-connect) para la configuración.
* Asegúrese de que su firewall permita los hosts enumerados en [Requisitos de acceso a la red](/docs/es/network-config#network-access-requirements)
* Los fallos intermitentes se [reintentan automáticamente](#automatic-retries); los fallos persistentes apuntan a un problema de red local

Si `curl` tiene éxito pero Claude Code aún falla, la causa suele ser algo entre el tiempo de ejecución y la red en lugar de la red misma:

* En Linux y WSL, verifique `/etc/resolv.conf` para un servidor de nombres inaccesible. WSL en particular puede heredar un resolutor roto del host.
* En macOS, un cliente VPN que fue desconectado o desinstalado puede dejar una interfaz de túnel o una regla de enrutamiento. Verifique `ifconfig` para interfaces `utun` obsoletas y elimine la extensión de red de la VPN en Configuración del Sistema.
* Docker Desktop y tiempos de ejecución de contenedores similares pueden interceptar el tráfico saliente. Ciérrelos y reintente para descartar esto.

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  La respuesta de streaming de Bedrock tiene un content-type inesperado
</h3>

Una puerta de enlace o proxy entre Claude Code y [Amazon Bedrock](/docs/es/amazon-bedrock) está transformando el cuerpo de la respuesta de streaming o su encabezado `Content-Type`. Amazon Bedrock transmite respuestas como `application/vnd.amazon.eventstream`, y Claude Code rechaza una respuesta de streaming exitosa que reporta un content-type diferente en lugar de decodificar un cuerpo que no puede leer. La solicitud no se reintenta.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Antes de v2.1.208, la misma configuración incorrecta se presentaba como `API Error: Truncated event message received` después de que toda la respuesta había sido almacenada en búfer.

**Qué hacer:**

* Configure la puerta de enlace para pasar el cuerpo de la respuesta `InvokeModelWithResponseStream` y su encabezado `Content-Type` sin modificar. Un intermediario que reemite la transmisión como eventos enviados por el servidor es una causa común.
* Si la puerta de enlace reescribe solo el encabezado y pasa el cuerpo binario intacto, establezca [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/es/env-vars) para omitir la verificación hasta que la puerta de enlace se corrija. Consulte [Errores de streaming detrás de una puerta de enlace o proxy](/docs/es/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  Errores de certificado SSL
</h3>

Un proxy o dispositivo de seguridad en su red está interceptando el tráfico TLS con su propio certificado, y Claude Code no confía en él.

```text theme={null}
Unable to connect to API: SSL certificate verification failed. Check your proxy or corporate SSL certificates
Unable to connect to API: Self-signed certificate detected
```

A partir de v2.1.199, un fallo de validación de certificado no se reintenta, por lo que este error aparece en el primer intento en lugar de después del [presupuesto de reintentos](#automatic-retries) completo. Las versiones anteriores pasaban unos minutos reintentando antes de mostrarlo. Las condiciones TLS transitorias, como un tiempo de espera de protocolo de enlace, aún se reintentan.

Durante `/login` y la verificación de conectividad de inicio, el mismo fallo se reporta con el código OpenSSL y la solución en línea:

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

**Qué hacer:**

* Exporte el paquete de CA de su organización y apunte Claude Code a él con `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Consulte [Configuración de red](/docs/es/network-config#custom-ca-certificates) para obtener instrucciones de configuración completas
* No establezca `NODE_TLS_REJECT_UNAUTHORIZED=0`, que desactiva completamente la validación de certificados

<h3 id="host-not-allowed-in-a-cloud-session">
  Host no permitido en una sesión en la nube
</h3>

Una solicitud HTTP saliente desde una sesión en la nube o rutina fue bloqueada por la política de red del entorno.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

También puede ver un certificado TLS que no coincide con el certificado real del destino. El entorno en la nube enruta el tráfico saliente a través de un proxy que aplica la política de red, por lo que un certificado no coincidente significa que el proxy terminó la conexión, no el destino.

Este no es un problema de red del lado del cliente. Las sesiones en la nube y las [rutinas](/docs/es/routines) se ejecutan dentro de un entorno aislado cuyo tráfico saliente se filtra a la lista de permitidos del entorno. El entorno **Default** utiliza acceso **Trusted**, que permite la [lista de permitidos predeterminada](/docs/es/claude-code-on-the-web#default-allowed-domains) de registros de paquetes, API de proveedores de nube, registros de contenedores y dominios de desarrollo comunes, pero bloquea todo lo demás.

**Qué hacer:**

* Abra la rutina para editar, o inicie una sesión en la nube. Seleccione el icono de nube que muestra el nombre de su entorno, como **Default**, para abrir el selector. Pase el cursor sobre su entorno y haga clic en el icono de configuración.
* En el diálogo **Update cloud environment**, cambie **Network access** de **Trusted** a **Custom**, luego agregue el dominio bloqueado a **Allowed domains**. Ingrese un dominio por línea. Marque **Also include default list of common package managers** para mantener la [lista de permitidos predeterminada](/docs/es/claude-code-on-the-web#default-allowed-domains) junto con sus dominios personalizados. Seleccione **Full** en su lugar si desea acceso sin restricciones.
* Haga clic en **Save changes**. La siguiente ejecución utiliza la lista de permitidos actualizada.

Consulte [Network access](/docs/es/claude-code-on-the-web#network-access) para los niveles de acceso y la lista de permitidos predeterminada. Las sesiones de CLI locales no se ven afectadas por esta política.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  No se pudo reconectar a su sesión de Remote Control
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

Reanudar con `claude --resume` o `claude --continue` se reconecta a la sesión de [Remote Control](/docs/es/remote-control) registrada en esa conversación. Este mensaje significa que la reconexión falló por una razón que puede ser temporal, como una interrupción de red o un error del servidor, por lo que Claude Code no puede confirmar si la sesión remota aún existe. Su sesión local continúa ejecutándose sin Remote Control.

**Qué hacer:**

* Ejecute `/remote-control` para reintentar la conexión
* Inicie Claude Code sin `--resume` para crear una nueva sesión de Remote Control
* Para otros mensajes de inicio de Remote Control, consulte [Solucionar problemas de Remote Control](/docs/es/remote-control#troubleshooting)

No verá este mensaje cuando el servidor confirme que la sesión anterior ya no existe; Claude Code crea una nueva en ese caso. Antes de v2.1.200, cualquier fallo de reconexión creaba una nueva sesión de Remote Control, que dejaba sesiones adicionales en la lista de sesiones en claude.ai/code.

<h2 id="request-errors">
  Errores de solicitud
</h2>

Estos errores se relacionan con el contenido de su solicitud. La mayoría provienen de la API después de rechazar la solicitud; algunos son producidos localmente por Claude Code antes de que se envíe cualquier solicitud.

<h3 id="prompt-is-too-long">
  El prompt es demasiado largo
</h3>

La conversación más los archivos adjuntos exceden la ventana de contexto del modelo.

```text theme={null}
Prompt is too long
```

**Qué hacer:**

* Ejecute `/compact` para resumir turnos anteriores y liberar espacio, o `/clear` para comenzar de nuevo
* Ejecute `/context` para ver un desglose de lo que está consumiendo la ventana: prompt del sistema, herramientas, archivos de memoria y mensajes
* Deshabilite los servidores MCP que no está utilizando con `/mcp disable <name>` para eliminar sus definiciones de herramientas del contexto
* Recorte archivos de memoria `CLAUDE.md` grandes, o mueva instrucciones a [reglas con alcance de ruta](/docs/es/memory#path-specific-rules) que se carguen solo cuando sea relevante
* Los suagentes heredan cada definición de herramienta MCP de la sesión principal, lo que puede llenar su ventana de contexto antes del primer turno. Deshabilite los servidores MCP que no está utilizando antes de generar suagentes.
* Auto-compact está habilitado de forma predeterminada y normalmente previene este error. Si ha establecido [`DISABLE_AUTO_COMPACT`](/docs/es/env-vars), vuelva a habilitarlo o ejecute `/compact` manualmente antes de que la ventana se llene.

Consulte [Explore the context window](/docs/es/context-window) para una vista interactiva de cómo se llena el contexto.

<h3 id="error-during-compaction-conversation-too-long">
  Error durante la compactación: Conversación demasiado larga
</h3>

`/compact` falló porque no hay suficiente contexto libre para contener el resumen que produce.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Esto puede suceder cuando la ventana ya está llena en el momento en que se activa auto-compact, o cuando ejecuta `/compact` después de ver `Prompt is too long`.

**Qué hacer:**

* Presione Esc dos veces para abrir la lista de mensajes y retroceder varios turnos. Esto elimina los mensajes más recientes del contexto. Luego ejecute `/compact` nuevamente.
* Si retroceder no libera suficiente espacio, ejecute `/clear` para iniciar una sesión nueva. Su conversación anterior se conserva y puede reabrirse con `/resume`.

<h3 id="request-too-large">
  Solicitud demasiado grande
</h3>

El cuerpo de la solicitud sin procesar excedió el límite de bytes de la API antes de la tokenización, generalmente debido a un archivo o adjunto pegado grande.

```text theme={null}
Request too large (max 30 MB). Double press esc to go back and remove or shrink the attached content.
```

Este es un límite de tamaño en la solicitud HTTP, separado del [límite de ventana de contexto](#prompt-is-too-long).

**Qué hacer:**

* Presione Esc dos veces y retroceda más allá del turno que agregó el contenido de tamaño excesivo
* Haga referencia a archivos grandes por ruta en lugar de pegar su contenido, para que Claude pueda leerlos en fragmentos
* Para imágenes, consulte [Image was too large](#image-was-too-large) a continuación

<h3 id="image-was-too-large">
  La imagen era demasiado grande
</h3>

Una imagen pegada o adjunta excede los límites de tamaño o dimensión de la API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code reemplaza la imagen no procesable con un marcador de posición de texto y reintenta, por lo que los mensajes posteriores tienen éxito. En versiones anteriores a 2.1.142, una imagen pegada podría permanecer en la conversación y repetir el mismo error en cada mensaje posterior. Para recuperarse en esas versiones, presione Esc dos veces y retroceda más allá del turno donde se agregó la imagen.

**Qué hacer:**

* Cambie el tamaño de la imagen antes de pegarla. La API acepta imágenes de hasta 8000 píxeles en el borde más largo para una sola imagen, o 2000 píxeles cuando hay muchas imágenes en contexto.
* Tome una captura de pantalla más ajustada de la región relevante en lugar de la pantalla completa

<h3 id="unable-to-resize-image">
  No se puede cambiar el tamaño de la imagen
</h3>

Claude Code no pudo reducir la escala de una imagen adjunta antes de enviarla a la API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
```

Claude Code normalmente cambia el tamaño de las imágenes grandes automáticamente. Estos errores significan que el procesador de imágenes nativo no pudo cargar o devolvió un error, por lo que la imagen no se pudo cambiar de tamaño para ajustarse a los límites de la API.

**Qué hacer:**

* Si el mensaje le pide que convierta la imagen, conviértala a PNG, JPEG, GIF o WebP y adjúntela nuevamente. Claude Code puede verificar dimensiones para estos formatos sin el procesador de imágenes.
* Si el mensaje informa un límite de dimensión o tamaño, cambie el tamaño o recomprima la imagen por debajo de ese límite antes de adjuntarla.

<h3 id="pdf-errors">
  Errores de PDF
</h3>

El PDF que adjuntó no se pudo procesar.

```text theme={null}
PDF too large (max 100 pages, 32 MB). Try splitting it or extracting text first.
PDF is password protected. Try removing protection or extracting text first.
The PDF file was not valid. Try converting to a different format first.
```

**Qué hacer:**

* Para PDF de tamaño excesivo, pida a Claude que lea un rango de páginas con la herramienta Read en lugar de adjuntar el archivo completo, o extraiga texto con una herramienta como `pdftotext` y haga referencia al archivo de salida por ruta
* Para PDF protegidos o inválidos, elimine la contraseña o reexporte el archivo desde su aplicación de origen, luego intente nuevamente

<h3 id="extra-inputs-are-not-permitted">
  No se permiten entradas adicionales
</h3>

Un proxy o puerta de enlace LLM entre Claude Code y la API eliminó el encabezado de solicitud `anthropic-beta`, por lo que la API rechazó campos que dependen de él.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Extra inputs are not permitted ... tools.0.custom.input_examples
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code envía campos solo de beta como `context_management`, `effort` e `input_examples` de herramientas junto con un encabezado `anthropic-beta` que los habilita. Cuando una puerta de enlace reenvía el cuerpo pero elimina el encabezado, la API ve campos que no reconoce.

**Qué hacer:**

* Configure su puerta de enlace para reenviar el encabezado `anthropic-beta`. Consulte [feature pass-through](/docs/es/llm-gateway-protocol#feature-pass-through) para saber qué deben reenviar las puertas de enlace.
* Como alternativa, establezca [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/es/env-vars) antes de iniciar. Esto deshabilita características que requieren el encabezado beta para que las solicitudes tengan éxito a través de una puerta de enlace que no puede reenviarlo.

<h3 id="theres-an-issue-with-the-selected-model">
  Hay un problema con el modelo seleccionado
</h3>

El nombre del modelo configurado no fue reconocido o su cuenta carece de acceso a él. A partir de v2.1.160, la sugerencia final, que se muestra aquí en su forma interactiva, varía según la superficie.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**Qué hacer:**

* **CLI interactivo**: ejecute `/model` para elegir entre los modelos disponibles para su cuenta.
* **Modo no interactivo (`-p`)**: pase `--model` con un alias o ID válido, o establezca [`ANTHROPIC_MODEL`](/docs/es/env-vars). El texto de error muestra `Run --model` en esta superficie.
* **Agent SDK**: el texto de error omite la sugerencia porque el modelo se establece mediante programación. Establezca [`model` en `Options`](/docs/es/agent-sdk/typescript#options) en TypeScript o [`ClaudeAgentOptions(model=...)`](/docs/es/agent-sdk/python#claudeagentoptions) en Python, y maneje el error estructurado `model_not_found` para mostrar su propio reintento o selector de modelo.
* Use un alias como `sonnet` u `opus` en lugar de un ID versionado completo. Los alias se resuelven a un valor predeterminado mantenido para que no se vuelvan obsoletos. Consulte [Model configuration](/docs/es/model-config).
* Si el modelo incorrecto sigue apareciendo en la CLI, hay un ID obsoleto establecido en algún lugar. Verifique en [orden de prioridad](/docs/es/model-config#setting-your-model): la bandera `--model`, la variable de entorno `ANTHROPIC_MODEL`, luego el campo `model` en `.claude/settings.local.json`, el `.claude/settings.json` de su proyecto y `~/.claude/settings.json`. Elimine el valor obsoleto y Claude Code vuelve a su valor predeterminado de cuenta.
* Claude Code reporta un inicio de sesión de claude.ai expirado como [Login expired](#login-expired), no como este error. Antes de v2.1.206, un inicio de sesión expirado que ya no podía actualizarse fallaba en cada modelo con este error; ejecute `/login` si ve eso en una versión anterior.
* Para implementaciones de Google Cloud's Agent Platform, consulte [Google Cloud's Agent Platform troubleshooting](/docs/es/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  El modelo no es un ID de modelo reconocido
</h3>

La cadena de modelo que pasó a un cambio de modelo no es un alias de modelo, un ID de modelo que esta versión de Claude Code conoce, o un ID que comienza con `claude-`. Las causas habituales son un error tipográfico en el ID, un nombre para mostrar como `Sonnet 5` donde se espera el ID `claude-sonnet-5`, o un alias que solo las versiones más nuevas de Claude Code reconocen. Claude Code rechaza el cambio inmediatamente. Antes de v2.1.200, Claude Code guardaba la cadena y fallaba en la siguiente solicitud con [Hay un problema con el modelo seleccionado](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

La sugerencia final nombra el alias o ID de modelo más cercano. Cuando nada es lo suficientemente cercano, dice `Run /model to see available models.` en su lugar.

Claude Code produce este error localmente en el momento en que se solicita el cambio, antes de que se realice cualquier solicitud de API. Se aplica cuando un modelo se establece a través del método [Agent SDK](/docs/es/agent-sdk/typescript) `setModel()` o por una aplicación como la [Desktop app](/docs/es/desktop) que ejecuta la CLI de Claude Code para usted.

**Qué hacer:**

* Ejecute `/model` sin argumento para abrir el selector y elegir entre los modelos disponibles para su cuenta, luego pase el alias o ID que se muestra allí
* Si utilizó un alias que una versión más nueva de Claude Code admite, ejecute `claude update`. Un ID completo que comienza con `claude-` pasa esta verificación incluso cuando el modelo es más nuevo que su versión de Claude Code, por lo que la actualización no es necesaria para esos.
* Un modelo guardado antes de v2.1.200 no se repara con esta verificación. Si un valor obsoleto sigue apareciendo, elimínelo de las ubicaciones enumeradas en [Hay un problema con el modelo seleccionado](#theres-an-issue-with-the-selected-model).
* La verificación se ejecuta solo en la API de Anthropic. En Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/es/claude-platform-on-aws) y detrás de una [LLM gateway](/docs/es/llm-gateway) o un `ANTHROPIC_BASE_URL` personalizado, su proveedor o puerta de enlace define los nombres de modelo, por lo que Claude Code acepta cualquier cadena y la pasa.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus no está disponible con el plan Claude Pro
</h3>

Su plan de suscripción activo no incluye el modelo que seleccionó.

```text theme={null}
Claude Opus is not available with the Claude Pro plan · Select a different model in /model
```

**Qué hacer:**

* Ejecute `/model` y seleccione un modelo que su plan incluya
* Si actualizó su plan recientemente y aún ve esto, ejecute `/logout` luego `/login`. El token almacenado refleja su plan en el momento en que inició sesión, por lo que actualizar en la web no entra en vigencia en una sesión existente hasta que se reautentica.
* Consulte [claude.com/pricing](https://claude.com/pricing) para ver qué modelos incluye cada plan

<h3 id="model-is-restricted-by-your-organizations-settings">
  El modelo está restringido por la configuración de su organización
</h3>

Su administrador de organización ha deshabilitado este modelo en la consola de administración de claude.ai, o está excluido por una lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) en la configuración administrada. Cuando el modelo restringido se estableció con `--model`, `ANTHROPIC_MODEL` o la configuración `model`, Claude Code sustituye un modelo permitido y continúa. Escribir `/model <name>` para un modelo restringido se rechaza con `Run /model to choose a different model.` y la sesión mantiene su modelo actual.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Claude Code trata un alias de familia de modelo, uno de `opus`, `sonnet`, `haiku` o `fable`, como una solicitud de esa familia en lugar de su versión más nueva. En la API de Anthropic y en [Claude Platform on AWS](/docs/es/claude-platform-on-aws), un alias de familia restringido se resuelve a la versión más nueva de la familia que su organización y la lista de permitidos `availableModels` permiten, y el aviso de sustitución nombra esa versión. Claude Code rechaza `/model <alias>` solo cuando cada versión de la familia está restringida. Antes de v2.1.205, un alias de familia se sustituía o rechazaba basándose únicamente en su versión más nueva, incluso cuando una versión anterior de la misma familia estaba permitida.

**Qué hacer:**

* Ejecute `/model` para elegir entre los modelos que su organización permite. Los modelos restringidos están ocultos en el selector.
* Si el modelo restringido se estableció en `--model`, `ANTHROPIC_MODEL` o el campo `model` de un archivo de configuración, elimine o actualice ese valor para que el aviso no se repita en cada inicio
* Si necesita acceso al modelo restringido, pida a su administrador de organización que lo habilite. Consulte [Organization model restrictions](/docs/es/model-config#organization-model-restrictions).

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled no es compatible con este modelo
</h3>

Su versión de Claude Code es anterior a la mínima para Sonnet 5, Opus 4.8 u Opus 4.7. La CLI envió una configuración de pensamiento que el modelo ya no acepta.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**Qué hacer:**

* Ejecute `claude update` y reinicie Claude Code. Opus 4.7 necesita v2.1.111 o posterior. Opus 4.8 necesita v2.1.154 o posterior. Sonnet 5 necesita v2.1.197 o posterior
* Si no puede actualizar, ejecute `/model` y seleccione Opus 4.6 o Sonnet 4.6 en su lugar
* Si encuentra esto en el [Agent SDK](/docs/es/agent-sdk/overview), actualice el paquete SDK en su lugar. Opus 4.8 necesita TypeScript SDK v0.3.154 o posterior y Python SDK v0.2.88 o posterior. Sonnet 5 necesita TypeScript SDK v0.3.197 o posterior

<h3 id="thinking-budget-exceeds-output-limit">
  El presupuesto de pensamiento excede el límite de salida
</h3>

El presupuesto de pensamiento extendido configurado excede la longitud de respuesta máxima, por lo que no hay espacio para la respuesta real.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code ajusta estos valores automáticamente en la API de Anthropic. Normalmente ve este error en Amazon Bedrock o Google Cloud's Agent Platform cuando [`MAX_THINKING_TOKENS`](/docs/es/env-vars) se establece más alto que el límite de salida del proveedor, o cuando el modo de plan aumenta el presupuesto de pensamiento.

**Qué hacer:**

* Baje `MAX_THINKING_TOKENS`, o aumente [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/es/env-vars) por encima del presupuesto de pensamiento
* Consulte [Extended thinking](/docs/es/model-config#extended-thinking) para ver cómo el presupuesto interactúa con la longitud de salida

<h3 id="tool-use-or-thinking-block-mismatch">
  Desajuste de bloque de uso de herramienta o pensamiento
</h3>

El historial de conversación llegó a la API en un estado inconsistente, generalmente después de que se interrumpió una llamada de herramienta o se editó un turno a mitad de la transmisión.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Las tres variantes significan lo mismo: la secuencia de bloques `tool_use`, `tool_result` y `thinking` en el historial ya no coincide con lo que la API espera.

**Qué hacer:**

* Si está utilizando Opus 4.7 u Opus 4.8, ejecute `claude update` primero. Las versiones anteriores a v2.1.156 pueden desencadenar este error durante el uso normal de herramientas, y `/rewind` no lo borra.
* Ejecute `/rewind`, o presione Esc dos veces, para retroceder a un punto de control antes del turno corrupto y continuar desde allí. Consulte [Checkpointing](/docs/es/checkpointing) para ver cómo se crean y restauran los puntos de control.

<h3 id="usage-policy-refusal">
  Rechazo de Política de Uso
</h3>

La API se negó a responder porque el contenido en la conversación activó una verificación de [Política de Uso](https://www.anthropic.com/legal/aup). El mensaje incluye un ID de Solicitud que puede citar al soporte si cree que el rechazo es incorrecto.

```text theme={null}
API Error: Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.
```

La verificación evalúa la conversación completa, no solo su prompt más reciente, por lo que enviar un nuevo mensaje en la misma sesión generalmente reactiva el mismo rechazo. Lo mismo se aplica después de salir y reabrir la sesión con `--continue` o `--resume`, ya que la transcripción en disco aún contiene el contenido que desencadena. En [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai) y [Microsoft Foundry](/docs/es/microsoft-foundry), este mensaje también cubre solicitudes que las medidas de seguridad del modelo marcaron como un tema de ciberseguridad. Consulte [Safety measures flagged a cybersecurity topic](#safety-measures-flagged-a-cybersecurity-topic).

**Qué hacer:**

* Presione Esc dos veces o ejecute `/rewind` para retroceder a un punto de control antes del turno que desencadenó el rechazo, luego reformule o tome un enfoque diferente. Consulte [Checkpointing](/docs/es/checkpointing).
* Si no puede identificar qué turno lo causó, ejecute `/clear` para iniciar una conversación nueva en el mismo proyecto. Su conversación anterior se conserva en disco y permanece disponible en `/resume`.
* En [modo no interactivo](/docs/es/headless) (`-p`), donde rewind no está disponible, reintente con un prompt reformulado en una sesión nueva sin `--continue`. Las verificaciones de política varían según el modelo, por lo que cambiar a un modelo diferente con `--model` también puede resolver el rechazo en algunos casos.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Las medidas de seguridad marcaron un tema de ciberseguridad
</h3>

Las medidas de seguridad del modelo marcaron el contenido en la conversación como un tema de ciberseguridad. El mensaje nombra el modelo que marcó la solicitud:

```text theme={null}
API Error: Opus 4.8 has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude.

If you were not engaging in a cybersecurity topic, please send feedback via /feedback.
```

El mensaje vincula al [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), que otorga acceso para trabajo de ciberseguridad legítimo. La salvaguardia en sí es del lado del servidor y es anterior a v2.1.203; esta versión cambió solo la redacción del mensaje y la página a la que vincula.

Lo que ve depende de su proveedor y modo:

* En [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai) y [Microsoft Foundry](/docs/es/microsoft-foundry), una marca de ciberseguridad produce el mensaje de [rechazo de Política de Uso](#usage-policy-refusal) en su lugar.
* [Modo no interactivo](/docs/es/headless) omite la oración `/feedback`.

Antes de v2.1.203, el mensaje decía `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` seguido de un enlace de formulario de exención.

**Qué hacer:**

* Si su trabajo requiere este contenido, solicite acceso a través del [Cyber Verification Program](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Si su solicitud no era sobre un tema de ciberseguridad, ejecute `/feedback` para reportar el falso positivo
* Para continuar trabajando en la misma sesión, presione Esc dos veces o ejecute `/rewind` para retroceder a un punto de control antes del turno que desencadenó la marca, luego tome un enfoque diferente. Consulte [Checkpointing](/docs/es/checkpointing).

<h2 id="installation-errors">
  Errores de instalación
</h2>

Estos errores aparecen durante la instalación o actualización de Claude Code, desde el [script de instalación](/docs/es/setup#install-claude-code), `claude install`, o `claude update`. Para problemas de `command not found`, PATH, permisos y TLS durante la configuración, consulte [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  La instalación fue interrumpida antes de poder finalizar
</h3>

El script de instalación informa cuando el paso `claude install` es terminado por una señal. En Linux, el código de salida 137 significa que el proceso recibió SIGKILL, y en un host con poca memoria, generalmente es el asesino de falta de memoria (OOM) del kernel. El script imprime esta explicación y sale con el código 137:

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Para cualquier otra señal fatal, y para el código de salida 137 en macOS, el script imprime `Installation was killed before it could finish (exit code <N>)` con el código de salida real y omite la explicación de falta de memoria. El mensaje proviene del script de instalación que usan macOS y Linux, que también cubre instalaciones dentro de WSL; los scripts de instalación nativos de Windows nunca lo imprimen. Antes de v2.1.200, el script salía solo con la línea `Killed` del shell.

**Qué hacer:**

* Detenga otros procesos para liberar memoria, luego vuelva a ejecutar el instalador
* Agregue espacio de intercambio o muévase a una instancia más grande. Consulte [Instalación interrumpida en servidores Linux con poca memoria](/docs/es/troubleshoot-install#install-killed-on-low-memory-linux-servers) para los comandos del archivo de intercambio.

<h3 id="the-connection-dropped-while-downloading-the-update">
  La conexión se interrumpió mientras se descargaba la actualización
</h3>

La conexión al servidor de descarga se cerró mientras `claude install`, `claude update`, o el [actualizador automático](/docs/es/setup#auto-updates) estaba obteniendo el binario de Claude Code, y los reintentos no se recuperaron. Claude Code reintenta la descarga cuando la conexión se interrumpe, la transferencia se detiene o el archivo descargado falla su suma de verificación, hasta tres intentos en total. Un error HTTP completado, como un 404, no se reintenta porque el servidor ya respondió. Antes de v2.1.202, una única conexión interrumpida fallaba la descarga inmediatamente con el error simple `aborted` en lugar de reintentar.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

El texto entre paréntesis nombra qué intento falló y el error de red subyacente. `claude update` precede el mensaje con `Error: Failed to install native update` en stderr.

Una descarga que permanece conectada pero no se completa dentro de 10 minutos falla con `Download timed out: exceeded the total deadline` en su lugar. Claude Code no reintenta una descarga agotada, porque una conexión demasiado lenta para terminar dentro del plazo no terminará en un reintento inmediato. Los pasos a continuación se aplican a ambos mensajes. Antes de v2.1.205, el mismo plazo de 10 minutos se reportaba como el genérico `timeout of 600000ms exceeded` del cliente HTTP.

La causa usual es un proxy o puerta de enlace que cierra una transferencia larga antes de que se complete. El binario de Claude Code es una descarga grande, por lo que un límite de conexión de proxy que nunca afecta el tráfico normal de API aún puede interrumpirlo.

**Qué hacer:**

* Ejecute `claude update` nuevamente. En una red de otro modo saludable, la descarga generalmente tiene éxito en la siguiente ejecución. Para el mensaje agotado, ejecútelo nuevamente desde una red más rápida o menos limitada.
* Si su red requiere un proxy, establezca `HTTPS_PROXY` antes de ejecutar el instalador o `claude update`. Consulte [Verificar conectividad de red](/docs/es/troubleshoot-install#check-network-connectivity).
* Si un proxy corporativo sigue cerrando la transferencia, pida a su equipo de red que permita la descarga completa desde `downloads.claude.ai`. Consulte [Requisitos de acceso a la red](/docs/es/network-config#network-access-requirements).
* Ejecute `claude doctor` desde su shell para diagnósticos de instalación

<h2 id="command-line-errors">
  Errores de línea de comandos
</h2>

Estos errores provienen del comando `claude` de línea de comandos y sus subcomandos. Claude Code los imprime antes de ejecutar su prompt o enviar cualquier solicitud de API.

<h3 id="conflict-between-bg-and-print">
  Conflicto entre --bg y --print
</h3>

Este mensaje requiere Claude Code v2.1.198 o posterior. Combinó `--bg` con `-p` o `--print` en la misma invocación de `claude`. `--bg` inicia una [sesión en segundo plano](/docs/es/agent-view#from-your-shell) a la que se conecta posteriormente con `claude agents`, mientras que `--print` se ejecuta [de forma no interactiva](/docs/es/headless) y nunca inicia la sesión interactiva a la que se conecta `claude agents`. Antes de v2.1.198, esta combinación creaba silenciosamente un trabajo en segundo plano que nunca podía conectarse.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**Qué hacer:**

* Elimine `-p` o `--print`. `--bg` toma el prompt como su argumento posicional, por lo que `claude --bg "<task>"` es el comando completo. Consulte [Dispatch new agents from your shell](/docs/es/agent-view#from-your-shell).
* Para ejecutar el prompt de forma no interactiva e imprimir el resultado en lugar de crear una sesión en segundo plano, elimine `--bg` y ejecute `claude -p "<task>"`

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  El valor de --json-schema no es un JSON Schema válido
</h3>

El esquema que pasó a [`--json-schema`](/docs/es/cli-reference#cli-flags) en [modo no interactivo](/docs/es/headless#get-structured-output) falló en la compilación del JSON Schema, por lo que `claude` sale con código 1 en lugar de ejecutar el prompt. Antes de v2.1.205, un esquema inválido producía salida no estructurada sin error, y cualquier esquema que usara la palabra clave `format` se trataba como inválido.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

El texto después del segundo colon es el diagnóstico del validador y nombra la palabra clave o ubicación que falló. Los esquemas que usan la palabra clave `format`, como `"format": "email"`, son válidos: Claude Code acepta `format` como una anotación y no la aplica.

Claude Code ejecuta dos comprobaciones antes de la compilación del esquema: rechaza un valor que no es JSON analizable con `Error: --json-schema is not valid JSON`, y JSON válido que no es un objeto con `Error: --json-schema must be a JSON object`.

**Qué hacer:**

* Corrija la parte del esquema que nombra el diagnóstico y luego vuelva a ejecutar el comando
* Si el diagnóstico es `schema too large`, reduzca el anidamiento del esquema y la reutilización de `$ref`
* Consulte [Get structured output](/docs/es/headless#get-structured-output) para un esquema y comando que funcionen

<h3 id="could-not-import-a-server-from-claude-desktop">
  No se pudo importar un servidor desde Claude Desktop
</h3>

Claude Code no pudo agregar uno de los servidores que seleccionó en `claude mcp add-from-claude-desktop`. El comando aún importa los otros servidores seleccionados e imprime una línea por cada servidor que no pudo agregar. Antes de v2.1.205, el primer servidor que falló detuvo la importación y ninguno de los servidores seleccionados se agregó.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

El texto después del nombre del servidor es la razón. La más común es la comprobación de nombre: Claude Desktop permite caracteres en nombres de servidores, como espacios y puntos, que `claude mcp` restringe a letras, números, guiones y guiones bajos. Otras razones incluyen una configuración de servidor que falla en la validación y un servidor bloqueado por la [política MCP](/docs/es/managed-mcp) de su organización.

**Qué hacer:**

* Cambie el nombre del servidor en `claude_desktop_config.json` para usar solo letras, números, guiones y guiones bajos, luego ejecute `claude mcp add-from-claude-desktop` nuevamente
* Agregue ese servidor directamente con `claude mcp add` o `claude mcp add-json` bajo un nombre válido. Consulte [Import MCP servers from Claude Desktop](/docs/es/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="mcp-permission-prompt-tool-not-found">
  Herramienta de solicitud de permiso MCP no encontrada
</h3>

La herramienta que pasó a [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags) no estaba entre las herramientas MCP conectadas cuando la ejecución necesitó por primera vez una decisión de permiso, ya sea porque su servidor nunca se conectó o porque ningún servidor conectado expone una herramienta con ese nombre. Claude Code aún envía su prompt: la ejecución [no interactiva](/docs/es/headless) sale con este error, y código de salida 1, en la primera llamada de herramienta que necesita aprobación, por lo que no produce respuesta aunque la solicitud se haya realizado. Antes del primer prompt, Claude Code espera hasta el tiempo de espera de conexión por servidor de 30 segundos establecido por [`MCP_TIMEOUT`](/docs/es/env-vars) para que ese servidor se conecte. Antes de v2.1.206, el inicio no esperaba a que el servidor terminara de conectarse, por lo que un servidor que se iniciaba lentamente pero estaba en buen estado también producía este error.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

La lista después de `Available MCP tools:` nombra las herramientas MCP que estaban conectadas cuando terminó la espera.

**Qué hacer:**

* Compruebe que el servidor se inicia y permanece conectado: ejecute `claude mcp list` en el mismo directorio y confirme que el servidor aparece como conectado
* Confirme que el nombre de la herramienta coincida con el nombre `mcp__<server>__<tool>` que expone el servidor
* Si el servidor necesita más de 30 segundos para iniciarse, aumente [`MCP_TIMEOUT`](/docs/es/env-vars)

<h2 id="plugin-errors">
  Errores de plugins
</h2>

Estos errores provienen de la configuración de [plugins](/docs/es/plugins) y [marketplace](/docs/es/plugin-marketplaces). Para problemas de plugins que no produzcan uno de los mensajes en esta página, como una URL de marketplace que no carga o un plugin que se instala pero no aparece, consulte [Solución de problemas de plugins](/docs/es/discover-plugins#troubleshooting).

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace registrado desde una fuente no confiable
</h3>

El marketplace está registrado bajo un nombre que está [reservado para marketplaces oficiales de Anthropic](/docs/es/plugin-marketplaces#marketplace-schema), pero su fuente registrada no es un repositorio de GitHub de `anthropics`. Claude Code vuelve a verificar los nombres reservados cada vez que carga o actualiza un marketplace, por lo que el marketplace y los plugins instalados desde él dejan de cargarse. Antes de v2.1.205, el nombre se verificaba solo cuando se agregaba el marketplace, por lo que una entrada registrada antes de que su nombre se reservara seguía cargándose.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

**Qué hacer:**

* Ejecute `claude plugin marketplace remove <name>`, luego agregue el marketplace nuevamente desde el repositorio oficial `github.com/anthropics`
* Si publica un marketplace de terceros que utilizó el nombre antes de que se reservara, cámbielo de nombre y pida a los usuarios que lo vuelvan a agregar desde su fuente
* Consulte la lista de nombres reservados en [Marketplace schema](/docs/es/plugin-marketplaces#marketplace-schema)

<h3 id="plugin-command-references-user-config">
  Plugin command references user\_config in a shell command
</h3>

Un hook de plugin, [monitor](/docs/es/plugins-reference#monitors), o comando MCP [`headersHelper`](/docs/es/mcp#use-dynamic-headers-for-custom-authentication) hace referencia a una [opción de plugin](/docs/es/plugins-reference#user-configuration) `${user_config.KEY}`, y la cadena sustituida se pasaría a un shell. Un valor configurado que contenga `$(...)`, comillas invertidas o `;` se ejecutaría como código allí, por lo que Claude Code se niega a iniciar el componente en lugar de sustituir el valor. La verificación se ejecuta en la plantilla de comando, por lo que el error aparece incluso cuando aún no hay ningún valor configurado. Antes de v2.1.207, el valor se sustituía en el comando del shell.

La redacción depende de qué superficie hizo referencia a la opción. Un hook de forma de shell informa:

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Un monitor informa:

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

Un `headersHelper` de MCP informa:

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**Qué hacer:**

* Para un hook, agregue una matriz `args` para que se ejecute en [forma exec](/docs/es/hooks#exec-form-and-shell-form), donde cada `${user_config.KEY}` se convierte en un argumento sin shell en el medio. O elimine la referencia y lea la variable de entorno `$CLAUDE_PLUGIN_OPTION_<KEY>` dentro del script
* Para un monitor, elimine la referencia y haga que el script del monitor lea el valor desde un archivo de configuración
* Para un `headersHelper`, mueva `${user_config.KEY}` al campo `headers` del servidor, que no se analiza como shell, o lea el valor dentro del script del helper

<h2 id="tool-errors">
  Errores de herramientas
</h2>

Estos errores provienen de las herramientas integradas de Claude que rechazan una entrada. Claude corrige la mayoría de los errores de herramientas por sí solo; los dos siguientes requieren un cambio de su parte, porque provienen de una definición de subagenteque usted controla o de una regla de permisos que usted controla.

<h3 id="agent-would-be-spawned-with-zero-tools">
  El agente se generaría con cero herramientas
</h3>

Nada en la [lista de `tools` de un subagente](/docs/es/sub-agents#supported-frontmatter-fields) se resolvió en una herramienta, por lo que Claude Code se niega a lanzar el subagente en lugar de iniciar uno que no pueda actuar. El mensaje agrupa las entradas por la razón por la que no se resolvieron: no es una herramienta reconocida, una herramienta que no está disponible para subagentes, o reconocida pero sin coincidencia con ninguna herramienta en la sesión actual. Omitir el campo `tools` nunca activa este rechazo. Un patrón de servidor MCP como `mcp__github__*` no está exento: cuando ninguna herramienta conectada proviene de ese servidor, el lanzamiento se rechaza con el patrón en el grupo sin coincidencias. Antes de v2.1.208, el subagente se lanzaba sin herramientas y devolvía un resultado vacío o confuso.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**Qué hacer:**

* Corrija cada entrada que el error nombra contra las [herramientas disponibles para subagentes](/docs/es/sub-agents#available-tools)
* Elimine las entradas de herramientas que la sesión no tiene, como herramientas MCP de un servidor que no está conectado
* Para dar al subagente todas las herramientas que tiene el padre, elimine el campo `tools` en lugar de listar herramientas

<h3 id="file-is-covered-by-a-read-deny-rule">
  El archivo está cubierto por una regla de denegación de Read
</h3>

La herramienta Edit fue llamada en una ruta coincidente con una [regla de denegación de `Read`](/docs/es/permissions#read-and-edit), incluyendo la creación de un nuevo archivo en esa ruta. Editar reescribe contenido que Claude debe poder leer de nuevo, por lo que la llamada se rechaza antes de cualquier acceso a archivos. La regla bloquea solo la herramienta Edit: Write y NotebookEdit no están cubiertos por reglas de denegación de `Read`. Antes de v2.1.208, solo una regla de denegación de `Edit` bloqueaba ediciones, y una regla de denegación de `Read` sola no lo hacía.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

**Qué hacer:**

* Si Claude debe poder editar el archivo, elimine o reduzca la regla de denegación de `Read` en `/permissions` o en [configuración](/docs/es/settings#permission-settings)
* Si el archivo debe permanecer intacto, mantenga la regla y agregue una regla de denegación de `Edit` para la misma ruta para que las herramientas Write y NotebookEdit también se bloqueen

<h2 id="background-session-errors">
  Errores de sesión en segundo plano
</h2>

[Las sesiones en segundo plano](/docs/es/agent-view) se ejecutan sin una terminal interactiva propia, por lo que los comandos que necesitan una se comportan de manera diferente allí. Estos mensajes aparecen en la transcripción de una sesión en segundo plano, en la vista de agente o después de conectarse.

<h3 id="commands-refused-in-a-background-session">
  Comandos rechazados en una sesión en segundo plano
</h3>

Los comandos que abren un diálogo interactivo se rechazan en una sesión en segundo plano con un mensaje que nombra un formulario que funciona allí o le indica que ejecute el comando desde una terminal normal. `/install-github-app`, la lista de configuración `/mcp`, y las acciones de autenticación en el menú del servidor MCP se rechazan de esta manera. Antes de v2.1.208, abrían su diálogo dentro de la sesión en segundo plano.
En v2.1.208 solamente, el selector `/model` también fue rechazado en una sesión en segundo plano, y `/upgrade` imprimió la URL de actualización en lugar de abrir un navegador.

La redacción nombra el comando que fue rechazado. La lista de configuración `/mcp` reporta:

```text theme={null}
Can't open MCP settings in a background session — use `/mcp enable|disable|reconnect <server>` to steer, or run /mcp from an interactive terminal to authenticate.
```

**Qué hacer:**

* Use el formulario que nombra el mensaje, como `/mcp reconnect <server>`, `/mcp enable`, o `/mcp disable`
* Para flujos de inicio de sesión y autorización, ejecute el comando desde una sesión `claude` normal en una terminal

<h3 id="claude_code_process_wrapper-launcher-errors">
  Errores del lanzador CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/es/corporate-launcher) está configurado, y su valor no se puede usar, por lo que Claude Code se niega a iniciar el proceso afectado en lugar de ejecutarlo sin el lanzador. Los problemas de configuración se reportan con un mensaje que comienza con el nombre de la variable y establece la razón, por ejemplo:

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Un lanzador que se inicia pero sale sin reemplazarse a sí mismo con Claude Code falla la sesión que estaba iniciando, y la fila de la sesión en la vista de agente reporta que el lanzador `must exec, not daemonize`, seguido de cualquier cosa que el lanzador haya impreso. Una sesión que no puede iniciarse o alcanzar el servicio en segundo plano debido al lanzador reporta el problema del lanzador como la razón dentro de `Couldn't reach the background service (...)`.

**Qué hacer:**

* Establezca la variable en la ruta absoluta de un ejecutable que termine llamando a `exec "$@"`. Consulte [el contrato del lanzador](/docs/es/corporate-launcher#the-launcher-contract) para el contrato completo
* Verifique `/status`, que muestra el comando de lanzamiento resuelto en su entrada Self-exec y advierte cuando el servicio en segundo plano en ejecución no coincide con él, o ejecute `claude daemon status` desde un shell
* Después de corregir el valor en el bloque `env` de [configuración](/docs/es/corporate-launcher#set-up-the-launcher), reinicie el servicio en segundo plano con `claude daemon stop --any` para que el siguiente envío inicie uno envuelto

<h2 id="configuration-warnings">
  Advertencias de configuración
</h2>

Claude Code escribe estos mensajes en stderr al inicio en lugar de mostrar un error en la conversación. Informan sobre la configuración que leyó pero no aplicó.

<h3 id="workspace-has-not-been-trusted">
  El espacio de trabajo no ha sido confiable
</h3>

Claude Code encontró reglas `permissions.allow` o entradas `permissions.additionalDirectories` en el archivo `.claude/settings.json` o `.claude/settings.local.json` del proyecto y no las aplicó, porque [las reglas de permiso del proyecto requieren confianza del espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust). El recuento, el nombre de la configuración y el archivo nombrado en el mensaje varían según su configuración. Las reglas `deny` y `ask` no se ven afectadas.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Qué hacer:**

* Ejecute `claude` en el directorio y acepte el diálogo de confianza. El diálogo aparece incluso cuando un directorio principal ya es confiable, enumera las reglas que se están reteniendo y le permite rechazar y continuar trabajando sin ellas. Antes de v2.1.200, no aparecía ningún diálogo en esa situación, por lo que este paso no se podía completar allí.
* En [modo no interactivo](/docs/es/headless) con `-p` no se muestra ningún diálogo. Establezca la entrada `hasTrustDialogAccepted` en `~/.claude.json` usando la clave exacta `projects` que imprime el mensaje.
* Si el mensaje nombra `.claude/settings.local.json` e inició Claude Code fuera de un repositorio git o en su directorio de inicio, actualice a v2.1.200 o posterior. Las versiones 2.1.196 a 2.1.199 trataron su propio `.claude/settings.local.json` como suministrado por el repositorio en esos espacios de trabajo. En v2.1.207 y posterior, actualizar no es suficiente fuera de un repositorio git si no ha confiado en la carpeta: determinar que una carpeta no está dentro de un repositorio ejecuta git, y Claude Code ejecuta esa verificación solo después de que acepte el diálogo de confianza, así que use el primer paso. Su directorio de inicio y cualquier otro [directorio de configuración](/docs/es/permissions#project-allow-rules-and-workspace-trust) están exentos y no esperan el diálogo. Consulte [Reglas de permiso del proyecto y confianza del espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust).

<h2 id="responses-seem-lower-quality-than-usual">
  Las respuestas parecen de menor calidad que lo habitual
</h2>

Si las respuestas de Claude parecen menos capaces de lo que espera pero no se muestra ningún error, la causa suele ser el estado de la conversación en lugar del modelo en sí. Claude Code no cambia silenciosamente las versiones del modelo. Puede cambiar a un modelo de respaldo en tres casos específicos:

* Un [`--fallback-model`](/docs/es/cli-reference#cli-flags) configurado toma el control después de un error de disponibilidad, solo para ese turno, con un aviso en la transcripción
* Una verificación de inicio de Amazon Bedrock o de la plataforma de agentes de Google Cloud encuentra que su modelo predeterminado no está disponible
* El [respaldo automático de modelo](/docs/es/model-config#automatic-model-fallback) en Fable 5 mueve la sesión al modelo Opus predeterminado y muestra un aviso en la transcripción

La verificación de selección de modelo a continuación detecta el segundo y tercer caso; el primero aparece como un aviso de transcripción en lugar de un cambio de `/model`. La [configuración de modelo](/docs/es/model-config) explica cuándo se aplica cada respaldo.

Verifique estos primero:

* **Selección de modelo**: ejecute `/model` para confirmar que está en el modelo que espera. Una opción anterior de `/model` o una variable de entorno `ANTHROPIC_MODEL` pueden tenerlo en un modelo más pequeño del que pretendía.
* **Nivel de esfuerzo**: ejecute `/effort` para verificar el nivel de razonamiento actual y auméntelo para depuración difícil o trabajo de diseño. Los valores predeterminados varían según el modelo, así que verifique antes de asumir que está por debajo del máximo. Consulte [Ajustar nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) para los valores predeterminados por modelo y el atajo `ultrathink`.
* **Presión de contexto**: ejecute `/context` para ver qué tan llena está la ventana. Si está cerca de la capacidad, ejecute `/compact` en un punto natural o `/clear` para comenzar de nuevo. Consulte [Explorar la ventana de contexto](/docs/es/context-window) para ver cómo auto-compact afecta los turnos anteriores.
* **Instrucciones obsoletas**: los archivos `CLAUDE.md` grandes u obsoletos y las definiciones de herramientas MCP consumen contexto y pueden dirigir las respuestas. La revisión `/doctor` marca archivos de memoria de gran tamaño y extensiones no utilizadas, y `/context` muestra el uso de tokens de herramientas MCP. Antes de v2.1.205, `/doctor` abría una pantalla de diagnósticos que marcaba archivos de memoria de gran tamaño y definiciones de subagentos.

Cuando una respuesta sale mal, retroceder generalmente funciona mejor que responder con correcciones. Presione Esc dos veces o ejecute `/rewind` para retroceder antes del turno incorrecto, luego reformule el mensaje con más especificidades. Corregir en el hilo mantiene el intento incorrecto en contexto, lo que puede anclar respuestas posteriores a él. Consulte [Checkpointing](/docs/es/checkpointing).

Si la calidad aún parece incorrecta después de verificar lo anterior, ejecute `/feedback` y describa qué esperaba versus qué obtuvo. La retroalimentación enviada de esta manera incluye la transcripción de la conversación, que es la forma más rápida para que Anthropic diagnostique una regresión real. Consulte [Reportar un error](#report-an-error) si `/feedback` no está disponible en su entorno.

Si Claude advierte sobre una inyección de mensaje sospechosa, o rechaza una solicitud debido a una inyección sospechada, y el texto que nombra la advertencia es contexto que Claude Code agrega a la conversación automáticamente en lugar de contenido de archivo o web, ejecute `claude update` e intente de nuevo. Si la advertencia se repite después de actualizar, [repórtela](#report-an-error) en lugar de pegar el contenido marcado nuevamente en el mensaje. Antes de v2.1.201, Sonnet 5 rechazaba algunas solicitudes de la misma manera.

<h2 id="report-an-error">
  Reportar un error
</h2>

Para errores de componentes que esta página no cubre, consulte la guía relevante:

* El servidor MCP no se pudo conectar o autenticar: [MCP](/docs/es/mcp)
* El script de hook falló o bloqueó una herramienta: [Depurar hooks](/docs/es/hooks#debug-hooks)
* Permiso denegado o errores del sistema de archivos durante la instalación: [Solucionar problemas de instalación e inicio de sesión](/docs/es/troubleshoot-install)

Si un error no aparece aquí o la corrección sugerida no ayuda:

* Ejecute `/feedback` dentro de Claude Code para enviar la transcripción y una descripción a Anthropic. El comando también ofrece abrir un problema de GitHub rellenado previamente. El envío a Anthropic requiere [autenticación](/docs/es/authentication). En Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry y otros proveedores de terceros, o cuando no hay credenciales de Anthropic configuradas, `/feedback` guarda un archivo local que puede enviar a su representante de cuenta de Anthropic en su lugar.
* Ejecute `claude doctor` desde su shell para un diagnóstico de solo lectura de su instalación, o ejecute la verificación `/doctor` dentro de Claude Code para encontrar y solucionar problemas de configuración
* Consulte [status.claude.com](https://status.claude.com) para incidentes activos
* Busque [problemas existentes](https://github.com/anthropics/claude-code/issues) en GitHub
