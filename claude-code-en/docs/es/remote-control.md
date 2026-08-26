> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Continúe sesiones locales desde cualquier dispositivo con Remote Control

> Continúe una sesión local de Claude Code desde su teléfono, tableta o cualquier navegador usando Remote Control. Funciona con claude.ai/code y la aplicación móvil de Claude.

<Note>
  Remote Control está en vista previa de investigación y está disponible en todos los planes. En Team y Enterprise, está deshabilitado de forma predeterminada hasta que un propietario habilite el botón de alternancia de Remote Control en [configuración de administración de Claude Code](https://claude.ai/admin-settings/claude-code).
</Note>

Remote Control conecta [claude.ai/code](https://claude.ai/code) o la aplicación Claude para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) y [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) a una sesión de Claude Code que se ejecuta en su máquina. Inicie una tarea en su escritorio y luego continúela desde su teléfono en el sofá o desde un navegador en otra computadora.

Cuando inicia una sesión de Remote Control en su máquina, Claude sigue ejecutándose localmente todo el tiempo, por lo que su ejecución de código y acceso al sistema de archivos permanecen en su máquina. Con Remote Control puede:

* **Usar su entorno local completo de forma remota**: su sistema de archivos, [MCP servers](/docs/es/mcp), herramientas y configuración del proyecto permanecen disponibles, y escribir `@` completa automáticamente las rutas de archivo de su proyecto local
* **Trabajar desde ambas superficies a la vez**: la conversación y el progreso de [subagentes](/docs/es/sub-agents) y [flujos de trabajo dinámicos](/docs/es/workflows) se mantienen sincronizados en todos los dispositivos conectados, por lo que puede enviar mensajes desde su terminal, navegador y teléfono indistintamente. Antes de v2.1.207, las sesiones alojadas por la [aplicación de escritorio](/docs/es/desktop) no enviaban el progreso de subagentes o flujos de trabajo a los dispositivos conectados.
* **Enviar imágenes y archivos desde su teléfono o navegador**: cuando agrega un archivo adjunto en la aplicación Claude o en claude.ai/code, Claude Code lo descarga a su máquina y lo pasa a Claude como una referencia de archivo `@`, con o sin un título. Antes de v2.1.202, Claude Code podría descartar un archivo adjunto enviado sin un título antes de que llegara a la sesión.
* **Sobrevivir a interrupciones**: si su portátil se duerme o su red se cae, la sesión se reconecta automáticamente cuando su máquina vuelve a estar en línea. Claude Code pone en cola las actualizaciones de estado de subagentes y flujos de trabajo mientras se reconstruye la conexión y las entrega una vez que se recupera. Antes de v2.1.207, una actualización enviada durante una reconexión o actualización de credenciales podría perderse, por lo que el dispositivo conectado seguía mostrando una tarea terminada como en ejecución.

A diferencia de [Claude Code en la web](/docs/es/claude-code-on-the-web), que se ejecuta en infraestructura en la nube, las sesiones de Remote Control se ejecutan directamente en su máquina e interactúan con su sistema de archivos local. Las interfaces web y móvil son una ventana a esa sesión local.

Esta página cubre la configuración, cómo iniciar y conectarse a sesiones, y cómo Remote Control se compara con Claude Code en la web.

<h2 id="requirements">
  Requisitos
</h2>

Antes de usar Remote Control, confirme que su entorno cumple con estas condiciones:

* **Suscripción**: disponible en planes Pro, Max, Team y Enterprise. Las claves API no son compatibles. En Team y Enterprise, un propietario debe habilitar primero el botón de alternancia de Remote Control en [configuración de administración de Claude Code](https://claude.ai/admin-settings/claude-code).
* **Autenticación**: ejecute `claude` y use `/login` para iniciar sesión a través de claude.ai si aún no lo ha hecho.
* **Punto final de API**: no disponible en Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. A partir de v2.1.196, Remote Control también se deshabilita cuando [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) apunta a un host distinto de `api.anthropic.com`, como una [puerta de enlace LLM](/docs/es/llm-gateway) o proxy. Desactive la variable para usar Remote Control.
* **Confianza del espacio de trabajo**: ejecute `claude` en su directorio de proyecto al menos una vez para aceptar el diálogo de confianza del espacio de trabajo.

<h2 id="start-a-remote-control-session">
  Inicie una sesión de Remote Control
</h2>

Puede iniciar una sesión de Remote Control desde la CLI o la extensión de VS Code. La CLI ofrece tres modos de invocación; VS Code usa el comando `/remote-control`.

<Tabs>
  <Tab title="Modo servidor">
    Navegue a su directorio de proyecto y ejecute:

    ```bash theme={null}
    claude remote-control
    ```

    El proceso sigue ejecutándose en su terminal en modo servidor, esperando conexiones remotas. Muestra una URL de sesión que puede usar para [conectarse desde otro dispositivo](#connect-from-another-device), y puede presionar la barra espaciadora para mostrar un código QR para acceso rápido desde su teléfono. Mientras una sesión remota está activa, la terminal muestra el estado de la conexión y la actividad de las herramientas.

    Banderas disponibles:

    | Bandera                                         | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
    | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | Establezca un título de sesión personalizado visible en la lista de sesiones en claude.ai/code.                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
    | `--remote-control-session-name-prefix <prefix>` | Prefijo para nombres de sesión generados automáticamente cuando no se establece un nombre explícito. El valor predeterminado es el nombre de host de su máquina, produciendo nombres como `myhost-graceful-unicorn`. Establezca `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` para el mismo efecto.                                                                                                                                                                                                                                                          |
    | `-c`, `--continue`                              | Reanude la sesión de Remote Control más reciente iniciada desde este directorio en lugar de crear una nueva. No se puede combinar con `--session-id`, `--spawn`, `--capacity`, o `--create-session-in-dir`. Requiere Claude Code v2.1.200 o posterior; las versiones anteriores rechazan la bandera como un argumento desconocido.                                                                                                                                                                                                                         |
    | `--session-id <id>`                             | Reanude una sesión de Remote Control específica por su ID. No se puede combinar con `--continue`, `--spawn`, `--capacity`, o `--create-session-in-dir`. Requiere Claude Code v2.1.200 o posterior; las versiones anteriores rechazan la bandera como un argumento desconocido.                                                                                                                                                                                                                                                                             |
    | `--spawn <mode>`                                | Cómo el servidor crea sesiones.<br />• `same-dir` (predeterminado): todas las sesiones comparten el directorio de trabajo actual, por lo que pueden entrar en conflicto si editan los mismos archivos.<br />• `worktree`: cada sesión bajo demanda obtiene su propio [git worktree](/docs/es/worktrees). Requiere un repositorio git.<br />• `session`: modo de sesión única. Sirve exactamente una sesión y rechaza conexiones adicionales. Se establece solo al inicio.<br />Presione `w` en tiempo de ejecución para alternar entre `same-dir` y `worktree`. |
    | `--capacity <N>`                                | Número máximo de sesiones concurrentes. El valor predeterminado es 32. No se puede usar con `--spawn=session`.                                                                                                                                                                                                                                                                                                                                                                                                                                             |
    | `--[no-]create-session-in-dir`                  | Pre-crear una sesión en el directorio actual cuando el servidor se inicia, para que tenga un lugar donde escribir inmediatamente. En modo `worktree` esta sesión permanece en el directorio actual mientras las sesiones bajo demanda obtienen worktrees aislados. Habilitado de forma predeterminada; pase `--no-create-session-in-dir` para comenzar sin ninguno.                                                                                                                                                                                        |
    | `--verbose`                                     | Mostrar registros detallados de conexión y sesión.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
    | `--sandbox` / `--no-sandbox`                    | Habilitar o deshabilitar [sandboxing](/docs/es/sandboxing) para aislamiento del sistema de archivos y red. Deshabilitado de forma predeterminada.                                                                                                                                                                                                                                                                                                                                                                                                               |
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

    Las banderas `--verbose`, `--sandbox` y `--no-sandbox` no están disponibles con este comando.
  </Tab>

  <Tab title="VS Code">
    En la [extensión de VS Code de Claude Code](/docs/es/vs-code), escriba `/remote-control` o `/rc` en el cuadro de solicitud, o abra el menú de comandos con `/` y selecciónelo.

    ```text theme={null}
    /remote-control
    ```

    Un banner aparece encima del cuadro de solicitud mostrando el estado de la conexión. Una vez conectado, haga clic en **Open in browser** en el banner para ir directamente a la sesión, o encuéntrela en la lista de sesiones en [claude.ai/code](https://claude.ai/code). La URL de la sesión también se publica en la conversación.

    Para desconectarse, haga clic en el icono de cierre en el banner o ejecute `/remote-control` nuevamente.

    A diferencia de la CLI, el comando de VS Code no acepta un argumento de nombre ni muestra un código QR. El título de la sesión se deriva del historial de conversación o del primer mensaje.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  Verifique el estado de la conexión
</h3>

En una sesión de terminal interactiva, un indicador `/rc active` se encuentra en el pie de página debajo del cuadro de entrada mientras la conexión está activa, y se oculta si la terminal es demasiado estrecha para ajustarlo. El texto del indicador es un enlace a la sesión en claude.ai. Selecciónelo con la tecla de flecha hacia abajo y presione Intro, o ejecute `/remote-control` nuevamente, para abrir un panel de estado con la URL de la sesión y un código QR que puede usar para [conectarse desde otro dispositivo](#connect-from-another-device).

Si la conexión falla, aparece una notificación con el motivo del fallo y el indicador desaparece del pie de página. Ejecute `/remote-control` nuevamente para reintentar.

<h3 id="connect-from-another-device">
  Conectarse desde otro dispositivo
</h3>

Una vez que una sesión de Remote Control está activa, tiene varias formas de conectarse desde otro dispositivo:

* **Abra la URL de la sesión** en cualquier navegador para ir directamente a la sesión en [claude.ai/code](https://claude.ai/code).
* **Escanee el código QR** que se muestra junto a la URL de la sesión para abrirlo directamente en la aplicación Claude. Con `claude remote-control`, presione la barra espaciadora para alternar la visualización del código QR.
* **Abra [claude.ai/code](https://claude.ai/code) o la aplicación Claude** y encuentre la sesión por nombre en la lista de sesiones. En la aplicación móvil Claude, toque **Code** en la navegación para llegar a la lista de sesiones. Las sesiones de Remote Control muestran un icono de computadora con un punto de estado verde cuando están en línea.

Cuando se conecta, el dispositivo muestra cualquier subagenteagente y flujo de trabajo que la sesión ya tenga ejecutándose en segundo plano. Antes de v2.1.208, un dispositivo que se conectaba a una sesión alojada en un terminal interactivo no mostraba subagenteagentes y flujos de trabajo que ya estaban ejecutándose hasta que uno de ellos se iniciaba o detenía.

El título de la sesión remota se elige en este orden:

1. El nombre que pasó a `--name`, `--remote-control`, o `/remote-control`
2. El título que estableció con `/rename`
3. El último mensaje significativo en el historial de conversación existente
4. Un nombre generado automáticamente como `myhost-graceful-unicorn`, donde `myhost` es el nombre de host de su máquina o el prefijo que estableció con `--remote-control-session-name-prefix`

Si no estableció un nombre explícito, el título se actualiza para reflejar su solicitud una vez que envíe una. A partir de Claude Code v2.1.176, los títulos generados automáticamente coinciden con el idioma de su conversación, o la configuración [`language`](/docs/es/settings#available-settings) si una está configurada. Renombrar una sesión desde claude.ai o la aplicación Claude también actualiza el título local que se muestra en `claude --resume`.

Si el entorno ya tiene una sesión activa, se le preguntará si desea continuarla o iniciar una nueva.

Si aún no tiene la aplicación Claude, use el comando `/mobile` dentro de Claude Code para mostrar un código QR de descarga para [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) o [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude).

<h3 id="enable-remote-control-for-all-sessions">
  Habilite Remote Control para todas las sesiones
</h3>

Remote Control solo se activa cuando ejecuta explícitamente `claude remote-control`, `claude --remote-control`, o `/remote-control`, a menos que la conexión automática esté activada. Para habilitarlo automáticamente para cada sesión interactiva, ejecute `/config` dentro de Claude Code y establezca **Enable Remote Control for all sessions** en `true`. Establézcalo en `false` para nunca conectarse automáticamente, o déjelo sin establecer para seguir el valor predeterminado de su organización. En la aplicación de escritorio, también puede alternar esto desde **Settings → Claude Code → Enable remote control by default**. En la [extensión de VS Code](/docs/es/vs-code#use-the-prompt-box), el mismo botón de alternancia aparece como **Enable Remote Control for all sessions** en la sección Configuración del menú de comandos; requiere Claude Code v2.1.203 o posterior.

Con esta configuración activada, cada proceso interactivo de Claude Code registra una sesión remota. Si ejecuta varias instancias, cada una obtiene su propio entorno y sesión. Para ejecutar varias sesiones concurrentes desde un único proceso, use el [modo servidor](#start-a-remote-control-session) en su lugar.

<h2 id="connection-and-security">
  Conexión y seguridad
</h2>

Su sesión local de Claude Code realiza solo solicitudes HTTPS salientes y nunca abre puertos entrantes en su máquina. Cuando inicia Remote Control, se registra con la API de Anthropic y sondea el trabajo. Cuando se conecta desde otro dispositivo, el servidor enruta mensajes entre el cliente web o móvil y su sesión local a través de una conexión de transmisión.

Todo el tráfico viaja a través de la API de Anthropic sobre TLS, el mismo transporte de seguridad que cualquier sesión de Claude Code. La conexión utiliza múltiples credenciales de corta duración, cada una limitada a un único propósito y expirando de forma independiente.

Mientras Remote Control está conectado, la transcripción de la sesión, incluidos sus mensajes, las respuestas de Claude y la actividad de herramientas, se almacena en los servidores de Anthropic. La transcripción almacenada mantiene la conversación sincronizada en todos sus dispositivos y permite que la sesión se reconecte después de una caída de red. La ejecución y el acceso al sistema de archivos permanecen en su máquina, y las transcripciones almacenadas se retienen bajo la política de [Uso de datos](/docs/es/data-usage).

Para desactivar Remote Control completamente, utilice la configuración [`disableRemoteControl`](/docs/es/settings#available-settings). Las organizaciones con requisitos de cumplimiento como Retención Cero de Datos no pueden habilitar Remote Control.

<h2 id="trusted-devices">
  Dispositivos de confianza
</h2>

<Note>
  Trusted Devices está actualmente en beta. Las características y funcionalidades pueden evolucionar a medida que se refina la experiencia.

  Trusted Devices está disponible en planes Team y Enterprise. Está deshabilitado de forma predeterminada hasta que un administrador lo habilite.
</Note>

Trusted Devices es una configuración de toda la organización que requiere que los miembros verifiquen su dispositivo antes de poder ver o controlar sesiones de Remote Control desde claude.ai, las aplicaciones móviles de Claude o Claude Desktop. Vincula el acceso a Remote Control a un dispositivo conocido y una autenticación reciente, no solo a una cuenta con sesión iniciada.

Cuando la configuración está activada, interactuar con una sesión de Remote Control requiere ambas de las siguientes:

* **Un dispositivo inscrito**: cada navegador, teléfono o aplicación de escritorio que un miembro usa para Remote Control inscribe su propia credencial. La inscripción solo se ofrece poco después de un inicio de sesión completo, por lo que un dispositivo se une a la lista de confianza como parte de una autenticación real en lugar de silenciosamente en el fondo.
* **Un inicio de sesión reciente**: el inicio de sesión del miembro no debe tener más de 18 horas. En lugar de iniciar sesión nuevamente cada día, los miembros confirman presencia con Face ID, Touch ID, Windows Hello o una passkey. Este paso de autenticación biométrica actualiza la sesión inmediatamente.

Las verificaciones biométricas se ejecutan en el dispositivo a través del sistema operativo o navegador, el mismo mecanismo que el inicio de sesión con passkey. Anthropic nunca recibe ni almacena huellas dactilares, datos faciales ni ninguna otra información biométrica. Solo se almacenan la clave pública del dispositivo y metadatos básicos como nombre de pantalla, plataforma y hora de inscripción.

La configuración se aplica solo a Remote Control. El chat regular de Claude, Claude Code en la terminal y el uso de API no se ven afectados.

<h3 id="enable-trusted-devices-for-your-organization">
  Habilite Trusted Devices para su organización
</h3>

Los administradores habilitan la configuración desde la consola de administración de Claude Code.

<Steps>
  <Step title="Abra la configuración de administración de Claude Code">
    Vaya a [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). El botón de alternancia **Require trusted devices** aparece bajo la configuración de Remote Control.
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

<h2 id="remote-control-vs-claude-code-on-the-web">
  Remote Control vs Claude Code en la web
</h2>

Remote Control y [Claude Code en la web](/docs/es/claude-code-on-the-web) ambos usan la interfaz claude.ai/code. La diferencia clave es dónde se ejecuta la sesión: Remote Control se ejecuta en su máquina, por lo que sus MCP servers locales, herramientas y configuración del proyecto permanecen disponibles. Claude Code en la web se ejecuta en infraestructura en la nube administrada por Anthropic.

Use Remote Control cuando esté en medio del trabajo local y desee continuar desde otro dispositivo. Use Claude Code en la web cuando desee iniciar una tarea sin ninguna configuración local, trabajar en un repositorio que no tiene clonado, o ejecutar varias tareas en paralelo.

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
* **El proceso local debe seguir ejecutándose**: Remote Control se ejecuta como un proceso local. Si cierra la terminal, cierra VS Code, o detiene el proceso `claude` de otra manera, la sesión finaliza.
* **Interrupción de red extendida**: si su máquina está despierta pero no puede alcanzar la red durante más de aproximadamente 10 minutos, la sesión agota el tiempo de espera y el proceso se cierra. Ejecute `claude remote-control` nuevamente para iniciar una nueva sesión.
* **Ultraplan desconecta Remote Control**: iniciar una sesión de [ultraplan](/docs/es/ultraplan) desconecta cualquier sesión de Remote Control activa porque ambas características ocupan la interfaz claude.ai/code y solo una puede estar conectada a la vez.
* **Algunos comandos son solo locales**: comandos que solo se ejecutan en la interfaz de terminal, como `/plugin` o `/resume`, funcionan solo desde la CLI local, independientemente de si pasa un argumento o no. Los siguientes funcionan desde móvil y web:
  * Comandos de salida de texto: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits` (ejecuta el formulario de texto en lugar de abrir el diálogo en la CLI), `/recap`, `/reload-plugins`
  * `/model`, `/effort`, `/fast`, `/color` y `/rename`: pase el valor como argumento, por ejemplo `/model sonnet` o `/effort high`. Desde móvil y web, `/model` y `/effort` toman el argumento en lugar del selector de terminal o deslizador.
  * `/mcp`, a partir de v2.1.166: desde la aplicación móvil, devuelve un resumen de texto del estado del servidor en lugar de abrir el selector. En la web, `/mcp` por sí solo abre un directorio de [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) en lugar de devolver el resumen. Los [subcomandos](/docs/es/commands#all-commands) `reconnect`, `enable` y `disable` funcionan desde ambos. A diferencia de la CLI local, `/mcp reconnect` sin nombre de servidor reconecta cada servidor que ha fallado o necesita autenticación.
  * `/config`, a partir de v2.1.181: desde la aplicación móvil, pase `key=value` para establecer una configuración, o ejecútelo sin argumentos para listar las claves que puede establecer. En la web, `/config` abre la sección Claude Code de su configuración en su lugar, e ignora el texto después del comando.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  "Remote Control requires a claude.ai subscription"
</h3>

No está autenticado con una cuenta de claude.ai. Ejecute `claude auth login` y elija la opción de claude.ai. Si `ANTHROPIC_API_KEY` está configurado en su entorno, desactívelo primero.

Antes de v2.1.206, ejecutar `/remote-control` mientras no estaba conectado reportaba `Unknown command: /remote-control` en lugar de este mensaje.

<h3 id="remote-control-requires-a-full-scope-login-token">
  "Remote Control requires a full-scope login token"
</h3>

Está autenticado con un token de larga duración de `claude setup-token` o la variable de entorno `CLAUDE_CODE_OAUTH_TOKEN`. Estos tokens se limitan a solo inferencia y no pueden establecer sesiones de Remote Control. Ejecute `claude auth login` para autenticarse con un token de sesión de alcance completo en su lugar.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  "Unable to determine your organization for Remote Control eligibility"
</h3>

Su información de cuenta en caché está obsoleta o incompleta. Ejecute `claude auth login` para actualizarla.

<h3 id="remote-control-is-not-yet-enabled-for-your-account">
  "Remote Control is not yet enabled for your account"
</h3>

La verificación de Remote Control no ha llegado a su cuenta, o sus derechos en caché están desactualizados. Si cambió recientemente de plan, ejecute `claude auth logout` y luego `claude auth login` para actualizarlos. Ejecute `claude doctor` para ver qué verificación de elegibilidad individual falló. Los conflictos de variables de entorno, las verificaciones inaccesibles y la política de la organización producen cada uno su propio mensaje, por lo que este error significa la puerta de verificación en sí.

<h3 id="couldn’t-verify-remote-control-eligibility">
  "Couldn't verify Remote Control eligibility"
</h3>

Claude Code no pudo alcanzar el servicio de banderas de características para verificar si Remote Control está habilitado para su cuenta, típicamente porque está sin conexión o un proxy está bloqueando la solicitud. Reintente una vez que tenga acceso a la red, o ejecute `claude doctor` para obtener detalles. El mensaje relacionado "Couldn't verify your organization's Remote Control policy" tiene la misma causa y la misma solución. Ambos mensajes se agregaron en v2.1.178.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  "Remote Control is only available when using Claude via api.anthropic.com"
</h3>

La sesión no está hablando directamente con la API de Anthropic, por lo que no hay un backend de claude.ai para emparejar. Esto ocurre en Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry. A partir de v2.1.196 también ocurre cuando [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) apunta a un host distinto de `api.anthropic.com`, como una [puerta de enlace LLM](/docs/es/llm-gateway) o proxy, incluso si inicia sesión con claude.ai. Desactive `ANTHROPIC_BASE_URL` y reinicie la sesión para usar Remote Control.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  "Remote Control is disabled by your organization's policy"
</h3>

Este error tiene cuatro causas distintas. Ejecute `/status` primero para ver qué método de inicio de sesión y suscripción está usando.

* **Está autenticado con una clave API o cuenta de Console**: Remote Control requiere OAuth de claude.ai. Ejecute `/login` y elija la opción de claude.ai. Si `ANTHROPIC_API_KEY` está configurado en su entorno, desactívelo.
* **Un propietario no lo ha habilitado para su organización**: Remote Control está deshabilitado de forma predeterminada en los planes Team y Enterprise. Un propietario puede habilitarlo en [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) activando el botón de alternancia **Remote Control**. Este botón de alternancia es una configuración de organización del lado del servidor.
* **El botón de alternancia del administrador está atenuado**: su organización tiene una configuración de retención de datos o cumplimiento que es incompatible con Remote Control. Esto no se puede cambiar desde el panel de administración. Póngase en contacto con el soporte de Anthropic para discutir opciones.
* **El error menciona `disableRemoteControl`**: su administrador de TI ha deshabilitado Remote Control en este dispositivo a través de [configuración administrada](/docs/es/settings#settings-files), independientemente del botón de alternancia de toda la organización.

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

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  "Couldn't reconnect to your Remote Control session"
</h3>

Cuando reanuda una conversación con `claude --resume` o `claude --continue`, Claude Code se reconecta a la sesión de Remote Control registrada en esa conversación. Este mensaje significa que la reconexión falló por una razón que puede ser temporal, como una interrupción de red o un error del servidor, por lo que Claude Code no puede confirmar si la sesión remota aún existe. Cuando el servidor confirma que la sesión anterior ya no existe, Claude Code crea una nueva sesión de Remote Control sin mostrar este mensaje.

Su sesión local continúa ejecutándose sin Remote Control. Ejecute `/remote-control` para reintentar la conexión, o inicie Claude Code sin `--resume` para crear una nueva sesión de Remote Control.

Antes de v2.1.200, una falla de reconexión creaba una nueva sesión de Remote Control en lugar de mostrar este mensaje, lo que dejaba sesiones adicionales en la lista de sesiones en claude.ai/code.

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

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Claude Code en la web](/docs/es/claude-code-on-the-web): ejecute sesiones en entornos en la nube administrados por Anthropic en lugar de en su máquina
* [Ultraplan](/docs/es/ultraplan): inicie una sesión de planificación en la nube desde su terminal y revise el plan en su navegador
* [Channels](/docs/es/channels): reenvíe Telegram, Discord o iMessage a una sesión para que Claude reaccione a los mensajes mientras está fuera
* [Dispatch](/docs/es/desktop#sessions-from-dispatch): envíe un mensaje de una tarea desde su teléfono y puede generar una sesión de Desktop para manejarla
* [Autenticación](/docs/es/authentication): configure `/login` y administre credenciales para claude.ai
* [Referencia de CLI](/docs/es/cli-reference): lista completa de banderas y comandos incluyendo `claude remote-control`
* [Seguridad](/docs/es/security): cómo las sesiones de Remote Control se ajustan al modelo de seguridad de Claude Code
* [Uso de datos](/docs/es/data-usage): qué datos fluyen a través de la API de Anthropic durante sesiones locales y remotas
