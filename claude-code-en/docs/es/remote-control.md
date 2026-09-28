> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Continúe sesiones locales desde cualquier dispositivo con Remote Control

> Continúe una sesión local de Claude Code desde su teléfono, tableta o cualquier navegador usando Remote Control. Funciona con claude.ai/code y la aplicación móvil de Claude.

<Note>
  Remote Control está disponible en todos los planes. En Team y Enterprise, está deshabilitado de forma predeterminada hasta que un propietario habilite el botón de alternancia de Remote Control en [configuración de administración de Claude Code](https://claude.ai/admin-settings/claude-code).
</Note>

Remote Control conecta [claude.ai/code](https://claude.ai/code) o la aplicación Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) y [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) a una sesión de Claude Code que se ejecuta en su máquina. Inicie una tarea en su escritorio y luego continúela desde su teléfono en el sofá o desde un navegador en otra computadora.

Cuando inicia una sesión de Remote Control en su máquina, Claude sigue ejecutándose localmente todo el tiempo, por lo que su ejecución de código y acceso al sistema de archivos permanecen en su máquina. Con Remote Control puede:

* **Usar su entorno local completo de forma remota**: su sistema de archivos, [MCP servers](/docs/es/mcp), herramientas y configuración del proyecto permanecen disponibles, y escribir `@` completa automáticamente las rutas de archivo de su proyecto local.
* **Trabajar desde ambas superficies a la vez**: la conversación y el progreso de [subagentes](/docs/es/sub-agents) y [flujos de trabajo dinámicos](/docs/es/workflows) se mantienen sincronizados en todos los dispositivos conectados, por lo que puede enviar mensajes desde su terminal, navegador y teléfono indistintamente.
* **Enviar imágenes y archivos desde su teléfono o navegador**: adjunte una foto o archivo en la aplicación Claude o en claude.ai/code, con o sin un título. Claude ve las fotos adjuntas directamente como parte de su mensaje. Claude Code descarga otros archivos a su máquina y los pasa a Claude como referencias de archivo `@`.
* **Sobrevivir a interrupciones**: si su portátil se duerme o su red se cae, Claude Code se reconecta automáticamente cuando su máquina vuelve a estar en línea. Mientras se reconstruye la conexión, Claude Code pone en cola mensajes, solicitudes de permiso y actualizaciones de estado de subagentes y flujos de trabajo, y los entrega una vez que se recupera la conexión.

A diferencia de [Claude Code en la web](/docs/es/claude-code-on-the-web), que se ejecuta en infraestructura en la nube, las sesiones de Remote Control se ejecutan directamente en su máquina e interactúan con su sistema de archivos local. Las interfaces web y móvil son una ventana a esa sesión local.

Esta página cubre la configuración, cómo iniciar y conectarse a sesiones, y cómo Remote Control se compara con Claude Code en la web.

<h2 id="requirements">
  Requisitos
</h2>

Antes de usar Remote Control, confirme que su entorno cumple con estas condiciones:

* **Suscripción**: disponible en planes Pro, Max, Team y Enterprise. Las claves API no son compatibles. En Team y Enterprise, un propietario debe habilitar primero el botón de alternancia de Remote Control en [configuración de administración de Claude Code](https://claude.ai/admin-settings/claude-code).
* **Autenticación**: ejecute `claude` y use `/login` para iniciar sesión a través de claude.ai si aún no lo ha hecho. Sin un inicio de sesión elegible, `claude remote-control` se cierra con un error, mientras que `claude --remote-control` aún inicia una sesión interactiva y muestra una notificación de fallo de Remote Control poco después del lanzamiento.
* **Punto final de API**: no disponible en ninguna de estas configuraciones:
  * Utiliza Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry.
  * Apunta [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) a un host distinto de `api.anthropic.com`, como una [puerta de enlace LLM](/docs/es/llm-gateway) o proxy. Desactive la variable para usar Remote Control. Antes de v2.1.196, Claude Code permitía Remote Control con un `ANTHROPIC_BASE_URL` personalizado.
  * Inicia sesión a través de una [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) empresarial.
* **Evaluación de indicadores de características**: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` y `DISABLE_GROWTHBOOK`](/docs/es/env-vars) cada una deshabilita la evaluación de indicadores de características de la que depende la disponibilidad de Remote Control. Desactive la variable dondequiera que esté configurada, en su entorno de shell o en el bloque `env` de un [archivo `settings.json`](/docs/es/settings-reference#all-settings), para usar Remote Control.
* **Confianza del espacio de trabajo**: ejecute `claude` en su directorio de proyecto al menos una vez para aceptar el diálogo de confianza del espacio de trabajo. El diálogo de confianza de inicio nunca guarda la confianza para su directorio de inicio, así que inicie Remote Control desde un directorio de proyecto.

<h2 id="start-a-remote-control-session">
  Inicie una sesión de Remote Control
</h2>

Puede iniciar una sesión de Remote Control desde la CLI o la extensión de VS Code. La CLI ofrece tres modos de invocación; VS Code usa el comando `/remote-control`.

<Tabs>
  <Tab title="Modo servidor">
    En su directorio de proyecto, ejecute:

    ```bash theme={null}
    claude remote-control
    ```

    Hasta que acepte la confirmación única de Remote Control, `claude remote-control` explica qué hace y pregunta `Enable Remote Control? (y/n)` antes de iniciar el servidor. Responda `y` para aceptar e iniciar el servidor. Si rechaza, Claude Code sale sin iniciar el servidor y pregunta nuevamente la próxima vez que ejecute el comando.

    El proceso sigue ejecutándose en su terminal en modo servidor, esperando conexiones remotas. Muestra una URL de sesión que puede usar para [conectarse desde otro dispositivo](#connect-from-another-device), y puede presionar la barra espaciadora para mostrar un código QR para acceso rápido desde su teléfono. Mientras una sesión remota está activa, la terminal muestra el estado de la conexión y la actividad de las herramientas.

    Banderas disponibles:

    | Bandera                                         | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
    | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Establezca un título de sesión personalizado visible en la lista de sesiones en claude.ai/code.                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
    | `--remote-control-session-name-prefix <prefix>` | Prefijo para nombres de sesión generados automáticamente cuando no se establece un nombre explícito. El valor predeterminado es el nombre de host de su máquina, produciendo nombres como `myhost-graceful-unicorn`. Establezca `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` para el mismo efecto.                                                                                                                                                                                                                                                          |
    | `-c`, `--continue`                              | Reanude la sesión que el último servidor en este directorio inició, en lugar de crear una nueva. Consulte [Reanude sesiones después de detener el servidor](#resume-sessions-after-stopping-the-server). No se puede combinar con `--session-id`, `--spawn`, `--capacity`, o `--create-session-in-dir`. Requiere Claude Code v2.1.200 o posterior; las versiones anteriores rechazan la bandera como un argumento desconocido.                                                                                                                             |
    | `--session-id <id>`                             | Reanude una sesión por su ID. Consulte [Reanude sesiones después de detener el servidor](#resume-sessions-after-stopping-the-server). No se puede combinar con `--continue`, `--spawn`, `--capacity`, o `--create-session-in-dir`. Requiere Claude Code v2.1.200 o posterior; las versiones anteriores rechazan la bandera como un argumento desconocido.                                                                                                                                                                                                  |
    | `--spawn <mode>`                                | Cómo el servidor crea sesiones.<br />• `same-dir` (predeterminado): todas las sesiones comparten el directorio de trabajo actual, por lo que pueden entrar en conflicto si editan los mismos archivos.<br />• `worktree`: cada sesión bajo demanda obtiene su propio [git worktree](/docs/es/worktrees). Requiere un repositorio git.<br />• `session`: modo de sesión única. Sirve exactamente una sesión y rechaza conexiones adicionales. Se establece solo al inicio.<br />Presione `w` en tiempo de ejecución para alternar entre `same-dir` y `worktree`. |
    | `--capacity <N>`                                | Número máximo de sesiones concurrentes. El valor predeterminado es 32. No se puede usar con `--spawn=session`.                                                                                                                                                                                                                                                                                                                                                                                                                                             |
    | `--[no-]create-session-in-dir`                  | Pre-crear una sesión en el directorio actual cuando el servidor se inicia, para que tenga un lugar donde escribir inmediatamente. En modo `worktree` esta sesión permanece en el directorio actual mientras las sesiones bajo demanda obtienen worktrees aislados. Habilitado de forma predeterminada. Si pasa `--no-create-session-in-dir` para comenzar sin ninguno, Claude Code archiva las sesiones del servidor cuando lo detiene, por lo que no hay nada que [reanudar](#resume-sessions-after-stopping-the-server).                                 |
    | `--permission-mode <mode>`                      | Establezca el [modo de permiso](/docs/es/permission-modes) inicial para las sesiones del servidor, como `acceptEdits`. Acepta `manual` como alias para `default`; un modo no reconocido detiene el servidor al inicio y enumera los modos válidos.                                                                                                                                                                                                                                                                                                              |
    | `--debug-file <path>`                           | Escriba registros de depuración en el archivo dado.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
    | `--verbose`                                     | Mostrar registros detallados de conexión y sesión.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
    | `--sandbox` / `--no-sandbox`                    | Habilitar o deshabilitar [sandboxing](/docs/es/sandboxing) para aislamiento del sistema de archivos y red. Deshabilitado de forma predeterminada.                                                                                                                                                                                                                                                                                                                                                                                                               |

    Proporcione estas banderas después de `remote-control`.

    Si pasa una bandera global `claude` antes de `remote-control`, o un script contenedor agrega una, Claude Code no lleva la bandera a las sesiones que crea el servidor. Claude Code deja pasar la bandera solo cuando se sabe que dejarla no cambia lo que esas sesiones pueden hacer, como `--verbose` o `--model`. Para cualquier otra bandera, como `--settings`, Claude Code [se niega a iniciar](/docs/es/errors#not-carried-over-to-the-sessions-remote-control-starts) y nombra la bandera a eliminar. Antes de v2.1.248, cualquier opción antes de `remote-control` hacía que Claude Code rechazara las banderas después de ella con un error `unknown option`.

    Claude Code verifica la elegibilidad de Remote Control antes de imprimir la ayuda, por lo que `claude remote-control --help` devuelve un error en lugar de esta lista de banderas cuando no ha iniciado sesión con una cuenta elegible.
  </Tab>

  <Tab title="Sesión interactiva">
    Para iniciar una sesión normal interactiva de Claude Code con Remote Control habilitado, use la bandera `--remote-control` (o `--rc`):

    ```bash theme={null}
    claude --remote-control
    ```

    Opcionalmente, pase un nombre para la sesión:

    ```bash theme={null}
    claude --remote-control "My Project"
    ```

    Esto le proporciona una sesión interactiva completa en su terminal que también puede controlar desde claude.ai o la aplicación Claude. A diferencia de `claude remote-control` (modo servidor), puede escribir mensajes localmente mientras la sesión también está disponible de forma remota.
  </Tab>

  <Tab title="Desde una sesión existente">
    Si ya está en una sesión de Claude Code y desea continuarla de forma remota, use el comando `/remote-control` (o `/rc`):

    ```text theme={null}
    /remote-control
    ```

    Pase un nombre como argumento para establecer un título de sesión personalizado:

    ```text theme={null}
    /remote-control My Project
    ```

    Esto inicia una sesión de Remote Control que lleva su historial de conversación actual.

    Hasta que acepte la confirmación única de Remote Control, aparece un diálogo antes de que `/remote-control` se conecte. Seleccione **Enable Remote Control** para aceptar y conectarse. Si selecciona **Never mind** o presiona Esc, Claude Code no se conecta y pregunta nuevamente la próxima vez que ejecute `/remote-control`.

    Las banderas `--verbose`, `--sandbox` y `--no-sandbox` no están disponibles con este comando.
  </Tab>

  <Tab title="VS Code">
    En la [extensión de VS Code de Claude Code](/docs/es/vs-code), escriba `/remote-control` o `/rc` en el cuadro de solicitud.

    ```text theme={null}
    /remote-control
    ```

    Mientras Remote Control está activado, Claude Code muestra un indicador **Remote Control** en el pie de página del cuadro de solicitud. Una vez que la sesión se conecta, haga clic en el indicador para ir directamente a la sesión, o encuéntrela en la lista de sesiones en [claude.ai/code](https://claude.ai/code). Claude Code también publica la URL de la sesión en la conversación. Para desconectarse, ejecute `/remote-control` nuevamente.

    A diferencia de la CLI, el comando de VS Code no acepta un argumento de nombre ni muestra un código QR. El título de la sesión se deriva del historial de conversación o del primer mensaje.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Verifique el estado de la conexión
</h3>

En una sesión interactiva, mientras Remote Control está conectado, la terminal muestra un indicador `/rc active` que se vincula a la sesión en claude.ai. El indicador se oculta cuando la terminal es demasiado estrecha para ajustarlo. Para ver la URL de la sesión y un código QR para [conectarse desde otro dispositivo](#connect-from-another-device), ejecute `/remote-control` nuevamente para abrir el panel de estado. El panel también le permite desconectar Remote Control mientras su sesión local sigue ejecutándose.

<span id="session-ended-elsewhere" />Si la conexión falla en una sesión interactiva, el indicador cambia para mostrar el fallo, y Claude Code muestra el motivo en una notificación y lo agrega a la conversación. Ejecute `/remote-control` para reconectarse, a menos que el motivo diga que la sesión cambió en otro lugar:

* **Otra conexión tomó esta sesión**: otro dispositivo o sesión de Claude Code la tiene ahora. Ejecute `/remote-control` solo si desea recuperarla.
* **Esta sesión fue terminada o archivada desde otro dispositivo o aplicación**: ejecute `/remote-control` solo si la desea de vuelta. Claude Code reabre una sesión archivada.
* **El servidor ya no reporta esta sesión**: puede haber sido eliminada desde otro dispositivo o aplicación.

<h3 id="session-url-reminders">
  Recordatorios de URL de sesión
</h3>

Mientras Remote Control está conectado, Claude Code le recuerda la URL de la sesión cuando cambiar a su teléfono o navegador es más útil, para que no tenga que encontrar el enlace en `/remote-control`. Un recordatorio aparece encima del cuadro de solicitud en cualquiera de estos momentos:

* **Turno largo**: cuando un turno se ejecuta más tiempo que un umbral ajustado por el servidor, Claude Code muestra una notificación **Still working** con un enlace **Check in from your phone**, para que pueda seguir el turno desde su teléfono o navegador en lugar de esperar en la terminal. Claude Code la elimina cuando el turno termina.
* **Solicitudes de permiso repetidas**: después de responder varios [solicitudes de permiso](/docs/es/permissions) en una sesión, aparece una notificación **Approve tool calls from your phone** que muestra la URL de la sesión. Claude Code la elimina cuando comienza su próximo turno.

Los recordatorios pueden aparecer en cualquier sesión conectada, incluidas aquellas donde Remote Control [se conecta automáticamente](#enable-remote-control-for-all-sessions). No aparecen cada vez que ocurren estas condiciones, y cada uno aparece solo algunas pocas veces en total entre sesiones. No puede configurarlos ni desactivarlos; cada uno se borra por sí solo.

<h3 id="connect-from-another-device">
  Conectarse desde otro dispositivo
</h3>

Una vez que una sesión de Remote Control está activa, tiene varias formas de conectarse desde otro dispositivo:

* **Abra la URL de la sesión** en cualquier navegador para ir directamente a la sesión en [claude.ai/code](https://claude.ai/code).
* **Escanee el código QR** que se muestra junto a la URL de la sesión para abrirlo directamente en la aplicación Claude. Con `claude remote-control`, presione la barra espaciadora para alternar la visualización del código QR.
* **Abra [claude.ai/code](https://claude.ai/code) o la aplicación Claude** y encuentre la sesión por nombre en la lista de sesiones. En la aplicación móvil Claude, toque **Code** en la navegación para llegar a la lista de sesiones. Las sesiones de Remote Control muestran un icono de computadora con un punto de estado verde cuando están en línea.

Cuando se conecta, el dispositivo muestra cualquier subagente y flujo de trabajo que la sesión ya tenga ejecutándose en segundo plano. Detenga uno de ellos desde el dispositivo, y Claude Code detiene esa tarea en su máquina.

El título de la sesión remota se elige en este orden:

1. El nombre que pasó a `--name`, `--remote-control`, o `/remote-control`
2. El título que estableció con `/rename`
3. El último mensaje significativo en el historial de conversación existente
4. Un nombre generado automáticamente como `myhost-graceful-unicorn`, donde `myhost` es el nombre de host de su máquina o el prefijo que estableció con `--remote-control-session-name-prefix`

Si no estableció un nombre explícito, Claude Code actualiza el título para reflejar su solicitud una vez que envíe una. Claude Code hace coincidir los títulos generados automáticamente con el idioma de su conversación, o con la configuración [`language`](/docs/es/settings-reference#language) si una está configurada.

Cuando renombra una sesión desde claude.ai o la aplicación Claude, Claude Code también actualiza el título local que se muestra en `claude --resume`. Claude Code aplica el mismo cambio de nombre al nombre de sesión que se muestra en la barra de solicitud, y en la lista `claude agents` cuando la sesión [se ejecuta en segundo plano](/docs/es/agent-view). Antes de v2.1.221, renombrar desde la lista de sesiones en claude.ai o en la aplicación Claude actualizaba solo el título, y la CLI mantenía su nombre de sesión anterior; `/rename`, que se ejecuta en la CLI misma, establece el nombre en cualquier versión.

Si aún no tiene la aplicación Claude, ejecute `/mobile` dentro de Claude Code para mostrar un código QR para [claude.ai/mobile](https://claude.ai/mobile), que abre la tienda de aplicaciones correcta para su teléfono.

<h3 id="what-connected-devices-see">
  Lo que ven los dispositivos conectados
</h3>

Un dispositivo conectado muestra la conversación en su terminal mientras sucede. Estos casos van más allá de los mensajes ordinarios:

* **Compactación y `/clear`**: mientras Claude Code [compacta la conversación](/docs/es/context-window#what-survives-compaction), los dispositivos conectados muestran el progreso y luego dónde se compactó la conversación. Cuando ejecuta `/clear`, la conversación se reinicia en los dispositivos conectados también.
* **Cambiar conversaciones con `/resume`**: el dispositivo conectado no recibe el título o historial anterior de la conversación a la que se cambió, pero los nuevos mensajes en ambas direcciones van hacia y desde cualquier conversación que esté abierta en su terminal. Para trabajar en la conversación original desde el dispositivo nuevamente, ejecute `/resume` en su terminal y vuelva a cambiar a ella.
* **Extraer una sesión con `/teleport`**: cuando extrae una [sesión de Claude Code en la web](/docs/es/claude-code-on-the-web#from-cloud-to-terminal) en su terminal con `/teleport`, el dispositivo conectado no recibe el historial anterior de la conversación extraída. Los nuevos mensajes en ambas direcciones van hacia y desde la conversación extraída, que ahora es la que está abierta en su terminal.
* **Mensajes de sus otras sesiones**: con [mensajería entre sesiones](/docs/es/cross-session-messaging), la misma conexión lleva mensajes entre sus propias sesiones en diferentes máquinas y desde sus sesiones [Claude Code en la web](/docs/es/claude-code-on-the-web), a través de servidores de Anthropic como el resto del tráfico de Remote Control. [Mensajería de sesiones en otras máquinas](/docs/es/cross-session-messaging#message-sessions-on-other-machines) cubre las reglas de entrega y [Controle los mensajes entrantes](/docs/es/cross-session-messaging#control-inbound-messages) cubre los controles entrantes. Requiere Claude Code v2.1.224 o posterior.
* **Solicitudes que envía a mitad de turno**: cuando envía una solicitud desde un dispositivo conectado antes de que termine el turno actual, Claude Code la pone en cola y la mantiene en la transcripción del dispositivo después de que ese turno termina.
* **Diferencia de sus cambios**: cuando el directorio de la sesión está en un repositorio git, el panel de diferencia de un dispositivo conectado muestra sus cambios. El dispositivo solicita la diferencia sobre la conexión, y Claude Code la calcula en su máquina. En una rama que tiene confirmaciones por delante de la rama predeterminada del repositorio, el panel muestra los cambios desde que la rama se separó de ella, incluidas sus ediciones sin confirmar. En la rama predeterminada misma, o en una rama que no está por delante de ella, el panel muestra solo sus cambios sin confirmar. Antes de v2.1.247, Claude Code reportaba la diferencia a los dispositivos conectados solo en sesiones servidas por `claude remote-control`.
* **Modelo**: cuando elige un [modelo](/docs/es/model-config) desde un dispositivo conectado, Claude Code ejecuta la sesión en ese modelo. El selector `/model` del terminal, `/status`, y `/config` muestran ese modelo. Requiere Claude Code v2.1.238 o posterior.
  * Un modelo que elige desde el control de modelo del dispositivo se aplica solo a la sesión actual. Cuando envía `/model <name>` desde el dispositivo a una sesión interactiva, Claude Code también establece su predeterminado para nuevas sesiones.
  * Si envía un nombre que Claude Code no reconoce, como un nombre para mostrar donde se espera un ID de modelo, Claude Code [rechaza la selección](/docs/es/errors#model-is-not-a-recognized-model-id) y la sesión mantiene su modelo actual. Antes de v2.1.260, Claude Code guardaba una selección no reconocida desde el control de modelo del dispositivo, y su próximo mensaje fallaba.
* **Nivel de esfuerzo**: cuando establece el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) desde un dispositivo conectado, con `/effort` o el control de esfuerzo del dispositivo, Claude Code lo aplica a la sesión en su máquina, y claude.ai/code muestra el nivel que la sesión está usando. Si fijó un nivel con `CLAUDE_CODE_EFFORT_LEVEL`, la sesión mantiene ese nivel, y Claude Code rechaza una selección diferente desde el control de esfuerzo. Seleccionar un nivel desde el control de esfuerzo requiere Claude Code v2.1.234 o posterior en su máquina.
* **Reconexión después de un fallo de conexión**: ejecute `/remote-control` para reconectarse. Si la compactación rescribió la conversación o cambió conversaciones con `/resume` mientras tanto, Claude Code archiva la sesión del servidor que estaba usando en lugar de dejarla en la lista de sesiones. Aún puede encontrarla [filtrando sesiones archivadas](/docs/es/claude-code-on-the-web#archive-sessions). Cambiar conversaciones mientras un dispositivo aún está conectado no archiva la sesión.

<h3 id="enable-remote-control-for-all-sessions">
  Habilite Remote Control para todas las sesiones
</h3>

Remote Control solo se activa cuando ejecuta explícitamente `claude remote-control`, `claude --remote-control`, o `/remote-control`, a menos que la conexión automática esté activada. Para activar la conexión automática para cada sesión interactiva, ejecute `/config` dentro de Claude Code y establezca **Enable Remote Control for all sessions**. El botón de alternancia toma tres valores:

* **`true`**: conectarse automáticamente cuando comienza una sesión interactiva.
* **`false`**: desactivar la conexión automática, aunque un `true` de [configuración administrada](/docs/es/managed-settings) lo supera, porque Claude Code guarda la opción en su configuración de usuario. Un `false` en configuración de proyecto o local (`.claude/settings.json`, `.claude/settings.local.json`) desactiva la conexión automática incluso sobre un `true` administrado.
* **`default`**: borrar su opción y seguir el predeterminado de administrador de su organización si uno está establecido, de lo contrario el predeterminado actual de Claude Code.

El mismo botón de alternancia aparece fuera de la CLI:

* **Aplicación de escritorio**: **Settings > Claude Code > Enable remote control by default**.
* **Extensión de VS Code**: **Enable Remote Control for all sessions** en la sección Configuración del [menú de comandos](/docs/es/vs-code#use-the-prompt-box). Requiere Claude Code v2.1.203 o posterior.

Para activar la conexión automática desde un archivo de configuración en su lugar, establezca [`remoteControlAtStartup`](/docs/es/settings-reference#remotecontrolatstartup) en `true` en su usuario `~/.claude/settings.json` o en [configuración administrada](/docs/es/managed-settings). En configuración de proyecto o local (`.claude/settings.json`, `.claude/settings.local.json`), Claude Code honra un `false` y desactiva la conexión automática para ese repositorio, pero ignora un `true`, por lo que un archivo registrado no puede activar Remote Control para todos los que abren el repositorio.

La conexión automática inicia sesión con su propia cuenta de claude.ai, por lo que una sesión que inicia aparece solo en sus propias aplicaciones Claude y no otorga acceso a nadie más.

Con esta configuración activada, cada proceso interactivo de Claude Code registra una sesión remota. Si ejecuta varias instancias, cada una obtiene su propia sesión remota. Para ejecutar varias sesiones concurrentes desde un único proceso, use [modo servidor](#start-a-remote-control-session) en su lugar.

<h3 id="resume-sessions-after-stopping-the-server">
  Reanude sesiones después de detener el servidor
</h3>

Cuando detiene `claude remote-control` con Ctrl+C, las sesiones que estaba sirviendo dejan de responder desde su teléfono o navegador. Siempre que no estuviera ejecutando otro `claude remote-control` en el mismo directorio y no haya iniciado este con `--no-create-session-in-dir`, Claude Code no las archiva. Para traerlas de vuelta, ejecute uno de estos comandos en el mismo directorio:

* **`claude remote-control`**: trae de vuelta cada sesión que el servidor estaba sirviendo.
* **`claude remote-control --continue`**: trae de vuelta solo la sesión que el servidor inició, y sale cuando esa sesión termina. Si este directorio no tiene registro, Claude Code usa la más nueva de los otros git worktrees de este repositorio.
* **`claude remote-control --session-id <id>`**: trae de vuelta solo la sesión cuyo ID pasa, y sale cuando esa sesión termina. El ID es la parte de la URL de la sesión en claude.ai/code entre `/code/` y cualquier `?`.

Estos comandos funcionan durante aproximadamente cuatro horas después de que el servidor se detuvo. Después de eso, ejecute `claude remote-control` para iniciar una nueva sesión. Si archivó una sesión mientras tanto, `--continue` y `--session-id` la desarchivan en Claude Code v2.1.228 o posterior.

Para traer de vuelta una sesión que inició con `claude --remote-control` o `/remote-control`, reanude la conversación con `claude --continue` o `claude --resume`. Si Claude Code se reconecta, y a qué sesión, depende del [registro de reconexión](#resume-outcomes) de la conversación.

Si reanuda la conversación en una segunda terminal mientras la primera aún tiene Remote Control activado, Claude Code imprime un aviso en la segunda terminal y deja Remote Control desactivado allí en lugar de tomar la sesión de la primera. Mientras Remote Control permanece desactivado allí, Claude en esa terminal no ve [sus sesiones en otras máquinas](/docs/es/cross-session-messaging#see-which-sessions-claude-can-reach), y no pueden alcanzarlo. Ejecute `/remote-control` en la segunda terminal para mover Remote Control a ella.

Cuando reanuda una conversación en Claude Desktop o una extensión de IDE que tenía Remote Control activado, Claude Code lo vuelve a adjuntar a la sesión de claude.ai existente en lugar de agregar una nueva a la lista de sesiones.

<h2 id="connection-and-security">
  Conexión y seguridad
</h2>

Su sesión local de Claude Code realiza solo solicitudes HTTPS salientes y nunca abre puertos entrantes en su máquina. Cuando inicia Remote Control, se registra con la API de Anthropic y sondea el trabajo. Cuando se conecta desde otro dispositivo, el servidor enruta mensajes entre el cliente web o móvil y su sesión local a través de una conexión de transmisión.

Todo el tráfico viaja a través de la API de Anthropic sobre TLS, el mismo transporte de seguridad que cualquier sesión de Claude Code. La conexión utiliza múltiples credenciales de corta duración, cada una limitada a un único propósito y expirando de forma independiente. Cuando la credencial de registro de un servidor `claude remote-control` expira, el servidor se registra nuevamente con la API de Anthropic y continúa sirviendo sus sesiones.

Mientras Remote Control está conectado, la transcripción de la sesión, incluidos sus mensajes, las respuestas de Claude y la actividad de herramientas, se almacena en los servidores de Anthropic. La transcripción almacenada mantiene la conversación sincronizada en todos sus dispositivos y permite que la sesión se reconecte después de una caída de red. La ejecución y el acceso al sistema de archivos permanecen en su máquina, y las transcripciones almacenadas se retienen bajo la política de [Uso de datos](/docs/es/data-usage).

Para desactivar Remote Control completamente, utilice la configuración [`disableRemoteControl`](/docs/es/settings-reference#disableremotecontrol). Las organizaciones con requisitos de cumplimiento como Retención Cero de Datos no pueden habilitar Remote Control.

<h2 id="trusted-devices">
  Dispositivos de confianza
</h2>

<Note>
  Trusted Devices está actualmente en beta. Las características y funcionalidades pueden evolucionar a medida que se refina la experiencia.

  Trusted Devices está disponible en planes Pro, Max, Team y Enterprise y está deshabilitado de forma predeterminada. En planes Team y Enterprise, un propietario lo habilita para la organización. En planes Pro y Max, usted habilita **Require trusted devices** en su configuración, en la página Cowork o Account.
</Note>

Trusted Devices requiere que cada miembro de su organización, o usted solo en un plan Pro o Max, verifique su dispositivo antes de poder ver o controlar sesiones de Remote Control desde claude.ai, las aplicaciones móviles de Claude o Claude Desktop. Vincula el acceso a Remote Control a un dispositivo conocido y una autenticación reciente, no solo a una cuenta con sesión iniciada.

Cuando la configuración está activada, interactuar con una sesión de Remote Control requiere ambas de las siguientes:

* **Un dispositivo inscrito**: cada navegador, teléfono o aplicación de escritorio que un miembro usa para Remote Control inscribe su propia credencial. La inscripción solo se ofrece poco después de un inicio de sesión completo, por lo que un dispositivo se une a la lista de confianza como parte de una autenticación real en lugar de silenciosamente en el fondo.
* **Un inicio de sesión reciente**: el inicio de sesión del miembro no debe tener más de 18 horas. En lugar de iniciar sesión nuevamente cada día, los miembros confirman presencia con Face ID, Touch ID, Windows Hello o una passkey. Este paso de autenticación biométrica actualiza la sesión inmediatamente.

Las verificaciones biométricas se ejecutan en el dispositivo a través del sistema operativo o navegador, el mismo mecanismo que el inicio de sesión con passkey. Anthropic nunca recibe ni almacena huellas dactilares, datos faciales ni ninguna otra información biométrica. Solo se almacenan la clave pública del dispositivo y metadatos básicos como nombre de pantalla, plataforma y hora de inscripción.

La configuración se aplica solo a Remote Control. El chat regular de Claude, Claude Code en la terminal y el uso de API no se ven afectados.

<h3 id="enable-trusted-devices-for-your-organization">
  Habilite Trusted Devices para una organización Team o Enterprise
</h3>

Un propietario habilita la configuración desde la configuración de la organización de claude.ai.

<Steps>
  <Step title="Vaya a la página Capabilities">
    Vaya a [**Organization settings > Capabilities > Remote sessions**](https://claude.ai/admin-settings/capabilities). El botón de alternancia **Require trusted devices** aparece en esa sección.
  </Step>

  <Step title="Active Require trusted devices">
    La configuración se aplica a cada miembro de la organización y a las sesiones de Remote Control iniciadas después de habilitar la opción. Las sesiones que ya se estaban ejecutando antes de activar el botón de alternancia no están protegidas retroactivamente y continúan sin el requisito de dispositivo hasta que finalicen. El alcance por equipo o por proyecto no está disponible.
  </Step>

  <Step title="Informe a los miembros qué esperar">
    La primera vez que un miembro ve o controla una nueva sesión de Remote Control desde un navegador, teléfono o aplicación de escritorio después de habilitar la configuración, se le solicita que inscriba ese dispositivo. Informarles con anticipación evita confusión.
  </Step>
</Steps>

<h3 id="what-members-see">
  Qué ven los miembros
</h3>

La inscripción es un paso único por dispositivo. Después de eso, el único cambio visible es un mensaje biométrico ocasional.

* **Primer uso en cada dispositivo**: se solicita al miembro que se inscriba. Si su inicio de sesión no es reciente, primero inicia sesión a través de su flujo normal, incluido SSO si está configurado, y luego confirma la inscripción.
* **Día a día**: los miembros con un dispositivo inscrito y un inicio de sesión reciente no ven mensajes. Cuando el inicio de sesión envejece más de 18 horas, la siguiente interacción de Remote Control muestra un único mensaje de Face ID, Touch ID, Windows Hello o passkey.
* **Dispositivos no inscritos**: las sesiones de Remote Control no se pueden ver ni controlar hasta que el dispositivo esté inscrito. El chat regular de Claude en ese dispositivo no se ve afectado.
* **Sin autenticador de plataforma**: los miembros en una máquina sin Face ID, Touch ID o Windows Hello pueden usar una clave de seguridad de hardware, o iniciar sesión nuevamente en lugar de autenticarse.
* **En la terminal**: la máquina que ejecuta Claude Code recibe su propia credencial automáticamente cuando el desarrollador inicia sesión en la CLI. No hay un paso de inscripción separado en la terminal.

<h3 id="manage-enrolled-devices">
  Administre dispositivos inscritos
</h3>

Los miembros pueden revisar y revocar sus propios dispositivos desde la configuración de la cuenta.

Abra [claude.ai/settings/account](https://claude.ai/settings/account#trusted-devices) y encuentre la sección **Trusted devices** para ver cada dispositivo inscrito con su nombre, plataforma y fecha de inscripción. Eliminar un dispositivo revoca su credencial inmediatamente, y el dispositivo puede reinscribirse más tarde después de un nuevo inicio de sesión. Las credenciales también expiran por sí solas si no se renuevan, por lo que un dispositivo no utilizado se cae de la lista de confianza automáticamente.

Para un dispositivo perdido o robado, el miembro lo elimina de esta página. Si el miembro no puede iniciar sesión, un administrador puede usar **Sign out everywhere** en la consola de administración para revocar cada sesión y dispositivo inscrito para ese miembro, después de lo cual el miembro reinscribe los dispositivos que aún posee.

<h2 id="remote-control-vs-cloud-sessions">
  Remote Control vs sesiones en la nube
</h2>

Remote Control y [sesiones en la nube](/docs/es/claude-code-on-the-web) ambos usan la interfaz claude.ai/code. La diferencia clave es dónde se ejecuta la sesión: Remote Control se ejecuta en su máquina, por lo que sus servidores MCP locales, herramientas y configuración del proyecto permanecen disponibles. Una sesión en la nube se ejecuta en infraestructura en la nube, administrada por Anthropic de forma predeterminada.

Use Remote Control cuando esté en medio del trabajo local y desee continuar desde otro dispositivo. Use una sesión en la nube cuando desee iniciar una tarea sin ninguna configuración local, trabajar en un repositorio que no tiene clonado, o ejecutar varias tareas en paralelo.

<h2 id="mobile-push-notifications">
  Notificaciones push móviles
</h2>

Cuando Remote Control está activo, Claude puede enviar notificaciones push a su teléfono.

Claude decide cuándo enviar. Típicamente envía una cuando una tarea de larga duración finaliza o cuando necesita una decisión de usted para continuar. También puede solicitar un push en su solicitud, por ejemplo `notify me when the tests finish`. Más allá de los dos botones de alternancia activado/desactivado a continuación, no hay configuración por evento.

Para configurar notificaciones push móviles:

<Steps>
  <Step title="Instale la aplicación móvil Claude">
    Descargue la aplicación Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) o [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude).
  </Step>

  <Step title="Inicie sesión con su cuenta de Claude Code">
    Use la misma cuenta y organización que usa para Claude Code en la terminal.
  </Step>

  <Step title="Permita notificaciones">
    Acepte el mensaje de solicitud de permiso de notificación del sistema operativo.
  </Step>

  <Step title="Habilite push en Claude Code">
    En su terminal, ejecute `/config` y habilite **Push when Claude decides** para notificaciones proactivas, **Push when actions required** para mensajes de solicitud de permiso y preguntas, o ambos.
  </Step>
</Steps>

Si las notificaciones no llegan:

* Si `/config` muestra **No mobile registered**, abra la aplicación Claude en su teléfono para que pueda actualizar su token push. La advertencia se borra la próxima vez que Remote Control se conecte.
* En iOS, los modos Focus y los resúmenes de notificaciones pueden suprimir o retrasar los pushes. Verifique Configuración → Notificaciones → Claude.
* En Android, la optimización agresiva de batería puede retrasar la entrega. Exima la aplicación Claude de la optimización de batería en la configuración del sistema.

Claude Code omite las notificaciones push móviles mientras usted está escribiendo o enfocado en la terminal conectada. A partir de v2.1.181, puede establecer [`CLAUDE_CLIENT_PRESENCE_FILE`](/docs/es/env-vars) en una ruta de archivo marcador para extender esto a cualquier momento en que esté en la máquina, incluso en otra ventana: las notificaciones se omiten mientras el archivo existe. Configure un escucha de bloqueo de pantalla o una herramienta similar para crear el archivo cuando su pantalla se desbloquea y eliminarlo cuando su pantalla se bloquea.

<h2 id="limitations">
  Limitaciones
</h2>

* **Una sesión remota por proceso interactivo**: fuera del modo servidor, cada instancia de Claude Code admite una sesión remota a la vez. Use el [modo servidor](#start-a-remote-control-session) para ejecutar varias sesiones concurrentes desde un único proceso.
* **El proceso local debe seguir ejecutándose**: Remote Control se ejecuta como un proceso local. Si cierra la terminal, cierra VS Code, o detiene el proceso `claude` de otra manera, la sesión se desconecta hasta que la [reanude](#resume-sessions-after-stopping-the-server). A menos que Claude esté en medio de una tarea, claude.ai y la aplicación Claude muestran la sesión como desconectada en segundos después de que el proceso se cierre. Para mantener una sesión ejecutándose en una máquina remota después de desconectarse de SSH, inicie la sesión dentro de `tmux` o `screen`.
* **Sesiones bloqueadas en modo servidor**: si una sesión servida por `claude remote-control` se bloquea, envíele un mensaje desde un dispositivo conectado. Claude Code la sirve nuevamente. No tiene que reiniciar el servidor. Requiere Claude Code v2.1.238 o posterior.
* **Rechazos HTTP 403 en una sesión conectada**: una vez que una sesión interactiva está conectada, Claude Code sigue reintentando hasta tres minutos cuando algo entre su máquina y los servidores de Anthropic responde con HTTP 403, como puede suceder después de un cambio de VPN o red. Si los rechazos duran más tiempo, Claude Code se desconecta, y el motivo nombra lo que rechazó: un borde de red, o un proxy, VPN o firewall en su propia red.
* **Interrupción de red extendida**: si su máquina está despierta pero no puede alcanzar la red, lo que haga a continuación depende del modo:
  * **Modo servidor**: Claude Code se rinde después de aproximadamente 10 minutos y el proceso `claude remote-control` se cierra. Ejecute `claude remote-control` nuevamente para iniciar una nueva sesión.
  * **Sesión interactiva**: siga trabajando localmente. Claude Code reintenta mientras dure la interrupción y se reconecta automáticamente cuando la red vuelva.
* **Latidos de presencia fallidos**: si una sesión interactiva se desconecta con `could not reach the Remote Control server for about 30 minutes`, ejecute `/remote-control` para reconectarse. Claude Code muestra este mensaje solo cuando los latidos de presencia de la sesión han estado fallando mientras el resto de la conexión se mantuvo activa; vuelve a registrar la sesión durante aproximadamente 30 minutos antes de desconectarse.
* **Los diálogos reenviados expiran**: Claude Code mantiene los avisos de permiso y las preguntas de `AskUserQuestion` abiertas hasta que las responda. Cuando Claude Code reenvía otro tipo de diálogo a la sesión remota, como el aviso de selección de modelo que se muestra después de un rechazo de seguridad, espera cinco minutos por defecto, luego cierra el diálogo y continúa con el valor predeterminado sin acción del diálogo. Establezca [`dialogExpiry`](/docs/es/settings-reference#dialogexpiry) para ajustar o deshabilitar el plazo. Requiere Claude Code v2.1.224 o posterior.
* **El aviso de consentimiento de créditos de uso de Fable no se reenvía**: Claude Code muestra el aviso de consentimiento de créditos de uso de [Fable](/docs/es/model-config#fable-and-usage-credits) a mitad de sesión solo donde se ejecuta la sesión, no en su dispositivo. Cuando la sesión se ejecuta en una terminal y nadie allí responde antes de que Claude Code cierre el aviso, el turno finaliza sin enviar la solicitud; consulte [El aviso para confirmar no fue respondido](/docs/es/errors#the-prompt-to-confirm-went-unanswered).
* **Algunos comandos son solo locales**: comandos que solo se ejecutan en la interfaz de terminal, como `/plugin` o `/resume`, funcionan solo desde la CLI local, independientemente de si pasa un argumento o no. Los siguientes funcionan desde móvil y web:
  * Comandos de salida de texto: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits`, `/recap` y `/reload-plugins`. `/usage-credits` imprime la URL de facturación en lugar de abrir un navegador. `/reload-plugins` funciona solo cuando la sesión se ejecuta en una terminal interactiva; una sesión sin una la rechaza.
  * `/model`, `/effort`, `/fast`, `/color` y `/rename`: pase el valor como argumento, por ejemplo `/model sonnet` o `/effort high`. Desde móvil y web, `/model` y `/effort` toman el argumento en lugar del selector de terminal o deslizador.
  * `/mcp`: desde la aplicación móvil, devuelve un resumen de texto del estado del servidor en lugar de abrir el selector. En la web, `/mcp` por sí solo abre un directorio de [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) en lugar de devolver el resumen. Los [subcomandos](/docs/es/commands#all-commands) `reconnect`, `enable` y `disable` funcionan desde ambos. A diferencia de la CLI local, `/mcp reconnect` sin nombre de servidor reconecta cada servidor que ha fallado o necesita autenticación.
  * `/config`: desde la aplicación móvil, pase `key=value` para establecer una configuración, o ejecútelo sin argumentos para listar las claves que puede establecer. En la web, `/config` abre la sección Claude Code de su configuración en su lugar, e ignora el texto después del comando.
  * En Team y Enterprise, `/usage-credits` desde móvil o web no envía una [solicitud de créditos de uso a su administrador](/docs/es/costs#add-usage-credits-to-your-subscription). El envío requiere una confirmación que aparece solo en la CLI interactiva, por lo que el comando le indica que lo ejecute allí en su lugar. Antes de v2.1.211, el formulario de texto enviaba la solicitud sin confirmación.
  * `/autocompact`, a partir de v2.1.221: pase el tamaño de la ventana como argumento, por ejemplo `/autocompact 500k`. Sin argumento, imprime el tamaño de ventana actual como texto en lugar de abrir el diálogo que muestra el comando en una sesión de terminal.
  * `/advisor`, a partir de v2.1.260: pase el modelo como argumento, por ejemplo `/advisor opus`, o pase `off` para desactivar el asesor. Ambas formas se aplican solo a la sesión actual y dejan su valor predeterminado guardado sin cambios. Sin argumento, imprime el asesor actual como texto en lugar de abrir el selector.
  * `/output-style`, a partir de v2.1.269: pase el nombre del estilo como argumento, por ejemplo `/output-style concise`, o ejecútelo sin argumento para listar los estilos. Desde móvil y web, puede listar y seleccionar solo [estilos integrados](/docs/es/output-styles#built-in-output-styles). Para usar un [estilo personalizado](/docs/es/output-styles#create-a-custom-output-style), selecciónelo en la sesión misma.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  "Remote Control requires a claude.ai subscription"
</h3>

No está autenticado con una cuenta de claude.ai, o bien otra credencial está tomando precedencia sobre su inicio de sesión. El mensaje toma una de estas formas:

* Sin sesión iniciada, desde `/remote-control` o `--remote-control`: `Remote Control requires a claude.ai subscription.` o `/remote-control requires a claude.ai subscription.`
* Sin sesión iniciada, desde `claude remote-control`: `You must be logged in to use Remote Control. Remote Control is only available with claude.ai subscriptions.`
* Con sesión iniciada, pero se está usando una clave API o token: `Remote Control requires claude.ai subscription auth.` seguido de la credencial en uso, como `ANTHROPIC_API_KEY is set, so this session is using API-key auth`. Una configuración `apiKeyHelper` y `ANTHROPIC_AUTH_TOKEN` se nombran de la misma manera.

Ejecute `claude auth login` y elija la opción de claude.ai. Si el mensaje menciona `ANTHROPIC_API_KEY` o `ANTHROPIC_AUTH_TOKEN`, elimínelo dondequiera que esté configurado: su entorno de shell o el bloque `env` de un [archivo de configuración](/docs/es/settings-reference#env). Si menciona `apiKeyHelper`, elimine esa configuración.

Antes de v2.1.206, ejecutar `/remote-control` mientras no estaba conectado reportaba `Unknown command: /remote-control` en lugar de este mensaje.

<h3 id="remote-control-requires-a-full-scope-login-token">
  "Remote Control requires a full-scope login token"
</h3>

Está autenticado con un token de larga duración de `claude setup-token` o la variable de entorno `CLAUDE_CODE_OAUTH_TOKEN`. Estos tokens solo pueden hacer solicitudes de modelo, por lo que no pueden establecer sesiones de Remote Control. Ejecute `claude auth login` para autenticarse con un token de sesión de alcance completo en su lugar.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  "Unable to determine your organization for Remote Control eligibility"
</h3>

Su información de cuenta en caché está obsoleta o incompleta. Ejecute `claude auth login` para actualizarla.

<h3 id="remote-control-isn’t-enabled-for-this-account">
  "Remote Control isn't enabled for this account"
</h3>

Claude Code verificó la disponibilidad de Remote Control para la cuenta con la que está conectado y la verificación resultó desactivada. La causa habitual son los derechos en caché que están desactualizados después de un cambio de plan. Ejecute `claude auth logout` y luego `claude auth login` para actualizarlos, y actualice Claude Code si está usando una versión antigua.

Ejecute `claude doctor` para ver qué verificación de elegibilidad individual falló. Los conflictos de variables de entorno, las verificaciones inaccesibles y la configuración de Remote Control de su organización producen cada uno su propio mensaje, por lo que este error significa la verificación a nivel de cuenta en sí.

Antes de v2.1.239, este mensaje decía "Remote Control is not yet enabled for your account". Antes de v2.1.154, una variable que desactiva la evaluación de banderas de características, como `DISABLE_TELEMETRY` o `DO_NOT_TRACK`, también producía este mensaje; la entrada "Remote Control requires feature-flag evaluation" a continuación cubre esa configuración.

<h3 id="couldn’t-verify-remote-control-eligibility">
  "Couldn't verify Remote Control eligibility"
</h3>

Claude Code no pudo alcanzar el servicio de banderas de características para verificar si Remote Control está habilitado para su cuenta, típicamente porque está sin conexión o un proxy está bloqueando la solicitud. Reintente una vez que tenga acceso a la red, o ejecute `claude doctor` para obtener detalles. El mensaje relacionado "Couldn't verify your organization's Remote Control policy" significa que Claude Code no pudo leer esa política, y tiene la misma solución. Ambos mensajes se agregaron en v2.1.178.

<h3 id="remote-control-requires-feature-flag-evaluation">
  "Remote Control requires feature-flag evaluation"
</h3>

Una de estas variables está configurada: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, o `DISABLE_GROWTHBOOK`](/docs/es/env-vars). Cada una de ellas desactiva la evaluación de banderas de características de la que depende la disponibilidad de Remote Control, y el mensaje completo nombra la variable que Claude Code encontró. Desactive esa variable dondequiera que esté configurada, en su entorno de shell o en el bloque `env` de un [archivo `settings.json`](/docs/es/settings-reference#all-settings). En versiones anteriores a 2.1.154, la misma configuración produce "Remote Control is not yet enabled for your account" en su lugar.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  "Remote Control is only available when using Claude via api.anthropic.com"
</h3>

La sesión no está hablando directamente con la API de Anthropic, por lo que no hay un backend de claude.ai para emparejar. Esto ocurre en Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry. También ocurre cuando [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) apunta a un host distinto de `api.anthropic.com`, como una [puerta de enlace LLM](/docs/es/llm-gateway) o proxy, incluso si inicia sesión con claude.ai. Antes de v2.1.196, Claude Code no mostraba este mensaje para un `ANTHROPIC_BASE_URL` personalizado. Vea la [referencia de errores](/docs/es/errors#remote-control-requires-the-anthropic-api) para la lista completa de causas.

El mensaje nombra lo que enrutó la sesión lejos de la API de Anthropic, como `CLAUDE_CODE_USE_BEDROCK` o un `ANTHROPIC_BASE_URL` personalizado. Si tiene un inicio de sesión de claude.ai elegible, desactive la variable nombrada, elimínela de la clave `env` en [configuración](/docs/es/settings) si la configuró allí, y reinicie la sesión. Antes de v2.1.219, el mensaje era solo la oración en el encabezado de esta sección, por lo que en versiones anteriores verifique su entorno usted mismo para variables de proveedor como `CLAUDE_CODE_USE_BEDROCK` y `CLAUDE_CODE_USE_VERTEX`, y para `ANTHROPIC_BASE_URL`.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  "Remote Control is disabled by your organization's policy"
</h3>

Una política bloquea Remote Control, o Claude Code no pudo cargar la política de su organización en esta máquina y mantiene Remote Control desactivado mientras tanto. Verifique estas causas en orden:

* **El error menciona `disableRemoteControl`**: su administrador de TI ha deshabilitado Remote Control en este dispositivo a través de [configuración administrada](/docs/es/managed-settings), independientemente del botón de alternancia de toda la organización y de cómo esté conectado.
* **Su plan de claude.ai es Pro o Max**: Claude Code aún está conectado bajo una organización Team o Enterprise de un inicio de sesión anterior, por lo que verifica la política de Remote Control de esa organización. Ejecute `/status` para ver qué plan y organización usa su inicio de sesión. Ejecute `claude auth logout` y luego `claude auth login` para conectarse nuevamente bajo su plan actual.
* **La política de la organización no se cargó en esta máquina**: ejecute `claude doctor` y lea la línea `Organization policy`. Si la línea muestra que la política no está cargada, eso es lo que mantiene Remote Control desactivado. Antes de v2.1.261, `claude doctor` no imprimía esta línea.
* **El mensaje no dice que contacte a su administrador de la organización**: su organización tiene una configuración de HIPAA que es incompatible con Remote Control, y `/status` enumera `HIPAA` en su fila `Compliance`. En este estado, el botón de alternancia de Remote Control del panel de administración está atenuado, por lo que un propietario no puede cambiarlo allí. Póngase en contacto con el soporte de Anthropic para discutir opciones. Antes de v2.1.267, este caso mostraba "Remote Control isn't available for your organization due to its compliance policy" en su lugar.
* **De lo contrario, un propietario no lo ha habilitado para su organización**: Remote Control está desactivado de forma predeterminada en los planes Team y Enterprise. Un propietario puede habilitarlo en [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) activando el botón de alternancia **Remote Control**. Este botón de alternancia es una configuración de organización del lado del servidor.

<h3 id="remote-credentials-fetch-failed">
  "Remote credentials fetch failed"
</h3>

Claude Code no pudo obtener una credencial de corta duración de la API de Anthropic para establecer la conexión. Vuelva a ejecutar con `--verbose` para ver el error completo:

```bash theme={null}
claude remote-control --verbose
```

Causas comunes:

* No ha iniciado sesión: ejecute `claude` y use `/login` para autenticarse con su cuenta de claude.ai. La autenticación con clave API no es compatible con Remote Control.
* Problema de red o proxy: un firewall o proxy puede estar bloqueando la solicitud HTTPS saliente. Remote Control requiere acceso a la API de Anthropic en el puerto 443.
* Error en la creación de sesión: si también ve `Session creation failed — see debug log`, el error ocurrió antes en la configuración. Verifique que su suscripción esté activa.

Un token de inicio de sesión obsoleto no causa este error. Cuando la API de Anthropic rechaza el token guardado, por ejemplo porque otro proceso de Claude Code ya lo actualizó, Claude Code actualiza el token y reintenta por su cuenta. Antes de v2.1.224, un token obsoleto fallaba en el inicio de Remote Control con este mensaje, por lo que las sesiones configuradas para [conectarse automáticamente](#enable-remote-control-for-all-sessions) podrían fallar intermitentemente al iniciarse.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  "Couldn't reconnect to your Remote Control session"
</h3>

Cuando reanuda una conversación con `claude --resume` o `claude --continue`, Claude Code se reconecta a la sesión de Remote Control registrada en esa conversación. Este mensaje significa que la reconexión falló por una razón que puede ser temporal, como una interrupción de red o un error del servidor, por lo que Claude Code no puede confirmar si la sesión remota aún existe.

Ejecute `/remote-control` para reintentar la conexión, o inicie una nueva sesión con `claude --remote-control` para crear una nueva sesión de Remote Control. Su sesión local continúa ejecutándose sin Remote Control mientras tanto.

<span id="resume-outcomes" />Cuando reanuda, también puede obtener uno de estos resultados en lugar de este mensaje:

* **El servidor reporta que la sesión registrada se ha ido, o el registro de reconexión nombra una cuenta diferente**: Claude Code se guía por lo que dice el registro de reconexión de la conversación:
  * **El registro nombra su cuenta conectada**: Claude Code inicia una sesión de reemplazo con un nombre generado automáticamente y deja los mensajes anteriores de la conversación fuera de ella. Obtiene esto después de eliminar la sesión de claude.ai o de la aplicación Claude, por ejemplo.
  * **El registro nombra una cuenta diferente**: Claude Code inicia una nueva sesión sin los mensajes anteriores de la conversación y sin mostrar un mensaje, independientemente de si la sesión registrada aún existe.
  * **El registro no dice qué cuenta era propietaria de la sesión, o Claude Code no puede leer su inicio de sesión guardado**: Claude Code muestra [`Previous session is unavailable — run /remote-control to start a new one`](#previous-session-is-unavailable) en lugar de este mensaje, no inicia nada, y elimina el registro de la conversación.
* **Desactivó Remote Control antes de reanudar**: a menos que la aplicación que aloja Claude Code le haya dicho que la aplicación es propietaria de la sesión de claude.ai, Claude Code eliminó el registro de reconexión cuando desactivó Remote Control desde el [panel de estado](#check-connection-status) de la CLI, la extensión de VS Code, o un host construido en el [Agent SDK](/docs/es/agent-sdk/overview), por lo que no se reconecta. Cuando una aplicación propietaria lo desactivó, Claude Code mantuvo el registro y se reconecta.
* **Otro Claude Code en esta máquina aún tiene la sesión**: ve un aviso que comienza con `Remote Control not started here`, y Claude Code [deja Remote Control desactivado en la sesión reanudada](#resume-sessions-after-stopping-the-server). Ejecute `/remote-control` allí para moverlo.

<span id="reconnect-history" />Antes de v2.1.232, Claude Code respondía de manera diferente cuando el servidor reportaba que la sesión registrada se había ido. De v2.1.227 a v2.1.231, Claude Code se negaba a iniciar un reemplazo incluso cuando el registro coincidía con su cuenta. Hasta v2.1.226, Claude Code iniciaba un reemplazo independientemente de si el registro coincidía con su cuenta, y en v2.1.224 a v2.1.226 lo creaba bajo la cuenta conectada en esa máquina, nunca bajo la cuenta de otro, sin cargar los mensajes anteriores de la conversación en ella. Antes de v2.1.200, Claude Code creaba una nueva sesión después de cualquier falla de reconexión.

<h3 id="previous-session-is-unavailable">
  "Previous session is unavailable — run /remote-control to start a new one"
</h3>

Claude Code no pudo recuperar la sesión anterior de Remote Control y se detuvo en lugar de iniciar una nueva por su cuenta. Puede ver este mensaje después de reanudar una conversación con `claude --resume` o `claude --continue`, o después de que Claude Code [se reconecte por su cuenta después de una desconexión](/docs/es/errors#remote-control-couldnt-refresh-your-login).

Ejecute `/remote-control` para iniciar una nueva sesión de Remote Control bajo el inicio de sesión actual; su sesión local continúa ejecutándose sin Remote Control mientras tanto. El mensaje relacionado `Remote Control could not verify the signed-in account — run /remote-control to reconnect` tiene la misma solución; Claude Code lo muestra cuando la cuenta conectada cambió o no se pudo leer entre validarla y reconectarse. Si ejecuta `/remote-control` después de `Previous session is unavailable` sin reiniciar Claude Code primero, Claude Code deja los mensajes anteriores de la conversación fuera de la nueva sesión.

Al reanudar, Claude Code [inicia una nueva sesión en su lugar](#resume-outcomes) solo si el registro de reconexión de la conversación nombra la cuenta que era propietaria de la sesión, porque el servidor reporta una sesión que eliminó y una sesión propiedad de otra cuenta de la misma manera. Claude Code anterior a v2.1.227 no registraba esa cuenta, y Claude Code no puede verificar el registro cuando no puede leer su inicio de sesión guardado. Claude Code anterior a v2.1.232 mostraba `Remote Control could not resume the previous session under the current login — run /remote-control to start fresh` en su lugar, en [un conjunto diferente de casos](#reconnect-history).

<h3 id="remote-control-got-an-unexpected-server-response">
  "Remote Control got an unexpected server response"
</h3>

El servidor de Remote Control aceptó una solicitud pero respondió de una forma que esta versión de Claude Code no pudo leer, mientras creaba la sesión remota u obtenía sus credenciales. Reintentar en la misma versión falla de la misma manera. Ejecute `claude update`, luego ejecute `/remote-control` para reconectarse. Este mensaje se agregó en v2.1.225.

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  "Your organization requires Trusted Devices for Remote Control, but this device is not enrolled"
</h3>

Su organización tiene [Trusted Devices](#trusted-devices) habilitado y esta máquina aún no se ha inscrito. Ejecute `/login` en Claude Code. La inscripción ocurre como parte del inicio de sesión, y no hay un comando de inscripción separado.

<h3 id="session-expired-for-trusted-device-check">
  "session expired for trusted-device check"
</h3>

Su inicio de sesión tiene más de 18 horas. Ejecute `/login` en Claude Code, o confirme con Face ID, Touch ID, Windows Hello o una passkey cuando claude.ai o la aplicación móvil se lo solicite. Vea [Trusted Devices](#trusted-devices).

<h2 id="choose-the-right-approach">
  Elija el enfoque correcto
</h2>

Claude Code ofrece varias formas de trabajar cuando no está en su terminal. Difieren en lo que desencadena el trabajo, dónde se ejecuta Claude y cuánta configuración necesita.

|                                                         | Desencadenante                                                                                                  | Claude se ejecuta en                                                                       | Configuración                                                                                                                                  | Mejor para                                                         |
| :------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------- |
| [Dispatch](/docs/es/desktop#sessions-from-dispatch)          | Envíe un mensaje con una tarea desde la aplicación móvil de Claude                                              | Su máquina (Desktop)                                                                       | [Empareje la aplicación móvil con Desktop](https://support.claude.com/en/articles/13947068)                                                    | Delegar trabajo mientras está fuera, configuración mínima          |
| [Remote Control](/docs/es/remote-control)                    | Controle una sesión en ejecución desde [claude.ai/code](https://claude.ai/code) o la aplicación móvil de Claude | Su máquina (CLI o VS Code)                                                                 | Ejecute `claude remote-control`                                                                                                                | Dirigir el trabajo en progreso desde otro dispositivo              |
| [Channels](/docs/es/channels)                                | Envíe eventos desde una aplicación de chat como Telegram o Discord, o su propio servidor                        | Su máquina (CLI)                                                                           | [Instale un plugin de canal](/docs/es/channels#quickstart) o [cree el suyo propio](/docs/es/channels-reference)                                          | Reaccionar a eventos externos como fallos de CI o mensajes de chat |
| [Slack](/docs/es/slack)                                      | Mencione `@Claude` en un canal de equipo                                                                        | Nube de Anthropic                                                                          | [Instale la aplicación de Slack](/docs/es/slack#setting-up-claude-code-in-slack) con [Claude Code en la web](/docs/es/claude-code-on-the-web) habilitado | PRs y revisiones desde el chat del equipo                          |
| [Entornos autohospedados](/docs/es/self-hosted-environments) | Inicie una [sesión en la nube](/docs/es/claude-code-on-the-web) y seleccione el entorno de su organización           | La infraestructura de su organización                                                      | [Implemente ejecutores](/docs/es/self-hosted-environments-quickstart), en planes Team y Enterprise                                                  | Sesiones en la nube que deben ejecutarse dentro de su red          |
| [Tareas programadas](/docs/es/scheduled-tasks)               | Establezca una programación                                                                                     | [CLI](/docs/es/scheduled-tasks), [Desktop](/docs/es/desktop-scheduled-tasks), o [nube](/docs/es/routines) | Seleccione una frecuencia                                                                                                                      | Automatización recurrente como revisiones diarias                  |

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Claude Code en la web](/docs/es/claude-code-on-the-web): ejecute sesiones en la nube en lugar de en su máquina, configuradas a través de [entornos en la nube](/docs/es/cloud-environments)
* [Mensajería entre sesiones](/docs/es/cross-session-messaging): permita que Claude envíe mensajes a sus sesiones en otras máquinas o en [sesiones en la nube](/docs/es/claude-code-on-the-web)
* [Channels](/docs/es/channels): reenvíe Telegram, Discord o iMessage a una sesión para que Claude reaccione a los mensajes mientras está fuera
* [Dispatch](/docs/es/desktop#sessions-from-dispatch): envíe un mensaje de una tarea desde su teléfono y puede generar una sesión de Desktop para manejarla
* [Autenticación](/docs/es/authentication): configure `/login` y administre credenciales para claude.ai
* [Referencia de CLI](/docs/es/cli-reference): lista completa de banderas y comandos incluyendo `claude remote-control`
* [Seguridad](/docs/es/security): cómo las sesiones de Remote Control se ajustan al modelo de seguridad de Claude Code
* [Uso de datos](/docs/es/data-usage): qué datos fluyen a través de la API de Anthropic durante sesiones locales, Remote Control y en la nube
