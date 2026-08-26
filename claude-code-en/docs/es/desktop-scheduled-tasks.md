> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Programar tareas recurrentes en Claude Code Desktop

> Configure tareas programadas en Claude Code Desktop para ejecutar Claude automáticamente de forma recurrente para revisiones de código diarias, auditorías de dependencias o resúmenes matutinos.

Las tareas programadas inician una nueva sesión automáticamente en la hora y frecuencia que elija. Úselas para trabajos recurrentes como revisiones de código diarias, comprobaciones de actualizaciones de dependencias o resúmenes matutinos que extraigan información de su calendario e bandeja de entrada.

La página **Routines** de la aplicación de escritorio le permite crear tanto tareas programadas locales como [routines](/docs/es/routines) remotas. Una tarea local se ejecuta en su máquina con acceso directo a sus archivos y herramientas, pero solo se activa mientras la aplicación está abierta y su computadora está despierta. Una routine remota se ejecuta en la infraestructura en la nube administrada por Anthropic incluso cuando su computadora está apagada, y también puede activarse mediante llamadas API o eventos de GitHub. Esta página cubre tareas programadas locales; para routines remotas y sus opciones de activación, consulte [Routines](/docs/es/routines).

<h2 id="compare-scheduling-options">
  Comparar opciones de programación
</h2>

Claude Code offers three ways to schedule recurring or one-off work:

|                            | [Cloud](/docs/en/routines)               | [Desktop](/docs/en/desktop-scheduled-tasks) | [`/loop`](/docs/en/scheduled-tasks)      |
| :------------------------- | :---------------------------------- | :------------------------------------- | :---------------------------------- |
| Runs on                    | Cloud, Anthropic-managed by default | Your machine                           | Your machine                        |
| Requires machine on        | No                                  | Yes                                    | Yes                                 |
| Requires open session      | No                                  | No                                     | Yes                                 |
| Persistent across restarts | Yes                                 | Yes                                    | Restored on `--resume` if unexpired |
| Access to local files      | No (fresh clone)                    | Yes                                    | Yes                                 |
| MCP servers                | Connectors configured per task      | [Config files](/docs/en/mcp) and connectors | Inherits from session               |
| Permission prompts         | No (runs autonomously)              | Configurable per task                  | Inherits from session               |
| Customizable schedule      | Via `/schedule` in the CLI          | Yes                                    | Yes                                 |
| Minimum interval           | 1 hour                              | 1 minute                               | 1 minute                            |

<Tip>
  Use **cloud tasks** for work that should run reliably without your machine. Use **Desktop tasks** when you need access to local files and tools. Use **`/loop`** for quick polling during a session.
</Tip>

