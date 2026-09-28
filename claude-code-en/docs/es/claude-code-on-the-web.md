> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usar Claude Code en la nube

> Ejecute sesiones de Claude Code en la nube desde su navegador, teléfono, aplicación de escritorio o terminal, muévalas con --cloud y --teleport, y corrija automáticamente solicitudes de extracción.

<Note>
  Las sesiones en la nube están disponibles en los planes Pro, Max y Team, y para usuarios Enterprise con asientos premium o asientos Chat + Claude Code.
</Note>

Una sesión en la nube es una sesión de Claude Code que se ejecuta en infraestructura en la nube en lugar de en su máquina. De forma predeterminada, se ejecuta en infraestructura que Anthropic administra, o en el [entorno autohospedado](/docs/es/self-hosted-environments) de su organización cuando se enruta allí. La sesión sigue ejecutándose después de cerrar su portátil, y puede verificarla o dirigirla desde cualquier dispositivo.

Puede iniciar una sesión en la nube desde cualquiera de estas superficies:

* **Navegador**: [claude.ai/code](https://claude.ai/code), también llamado Claude Code en la web
* **Móvil**: la pestaña **Code** en la [aplicación Claude](/docs/es/mobile)
* **Aplicación de escritorio**: seleccione **Cloud** en lugar de **Local** cuando [inicie una sesión](/docs/es/desktop#run-long-running-tasks-in-the-cloud)
* **Terminal**: [`claude --cloud`](#from-terminal-to-cloud)
* **Rutinas**: [ejecuciones programadas y activadas](/docs/es/routines) cada una se ejecuta como una sesión en la nube

Para que Claude inicie y realice un seguimiento de muchas sesiones en la nube para un cuerpo de trabajo, use un [proyecto](/docs/es/claude-projects). Una sesión en su terminal, su IDE o la aplicación de escritorio con **Local** seleccionado se ejecuta en su propia máquina en su lugar. Para dirigir una de esas sesiones locales desde su teléfono o navegador, use [Control remoto](/docs/es/remote-control).

<Tip>
  ¿Nuevo en sesiones en la nube? Comience con [Primeros pasos](/docs/es/web-quickstart) para conectar su cuenta de GitHub y enviar su primera tarea.
</Tip>

Esta página cubre:

* [Entornos en la nube](#cloud-environments): dónde se ejecutan las sesiones y dónde configurar eso
* [Opciones de autenticación de GitHub](#github-authentication-options): dos formas de conectar GitHub
* [Mover tareas entre terminal y nube](#move-tasks-between-terminal-and-cloud) con `--cloud` y `--teleport`
* [Trabajar con sesiones](#work-with-sessions): modos de permisos, revisión, uso compartido, archivo, eliminación
* [Correcciones automáticas de solicitudes de extracción](#auto-fix-pull-requests): responder automáticamente a fallos de CI y comentarios de revisión
* [Seguridad y aislamiento](#security-and-isolation): cómo se aíslan las sesiones
* [Limitaciones](#limitations): límites de velocidad y restricciones de plataforma

<h2 id="cloud-environments">
  Entornos en la nube
</h2>

Cada sesión en la nube se ejecuta en un [entorno en la nube](/docs/es/cloud-environments), la configuración guardada que controla el acceso a la red, las variables de entorno y los scripts de configuración. Si aún no tiene un entorno, la incorporación configura un entorno **Predeterminado** con [acceso a la red **Confiable**](/docs/es/cloud-environments#access-levels), ya sea creándolo para usted o pidiéndole que lo cree. Consulte [El entorno predeterminado](/docs/es/cloud-environments#the-default-environment) para ver cuál de esos sucede en su plan y cómo las sesiones eligen un entorno cuando tiene más de uno.

Los mismos entornos se aplican dondequiera que inicie una sesión en la nube: el navegador, la terminal, [Claude Tag](https://claude.com/docs/claude-tag/overview), [rutinas](/docs/es/routines) y las aplicaciones móvil y de escritorio. Las sesiones de canales de Claude Tag utilizan solo entornos a nivel de organización, ya sean [entornos compartidos](/docs/es/cloud-environments#organization-shared-environments) o [entornos autohospedados](/docs/es/self-hosted-environments).

Consulte [Configurar entornos en la nube](/docs/es/cloud-environments) para cambiar lo que permite un entorno, establecer variables o agregar un script de configuración, y [Herramientas instaladas](/docs/es/cloud-environments#installed-tools) para ver qué incluyen las sesiones sin ninguna configuración.

<h2 id="github-authentication-options">
  Opciones de autenticación de GitHub
</h2>

Las sesiones en la nube necesitan acceso a sus repositorios de GitHub para clonar código e insertar ramas. Puede otorgar acceso de dos formas:

| Método           | Cómo se conecta                                                                               | Repositorios que las sesiones pueden alcanzar                                                                   | Mejor para                                                                                         |
| :--------------- | :-------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| **GitHub App**   | Autorice la aplicación Claude GitHub durante la [incorporación web](/docs/es/web-quickstart)       | Cualquier repositorio público, y repositorios privados en los que está instalada la aplicación Claude GitHub    | Incorporación en navegador; equipos que desean [Correcciones automáticas](#auto-fix-pull-requests) |
| **`/web-setup`** | Ejecute `/web-setup` en su terminal para enviar su token local de CLI `gh` a su cuenta Claude | Cualquier repositorio al que pueda acceder su token `gh`, independientemente de si está instalada la aplicación | Desarrolladores individuales que ya usan `gh`                                                      |

Instalar la aplicación Claude GitHub en un repositorio también habilita [Correcciones automáticas](#auto-fix-pull-requests) para solicitudes de extracción en él.

Los hilos en un [proyecto](/docs/es/claude-projects) necesitan que la aplicación esté instalada en cada repositorio que clonan, independientemente del método con el que se conectó. Consulte [Configurar acceso a GitHub](/docs/es/claude-projects#set-up-github-access).

Para ver cómo `/schedule` verifica el acceso al repositorio antes de crear una rutina, consulte [Repositorios y permisos de rama](/docs/es/routines#repositories-and-branch-permissions). Consulte [Conectar desde su terminal](/docs/es/web-quickstart#connect-from-your-terminal) para el tutorial de `/web-setup`, incluyendo qué almacena `/web-setup` y cómo eliminarlo.

La configuración rápida de web es una configuración de organización que permite a los miembros conectar GitHub con `/web-setup`, omite la solicitud de instalación de la aplicación Claude GitHub durante la incorporación en navegador, y hace que la incorporación en navegador cree el entorno [**Predeterminado**](/docs/es/cloud-environments#the-default-environment) para ellos en lugar de mostrar el formulario de entorno. En planes Team y Enterprise está deshabilitada de forma predeterminada, lo que oculta `/web-setup`. Un [Propietario](/docs/es/server-managed-settings#access-control) la activa con el interruptor **Configuración rápida de web** en [**Configuración de administrador > Claude Code**](https://claude.ai/admin-settings/claude-code).

<Note>
  Las organizaciones con [Retención de datos cero](/docs/es/zero-data-retention) habilitada no pueden usar `/web-setup` u otras características de sesión en la nube.
</Note>

<h2 id="move-tasks-between-terminal-and-cloud">
  Mover tareas entre terminal y nube
</h2>

Estos flujos de trabajo requieren la [CLI de Claude Code](/docs/es/quickstart) conectada a la misma cuenta de claude.ai. Puede iniciar nuevas sesiones en la nube desde su terminal, o extraer sesiones en la nube en su terminal para continuar localmente. Las sesiones en la nube persisten incluso si cierra su portátil, y puede monitorearlas desde cualquier lugar, incluyendo la aplicación móvil Claude.

<Note>
  Desde la CLI, la transferencia de sesión es unidireccional: puede extraer sesiones en la nube en su terminal con `--teleport`, pero no puede insertar una sesión de terminal existente en la nube. La bandera `--cloud` con una descripción de tarea crea una nueva sesión en la nube para su repositorio actual; con `-p` y un ID de sesión o URL de claude.ai/code, en su lugar [pone en cola un mensaje en esa sesión existente](/docs/es/claude-code-on-the-web#send-follow-ups-from-the-cli). La [aplicación de escritorio](/docs/es/desktop#continue-in-another-surface) proporciona un menú **Continuar en** que puede enviar una sesión local a la nube.
</Note>

<h3 id="from-terminal-to-cloud">
  De terminal a nube
</h3>

Inicie una sesión en la nube desde la línea de comandos con la bandera `--cloud`:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

Esto crea una nueva sesión en la nube en claude.ai. La VM en la nube clona el remoto de GitHub de su directorio actual en su rama actual, no su desprotección local, así que inserte primero si tiene confirmaciones locales. Consulte [Envíe repositorios locales sin GitHub](#send-local-repositories-without-github) para los casos en los que Claude Code carga su repositorio local en lugar de clonar.

`--cloud` funciona con un único repositorio a la vez. La tarea se ejecuta en la nube mientras continúa trabajando localmente. La ortografía anterior `--remote` aún funciona como un alias deprecado para `--cloud`.

Mientras se inicia el contenedor en la nube, la CLI muestra una lista de verificación en vivo de pasos de configuración, como clonar el repositorio y ejecutar su [script de configuración](/docs/es/cloud-environments#setup-scripts). Pone en cola los mensajes que escribe durante el aprovisionamiento y los envía una vez que la sesión está lista.

<Note>
  `--cloud` crea sesiones en la nube. `--remote-control` no está relacionado: permite monitorear y dirigir una sesión de CLI local desde claude.ai o la aplicación Claude. Consulte [Remote Control](/docs/es/remote-control).
</Note>

Abra la sesión en claude.ai o la aplicación móvil Claude para verificar el progreso o interactuar directamente. Desde allí puede dirigir Claude, proporcionar retroalimentación o responder preguntas como en cualquier otra conversación.

Si Claude hace una pregunta y la sesión permanece inactiva, aún puede responder cuando regrese, hasta [vencimiento del entorno](#environment-expired), y la sesión continúa desde su respuesta.

<h4 id="tips-for-cloud-tasks">
  Consejos para tareas en la nube
</h4>

**Planifique localmente, ejecute en la nube**: para tareas complejas, inicie Claude en modo de plan para colaborar en el enfoque, luego envíe el trabajo a la nube:

```bash theme={null}
claude --permission-mode plan
```

En modo de plan, Claude lee archivos, ejecuta comandos para explorar y propone un plan sin editar código fuente. Una vez que esté satisfecho, guarde el plan en el repositorio, comprométalo e insértelo para que la VM en la nube pueda clonarlo. Luego inicie una sesión en la nube para ejecución autónoma:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

**Ejecute tareas en paralelo**: cada comando `--cloud` crea su propia sesión en la nube que se ejecuta de forma independiente. Puede iniciar múltiples tareas y todas se ejecutarán simultáneamente en sesiones separadas:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Cuando una sesión se completa, puede crear una PR desde claude.ai/code o [teleportar](#from-cloud-to-terminal) la sesión a su terminal para continuar trabajando.

<h4 id="send-local-repositories-without-github">
  Envíe repositorios locales sin GitHub
</h4>

Cuando ejecuta `claude --cloud` desde un repositorio que no tiene un remoto de git, o desde un repositorio de github.com en el que no está instalada la aplicación Claude GitHub, Claude Code agrupa su repositorio local y lo carga directamente a la sesión en la nube. Esto se aplica incluso si conectó GitHub con `/web-setup`. El paquete incluye su historial de repositorio completo en todas las ramas, más cambios sin confirmar en archivos rastreados.

En macOS, Linux y WSL, Claude Code deja fuera de la carga cambios sin confirmar en archivos nombrados como credenciales o claves, y nombra los archivos que dejó fuera. Esto cubre archivos `.env`, archivos `*.tfvars` de Terraform y archivos de claves como `id_rsa` y `*.pem`. La sesión comienza con la versión confirmada de cada uno, o sin el archivo si ninguno está confirmado. En un worktree vinculado, submódulo o diseño similar, Claude Code carga estos cambios con el resto y nombra los archivos que carga.

Para cargar un paquete incluso cuando Claude Code de otro modo clonaría desde el remoto, establezca `CCR_FORCE_BUNDLE=1`:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

Los repositorios agrupados deben cumplir estos límites:

* El directorio debe ser un repositorio de git con al menos una confirmación
* El repositorio agrupado debe ser menor de 100 MB. Los repositorios más grandes se replieguen a agrupar solo la rama actual, luego a una instantánea única comprimida del árbol de trabajo, y fallan solo si la instantánea aún es demasiado grande
* Los archivos sin rastrear no se incluyen; ejecute `git add` en archivos que desea que la sesión en la nube vea
* Las sesiones creadas desde un paquete pueden insertar de vuelta a un remoto de GitHub solo cuando su [conexión de GitHub](#github-authentication-options) tiene acceso de inserción a ese repositorio

<h3 id="send-follow-ups-from-the-cli">
  Envíe seguimientos desde la CLI
</h3>

Una vez que una sesión en la nube se está ejecutando, dondequiera que se ejecute, envíele un mensaje de seguimiento desde la CLI `claude` en cualquier máquina donde esté conectado con `claude auth login`. La CLI se autentica con sus credenciales de cuenta de Anthropic y no envía estado de sesión local, por lo que el comando no necesita ejecutarse desde la máquina que inició la sesión, y es igual en cada shell, incluyendo PowerShell.

El comando publica un mensaje y sale:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

La CLI pone en cola el mensaje en la sesión y sale sin esperar una respuesta. Úselo para dirigir una sesión de larga duración, poner en cola el siguiente paso mientras el actual aún se está terminando, o enviar seguimientos desde un [script de CI](/docs/es/self-hosted-environments-testing#run-the-test-loop). También puede canalizar el mensaje en stdin en lugar de pasarlo como argumento: `echo "your message" | claude -p --cloud <session-id>`.

Para `<session-id>`, pase el ID desnudo, como `session_...` o `cse_...`, o la URL `claude.ai/code/<id>` de la sesión, con o sin el esquema o cadena de consulta. Encuentre el ID en su lista de sesiones en claude.ai/code.

<Note>
  `--cloud` requiere una cuenta de Anthropic. No está disponible cuando Claude Code está configurado para Amazon Bedrock, Google Cloud's Agent Platform u otro proveedor de terceros. Una [puerta de enlace LLM](/docs/es/llm-gateway) configurada solo a través de `ANTHROPIC_BASE_URL` no cuenta como proveedor de terceros para esta verificación, pero aún necesita iniciar sesión con `claude auth login`. La política `allow_remote_sessions` de su organización también debe estar habilitada. Un Propietario puede activarla en la configuración de administrador de Claude Code en claude.ai/admin-settings/claude-code.
</Note>

<h4 id="output-and-errors">
  Salida y errores
</h4>

En caso de éxito, el comando imprime el ID de sesión y un enlace para ver la sesión:

```
Sent to cloud session.
Session ID: session_01DiUkqY2kzbUbDmW1w96rfi
View: https://claude.ai/code/session_01DiUkqY2kzbUbDmW1w96rfi?from=cli&m=0
```

Pase `--output-format json` para un resultado legible por máquina: `{ok, session_id, url}` en caso de éxito, o `{ok: false, session_id, error}` cuando el envío falla, por ejemplo cuando falta la sesión o está archivada. Los errores de configuración, como un proveedor no compatible o una política de organización deshabilitada, se imprimen en stderr sin JSON. `--output-format stream-json` no es compatible con `--cloud <session-id>`.

La CLI prefija los errores con `Error: `. Una entrega fallida se envuelve como `failed to send message to cloud session <id>: <reason>`.

| Mensaje                                                                                                                     | Qué significa                                                                                                                                                                                                                                                                                                                                          |
| --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Cloud sessions aren't available with <provider>. They run on Anthropic's infrastructure and require an Anthropic account.` | Claude Code está configurado para un proveedor de terceros. El mensaje nombra el proveedor con la etiqueta que usa su configuración, como `Amazon Bedrock` o `Google Vertex AI`. Elimine la configuración de ese proveedor, por ejemplo desestableciendo `CLAUDE_CODE_USE_BEDROCK`, e inicie sesión con una cuenta de Anthropic (`claude auth login`). |
| `Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.`                | La política de organización `allow_remote_sessions` está deshabilitada.                                                                                                                                                                                                                                                                                |
| `Couldn't verify your organization's policy for cloud sessions. Check your network connection and try again.`               | Claude Code no pudo obtener la política de su organización, por lo que rechaza el envío en lugar de asumir que las sesiones en la nube están permitidas. Verifique su conexión de red e intente de nuevo.                                                                                                                                              |
| `Attaching to an existing cloud session is not enabled for your account.`                                                   | Ejecutó `--cloud <session-id>` sin `-p`. Envíe el mensaje con `claude -p "your message" --cloud <session-id>`.                                                                                                                                                                                                                                         |
| `Session not found: <id>`                                                                                                   | El ID o URL no coincide con una sesión a la que pueda acceder. Verifíquelo contra la URL de claude.ai/code de la sesión.                                                                                                                                                                                                                               |
| `cloud session <id> is archived and cannot accept new messages`                                                             | La sesión ha sido archivada. Inicie una nueva sesión en su lugar.                                                                                                                                                                                                                                                                                      |

<h3 id="from-cloud-to-terminal">
  De nube a terminal
</h3>

Extraiga una sesión en la nube en su terminal usando cualquiera de estos:

* **Usando `--teleport`**: desde la línea de comandos, ejecute `claude --teleport` para un selector de sesión interactivo, o `claude --teleport <session-id>` para reanudar una sesión específica directamente. Si tiene cambios sin confirmar, se le pedirá que los guarde primero.
* **Usando `/teleport`**: dentro de una sesión de CLI existente, ejecute `/teleport` o `/tp` para abrir el mismo selector de sesión sin reiniciar Claude Code.
* **Desde `/tasks`**: ejecute `/tasks` para ver sus sesiones de fondo, luego presione `t` para teleportarse a una.
* **Desde claude.ai/code**: seleccione **Abrir en > Terminal** desde el menú de sesión para copiar un comando que puede pegar en su terminal.
* **Desde dentro de la sesión en la nube**: escriba `/teleport` y Claude Code responde con el comando exacto `claude --teleport <session-id>` para esa sesión, listo para ejecutar desde una desprotección del repositorio. Requiere Claude Code v2.1.223 o posterior en el entorno de la sesión.

Cuando teleporta una sesión, Claude verifica que esté en el repositorio correcto, obtiene y verifica la rama de la sesión en la nube, y carga el historial de conversación completo en su terminal. La terminal obtiene su propia copia de la sesión: el nuevo trabajo allí permanece local y no aparece en la sesión en la nube en claude.ai o la aplicación móvil Claude. Para continuar dirigiendo desde su teléfono después de teleportar, inicie [`/remote-control`](/docs/es/remote-control) en la sesión local.

`--teleport` es distinto de `--resume`. `--resume` reabre una conversación del historial local de esta máquina y no enumera sesiones en la nube; `--teleport` extrae una sesión en la nube y su rama.

<h4 id="teleport-requirements">
  Requisitos de teleportación
</h4>

Teleport verifica estos requisitos antes de reanudar una sesión. Si algún requisito no se cumple, verá un error o se le pedirá que resuelva el problema.

| Requisito            | Detalles                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Estado de git limpio | Su directorio de trabajo no debe tener cambios sin confirmar. Teleport le pide que guarde los cambios si es necesario.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Repositorio correcto | Debe ejecutar `--teleport` desde un checkout del mismo repositorio, no desde una bifurcación. Si lo ejecuta desde un checkout de un repositorio diferente, Claude Code muestra un error que nombra tanto el repositorio de la sesión como el repositorio de su checkout. Antes de v2.1.219, el error no nombraba el repositorio de su checkout. Si Claude Code no puede analizar su remoto en un nombre de host, por ejemplo un alias de host SSH como `git@work:owner/repo.git`, le pide que confirme, y acepta el checkout cuando el propietario del remoto y el nombre del repositorio coinciden con el repositorio de la sesión. |
| Rama disponible      | La rama de la sesión en la nube debe haber sido insertada en el remoto. Teleport la obtiene y verifica automáticamente.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| Misma cuenta         | Debe estar autenticado en la misma cuenta de claude.ai utilizada en la sesión en la nube.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

<h4 id="teleport-is-unavailable">
  `--teleport` no está disponible
</h4>

Teleport requiere autenticación de suscripción de claude.ai. Si está autenticado a través de clave de API, ejecute `/login` para iniciar sesión con su cuenta de claude.ai en su lugar. Si el error nombra su proveedor en su lugar, las sesiones en la nube no están disponibles a través de proveedores de terceros; consulte la [tabla de errores](#output-and-errors). Si ya está conectado a través de claude.ai y `--teleport` aún no está disponible, su organización puede haber deshabilitado sesiones en la nube.

<h2 id="work-with-sessions">
  Trabajar con sesiones
</h2>

Las sesiones aparecen en la barra lateral en claude.ai/code. Desde allí puede revisar cambios, compartir con compañeros de equipo, archivar trabajo terminado o eliminar sesiones de forma permanente.

<h3 id="take-back-a-queued-message">
  Recuperar un mensaje en cola
</h3>

Si envía un mensaje mientras Claude está trabajando, el mensaje se pone en cola hasta que Claude lo lee. Para recuperar un mensaje en cola, haga clic en la ✕ que aparece en él. El texto vuelve al cuadro de mensaje para que pueda editarlo o enviar algo más.

Si Claude ya ha leído el mensaje, permanece en la conversación.

<h3 id="manage-context">
  Gestionar contexto
</h3>

Las sesiones en la nube admiten [comandos integrados](/docs/es/commands) que producen salida de texto. Los comandos que solo se ejecutan en la interfaz de terminal, como `/plugin` o `/resume`, no están disponibles. Los comandos que abren un selector o panel en el terminal se comportan de manera diferente en sesiones en la nube:

* **`/model`, `/effort`, `/color` y `/rename`**: pase el valor como argumento, por ejemplo `/model sonnet`, en lugar de abrir el selector de terminal o el control deslizante. Los formularios de argumento requieren Claude Code v2.1.205 o posterior en el entorno de la sesión y siguen las [notas de disponibilidad](/docs/es/commands#all-commands) de cada comando.
* **`/fast`**: alterna el [modo rápido](/docs/es/fast-mode#use-fast-mode-in-cloud-sessions) para la sesión cuando el modo rápido está [disponible en su cuenta](/docs/es/fast-mode#requirements). Requiere Claude Code v2.1.271 o posterior en el entorno de la sesión.
* **`/config`**: en su navegador en claude.ai/code, abre la sección Claude Code de su configuración en lugar de establecer un valor, y el texto después del comando, incluido `key=value`, se ignora. Para cambiar una configuración de una sesión en la nube, establezca una [variable de entorno](/docs/es/cloud-environments#set-environment-variables) en el entorno, o en una sesión con un repositorio, confirme la clave en el archivo `.claude/settings.json` de ese repositorio. [Configuración en sesiones en la nube](/docs/es/settings#settings-in-cloud-sessions) enumera lo que cada sesión lee.

Para la gestión de contexto específicamente:

| Comando    | Funciona en sesiones en la nube | Notas                                                                                                                         |
| :--------- | :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------- |
| `/compact` | Sí                              | Resume la conversación para liberar contexto. Acepta instrucciones de enfoque opcionales como `/compact keep the test output` |
| `/context` | Sí                              | Muestra lo que está actualmente en la ventana de contexto                                                                     |
| `/clear`   | No                              | Inicie una nueva sesión desde la barra lateral en su lugar                                                                    |

La compactación automática se ejecuta automáticamente cuando la ventana de contexto se aproxima a la capacidad. Las sesiones en la nube establecen [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/es/env-vars) por sí mismas, por lo que la compactación se activa a mitad de la [ventana de compactación automática](/docs/es/model-config#set-the-auto-compact-window) en lugar de cuando la ventana se llena. Ese valor anula uno que agregue en sus [variables de entorno](/docs/es/cloud-environments#set-environment-variables), por lo que agregar la variable allí no cambia cuándo se activa la compactación.

Para cambiar la ventana de compactación automática en su lugar, establezca [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/es/env-vars) en sus variables de entorno, o ejecute [`/autocompact`](/docs/es/commands#all-commands) con un recuento de tokens en una sesión donde la variable no está establecida.

Los [subagentes](/docs/es/sub-agents) funcionan de la misma manera que lo hacen localmente. Claude puede generarlos con la herramienta Agent para descargar investigación o trabajo paralelo en una ventana de contexto separada, manteniendo la conversación principal más ligera. Los subagentes definidos en `.claude/agents/` de su repositorio se recogen automáticamente.

Los [equipos de agentes](/docs/es/agent-teams) están desactivados de forma predeterminada pero se pueden habilitar agregando `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` a sus [variables de entorno](/docs/es/cloud-environments#set-environment-variables).

<h3 id="permission-modes-in-cloud-sessions">
  Modos de permiso en sesiones en la nube
</h3>

Elige el [modo de permiso](/docs/es/permission-modes) de una sesión en la nube desde el [menú desplegable de modo](/docs/es/permission-modes#switch-permission-modes), tanto cuando crea la tarea como mientras se ejecuta la sesión. Cuando reabre una sesión cuyo [entorno alojado por Anthropic ha expirado](#environment-expired), o envía un mensaje a una sesión que un ejecutor autohospedado [liberó mientras estaba inactivo](/docs/es/self-hosted-environments-reference#runner-cli-flags), Claude Code reanuda la sesión en el modo de permiso en el que estaba.

<h3 id="review-changes">
  Revisar cambios
</h3>

Cada sesión muestra un indicador de diferencia con líneas agregadas y eliminadas, como `+42 -18`. Selecciónelo para abrir la vista de diferencia, dejar comentarios en línea en líneas específicas y enviarlos a Claude con su próximo mensaje.

La vista de diferencia compara los cambios de la sesión contra su rama base de forma predeterminada. Para comparar contra cualquier otra rama en el repositorio, seleccione **Compare against** y elija una.

Claude Code calcula estas diferencias, incluidas las diferencias por archivo mostradas mientras Claude edita, a partir del contenido de blob de git sin procesar, por lo que los controladores de diferencia y los filtros `textconv` configurados en el repositorio no se aplican. Para un archivo en un repositorio que no es uno de los propios checkouts de la sesión, como uno clonado dentro del espacio de trabajo durante la sesión, la diferencia por archivo muestra la edición de Claude en sí misma en lugar de una comparación de git.

Consulte [Revisar e iterar](/docs/es/web-quickstart#review-and-iterate) para el tutorial completo que incluye la creación de PR. Para que Claude monitoree automáticamente el PR para detectar fallos de CI y comentarios de revisión, consulte [Corregir automáticamente solicitudes de extracción](#auto-fix-pull-requests).

<h3 id="share-sessions">
  Compartir sesiones
</h3>

Para compartir una sesión, alterne su visibilidad de acuerdo con los tipos de cuenta a continuación. Después de eso, comparta el enlace de sesión tal como está. Los destinatarios ven el estado más reciente cuando abren el enlace, pero su vista no se actualiza en tiempo real.

<h4 id="share-from-an-enterprise-or-team-account">
  Compartir desde una cuenta Enterprise o Team
</h4>

Para cuentas Enterprise y Team, las dos opciones de visibilidad son **Private** y **Team**. La visibilidad de Team hace que la sesión sea visible para otros miembros de su organización claude.ai. Las sesiones de [Claude en Slack](/docs/es/slack) se comparten automáticamente con visibilidad de Team.

La verificación de acceso al repositorio está habilitada de forma predeterminada, según la cuenta de GitHub conectada a la cuenta del destinatario. El nombre para mostrar de su cuenta es visible para todos los destinatarios con acceso.

<h4 id="share-from-a-max-or-pro-account">
  Compartir desde una cuenta Max o Pro
</h4>

Para cuentas Max y Pro, las dos opciones de visibilidad son **Private** y **Public**. La visibilidad pública hace que la sesión sea visible para cualquier usuario que haya iniciado sesión en claude.ai.

Verifique su sesión para detectar contenido sensible antes de compartir. Las sesiones pueden contener código y credenciales de repositorios privados de GitHub. La verificación de acceso al repositorio no está habilitada de forma predeterminada.

Para requerir que los destinatarios tengan acceso al repositorio, u ocultar su nombre de sesiones compartidas, vaya a [**Settings > Claude Code > Sharing settings**](https://claude.ai/settings/claude-code).

<h3 id="archive-sessions">
  Archivar sesiones
</h3>

Puede archivar sesiones para mantener su lista de sesiones organizada. Las sesiones archivadas se ocultan de la lista de sesiones predeterminada pero se pueden ver filtrando sesiones archivadas.

Para archivar una sesión, pase el cursor sobre la sesión en la barra lateral y seleccione el icono de archivo.

<h3 id="delete-sessions">
  Eliminar sesiones
</h3>

Eliminar una sesión elimina permanentemente la sesión y sus datos. Esta acción no se puede deshacer. Puede eliminar una sesión de dos formas:

* **Desde la barra lateral**: filtre sesiones archivadas, luego pase el cursor sobre la sesión que desea eliminar y seleccione el icono de eliminar
* **Desde el menú de sesión**: abra una sesión, seleccione el menú desplegable junto al título de la sesión y seleccione **Delete**

Se le pedirá que confirme antes de que se elimine una sesión.

<h2 id="auto-fix-pull-requests">
  Correcciones automáticas de solicitudes de extracción
</h2>

Claude puede observar una solicitud de extracción y responder automáticamente a fallos de CI y comentarios de revisión. Claude se suscribe a la actividad de GitHub en la PR, y cuando falla una verificación o un revisor deja un comentario, Claude investiga e inserta una corrección si es clara.

<Note>
  Las correcciones automáticas requieren que la aplicación Claude GitHub esté instalada en su repositorio. Si aún no lo ha hecho, instálela desde la [página de la aplicación GitHub](https://github.com/apps/claude).
</Note>

Hay algunas formas de activar correcciones automáticas dependiendo de dónde provenga la PR y qué dispositivo esté usando:

* **PRs creadas en una sesión en la nube**: abra la sesión en claude.ai/code, abra la barra de estado de CI y seleccione **Correcciones automáticas**
* **Desde su terminal**: ejecute [`/autofix-pr`](/docs/es/commands) mientras está en la rama de la PR. Claude Code detecta la PR abierta con `gh`, genera una sesión en la nube y activa correcciones automáticas en un paso
* **Desde la aplicación móvil**: dígale a Claude que corrija automáticamente la PR, por ejemplo "observa esta PR y corrige cualquier fallo de CI o comentario de revisión"
* **Cualquier PR existente**: pegue la URL de la PR en una sesión y dígale a Claude que la corrija automáticamente

Las correcciones automáticas son un control por PR. Para dejar de monitorear, abra la barra de estado de CI en la sesión en claude.ai/code y desactive el control **Correcciones automáticas**, o dígale a Claude que deje de observar la PR.

<h3 id="how-claude-responds-to-pr-activity">
  Cómo Claude responde a la actividad de PR
</h3>

Cuando las correcciones automáticas están activas, Claude recibe eventos de GitHub para la PR incluyendo nuevos comentarios de revisión y fallos de verificación de CI. Para cada evento, Claude investiga y decide cómo proceder:

* **Correcciones claras**: si Claude está seguro de una corrección y no entra en conflicto con instrucciones anteriores, Claude realiza el cambio, lo inserta y explica qué se hizo en la sesión
* **Solicitudes ambiguas**: si el comentario de un revisor podría interpretarse de múltiples formas o implica algo arquitectónicamente significativo, Claude le pregunta antes de actuar
* **Eventos duplicados o sin acción**: si un evento es un duplicado o no requiere cambio, Claude lo anota en la sesión y continúa

GitHub no emite un webhook cuando la rama base avanza y crea un conflicto de fusión, por lo que las correcciones automáticas no pueden reaccionar a conflictos por sí solas. Para resolver un conflicto, abra la sesión y pídele a Claude que haga un rebase.

Claude puede responder a hilos de comentarios de revisión en GitHub como parte de resolverlos. Estas respuestas se publican usando su cuenta de GitHub, por lo que aparecen bajo su nombre de usuario, pero cada respuesta está etiquetada como proveniente de Claude Code para que los revisores sepan que fue escrita por el agente y no por usted directamente.

<Warning>
  Si su repositorio utiliza automatización activada por comentarios como Atlantis, Terraform Cloud o GitHub Actions personalizadas que se ejecutan en eventos `issue_comment`, tenga en cuenta que Claude puede responder en su nombre, lo que puede activar esos flujos de trabajo. Revise la automatización de su repositorio antes de habilitar correcciones automáticas y considere deshabilitar correcciones automáticas para repositorios donde un comentario de PR puede implementar infraestructura o ejecutar operaciones privilegiadas.
</Warning>

<h2 id="security-and-isolation">
  Seguridad y aislamiento
</h2>

Cada sesión en la nube se separa de su máquina y de otras sesiones a través de varias capas:

* **Máquinas virtuales aisladas**: cada sesión se ejecuta en una VM aislada administrada por Anthropic. Las sesiones que su organización enruta a un [entorno autohospedado](/docs/es/self-hosted-environments) se ejecutan en su propia infraestructura en su lugar, donde el aislamiento es responsabilidad de su implementación
* <span id="default-allowed-domains" />**Controles de acceso a la red**: en entornos alojados por Anthropic, el acceso a la red se limita de forma predeterminada y puede deshabilitarse. Consulte [Acceso a la red](/docs/es/cloud-environments#network-access) para los niveles de acceso, los [dominios permitidos predeterminados](/docs/es/cloud-environments#default-allowed-domains) y el tráfico que no pasa por la lista de permitidos. En un entorno autohospedado, usted restringe la salida de la sesión en su propio límite de red. Cuando se ejecuta con acceso a la red deshabilitado, Claude Code aún puede comunicarse con la API de Anthropic, lo que puede permitir que los datos salgan de la VM.
* **Protección de credenciales**: en entornos alojados por Anthropic, las credenciales de git y las claves de firma permanecen fuera del sandbox, y un proxy se autentica en nombre de la sesión con credenciales de alcance. En un entorno autohospedado, su implementación proporciona credenciales de git; consulte [Configurar git](/docs/es/self-hosted-environments-deploy#configure-git)
* **Credenciales de API**: en entornos alojados por Anthropic en planes Pro y Max, las claves que [agrega a un entorno en la nube](/docs/es/cloud-environments#add-api-credentials) permanecen fuera del sandbox de la misma manera, adjuntas a solicitudes coincidentes después de que salen de la sesión. Un entorno autohospedado no tiene credenciales de API, y los planes Team y Enterprise aún no las tienen
* **Análisis seguro**: el código se analiza y modifica dentro del entorno aislado de la sesión antes de crear PRs

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Para errores de API en tiempo de ejecución que aparecen en la conversación como `API Error: 500`, `529 Overloaded`, `429` o `Prompt is too long`, consulte la [referencia de errores](/docs/es/errors). Esos errores y sus soluciones se comparten con la CLI y la aplicación de escritorio. Las secciones a continuación cubren problemas específicos de sesiones en la nube.

<h3 id="session-creation-failed">
  Falló la creación de sesión
</h3>

Si una nueva sesión no se inicia con `Session creation failed` o se detiene en el aprovisionamiento, Claude Code no pudo asignar una VM para la sesión.

* Verifique [status.claude.com](https://status.claude.com) para incidentes de sesión en la nube
* Reintente después de un minuto, ya que la capacidad se aprovisiona bajo demanda
* Confirme que su conexión de GitHub puede alcanzar el repositorio siguiendo [No repositories appear after connecting GitHub](/docs/es/web-quickstart#no-repositories-appear-after-connecting-github)

<h3 id="unable-to-get-organization-uuid">
  No se puede obtener UUID de organización
</h3>

`claude --cloud` y `claude --teleport` requieren iniciar sesión con una cuenta de claude.ai. Si se autentica con una clave de API, o sus detalles de cuenta almacenados están obsoletos, estos comandos fallan con `Unable to get organization UUID` o un mensaje de que la autenticación de clave de API no es suficiente. Con autenticación de clave de API o detalles de cuenta obsoletos, ejecutar `claude --teleport` sin un ID de sesión muestra `Error loading Claude Code sessions` en el selector de sesión en lugar de cualquiera de los dos mensajes, y se aplica la misma solución.

Ejecute `/login` para iniciar sesión con su cuenta de claude.ai, luego reintente el comando. Si el error nombra su proveedor en su lugar, consulte la [tabla de errores](#output-and-errors): las sesiones en la nube no están disponibles a través de proveedores de terceros.

<h3 id="remote-control-session-expired-or-access-denied">
  Sesión de Control Remoto expirada o acceso denegado
</h3>

`--teleport` se conecta a través de la misma infraestructura de sesión de Control Remoto que usan las sesiones en la nube, por lo que los errores de autenticación y vencimiento de sesión aparecen con la redacción de Control Remoto. Puede ver `Remote Control session expired` o `Access denied`. El token de conexión es de corta duración y está limitado a su cuenta.

* Ejecute `/login` localmente para actualizar sus credenciales, luego reconecte
* Confirme que está conectado a la misma cuenta que posee la sesión
* Si ve `Remote Control may not be available for this organization`, un propietario no ha habilitado sesiones en la nube para su organización

<h3 id="environment-expired">
  Entorno expirado
</h3>

Las sesiones en la nube se detienen después de un período de inactividad y la VM de la sesión se reclama. Una sesión cuenta como inactiva mientras espera que apruebe una llamada de herramienta de [conector MCP](/docs/es/cloud-environments#network-access) o para iniciar sesión en un servidor MCP, y puede expirar durante esa espera.

Reabra la sesión desde [claude.ai/code](https://claude.ai/code) para aprovisionar una VM nueva con su historial de conversación restaurado. El trabajo de fondo que aún se estaba ejecutando cuando se reclamó la VM, como subagentes y comandos de shell, no se restaura.

<h2 id="limitations">
  Limitaciones
</h2>

Antes de confiar en sesiones en la nube para un flujo de trabajo, tenga en cuenta estas restricciones:

* **Límites de velocidad**: las sesiones en la nube comparten límites de velocidad con todo otro uso de Claude y Claude Code dentro de su cuenta. Ejecutar múltiples tareas en paralelo consume más límites de velocidad proporcionalmente. No hay cargo de computación separado para la VM en la nube.
* **Autenticación de repositorio**: solo puede extraer una sesión en la nube a su terminal cuando está autenticado en la misma cuenta
* **Restricciones de plataforma**: la clonación de repositorio y la creación de solicitudes de extracción requieren GitHub. Las instancias autohospedadas de [GitHub Enterprise Server](/docs/es/github-enterprise-server) son compatibles con planes de Team y Enterprise. Puede enviar un repositorio de GitLab, Bitbucket u otro que no sea GitHub a una sesión en la nube como un [paquete local](#send-local-repositories-without-github) estableciendo `CCR_FORCE_BUNDLE=1`, pero la sesión no puede insertar resultados de vuelta a ese remoto
* **Lista de permitidos de IP de la organización**: las sesiones en la nube llaman a la API de Anthropic desde infraestructura administrada por Anthropic, no desde su red, mientras que las sesiones en un [entorno autohospedado](/docs/es/self-hosted-environments) la llaman desde su propia red. Si su organización tiene [lista de permitidos de IP](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) habilitada, cada sesión en la nube alojada por Anthropic falla con un error de autenticación. Lo mismo se aplica a [Revisión de código](/docs/es/code-review) y a [Rutinas](/docs/es/routines) que se ejecutan en entornos alojados por Anthropic; una rutina enrutada a un entorno autohospedado llama a la API desde su propia red. Contacte al [soporte de Anthropic](https://support.claude.com/) para eximir los servicios alojados por Anthropic de la lista de permitidos de IP de su organización.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Entornos en la nube](/docs/es/cloud-environments): configure el acceso a la red, las variables de entorno y los scripts de configuración para sesiones en la nube
* [Proyectos](/docs/es/claude-projects): una conversación donde Claude coordina sesiones en la nube paralelas en sus repositorios e informa los resultados
* [Ultrareview](/docs/es/ultrareview): ejecute una revisión de código profunda de múltiples agentes en un sandbox en la nube
* [Rutinas](/docs/es/routines): automatice el trabajo en un cronograma, a través de llamada de API o en respuesta a eventos de GitHub
* [Configuración de hooks](/docs/es/hooks): ejecute scripts en eventos del ciclo de vida de la sesión
* [Toda la configuración](/docs/es/settings-reference): todas las opciones de configuración
* [Seguridad](/docs/es/security): garantías de aislamiento y manejo de datos
* [Uso de datos](/docs/es/data-usage): qué retiene Anthropic de sesiones en la nube
* [Claude Tag](https://claude.com/docs/claude-tag/overview): una @Claude administrada por la organización en Slack que se ejecuta en la misma infraestructura en la nube
