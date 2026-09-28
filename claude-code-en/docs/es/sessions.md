> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestionar sesiones

> Nombre, reanude, ramifique y cambie entre conversaciones de Claude Code. Cubre `--continue`, `--resume`, `--from-pr`, el selector `/resume`, nombres de sesión, exportación de transcripciones y dónde se almacenan las transcripciones.

Una sesión es una conversación guardada vinculada a un directorio de proyecto. Claude Code la almacena localmente mientras trabaja, para que pueda reanudar donde lo dejó, ramificarse para probar un enfoque diferente o cambiar entre tareas.

La [aplicación de escritorio](/docs/es/desktop#work-in-parallel-with-sessions), [Claude Code en la web](/docs/es/claude-code-on-the-web) y la [extensión de VS Code](/docs/es/vs-code#resume-past-conversations) mantienen cada una su propio historial de sesiones. Esta página cubre la CLI.

<h2 id="resume-a-session">
  Reanude una sesión
</h2>

Las sesiones se guardan continuamente en [archivos de transcripción locales](#export-and-locate-session-data) mientras trabaja, para que pueda volver a una después de salir o ejecutar `/clear`. Use estos puntos de entrada:

| Comando                             | Qué hace                                                                                                                         |
| :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- |
| `claude --continue`                 | Reabre la conversación más reciente en el directorio actual                                                                      |
| `claude --resume`                   | Abre el [selector de sesiones](#use-the-session-picker)                                                                          |
| `claude --resume <name>`            | Reanuda la sesión nombrada directamente                                                                                          |
| `claude --resume <transcript-path>` | Reanuda la conversación almacenada en el archivo de [transcripción](#where-transcripts-are-stored) `.jsonl` en esa ruta absoluta |
| `claude --from-pr <number>`         | Abre el selector de sesiones filtrado a sesiones vinculadas a esa solicitud de extracción                                        |
| `/resume`                           | Cambia a una conversación diferente desde dentro de una sesión activa                                                            |

Claude Code deja fuera del selector de sesiones y fuera de `claude --continue` las sesiones creadas con [`claude -p`](/docs/es/headless) o el [Agent SDK](/docs/es/agent-sdk/overview). Aún puede reanudar una pasando su ID de sesión a `claude --resume <session-id>`. Con `claude --continue`, Claude Code también omite [sesiones cuya primera solicitud fue `/loop`](#where-the-session-picker-looks). Cuando ejecuta [`claude -p --continue`](/docs/es/headless#continue-conversations), Claude Code incluye sesiones `-p`, SDK y `/loop`.

`claude --continue` abre una [sesión en segundo plano](/docs/es/agent-view) que ha terminado, pero no una que aún se está ejecutando; abrir sesiones en segundo plano terminadas requiere Claude Code v2.1.257 o posterior. Si su conversación más reciente es una que [movió al segundo plano](/docs/es/agent-view#send-the-session-to-the-background) y aún se está ejecutando allí, Claude Code sale con `Your most recent conversation is running in the background` y el ID de esa sesión. Conéctese a la sesión desde [`claude agents`](/docs/es/agent-view#attach-to-a-session), o ejecute `claude --resume` para elegir otra.

Puede ejecutar `claude --resume <session-id>` desde cualquier directorio: Claude Code busca el ID en el directorio del proyecto actual y sus git worktrees primero, luego en todos los demás proyectos en esta máquina, por lo que encuentra una sesión que comenzó en otro lugar o se movió con [`/cd`](/docs/es/commands). La búsqueda entre proyectos resuelve el ID solo cuando exactamente otro proyecto contiene una transcripción con mensajes para él, por lo que un duplicado copiado manualmente hace que Claude Code reporte no encontrado en lugar de reanudar una copia arbitraria. Si ninguna sesión almacenada coincide con el ID, Claude Code reporta `No conversation found with session ID: <session-id>`. Antes de v2.1.223, la búsqueda se detenía en el directorio del proyecto actual y sus git worktrees, por lo que tenía que reanudar desde el directorio en el que la sesión trabajó por última vez.

<h3 id="what-a-resumed-session-restores">
  Qué restaura una sesión reanudada
</h3>

Una sesión reanudada restaura la conversación junto con el estado guardado en ella:

* Historial de conversación: el historial completo, incluidas las llamadas a herramientas y los resultados. Una herramienta que aún se estaba ejecutando cuando terminó el proceso anterior, por ejemplo en un bloqueo, no termina ni se ejecuta de nuevo cuando reanuda. Claude ve la llamada marcada como cortada antes de que se registrara su resultado y se le indica que verifique si tuvo efecto antes de ejecutarla de nuevo, a menos que [`CLAUDE_CODE_RESUME_INTERRUPTED_TURN`](/docs/es/env-vars#variables) esté configurado. Antes de v2.1.281, Claude Code eliminaba la llamada cortada de la conversación o se la mostraba a Claude como una que usted interrumpió.
* Modelo: la sesión continúa en el modelo que estaba usando. El modelo no se restaura cuando ha sido retirado o no está permitido por `availableModels`, cuando una bandera `--model` o una variable de entorno de la familia `ANTHROPIC_MODEL` elige una en el lanzamiento, o en proveedores que usan ID de implementación específicos del proveedor, como [Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry](/docs/es/third-party-integrations); consulte [configuración de modelo](/docs/es/model-config#setting-your-model) para el orden de resolución.
* Agente: una sesión iniciada con [`--agent`](/docs/es/sub-agents#invoke-subagents-explicitly) o la configuración `agent` continúa como ese agente, manteniendo sus restricciones de herramientas y modelo. Pase `--agent` al reanudar para elegir uno diferente; para la solicitud del sistema en cualquier caso, consulte [Banderas de solicitud del sistema en conversaciones reanudadas](/docs/es/cli-reference#system-prompt-flags-in-resumed-conversations). Claude Code busca el agente en dos lugares: el directorio original de la sesión, siempre que haya [confiado en ese espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust), y luego el directorio desde el que reanuda, por lo que un agente con alcance de proyecto aún se carga cuando reanuda desde otro directorio. Si Claude Code no encuentra el agente en ninguno de los dos lugares, la sesión se reanuda con las herramientas predeterminadas y muestra una [advertencia que nombra el agente](/docs/es/errors#session-agent-no-longer-available).
* Modo de permisos: si reanuda desde una terminal con `claude --continue`, `claude --resume <session-id>` o `claude --resume <name>` cuando el nombre coincide con una sesión, sin `-p`, Claude Code restaura el modo de permisos en el que estaba la sesión, excepto en los casos en [modo de permisos al reanudar](#permission-mode-on-resume), que también cubre el selector de sesiones, `/resume` y reanudar con `claude -p`. Pase `--permission-mode` o `--dangerously-skip-permissions` para anular el modo restaurado.
* Objetivo activo: un [objetivo](/docs/es/goal#resume-with-an-active-goal) que aún estaba activo cuando terminó la sesión se transfiere; su recuento de turnos, temporizador y línea base de gasto de tokens se reinician.
* Tareas programadas: [tareas que no han expirado](/docs/es/scheduled-tasks#limitations) se restauran. Las tareas de Bash en segundo plano y de monitoreo no.

No se restaura cada bandera de configuración del lanzamiento original. Si la sesión dependía de `--mcp-config`, `--settings`, `--plugin-dir`, `--fallback-model` o directorios agregados con `--add-dir`, páselos de nuevo cuando reanude; los directorios agregados a mitad de sesión con `/add-dir` tampoco se restauran, aunque el selector de sesiones aún los usa para localizar la sesión. Los archivos de configuración estándar, como `settings.json` y `settings.local.json`, se leen nuevamente en el lanzamiento, por lo que la configuración que vive en ellos no necesita pasarse de nuevo. Para `--system-prompt` y `--append-system-prompt`, consulte [Banderas de solicitud del sistema en conversaciones reanudadas](/docs/es/cli-reference#system-prompt-flags-in-resumed-conversations).

<h4 id="permission-mode-on-resume">
  Modo de permisos al reanudar
</h4>

El modo de permisos en el que Claude Code inicia una sesión reanudada depende de cómo reanude:

* Terminal: `claude --continue`, `claude --resume <session-id>` o `claude --resume <name>` cuando el nombre coincide con una sesión, sin `-p`. Claude Code restaura el modo de permisos en el que estaba la sesión, excepto en los casos de la tabla. Pase `--permission-mode` o `--dangerously-skip-permissions` para anular el modo restaurado.
* No interactivo: `claude -p --resume` o `claude -p --continue`. Claude Code inicia la ejecución en el modo de permisos en el que se iniciaría una nueva ejecución de `claude -p`, excepto que una sesión que terminó en modo de plan se reanuda en modo de plan bajo las [condiciones a continuación](#resume-in-plan-mode-with-p).
* VS Code: el panel de conversación de la extensión. La tabla cubre solo una conversación que terminó en modo de plan; para el resto, consulte [reanudar conversaciones pasadas](/docs/es/vs-code#resume-past-conversations).
* Selector de sesiones en el lanzamiento: una sesión que selecciona del [selector de sesiones](#use-the-session-picker), ya sea que lo haya abierto con `claude --resume` solo, `claude --from-pr` o un nombre que coincida con más de una sesión. Claude Code no restaura el modo de permisos almacenado. Inicia la sesión en el modo de permisos en el que iniciaría una nueva sesión desde la misma línea de comandos.
* `/resume` dentro de una sesión, con o sin argumento: Claude Code no restaura el modo de permisos almacenado. La conversación a la que cambia continúa en el modo de permisos en el que está su sesión actual.

Restaurar el modo de plan en las rutas no interactivas y VS Code requiere Claude Code v2.1.246 o posterior. Cada fila nombra el modo de permisos en el que terminó la sesión, cuál de las rutas de terminal, no interactiva y VS Code la reanuda, y el modo de permisos en el que Claude Code inicia la sesión reanudada.

| Sesión terminada en | Cómo reanuda                                                                       | Modo de permisos después de reanudar                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------ | :--------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `bypassPermissions` | Terminal                                                                           | El modo de permisos en el que se iniciaría una nueva sesión. Para [omitir permisos](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode) de nuevo, habilítelo en el lanzamiento con una de sus banderas de lanzamiento o `permissions.defaultMode: "bypassPermissions"` en [configuración de usuario, `--settings` o configuración administrada](/docs/es/settings-reference#permissions-defaultmode) |
| `plan`              | Terminal                                                                           | El modo de permisos en el que se iniciaría una nueva sesión                                                                                                                                                                                                                                                                                                                                                   |
| `auto`              | Terminal                                                                           | `auto`, solo cuando su cuenta aún cumple con los [requisitos del modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                                                                                                                                                                                      |
| Manual              | Terminal                                                                           | Manual cuando una nueva sesión se iniciaría en modo automático desde el [valor predeterminado integrado](/docs/es/permission-modes#which-mode-a-session-starts-in). Cuando un `defaultMode` de un archivo de configuración [entra en vigor](/docs/es/permission-modes#which-mode-a-session-starts-in), Claude Code inicia la sesión reanudada en ese modo en su lugar                                                   |
| `plan`              | No interactivo, bajo las [condiciones a continuación](#resume-in-plan-mode-with-p) | Modo de plan                                                                                                                                                                                                                                                                                                                                                                                                  |
| Cualquier modo      | No interactivo, en cualquier otro caso                                             | El modo de permisos en el que se iniciaría una nueva ejecución de `claude -p`                                                                                                                                                                                                                                                                                                                                 |
| `plan`              | VS Code                                                                            | Modo de plan, con [las excepciones en la página de VS Code](/docs/es/vs-code#resume-past-conversations)                                                                                                                                                                                                                                                                                                            |

<h5 id="resume-in-plan-mode-with-p">
  Reanude en modo de plan con `-p`
</h5>

Una ejecución de `claude -p --resume` o `claude -p --continue` se reanuda en modo de plan solo cuando se cumplen las cuatro condiciones:

* Pasa [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags), para que Claude Code pueda presentar el plan para aprobación
* No pasa `--permission-mode` o `--dangerously-skip-permissions`
* No pasa `--fork-session`
* La ejecución no se inicia a través de [canales](/docs/es/channels)

<h3 id="resume-from-a-summary">
  Reanude desde un resumen
</h3>

En un plan Pro o Max, cuando reanuda una sesión que ha estado inactiva durante más de una hora aproximadamente y tiene más de 100,000 tokens, Claude Code restaura la conversación y luego abre un diálogo antes de enviar su primer mensaje. La [caché de solicitud](/docs/es/prompt-caching#cache-lifetime) de la sesión habrá expirado para entonces, por lo que la siguiente solicitud procesa el historial completo una vez sin importar cuál de las opciones del diálogo elija.

El diálogo ofrece tres formas de continuar la sesión. Difieren en cuánta parte de la conversación cada una lleva adelante en solicitudes posteriores, lo que es un equilibrio entre mantener cada detalle y enviar menos tokens por solicitud:

* **Reanude desde resumen**: ejecuta [`/compact`](/docs/es/context-window#what-survives-compaction) inmediatamente. Claude Code envía una solicitud de resumen sobre el historial completo, luego reemplaza el historial con el resumen, sus intercambios más recientes y hasta cinco archivos leídos recientemente. Las solicitudes posteriores llevan el resumen en lugar del historial completo.
* **Reanude la sesión completa tal como está**: carga la conversación sin cambios. Después de enviar su primer mensaje, Claude Code reprocesa y almacena en caché nuevamente el historial completo, luego lo relee de la caché en solicitudes posteriores mientras la caché se mantiene activa.
* **No me preguntes de nuevo**: reanuda la sesión completa y deja de mostrar el diálogo en todos los reanudos futuros.

Reanudar tal como está mantiene cada detalle de la conversación disponible, a un costo por solicitud que se escala con el tamaño de la conversación. Reanudar desde el resumen cuesta menos en cada solicitud posterior porque lleva el resumen en lugar del historial completo, pero lo que sea que el resumen deje fuera ya no está en el contexto de Claude. Consulte [por qué el uso aumenta en una sesión larga](/docs/es/costs#why-usage-climbs-in-a-long-session) para saber de dónde proviene ese costo por solicitud.

<h3 id="where-the-session-picker-looks">
  Dónde busca el selector de sesiones
</h3>

Claude Code almacena sesiones por directorio de proyecto. De forma predeterminada, el selector de sesiones muestra:

* Sesiones del worktree actual, incluidas [sesiones en segundo plano](/docs/es/agent-view), que están marcadas como `bg` en la lista
* Sesiones iniciadas en otro lugar que agregaron el directorio actual con `/add-dir`

Use `Ctrl+W` para ampliar a todos los worktrees del repositorio o `Ctrl+A` para ampliar a cada proyecto en esta máquina.

Las sesiones cuya primera solicitud fue un comando [`/loop`](/docs/es/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) no aparecen en el selector, y `claude --continue` también las omite. Ejecutar `/loop` más adelante en una conversación no oculta la sesión. Antes de v2.1.211, una ejecución de `/loop` al principio de una conversación ocultaba la sesión del selector de forma permanente.

Mover una sesión con [`/cd`](/docs/es/commands) la traslada al almacenamiento del proyecto del nuevo directorio, por lo que aparece en el selector de ese directorio después. A partir de v2.1.196, una sesión movida se mantiene fuera del selector del directorio anterior incluso después de un bloqueo o salida forzada. En versiones anteriores, también podría reaparecer en la lista del directorio anterior después de una salida que no fue limpia cuando la ruta anterior contenía caracteres especiales como guiones bajos.

Cuando selecciona una sesión de otro worktree del mismo repositorio, Claude Code la reanuda en su lugar; cuando el propio worktree de la sesión ya no existe, Claude Code [la reanuda en su directorio actual](/docs/es/worktrees#resume-a-worktree-session). Cuando selecciona una sesión de un proyecto no relacionado, Claude Code copia un comando `cd` y reanuda a su portapapeles en su lugar. Si el directorio de ese proyecto ya no existe, Claude Code reanuda la sesión en su directorio actual en lugar de copiar un comando `cd` que fallaría.

Reanudar por nombre se resuelve en el repositorio actual y sus worktrees. Ambas formas buscan una coincidencia exacta y la reanudan directamente incluso si vive en un worktree diferente:

| Comando                  | Coincidencia exacta  | Nombre ambiguo                                                                            |
| :----------------------- | :------------------- | :---------------------------------------------------------------------------------------- |
| `claude --resume <name>` | Reanuda directamente | Abre el selector de sesiones con el nombre rellenado previamente como término de búsqueda |
| `/resume <name>`         | Reanuda directamente | Reporta un error; ejecute `/resume` sin argumentos para abrir el selector de sesiones     |

<h2 id="name-your-sessions">
  Nombre sus sesiones
</h2>

Dé a las sesiones nombres descriptivos para que sean encontrables en el selector de sesiones y reanudables por nombre. Esto es más importante cuando está trabajando en varias tareas en paralelo.

| Cuándo                                 | Cómo establecer el nombre                                                                                                                                                          |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Al inicio                              | `claude -n auth-refactor`                                                                                                                                                          |
| Durante una sesión                     | `/rename auth-refactor`. El nombre también aparece en la barra de indicaciones                                                                                                     |
| Desde el selector de sesiones          | Resalte una sesión y presione `Ctrl+R`                                                                                                                                             |
| Al aceptar un plan                     | Aceptar un plan en [modo de plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) da a la sesión un título generado basado en el plan a menos que ya la haya nombrado |
| Desde claude.ai o la aplicación Claude | Renombre una [sesión de Control Remoto](/docs/es/remote-control#connect-from-another-device); Claude Code aplica el mismo nombre en la CLI. Requiere Claude Code v2.1.221 o posterior   |
| Desde la aplicación de escritorio      | Renombre una sesión en la [aplicación de escritorio](/docs/es/desktop#work-in-parallel-with-sessions)                                                                                   |

Una vez que nombre una sesión a través de una ruta CLI o desde claude.ai, vuelva a ella con `claude --resume <name>` o `/resume <name>`; una sesión de la aplicación de escritorio se reanuda en la aplicación, que mantiene su propio historial de sesiones. Vea [Reanude una sesión](#resume-a-session) para saber cómo se comporta la resolución de nombres en worktrees.

Cuando inicia o reanuda una sesión interactiva con un nombre que otra sesión activa en esta máquina ya usa, o renombra una sesión a ese nombre, Claude Code deja el nombre con la sesión que ya lo tiene, renombra la suya a una variante con un sufijo de dos palabras, como `auth-refactor-graceful-unicorn`, y se lo comunica. Ejecute `/rename` con un nuevo nombre si prefiere elegir uno usted mismo. Antes de v2.1.232, ambas sesiones mantenían el nombre.

En tres casos Claude Code no renombra el duplicado, por lo que aún puede ver dos sesiones con el mismo nombre en los listados:

* No comprueba títulos generados por IA ni nombres de visualización predeterminados.
* No comprueba el `--name` de una sesión [en segundo plano](/docs/es/agent-view#from-your-shell) o `-p` al inicio.
* No puede renombrar una sesión en una versión anterior de Claude Code.

Las sesiones que no nombra aún obtienen dos etiquetas que Claude Code asigna. Solo el título generado funciona como identificador de reanudación:

* Nombre de visualización predeterminado: las sesiones interactivas que nunca nombra aún obtienen un nombre de visualización predeterminado cuando se inician. Requiere Claude Code v2.1.196 o posterior. El valor predeterminado combina el nombre del directorio de trabajo con un sufijo de dos caracteres, por ejemplo `my-app-3f`, e identifica la sesión en listados de sesiones en ejecución, como [vista de agente](/docs/es/agent-view) y salida de `claude agents --json`. El valor predeterminado no es un identificador de reanudación. Si lo pasa a `claude --resume` o `/resume`, Claude Code no encuentra la sesión. Nombrar la sesión reemplaza el valor predeterminado en esos listados, y también lo hace aceptar un plan.
* Título generado: si no nombra una sesión, Claude Code genera un título de sesión para ella. El título es un resumen breve de su primer indicador, escrito por una solicitud en segundo plano al modelo pequeño/rápido, normalmente un modelo de clase Haiku. Una ejecución `claude -p` que inicia directamente desde un shell o script no obtiene uno.

  Aceptar un plan reemplaza el título del primer indicador con un título basado en el plan. Nombrar la sesión también lo reemplaza.

  Verá el título del primer indicador en el [selector de sesiones](#use-the-session-picker) y en el campo [`session_name`](/docs/es/statusline) de la línea de estado cuando no hay nombre establecido. El título del plan se muestra en los mismos dos lugares y también en los listados de sesiones en ejecución, donde toma el lugar del nombre de visualización predeterminado.

  Puede pasar cualquiera de los dos títulos a `claude --resume` o `/resume`, y Claude Code lo resuelve de la misma manera que un nombre que usted establece.

<h2 id="use-the-session-picker">
  Usar el selector de sesiones
</h2>

Ejecute `/resume` dentro de una sesión, o `claude --resume` sin argumentos, para abrir el selector de sesiones interactivo. Use estos atajos de teclado para navegar, buscar y ampliar la lista:

| Atajo                                                  | Acción                                                                                                                                                                                 |
| :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`                                              | Navegar entre sesiones                                                                                                                                                                 |
| `→` / `←`                                              | Expandir o contraer sesiones agrupadas                                                                                                                                                 |
| `Enter`                                                | Reanuda la sesión resaltada                                                                                                                                                            |
| `Space`                                                | Previsualiza el contenido de la sesión. `Ctrl+V` también funciona en terminales que no lo capturan como pegado                                                                         |
| `Ctrl+R`                                               | Renombra la sesión resaltada                                                                                                                                                           |
| `/` o cualquier carácter imprimible que no sea `Space` | Ingrese al modo de búsqueda y filtre sesiones. Pegue una URL de solicitud de extracción o fusión de GitHub, GitHub Enterprise, GitLab o Bitbucket para encontrar la sesión que la creó |
| `Ctrl+A`                                               | Muestra sesiones de todos los proyectos en esta máquina. Presione nuevamente para volver al repositorio actual                                                                         |
| `Ctrl+W`                                               | Muestra sesiones de todos los worktrees del repositorio actual. Presione nuevamente para volver al worktree actual. Solo se muestra en repositorios con múltiples worktrees            |
| `Ctrl+B`                                               | Filtra a sesiones de la rama git actual. Presione nuevamente para mostrar todas las ramas                                                                                              |
| `Esc`                                                  | Salga del selector de sesiones o del modo de búsqueda                                                                                                                                  |

Cada fila muestra el nombre de la sesión si está establecido; de lo contrario, el título de sesión generado por IA, el resumen de la conversación o el primer indicador, junto con el tiempo desde la última actividad, la rama git y el tamaño del archivo. Amplíe a todos los proyectos con `Ctrl+A` para ver también la ruta del proyecto de cada sesión.

Las sesiones creadas con `/branch` o `--fork-session` obtienen sus propios ID de sesión y aparecen como filas separadas. Cuando el selector encuentra más de una entrada para la misma sesión, las agrupa bajo una sola fila. Presione `→` para expandir un grupo.

Si Claude Code no puede cargar la sesión que selecciona del selector `claude --resume`, imprime [`Failed to resume the conversation`](/docs/es/errors#failed-to-resume-the-conversation) con un comando para reintentar, luego sale con el código 1. Desde el selector `/resume` dentro de una sesión, Claude Code reporta el fallo y su conversación actual continúa ejecutándose.

<h2 id="branch-a-session">
  Ramifique una sesión
</h2>

La ramificación crea una copia de la conversación hasta ahora y lo cambia a ella, dejando el original intacto. Úselo para probar un enfoque diferente sin perder el camino en el que estaba.

Desde dentro de una sesión, ejecute `/branch` con un nombre opcional:

```text theme={null}
/branch try-streaming-approach
```

Si omite el nombre, Claude Code nombra la nueva rama después del primer prompt en la conversación. A partir de v2.1.198 esto también se aplica después de [compactación](/docs/es/how-claude-code-works#when-context-fills-up); las versiones anteriores recurrieron al nombre literal `Branched conversation` en lugar de mirar más allá del resumen de compactación al prompt original.

Desde la línea de comandos, combine `--continue` o `--resume` con `--fork-session`:

```bash theme={null}
claude --continue --fork-session
```

La confirmación de `/branch` imprime dos IDs de sesión: la nueva rama en la que se encuentra ahora y la original. El original no se modifica en el disco y permanece en el selector de sesiones; vuelva a él con `/resume <original-name>` o pasando su ID a `/resume`.

`/branch` copia la transcripción y cambia el proceso de Claude Code en ejecución para escribir en ella. Esa distinción determina lo que hereda la rama:

| Estado                                                                                                                                                                             | Después de `/branch`                                                                                                                                                                                                                 |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Historial de conversación                                                                                                                                                          | Copiado en la rama hasta el punto en el que ejecutó `/branch`                                                                                                                                                                        |
| Permisos de "Permitir para esta sesión"                                                                                                                                            | Se transfieren; la rama se ejecuta en el mismo proceso, por lo que sus permisos existentes aún se aplican. Si bifurca en un proceso separado con `--fork-session`, el nuevo proceso comienza sin ellos y usted vuelve a aprobar allí |
| [Subagentes en segundo plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background) en vuelo y [comandos Bash en segundo plano](/docs/es/interactive-mode#background-bash-commands) | Continúan ejecutándose. Su salida aparece en la nueva rama a la que cambió, no en la sesión original                                                                                                                                 |
| Conexión de [Control Remoto](/docs/es/remote-control)                                                                                                                                   | Se mantiene conectada. Un teléfono o navegador conectado a la sesión lo sigue a la rama y continúa recibiendo nuevos mensajes allí                                                                                                   |

Si reanuda la misma sesión en dos terminales sin bifurcar, los mensajes de ambos se intercalan en una transcripción. Para rewind basado en puntos de control dentro de una sola sesión, vea [Checkpointing](/docs/es/checkpointing).

<h2 id="manage-context-within-a-session">
  Gestione el contexto dentro de una sesión
</h2>

Estos comandos controlan qué hay en la ventana de contexto sin dejar la sesión:

* **`/clear`**: comience de nuevo con un contexto vacío. Claude Code guarda la conversación anterior; reanúdela con `/resume`, o, en el mismo proceso de Claude Code, desde [la entrada de sesión anterior del menú de rewind](/docs/es/checkpointing#rewind-past-a-cleared-conversation). Sin argumentos, la nueva conversación mantiene un nombre que estableció con `--name` o `/rename`, pero no un título de sesión generado por IA. Para nombrar la conversación que está dejando, pase el nombre, como en `/clear release-prep`; la nueva conversación entonces comienza sin nombre
* **`/compact [instructions]`**: reemplace el historial con un resumen, opcionalmente enfocado en lo que especifique
* **`/context`**: muestre qué está consumiendo actualmente el contexto

Para saber cómo la compactación interactúa con CLAUDE.md, skills y reglas, vea la [guía de ventana de contexto](/docs/es/context-window). Para estrategias sobre cuándo limpiar versus compactar, vea [Mejores prácticas](/docs/es/best-practices#manage-your-session).

<h2 id="export-and-locate-session-data">
  Exporte y localice datos de sesión
</h2>

Ejecute `/export` para abrir un menú que le permita copiar la conversación actual a su portapapeles o guardarla como un archivo de texto sin formato, con mensajes y salidas de herramientas renderizadas como texto legible. Pase un nombre de archivo para omitir el menú y escribir directamente en ese archivo.

<h3 id="access-conversations-from-scripts">
  Acceda a conversaciones desde scripts
</h3>

`/export` produce una transcripción renderizada para que una persona la lea. Las interfaces a continuación producen datos estructurados para que un script analice: un resultado JSON de una ejecución, la ruta al archivo de transcripción de una sesión, o un flujo en vivo de eventos. Elija según lo que active el script:

* **Ejecute Claude una vez y capture el resultado**: invoque `claude -p` con [`--output-format json` o `stream-json`](/docs/es/headless#get-structured-output) para capturar el resultado, ID de sesión, uso y costo de una ejecución no interactiva como JSON estructurado.
* **Haga una pregunta a una sesión existente**: pase un ID de sesión a [`claude -p --resume`](/docs/es/headless#continue-conversations) para enviar un mensaje de seguimiento, como una solicitud de resumen, y capture la respuesta estructurada.
* **Reaccione a eventos de sesión**: lea el campo `transcript_path` que [hooks](/docs/es/hooks#common-input-fields) y [comandos de línea de estado](/docs/es/statusline#available-data) reciben como entrada. Un hook `SessionEnd` puede archivar la transcripción cuando finaliza una sesión.
* **Integre Claude en una aplicación TypeScript o Python**: use el [Agent SDK](/docs/es/agent-sdk/overview) para recibir cada mensaje mediante programación.

El ejemplo a continuación utiliza la segunda interfaz. Envía un mensaje de seguimiento a una sesión existente y lee la respuesta con `jq`:

```bash theme={null}
claude -p --resume <session-id> --output-format json "summarize what we changed" | jq -r '.result'
```

<h3 id="where-transcripts-are-stored">
  Dónde se almacenan las transcripciones
</h3>

De forma predeterminada, Claude Code almacena transcripciones como JSONL en `~/.claude/projects/<project>/<session-id>.jsonl`, donde `<project>` es la ruta de su directorio de trabajo con caracteres no alfanuméricos reemplazados por `-`. Para un directorio de trabajo cuyo nombre convertido excede 200 caracteres, Claude Code trunca el nombre a 200 caracteres y añade un hash de la ruta completa, de modo que el nombre del directorio se mantenga dentro de los límites del sistema de archivos.

Cada línea es un objeto JSON para un mensaje, uso de herramienta o entrada de metadatos. El formato de entrada es interno de Claude Code y cambia entre versiones, por lo que los scripts que analizan estos archivos directamente pueden romperse en cualquier versión. Para construir sobre datos de sesión, use `/export` o las [interfaces de script](#access-conversations-from-scripts) en su lugar.

La ubicación, retención y comportamiento de escritura son configurables:

| Para                                                                                                                          | Establecer                                                                                  | Dónde                                                                |
| ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| Mover almacenamiento fuera de `~/.claude`                                                                                     | [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars)                                                         | Variable de entorno                                                  |
| [Nombre el directorio `<project>` usted mismo](#name-the-project-directory-yourself)                                          | [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/es/env-vars)                                              | Variable de entorno                                                  |
| Cambiar la retención de 30 días                                                                                               | [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays)                             | `settings.json`                                                      |
| Establecer un límite de edad para [transcripciones de Claude Desktop y Cowork](/docs/es/claude-directory#cleaned-up-automatically) | [`desktopSessionCleanupPeriodDays`](/docs/es/settings-reference#desktopsessioncleanupperioddays) | Configuración de usuario, configuración administrada, o `--settings` |
| Suprimir escrituras de transcripción en todos los modos                                                                       | [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/es/env-vars)                                           | Variable de entorno                                                  |
| Suprimir escrituras para una ejecución no interactiva                                                                         | [`--no-session-persistence`](/docs/es/cli-reference)                                             | Bandera CLI con `claude -p`                                          |

<h3 id="delete-session-data">
  Elimine datos de sesión
</h3>

Las transcripciones envejecen bajo las [reglas de barrido de retención](/docs/es/claude-directory#cleaned-up-automatically). Para eliminar las transcripciones de un proyecto y el estado relacionado más pronto, ejecute [`claude project purge`](/docs/es/claude-directory#clear-local-data). Si elimina una [sesión en segundo plano](/docs/es/agent-view) con [`claude rm <id>`](/docs/es/agent-view#what-deleting-a-session-removes), su transcripción permanece en el disco y sigue siendo accesible a través de `claude --resume`.

<h3 id="name-the-project-directory-yourself">
  Nombre el directorio del proyecto usted mismo
</h3>

De forma predeterminada, Claude Code deriva el nombre `<project>` de la ruta completa del directorio de trabajo. Para elegir el nombre usted mismo, establezca `CLAUDE_CODE_PROJECT_DIR_NAME` junto con `CLAUDE_CONFIG_DIR`. Claude Code entonces almacena las transcripciones de esa sesión y la [memoria automática](/docs/es/memory#auto-memory) bajo su nombre. Esto es adecuado para un host que integra Claude Code y proporciona a cada sesión su propio directorio de configuración. Requiere Claude Code v2.1.234 o posterior.

Por ejemplo, este lanzamiento mantiene los datos del inquilino A bajo `/srv/tenant-a` y nombra su directorio de proyecto `work`:

```bash theme={null}
CLAUDE_CONFIG_DIR=/srv/tenant-a CLAUDE_CODE_PROJECT_DIR_NAME=work claude
```

Claude Code escribe las transcripciones de la sesión en `/srv/tenant-a/projects/work/` y su memoria automática en `/srv/tenant-a/projects/work/memory/`, sea cual sea el directorio de trabajo.

Se aplican tres reglas cuando lo establece:

* **Establezca `CLAUDE_CONFIG_DIR` también**: el nombre no varía con el directorio de trabajo, por lo que bajo el `~/.claude` predeterminado fusionaría las transcripciones y la memoria automática de cada proyecto en un directorio. Claude Code ignora `CLAUDE_CODE_PROJECT_DIR_NAME` cuando `CLAUDE_CONFIG_DIR` no está establecido.
* **Use 1-64 letras, dígitos, guiones o guiones bajos**: no use un nombre de dispositivo Windows como `con`. Claude Code ignora cualquier otro valor y usa el nombre derivado.
* **Establézcalo en el entorno de shell que inicia `claude`**: Claude Code lo lee una sola vez al inicio desde ese entorno, por lo que un bloque `env` en un archivo de configuración no puede establecerlo.

Una vez que haya nombrado el directorio del proyecto de un directorio de configuración, continúe lanzando con ese nombre. Si inicia Claude Code con el mismo `CLAUDE_CONFIG_DIR` pero sin `CLAUDE_CODE_PROJECT_DIR_NAME`, lee y escribe el directorio derivado nuevamente. Las sesiones almacenadas bajo su nombre permanecen en el disco: presione `Ctrl+A` en el [selector de sesión](#use-the-session-picker) para enumerar sesiones de cada directorio de proyecto bajo ese directorio de configuración, incluido el fijado, y de cualquier manera que lance, [`claude --resume <session-id>`](#resume-a-session) encuentra una sesión almacenada bajo cualquiera de los nombres.

<h2 id="see-also">
  Ver también
</h2>

Estas páginas cubren mecánicas relacionadas de sesión y paralelismo:

* [Worktrees](/docs/es/worktrees): ejecute sesiones paralelas aisladas en ramas separadas
* [Checkpointing](/docs/es/checkpointing): rebobine código y conversación a un punto anterior
* [Ventana de contexto](/docs/es/context-window): qué llena el contexto y qué sobrevive a la compactación
* [Modo no interactivo](/docs/es/headless): comportamiento de sesión bajo `claude -p`