<Note>
  De forma predeterminada, las tareas programadas se ejecutan contra el estado en el que se encuentre su directorio de trabajo, incluidos los cambios no confirmados. Active el toggle de worktree al crear la tarea para dar a cada ejecución su propio Git worktree aislado, de la misma manera que funcionan las [sesiones paralelas](/docs/es/desktop#work-in-parallel-with-sessions).
</Note>

<h2 id="create-a-scheduled-task">
  Crear una tarea programada
</h2>

Haga clic en **Routines** en la barra lateral, luego haga clic en **New routine** y elija **Local**. Configure estos campos:

| Campo        | Descripción                                                                                                                                                                                                                                                                                                                |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Name         | Identificador de la tarea. Se convierte a kebab-case en minúsculas y se utiliza como nombre de carpeta en disco. Debe ser único en todas sus tareas.                                                                                                                                                                       |
| Description  | Resumen breve que se muestra en la lista de tareas.                                                                                                                                                                                                                                                                        |
| Instructions | Lo que Claude debe hacer cuando se ejecute la tarea. Escriba esto de la misma manera que escribiría cualquier mensaje en el cuadro de solicitud. La entrada de instrucciones incluye selectores para el modo de permiso y el modelo, y debajo selecciona la carpeta de trabajo y si desea ejecutar en un worktree aislado. |
| Schedule     | Con qué frecuencia se ejecuta la tarea. Consulte [opciones de programación](#schedule-options) a continuación.                                                                                                                                                                                                             |

Se requiere una carpeta antes de poder guardar la tarea. Si aún no ha confiado en esa carpeta, Desktop le solicita que la confíe antes de guardar.

También puede crear una tarea describiendo lo que desea en cualquier sesión. Por ejemplo, "configurar una revisión de código diaria que se ejecute cada mañana a las 9am" crea una tarea recurrente, y "recuérdame a las 3pm mañana que verifique el deploy" crea una tarea única que se desactiva después de ejecutarse.

<h2 id="schedule-options">
  Opciones de programación
</h2>

Elija un ajuste preestablecido del control Schedule:

* **Manual**: sin programación, solo se ejecuta cuando hace clic en **Run now**. Útil para guardar un prompt que activa bajo demanda
* **Hourly**: se ejecuta cada hora
* **Daily**: muestra un selector de hora, por defecto a las 9:00 AM hora local
* **Weekdays**: igual que Daily pero omite sábado y domingo
* **Weekly**: muestra un selector de hora y un selector de día

Para intervalos que el selector no ofrece, como cada 15 minutos, el primero de cada mes, o una única ejecución en un momento futuro específico, pídale a Claude en cualquier sesión de Desktop que configure la programación. Use lenguaje natural; por ejemplo, "programa una tarea para ejecutar todas las pruebas cada 6 horas."

<h2 id="how-scheduled-tasks-run">
  Cómo se ejecutan las tareas programadas
</h2>

Las tareas programadas se ejecutan en su máquina. Desktop verifica la programación cada minuto mientras la aplicación está abierta e inicia una sesión nueva cuando una tarea vence, independientemente de cualquier sesión manual que tenga abierta. Cada tarea obtiene un pequeño retraso de unos pocos minutos después de la hora programada para escalonar el tráfico de API. El retraso es determinista: la misma tarea siempre comienza en el mismo desplazamiento.

Cuando se activa una tarea, recibe una notificación de escritorio y aparece una nueva sesión bajo una sección **Scheduled** en la barra lateral. Ábrala para ver qué hizo Claude, revisar cambios o responder a solicitudes de permiso. La sesión funciona como cualquier otra: Claude puede editar archivos, ejecutar comandos, crear commits y abrir pull requests.

Las tareas solo se ejecutan mientras la aplicación de escritorio está en ejecución y su computadora está despierta. Si su computadora se duerme durante una hora programada, la ejecución se omite. Para evitar el reposo inactivo, active **Keep computer awake** en Configuración bajo **Desktop app → General**. Cerrar la tapa del portátil aún lo pone en reposo. Para tareas que necesitan ejecutarse incluso cuando su computadora está apagada, o que deben activarse mediante una llamada API o evento de GitHub, cree una [routine](/docs/es/routines) remota en su lugar.

<h2 id="missed-runs">
  Ejecuciones perdidas
</h2>

Cuando la aplicación se inicia o su computadora se despierta, Desktop verifica si cada tarea perdió alguna ejecución en los últimos siete días. Si es así, Desktop inicia exactamente una ejecución de recuperación para el tiempo más recientemente perdido y descarta cualquier cosa más antigua. Una tarea diaria que perdió seis días se ejecuta una vez al despertar. Desktop muestra una notificación cuando comienza una ejecución de recuperación.

Tenga esto en cuenta al escribir prompts. Una tarea programada para las 9am podría ejecutarse a las 11pm si su computadora estuvo dormida todo el día. Si el tiempo es importante, agregue protecciones al prompt mismo, por ejemplo: "Solo revise los commits de hoy. Si es después de las 5pm, omita la revisión y solo publique un resumen de lo que se perdió."

<h2 id="permissions-for-scheduled-tasks">
  Permisos para tareas programadas
</h2>

Cada tarea tiene su propio modo de permiso, que establece al crear o editar la tarea. Las reglas de permiso de `~/.claude/settings.json` también se aplican a sesiones de tareas programadas. Si una tarea se ejecuta en modo Ask y necesita ejecutar una herramienta para la que no tiene permiso, la ejecución se detiene hasta que la apruebe. La sesión permanece abierta en la barra lateral para que pueda responder más tarde.

Para evitar detenciones, haga clic en **Run now** después de crear una tarea, observe las solicitudes de permiso y seleccione "always allow" para cada una. Las ejecuciones futuras de esa tarea aprueban automáticamente las mismas herramientas sin solicitar. Puede revisar y revocar estas aprobaciones desde la página de detalles de la tarea.

Las herramientas de conector [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools) y las herramientas MCP marcadas como [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool) solicitan confirmación en cada llamada y no ofrecen una opción de permitir siempre. Las ejecuciones que llaman a estas herramientas se detienen cada vez.

<h2 id="manage-scheduled-tasks">
  Administrar tareas programadas
</h2>

Haga clic en una tarea en la lista **Routines** para abrir su página de detalles. Desde aquí puede:

* **Run now**: inicie la tarea inmediatamente sin esperar a la próxima hora programada
* **Status**: alterne entre Active y Paused para pausar o reanudar ejecuciones programadas sin eliminar la tarea
* **Edit**: cambie las instrucciones, la programación, la carpeta u otras configuraciones
* **Review history**: vea cada ejecución anterior, incluidas las ejecuciones omitidas. Pase el cursor sobre una entrada omitida para ver por qué: su computadora estaba dormida, la ejecución anterior aún estaba en progreso, u otras tareas programadas ya estaban en ejecución. Haga clic en **Show more** para cargar entradas más antiguas.
* **Review allowed permissions**: vea y revoque aprobaciones de herramientas guardadas para esta tarea desde el panel **Always allowed**
* **Delete**: elimine la tarea y archive todas las sesiones que creó. Aparece una casilla de verificación **Also delete files on disk** en el diálogo de confirmación; márquela para eliminar también el archivo `SKILL.md` de la tarea y los datos asociados de `~/.claude/scheduled-tasks/`.

También puede enumerar, crear, editar y pausar tareas pidiendo a Claude en cualquier sesión de Desktop. Por ejemplo, "pausa mi tarea dependency-audit" o "muéstrame mis tareas programadas." Para eliminar una tarea, use el botón **Delete** en su página de detalles.

Una tarea programada también puede modificar su propia programación o prompt desde dentro de una sesión en ejecución usando la herramienta MCP `update_scheduled_task`. Esto permite que una tarea se reprograme a sí misma según lo que encuentre, por ejemplo, reprogramar una revisión de código para ejecutarse antes cuando detecta que se ha creado una rama de lanzamiento.

Para editar el prompt de una tarea en disco, abra `~/.claude/scheduled-tasks/<task-name>/SKILL.md` (o bajo [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars) si está configurado). El archivo utiliza frontmatter YAML para `name` y `description`, con el prompt como cuerpo. Los cambios surten efecto en la próxima ejecución. La programación, carpeta, modelo y estado habilitado no están en este archivo: cámbielos a través del formulario Edit o pídale a Claude.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Routines](/docs/es/routines): ejecute tareas en infraestructura administrada por Anthropic en una programación, mediante llamada API o en respuesta a eventos de GitHub, incluso cuando su computadora está apagada
* [Ejecutar prompts en una programación](/docs/es/scheduled-tasks): programación con alcance de sesión con `/loop` en la CLI
* [Claude Code GitHub Actions](/docs/es/github-actions): ejecute Claude en una programación en CI en lugar de en su máquina
* [Usar Claude Code Desktop](/docs/es/desktop): la guía completa de la aplicación de escritorio
