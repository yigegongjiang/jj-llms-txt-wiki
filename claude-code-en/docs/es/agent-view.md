> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestionar múltiples agentes con la vista de agentes

> Distribuya y gestione muchas sesiones de Claude Code desde una pantalla. La vista de agentes muestra qué está haciendo cada sesión y cuáles necesitan su entrada.

La vista de agentes, abierta con `claude agents`, es una pantalla para todas sus sesiones en segundo plano: qué se está ejecutando, qué necesita su entrada y qué está hecho. Distribuya nuevas sesiones, observe su estado de un vistazo en lugar de desplazarse por transcripciones, e intervenga solo cuando una lo necesite. Cada sesión en segundo plano es una conversación completa de Claude Code que sigue ejecutándose sin una terminal conectada, por lo que puede abrirla, responder y marcharse cuando quiera.

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-light.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=7a186c96ed47d6700d084d77e786be65" className="dark:hidden" alt="Vista de agentes en una terminal: el encabezado muestra Claude Code v2.1.140, el modelo, el directorio de trabajo y un recuento de resumen. Las sesiones se agrupan bajo Necesita entrada, Trabajando y Completado, con una entrada de distribución en la parte inferior y un pie de página de sugerencias de teclado." width="1772" height="780" data-path="images/agent-view-light.png" />

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-dark.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=a5bed7434bae368faea3a8f023b52aa2" className="hidden dark:block" alt="Vista de agentes en una terminal: el encabezado muestra Claude Code v2.1.140, el modelo, el directorio de trabajo y un recuento de resumen. Las sesiones se agrupan bajo Necesita entrada, Trabajando y Completado, con una entrada de distribución en la parte inferior y un pie de página de sugerencias de teclado." width="1772" height="780" data-path="images/agent-view-dark.png" />

Utilice la vista de agentes cuando tenga varias tareas independientes en las que Claude pueda trabajar sin que usted observe cada paso. Distribuya una corrección de errores, una revisión de solicitud de extracción y una investigación de prueba inestable como tres filas, continúe trabajando en otra ventana y verifique cuando una fila muestre que la necesita o tenga un resultado.

Cuando desee trabajar de forma más directa en la sesión de cualquier agente, conéctese a la fila para entrar en la conversación completa.

Para comparar la vista de agentes con subagentes, equipos de agentes y worktrees, consulte [Ejecutar agentes en paralelo](/docs/es/agents).

<Note>
  La vista de agentes es una vista previa de investigación y requiere Claude Code v2.1.139 o posterior. Verifique su versión con `claude --version`. La interfaz y los atajos de teclado pueden cambiar a medida que la función evoluciona.
</Note>

Esta página cubre:

* [Inicio rápido](#quick-start): asigne a Claude una tarea para trabajar en segundo plano, verifique su estado e intervenga cuando sea necesario
* [Monitorear sesiones con la vista de agentes](#monitor-sessions-with-agent-view), incluidos iconos de estado, vista previa y respuesta, conexión, organización y atajos de teclado
* [Distribuir nuevos agentes](#dispatch-new-agents) desde la vista de agentes, desde dentro de una sesión o desde su shell
* [Gestionar sesiones desde el shell](#manage-sessions-from-the-shell) con `claude agents`, `claude attach` y comandos relacionados
* [Cómo se alojan las sesiones en segundo plano](#how-background-sessions-are-hosted) por el proceso supervisor

<h2 id="quick-start">
  Inicio rápido
</h2>

Este tutorial cubre el bucle principal de la vista de agentes: distribuir una tarea, observar cómo se actualiza su fila mientras Claude trabaja, echar un vistazo para verificar y responder, y conectarse para la conversación completa. La sesión que distribuye sigue ejecutándose después de cerrar la vista de agentes, por lo que puede irse y volver a ella.

<Steps>
  <Step title="Abrir la vista de agentes">
    Desde su shell, ejecute:

    ```bash theme={null}
    claude agents
    ```

    La vista de agentes se abre con una entrada en la parte inferior y una tabla que se completa a medida que comienzan las sesiones. Presione `Esc` en cualquier momento para volver a su shell. Sus sesiones siguen ejecutándose mientras está fuera y reaparecen la próxima vez que abra la vista de agentes.
  </Step>

  <Step title="Distribuir una sesión">
    Escriba un mensaje describiendo una tarea y presione `Enter`. Una nueva sesión en segundo plano comienza en esa tarea y aparece como una fila que muestra si está funcionando, esperando su entrada o está hecha. La nueva sesión utiliza el modelo mostrado en el encabezado de la vista de agentes y el mismo [modo de permisos](#permission-mode-model-and-effort) que obtendría ejecutando `claude` en ese directorio.

    Cada mensaje que ingrese aquí inicia su propia sesión nueva. Escribir otro mensaje y presionar `Enter` lanza una segunda sesión junto a la primera en lugar de enviar una continuación a ella. Puede ejecutar varias en paralelo de esta manera.

    Cada sesión utiliza su cuota de suscripción de forma independiente, así que consulte [Limitaciones](#limitations) antes de distribuir muchas a la vez.
  </Step>

  <Step title="Echar un vistazo y responder">
    Seleccione una fila con las teclas de flecha y presione `Space` para abrir el panel de vista previa. Muestra la salida más reciente de la sesión, o la pregunta en la que está esperando, en lugar de la transcripción completa. Escriba una respuesta y presione `Enter` para enviarla sin salir de la vista de agentes.
  </Step>

  <Step title="Conectar y desconectar">
    Presione `Enter` o `→` en una fila para conectarse cuando desee la conversación completa. La sesión toma el control de la terminal como una sesión completa e interactiva de Claude Code. Presione `←` en un mensaje vacío para desconectarse y volver a la tabla.
  </Step>

  <Step title="Traer una sesión existente">
    Este paso necesita una sesión en ejecución. Si siguió los pasos anteriores, no tiene una abierta en esta terminal, así que abra una sesión regular de `claude` en otra terminal y envíele un mensaje primero. Para mover una sesión que ya tiene abierta a la vista de agentes, ejecute `/bg` dentro de ella, o presione `←` en un mensaje vacío para enviarla al segundo plano y abrir la vista de agentes en un paso. La sesión sigue ejecutándose y aparece como una fila junto a las que distribuyó.
  </Step>
</Steps>

Puede usar `claude agents` como su punto de entrada principal en lugar de `claude`: distribuya cada tarea desde la vista de agentes, conéctese cuando desee la conversación completa, y presione `←` para volver a la tabla.

Dentro de una sesión regular de `claude`, la pista `←` del pie de página del mensaje cuenta los agentes en segundo plano que están esperando su entrada, como `← 2 agents`, y vuelve a `← for agents` cuando ninguno necesita entrada. Los conteos superiores a 99 se muestran como `99+`. El conteo se actualiza aproximadamente cada diez segundos mientras la terminal está enfocada e inmediatamente cuando el enfoque regresa. Cambia brevemente de color cuando se mueve y cuando un agente se completa, a menos que la configuración [`prefersReducedMotion`](/docs/es/settings#available-settings) esté activada, y se oculta en [modo de lector de pantalla](/docs/es/accessibility). En [Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry](/docs/es/third-party-integrations), la pista permanece en su forma simple `← for agents` sin el conteo. Requiere Claude Code v2.1.205 o posterior.

<h2 id="monitor-sessions-with-agent-view">
  Monitorear sesiones con la vista de agentes
</h2>

Ejecute `claude agents` para abrir la vista de agentes. Toma el control de la terminal completa y enumera cada sesión agrupada por estado, con sesiones fijadas y las que lo necesitan en la parte superior. Cada fila muestra el nombre de la sesión, la actividad actual y su antigüedad, contada desde cuándo se creó la sesión; la antigüedad de una sesión terminada se congela en cuánto tiempo tardó la ejecución.

El nombre está teñido con el color establecido por [`/color`](/docs/es/commands) en esa sesión. A partir de v2.1.199, el color se mantiene cuando [envía una sesión al segundo plano](#from-inside-a-session) con `←` o `/background`.

De forma predeterminada, la lista muestra cada sesión en segundo plano que ha iniciado, en todos sus proyectos. Una sesión que trabaja en un repositorio y otra en un worktree diferente aparecen aquí, independientemente de qué directorio abrió la vista de agentes. Para limitar la lista a un proyecto, pase `--cwd`:

```bash theme={null}
claude agents --cwd ~/projects/my-app
```

Esto muestra solo las sesiones iniciadas en ese directorio. Una sesión que se ha [movido a un worktree](#how-file-edits-are-isolated) bajo `~/projects/my-app/.claude/worktrees/` sigue contando como perteneciente a `~/projects/my-app`.

Las sesiones interactivas que tiene abiertas en otras terminales no aparecen hasta que las [envíe al segundo plano](#from-inside-a-session). Los [subagentes](/docs/es/sub-agents) y [compañeros de equipo](/docs/es/agent-teams) que una sesión genera no se enumeran como filas separadas.

```text theme={null}
Pinned
  ✽ clawd walk cycle          Drawing the walk-cycle sprite frames          3m

Ready for review
  ∙ jump physics              Opened PR with collision fix                 #2048  2h

Needs input
  ✻ power-up design           double jump or wall climb?                    1m

Working
  ✽ collision detection       Adding swept-AABB checks to CollisionSystem   2m
  ✢ playtest level 3          run 12 · all checkpoints cleared           in 4m

Completed
  ✻ title screen              result: menu, options, and credits done       9m
  ∙ sound effects             result: 14 SFX exported to assets/audio       4h
  … 6 more
```

<h3 id="read-session-state">
  Leer el estado de la sesión
</h3>

Cada fila comienza con un icono cuyo color y animación muestran el estado de la sesión:

| Estado           | El icono se muestra como | Qué significa                                                                       |
| :--------------- | :----------------------- | :---------------------------------------------------------------------------------- |
| Funcionando      | Animado                  | Claude está ejecutando activamente herramientas o generando una respuesta           |
| Necesita entrada | Amarillo                 | Claude está esperando una pregunta específica o una decisión de permiso de su parte |
| Inactivo         | Atenuado                 | La sesión no tiene nada que hacer y está lista para su próximo mensaje              |
| Completado       | Verde                    | La tarea se completó exitosamente                                                   |
| Falló            | Rojo                     | La tarea terminó con un error                                                       |
| Detenido         | Gris                     | La sesión fue detenida con `Ctrl+X` o `claude stop`                                 |

Por separado, la forma del icono muestra si el proceso subyacente está ejecutándose:

| Forma             | Qué significa                                                                                                                          |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `✻` o `✽` animado | El proceso de la sesión está activo y responde inmediatamente                                                                          |
| `∙`               | El proceso ha salido. Aún puede echar un vistazo, responder o conectarse, y Claude reinicia desde donde se quedó                       |
| `✢`               | Una sesión [`/loop`](/docs/es/scheduled-tasks) durmiendo entre iteraciones. La fila muestra su recuento de ejecución y una cuenta regresiva |

La etiqueta `#N` que puede aparecer en el borde derecho de una fila es la [solicitud de extracción que abrió la sesión](#pull-request-status), no parte del icono de estado.

El título de la pestaña de terminal muestra el recuento de entrada pendiente mientras la vista de agentes está abierta: `2 awaiting input · claude agents` cuando las sesiones necesitan entrada, o `claude agents` cuando no lo hacen.

A partir de v2.1.198, mientras la vista de agentes está abierta, Claude Code también envía una notificación a través de su [canal de notificación de terminal](/docs/es/terminal-config#get-a-terminal-bell-or-notification) configurado cuando una sesión en segundo plano local comienza a necesitar su entrada, se completa o falla. Las sesiones que se ejecutan según un cronograma, como sesiones [`/loop`](/docs/es/scheduled-tasks), notifican solo cuando necesitan su entrada. Las notificaciones utilizan la misma configuración [`preferredNotifChannel`](/docs/es/settings#available-settings) que el resto de Claude Code y activan el hook [`Notification`](/docs/es/hooks#notification) con el tipo `agent_needs_input` o `agent_completed`.

Las sesiones en segundo plano no necesitan ninguna terminal abierta para seguir funcionando. Un [proceso supervisor](#the-supervisor-process) separado las ejecuta, por lo que puede cerrar la vista de agentes, cerrar su shell o iniciar una nueva sesión interactiva y su trabajo distribuido sigue adelante.

El estado de la sesión persiste en el disco a través de actualizaciones automáticas y reinicios del supervisor. Las sesiones también se conservan cuando su máquina se duerme. Sus procesos se reanudan al despertar y el supervisor se reconecta a ellos en lugar de tratar la brecha de tiempo como inactividad. El apagado aún detiene las sesiones en ejecución; consulte [Las sesiones se muestran como fallidas después del apagado](#sessions-show-as-failed-after-shutdown) para saber cómo recuperarlas.

Cuando abre una sesión que ha dejado de responder, el supervisor reinicia su proceso y la sesión continúa la respuesta interrumpida desde donde se quedó. Una sesión puede terminar en ese estado cuando la máquina se duerme mientras está en medio de una respuesta. Requiere Claude Code v2.1.200 o posterior.

<h3 id="row-summaries">
  Resúmenes de filas
</h3>

El resumen de una línea en cada fila es generado por un [modelo de clase Haiku](/docs/es/model-config) para que la fila pueda decirle qué está haciendo la sesión, qué necesita o qué produjo sin abrir la transcripción. Mientras una sesión está funcionando activamente, el texto de la fila se actualiza como máximo una vez cada 15 segundos desde la salida reciente de la sesión sin enviar una solicitud de modelo, y el modelo escribe un resumen nuevo cuando cada turno termina.

Una fila funcionando muestra lo que la sesión dice que está haciendo, y una fila bloqueada muestra la pregunta que está haciendo. Durante un turno largo, el modelo también reescribe el resumen aproximadamente una vez por minuto, esperando el doble después de cada reescritura hasta cuatro minutos, por lo que una fila ocupada no sigue mostrando un resumen desactualizado. Antes de v2.1.205, una fila funcionando podría mostrar una invocación de herramienta sin procesar en lugar de un informe, y una sesión que ejecuta elementos de trabajo paralelos mostraba un recuento `done/total` como `2/5` antes del texto.

El texto de resumen llena el ancho restante de la fila y se trunca solo en el borde derecho de la terminal; abra el [panel de vista previa](#peek-and-reply) para leer una oración que el borde recorta. Antes de v2.1.206, el texto se cortaba en 64 columnas independientemente del ancho de la terminal.

Cuando la lista está [agrupada por directorio](#organize-the-list), el resumen se abre con el estado de la sesión como una palabra coloreada, como `Needs input · double jump or wall climb?`. En la agrupación de estado predeterminada, el encabezado del grupo ya nombra el estado, por lo que la fila muestra solo el resumen. Antes de v2.1.205, las filas agrupadas por directorio no llevaban palabra de estado.

Un turno cuya salida completa no contiene letras ni dígitos, como una sesión [`/loop`](/docs/es/scheduled-tasks) que imprime un símbolo solitario en una iteración tranquila, mantiene el resumen y estado anterior de la fila. Antes de v2.1.205, ese turno fue reclasificado y podría voltear una sesión que estaba esperando su entrada de vuelta a `Working`.

El resumen de fin de turno y cada reescritura a mitad de turno son una solicitud corta de clase Haiku a través de su proveedor normal, facturada y manejada bajo los mismos [términos de uso de datos](/docs/es/data-usage) que la sesión misma. Las actualizaciones de 15 segundos entre reescrituras de modelo reutilizan la salida de la sesión y no envían una solicitud. En proveedores de terceros como Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry y puertas de enlace personalizadas, la solicitud se revierte al modelo principal de la sesión cuando no hay ningún modelo Haiku configurado. Establezca [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/es/model-config#environment-variables) para elegir el modelo para estos resúmenes en esos proveedores.

<h3 id="pull-request-status">
  Estado de la solicitud de extracción
</h3>

Cuando una sesión abre una solicitud de extracción, aparece una etiqueta `#1234` en el borde derecho de la fila, vinculada a la solicitud de extracción en terminales que admiten hipervínculos. La etiqueta persiste cuando envía un seguimiento a la sesión, por lo que la solicitud de extracción permanece visible mientras la fila revierte al progreso en vivo. Las sesiones en segundo plano que aislaron sus cambios en un worktree abren estas solicitudes de extracción por sí solas; [Cómo se aíslan los cambios de archivo](#how-file-edits-are-isolated) cubre cuándo sucede eso y qué nunca hace una sesión sin preguntar.

Una sesión que trabaja en una solicitud de extracción existente está vinculada a ella de la misma manera. Editar, comentar, cerrar o marcar una solicitud de extracción como lista con `gh` vincula la solicitud de extracción que el resultado de su propio comando nombra, por lo que un comando `gh` cuya salida capturada no nombra ninguna solicitud de extracción no crea un vínculo; `gh pr merge` es el caso común, porque imprime su resultado solo en una terminal interactiva. Verificar una solicitud de extracción con `gh pr checkout`, o enviar a una rama que tiene una solicitud de extracción abierta, la vincula buscando esa rama con `gh pr view` en su lugar. Antes de v2.1.205, solo las solicitudes de extracción que la sesión creó o verificó fueron vinculadas, y un envío vinculó una solo cuando el nombre de rama local coincidía.

Claude Code lee la solicitud de extracción de la salida de comando completa, incluida la parte guardada en un archivo cuando la salida de un comando excede el límite en línea. Antes de v2.1.205, una solicitud de extracción creada en una llamada Bash cuya salida excedía aproximadamente 30,000 caracteres no fue vinculada.

Cuando una sesión está vinculada a más de una solicitud de extracción, la etiqueta muestra un recuento en su lugar, como `3 PRs`, coloreada por la solicitud de extracción abierta que más necesita atención. Abra el [panel de vista previa](#peek-and-reply) para verlas todas.

El número de la solicitud de extracción está coloreado por su estado:

| Color    | Estado de la solicitud de extracción                               |
| :------- | :----------------------------------------------------------------- |
| Amarillo | Esperando verificaciones o revisión, o las verificaciones fallaron |
| Verde    | Las verificaciones pasaron y ninguna revisión está bloqueando      |
| Púrpura  | Fusionado                                                          |
| Gris     | Borrador o cerrado                                                 |

Para la mayoría de las tareas, esta columna es donde recopila el resultado: revise y fusione la solicitud de extracción cuando su número se vuelva verde.

<h3 id="peek-and-reply">
  Echar un vistazo y responder
</h3>

Presione `Space` en una fila seleccionada para abrir el panel de vista previa. Se abre con la oración que la fila trunca en el borde de la terminal, y cuál es esa oración depende del estado de la sesión:

* Una sesión que está esperando su entrada: la pregunta exacta que está haciendo, encima de la entrada de respuesta
* Una sesión terminada: su resultado
* Una sesión funcionando: su oración de estado completo

Cualquier solicitud de extracción vinculada a la sesión se enumera a continuación. Para una sesión que está esperando su entrada, una línea como `waiting 3m` debajo de ellas muestra cuánto tiempo ha estado esperando, y es la única vez que se muestra en el panel. La antigüedad en el borde derecho de la fila es un número diferente: cuenta desde cuándo comenzó la sesión.

La mayoría de las veces el panel de vista previa es suficiente y no necesita abrir la transcripción completa.

Antes de v2.1.207, cada vista previa se abría con la oración de estado y una marca de tiempo desnuda, y una sesión bloqueada tenía su pregunta apareciendo debajo de ellas con el prefijo de la misma marca de tiempo una segunda vez.

Escriba una respuesta en el panel de vista previa y presione `Enter` para enviarla a esa sesión. Cuando la sesión está haciendo una pregunta de opción múltiple, el panel de vista previa muestra las opciones y puede presionar una tecla numérica para elegir una. Para otras sesiones bloqueadas, presione `Tab` para llenar la entrada con una respuesta sugerida que puede editar antes de enviar. Prefije una respuesta con `!` para enviar un comando Bash en su lugar.

Una respuesta que no se puede entregar, porque el servicio en segundo plano es inaccesible o el envío falla, se guarda y se envía a la sesión como su próximo mensaje cuando su proceso se inicia nuevamente, y el mensaje de error dice que la respuesta fue guardada. Una respuesta con prefijo `!` no se guarda, porque el texto guardado llegaría a la sesión como un mensaje simple en lugar de ejecutarse como un comando Bash.

Con [dictado de voz](/docs/es/voice-dictation) habilitado, mantenga o toque su tecla de pulsar para hablar mientras la entrada de respuesta está enfocada para dictar una respuesta en lugar de escribirla. Lo mismo funciona en la entrada de distribución en la parte inferior de la vista de agentes.

Use `↑` y `↓` para echar un vistazo a sesiones adyacentes sin cerrar el panel, o `→` para conectarse.

<h3 id="attach-to-a-session">
  Conectarse a una sesión
</h3>

Presione `Enter` o `→` en una fila seleccionada para conectarse. La vista de agentes es reemplazada por la sesión interactiva completa. Cuando se conecta, Claude publica un breve resumen de lo que sucedió mientras estaba fuera.

Mientras está conectado, la sesión se comporta como cualquier otra sesión de Claude Code: [comandos](/docs/es/commands), atajos de teclado y características funcionan todos, con las excepciones a continuación.

Una sesión en segundo plano rechaza `/install-github-app` y la lista de configuración [`/mcp`](/docs/es/mcp), incluidas sus acciones de autenticación, ya sea que esté conectado o respondiendo desde el panel de vista previa. El mensaje lo dirige a una sesión `claude` regular, y `/mcp reconnect <server>`, `/mcp enable` y `/mcp disable` aún funcionan.

Las sesiones conectadas siempre se renderizan en [modo de pantalla completa](/docs/es/fullscreen), independientemente de su configuración `tui`, porque una sesión en segundo plano no tiene desplazamiento de terminal para agregar. Desplácese con `PgUp`, `PgDn` o la rueda del ratón, y presione `Ctrl+O` para el modo de transcripción. El desplazamiento nativo de su terminal y el modo de copia de tmux muestran solo la ventana gráfica actual, igual que cuando ejecuta cualquier aplicación de pantalla completa.

Presione `←` en un mensaje vacío, o ejecute `/exit`, para desconectarse y volver a la vista de agentes. A partir de v2.1.198, esto funciona de la misma manera si abrió la sesión desde la vista de agentes o con `claude attach <id>` desde su shell.

`Ctrl+Z` también se desconecta pero vuelve a donde comenzó: la vista de agentes si se conectó desde allí, o su shell si ejecutó `claude attach`. Use `Ctrl+Z` cuando un diálogo tiene el enfoque y no responde a `←`.

`Ctrl+C` mantiene su comportamiento de interrupción estándar mientras está conectado: cancela una respuesta en ejecución o un comando shell `!` en lugar de desconectarse. Presionar `Ctrl+C` dos veces en un mensaje vacío se desconecta, igual que en cualquier sesión.

Desconectarse nunca detiene una sesión en segundo plano: `←`, `Ctrl+Z`, `/exit`, y doble `Ctrl+C` o doble `Ctrl+D` la dejan ejecutándose. Para terminar una sesión desde dentro de ella, ejecute `/stop`.

En una sesión que se ejecuta en primer plano, una que inició en la terminal en lugar de conectarse desde la vista de agentes, presionar `←` en un mensaje vacío la envía al segundo plano y abre la vista de agentes con esa fila seleccionada, por lo que puede cambiar de sesión sin salir de la terminal. El mismo presionar único desconecta una sesión conectada.

Si una herramienta se está ejecutando cuando presiona `←`, Claude Code espera hasta aproximadamente diez segundos para que termine antes de enviar al segundo plano, y la respuesta continúa en la sesión en segundo plano. Presione `←` nuevamente para enviar al segundo plano inmediatamente en lugar de esperar. Cuando el trabajo en vuelo no puede trasladarse a la sesión en segundo plano, aparece primero el diálogo `Background this session?`, igual que con [`/background`](#from-inside-a-session).

El límite de diez segundos no se aplica mientras [subagentes](/docs/es/sub-agents) están en ejecución. Claude Code sigue esperando para que su trabajo se traslade, y muestra un aviso `Still backgrounding after the current tool` mientras espera; presione `←` nuevamente para enviar al segundo plano sin esperar, lo que reinicia los subagentes desde el principio. Antes de v2.1.203, la espera terminaba después de diez segundos y los subagentes en ejecución se reiniciaban desde el principio sin advertencia.

La fila se crea incluso desde una sesión nueva sin historial de conversación, por lo que `→` vuelve a ella. Antes de v2.1.203, la vista de agentes mostraba una sugerencia de incorporación debajo de esa fila cuando era la única.

Puede desactivar este atajo con la configuración `leftArrowOpensAgents` en `/config`.

<h3 id="organize-the-list">
  Organizar la lista
</h3>

La vista de agentes agrupa sesiones para que las que necesitan entrada estén en la parte superior, con `Ready for review` y `Needs input` por encima de `Working` y `Completed`. Estos nombres de grupo no se asignan uno a uno a los [estados](#read-session-state) anteriores: una sesión se mueve a `Ready for review` cuando tiene una solicitud de extracción abierta, y `Completed` recopila sesiones terminadas, fallidas y detenidas juntas.

Presione `Ctrl+S` para agrupar por directorio en su lugar. Su elección persiste entre ejecuciones.

Dentro de un grupo:

* Presione `Ctrl+T` para fijar una sesión en la parte superior y [mantener su proceso ejecutándose](#the-supervisor-process) mientras está inactivo
* Presione `Shift+↑` o `Shift+↓` para reordenar sesiones
* Presione `Ctrl+R` para renombrar una sesión
* Presione `Enter` en un encabezado de grupo para contraerlo

Para eliminar una sesión de la lista, presione `Ctrl+X` para detenerla y `Ctrl+X` nuevamente dentro de dos segundos para eliminarla. Presionar `Ctrl+X` en un encabezado de grupo elimina cada sesión en ese grupo después de la confirmación.

Eliminar elimina la sesión de la vista de agentes. Si Claude [creó un worktree](#how-file-edits-are-isolated) para la sesión, eliminar elimina ese worktree también, incluidos los cambios sin confirmar en él, por lo que envíe o confirme el trabajo que desea conservar primero. Un worktree que creó usted mismo e inició la sesión dentro se deja en su lugar. La transcripción de conversación permanece en su máquina local y sigue siendo accesible a través de `claude --resume`.

Eliminar nunca elimina un worktree con commits que no se han enviado a ningún lugar, o uno que otra sesión en ejecución reclama o ha bloqueado. Claude Code mantiene el worktree y la sesión, y el pie de página nombra la ruta mantenida y la razón. Envíe los commits, o cierre la otra sesión, luego elimine nuevamente.

Eliminar también borra la sesión de la [lista de sesiones del supervisor](#the-supervisor-process), ya sea que elimine con `Ctrl+X` o con [`claude rm`](#manage-sessions-from-the-shell) desde el shell, por lo que la eliminación persiste entre reinicios del supervisor. Antes de v2.1.206, eliminar una sesión mientras el supervisor se estaba reiniciando o era inaccesible la dejaba en esa lista, y el siguiente supervisor reinició su proceso y mostró la fila nuevamente.

Las sesiones completadas que no caben en la pantalla se pliegan en una fila `… N more`. Los fallos y las sesiones con una solicitud de extracción abierta siempre permanecen visibles. El grupo `Completed` llena el espacio vertical restante después de los grupos activos, y en una terminal corta el encabezado se compacta a una línea de resumen única para que las sesiones que están funcionando o necesitan entrada permanezcan visibles.

<h3 id="filter-sessions">
  Filtrar sesiones
</h3>

Escriba en la entrada de distribución para filtrar en lugar de distribuir:

| Filtro                      | Muestra                                                                                                     |
| :-------------------------- | :---------------------------------------------------------------------------------------------------------- |
| `a:<name>`                  | Sesiones que ejecutan el agente nombrado                                                                    |
| `s:<state>`                 | Sesiones en el estado dado, como `s:working`. También acepta `s:blocked` para todo lo que lo espera a usted |
| `#<number>` o una URL de PR | La sesión que trabaja en esa solicitud de extracción                                                        |
| Cualquier otra URL          | La sesión cuyo primer mensaje contenía esa URL                                                              |

<h3 id="keyboard-shortcuts">
  Atajos de teclado
</h3>

Presione `?` en la vista de agentes para ver cada atajo en contexto. La tabla a continuación los resume.

| Atajo                 | Acción                                                                                                |
| :-------------------- | :---------------------------------------------------------------------------------------------------- |
| `↑` / `↓`             | Moverse entre filas                                                                                   |
| `Enter`               | Conectarse a la sesión seleccionada, o distribuir si hay texto en la entrada                          |
| `Space`               | Abrir o cerrar el panel de vista previa para la sesión seleccionada                                   |
| `Shift+Enter`         | Distribuir y conectarse inmediatamente                                                                |
| `→`                   | Conectarse a la sesión seleccionada                                                                   |
| `Alt+1`..`Alt+9`      | Conectarse a la sesión 1–9 en el directorio de la sesión enfocada                                     |
| `Tab`                 | En una entrada vacía, examinar todos los subagentes. De lo contrario, aplicar la sugerencia resaltada |
| `Ctrl+S`              | Cambiar agrupación entre estado y directorio                                                          |
| `Ctrl+T`              | Fijar o desfijar la sesión seleccionada                                                               |
| `Ctrl+R`              | Renombrar la sesión seleccionada                                                                      |
| `Ctrl+G`              | Abrir el mensaje de distribución en su `$VISUAL` o `$EDITOR`                                          |
| `Ctrl+X`              | Detener la sesión; presione nuevamente dentro de dos segundos para eliminarla                         |
| `Shift+↑` / `Shift+↓` | Reordenar la sesión seleccionada                                                                      |
| `Esc`                 | Cerrar el panel de vista previa, limpiar la entrada o salir                                           |
| `Ctrl+C`              | Limpiar la entrada; presione dos veces para salir                                                     |
| `?`                   | Mostrar todos los atajos                                                                              |

<h2 id="dispatch-new-agents">
  Distribuir nuevos agentes
</h2>

Puede distribuir nuevas sesiones en segundo plano desde la vista de agentes, enviar una sesión interactiva existente al segundo plano o iniciar una directamente desde el shell.

<h3 id="from-agent-view">
  Desde la vista de agentes
</h3>

Escriba un mensaje en la entrada en la parte inferior de la vista de agentes y presione `Enter` para iniciar una nueva sesión en segundo plano. La sesión se nombra automáticamente a partir del mensaje; renómbrela más tarde con `Ctrl+R`.

Un nombre que la sesión obtiene más tarde también aparece en su fila, incluido el nombre que Claude deriva cuando [acepta un plan](/docs/es/permission-modes#review-and-approve-a-plan) en esa sesión. Antes de v2.1.207, una sesión en segundo plano nombrada al aceptar un plan mostraba ese nombre en `/status` pero no en su fila de vista de agentes hasta que la renombraba usted mismo.

Pegue una imagen en el mensaje para incluir una captura de pantalla o diagrama con la tarea.

El texto pegado más largo que 800 caracteres o más de dos líneas se contrae a un marcador de posición `[Pasted text #N]` para que la entrada permanezca en una línea; el texto completo se envía cuando distribuye. Para revisar o editar el texto contraído antes de distribuir, pegue el mismo texto nuevamente y el marcador de posición se expande nuevamente en la entrada. Un recordatorio `paste again to expand` aparece debajo de la entrada durante unos segundos después del pegado en terminales de al menos 90 columnas de ancho. Antes de v2.1.207, pegar el mismo texto nuevamente agregaba un segundo marcador de posición en lugar de expandir el primero.

Prefije o mencione partes del mensaje para controlar cómo comienza la sesión:

| Entrada                                          | Efecto                                                                                                                                                                                           |
| :----------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `<agent-name> <prompt>`                          | Si la primera palabra coincide con un nombre de [subagente](/docs/es/sub-agents) personalizado, ese subagente se ejecuta como el agente principal de la sesión con la configuración de su frontmatter |
| `@<agent-name>`                                  | Mencione un subagente personalizado en cualquier lugar del mensaje para ejecutarlo como el agente principal                                                                                      |
| `@<repo>`                                        | Mencione un repositorio para ejecutar la sesión allí. Consulte [Distribuir a un directorio específico](#dispatch-to-a-specific-directory) para ver qué repositorios se enumeran                  |
| `/<command>`                                     | Sugiera [skills](/docs/es/skills) y [commands](/docs/es/commands) para distribuir como el mensaje                                                                                                          |
| `! <command>`                                    | Ejecute un comando de shell como un trabajo en segundo plano en lugar de iniciar una sesión de Claude. El trabajo aparece como una fila a la que puede conectarse, observar y desconectarse      |
| `#<number>` o una URL de solicitud de extracción | Si una sesión ya está trabajando en ese PR, selecciónela en lugar de distribuir                                                                                                                  |
| `Shift+Enter`                                    | Distribuya e inmediatamente conéctese a la nueva sesión                                                                                                                                          |

Un pequeño conjunto de comandos se ejecutan en la vista de agentes en sí en lugar de distribuirse:

* `/exit` y `/quit` cierran la vista de agentes
* `/logout` cierra su sesión
* `/model` establece el [modelo de distribución](#set-the-model)
* A partir de v2.1.198, `/login` abre el diálogo de inicio de sesión para que pueda iniciar sesión nuevamente sin conectarse a una sesión

Skills, sus propios comandos y built-ins que expanden prompts como `/init` se envían a una nueva sesión en segundo plano como su primer mensaje. Otros comandos built-in muestran una sugerencia `attach to a session to run it` en su lugar. Todo lo que escribió permanece en la entrada junto a la sugerencia para que pueda editarlo. Antes de v2.1.203, la sugerencia borraba la entrada y el texto escrito se perdía.

Empaquetar una tarea recurrente como un [skill](/docs/es/skills) le permite iniciar el mismo flujo de trabajo desde la vista de agentes repetidamente sin reescribir el mensaje.

Cuando el mismo `@name` coincide tanto con un subagente como con un repositorio hermano, el subagente tiene prioridad. La coincidencia de primera palabra sin `@` también se aplica, por lo que un mensaje que comienza con uno de sus nombres de subagente distribuye ese subagente en lugar de tratar la palabra como texto plano. Use la forma `@` cuando desee ser explícito, o comience el mensaje con una palabra diferente para evitar la coincidencia.

<h4 id="dispatch-to-a-specific-directory">
  Distribuir a un directorio específico
</h4>

Una nueva sesión se ejecuta en el directorio desde el que abrió la vista de agentes. Para dirigirse a un directorio diferente, use cualquiera de estos:

* Abra `claude agents` en ese directorio.
* Abra `claude agents` en un directorio padre y mencione un repositorio hijo con `@<repo>` en el mensaje. Escribir `@` enumera estos objetivos:

  * Repositorios Git un nivel por debajo del directorio de lanzamiento
  * Los [git worktrees](/docs/es/worktrees) registrados del repositorio desde el que se lanzó que viven dentro de su árbol de directorios, como los que Claude crea bajo `.claude/worktrees/`, etiquetados con su rama extraída. Los worktrees agregados fuera del repositorio, como con `git worktree add ../feature`, no se enumeran
  * Cualquier directorio que ya tenga una sesión en la lista

  Un directorio cuyo nombre contiene un espacio no se enumera. Antes de v2.1.203, los worktrees registrados no se enumeraban, por lo que distribuir en uno significaba ejecutar `claude --bg` desde el directorio de ese worktree.
* Desde el shell, `cd` al directorio y ejecute `claude --bg "<prompt>"`.

Cuando la vista de agentes se agrupa por directorio, el directorio de la fila resaltada se convierte en el objetivo de distribución, por lo que puede desplazarse a un grupo y distribuir en él sin reescribir la ruta.

<h3 id="from-inside-a-session">
  Desde dentro de una sesión
</h3>

Ejecute `/background` o su alias `/bg` para mover la conversación actual a una sesión en segundo plano. Pase un mensaje como `/bg run the test suite and fix any failures` para dar una instrucción más primero. Si Claude está respondiendo cuando ejecuta `/bg`, la respuesta continúa en la sesión en segundo plano.

Salir de una sesión interactiva que aún tiene trabajo en segundo plano en ejecución, como subagentes, comandos de shell en segundo plano, flujos de trabajo o [monitores](/docs/es/tools-reference#monitor-tool), muestra un diálogo `Background work is running` en lugar de salir inmediatamente. A partir de v2.1.198, el diálogo ofrece `Move to background and exit` junto con `Exit anyway` y `Stay`. Elegirlo mueve la sesión al segundo plano de la misma manera que `/background` lo hace, luego lo devuelve a su shell, por lo que el trabajo que puede continuar sigue ejecutándose y la sesión aparece en la vista de agentes. La opción no se muestra cuando la vista de agentes está [desactivada](#turn-off-agent-view).

Enviar al segundo plano desde una sesión interactiva inicia un proceso nuevo que se reanuda desde la conversación guardada, y el trabajo en vuelo se traslada a él: comandos shell en segundo plano en ejecución, subagentes en segundo plano, flujos de trabajo dinámicos y tareas programadas que creó con [`/loop`](/docs/es/scheduled-tasks) se trasladan a la sesión en segundo plano y siguen ejecutándose allí. Un subagente se traslada junto con todo lo que inició, por lo que se traslada solo cuando todo ese trabajo puede trasladarse también, incluso en Windows. Para detener el trabajo en vuelo en lugar de trasladarlo, establezca la variable de entorno [`CLAUDE_DISABLE_ADOPT=1`](/docs/es/env-vars#variables); Claude Code luego le pide que confirme antes de enviar al segundo plano.

El trabajo que no puede trasladarse, como un [monitor](/docs/es/tools-reference#monitor-tool) en ejecución, se detiene. Un subagente en segundo plano que posee un monitor se detiene junto con él. Cuando algún trabajo de este tipo se está ejecutando, Claude Code muestra un diálogo `Background this session?` para que pueda confirmar antes de que se detenga.

Una vez en segundo plano, la sesión puede iniciar nuevos subagentes, monitores y comandos en segundo plano, y esos continúan ejecutándose en desconexiones y reconexiones posteriores.

Las banderas de configuración del lanzamiento original se transfieren a la sesión enviada al segundo plano, por lo que sus servidores MCP, configuración y modelo de respaldo permanecen en vigor:

* `--mcp-config` y `--strict-mcp-config`
* `--settings`
* `--add-dir`
* `--plugin-dir`
* `--fallback-model`
* `--allow-dangerously-skip-permissions`

Los directorios que agregó durante la sesión con [`/add-dir`](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) también se transfieren.

Transferir `--allow-dangerously-skip-permissions` mantiene `bypassPermissions` accesible en la sesión enviada al segundo plano, pero no otorga nada nuevo. El modo aún requiere la misma aceptación interactiva única descrita en [Modo de permiso, modelo y esfuerzo](#permission-mode-model-and-effort) antes de que cualquier sesión pueda usarlo.

<h3 id="from-your-shell">
  Desde su shell
</h3>

Pase `--bg` o su forma larga `--background` para iniciar una sesión que vaya directamente al segundo plano:

```bash theme={null}
claude --bg "investigate the flaky SettingsChangeDetector test"
```

El mensaje es el argumento posicional, no un valor `-p`. A partir de v2.1.198, combinar `--bg` con `-p` o `--print` se rechaza con un error antes de que se cree cualquier sesión, porque `--print` nunca inicia la sesión interactiva a la que `claude agents` se conecta.

Para ejecutar un subagente específico como el agente principal de la sesión, combine `--bg` con `--agent`:

```bash theme={null}
claude --agent code-reviewer --bg "address review comments on PR 1234"
```

Pase `--name` para establecer el nombre de visualización de la sesión en la vista de agentes en lugar del generado automáticamente:

```bash theme={null}
claude --bg --name "flaky-test-fix" "investigate the flaky SettingsChangeDetector test"
```

Después de enviar al segundo plano, Claude imprime el ID corto de la sesión y los comandos para administrarla. Cuando el servicio que aloja sesiones en segundo plano no está ya en ejecución, `--bg` puede imprimir primero `Starting background service…` encima de esta salida. Cuando pasa `--name`, el nombre aparece después del ID corto:

```text theme={null}
backgrounded · 7c5dcf5d · flaky-test-fix
  claude agents             list sessions
  claude attach 7c5dcf5d    open in this terminal
  claude logs 7c5dcf5d      show recent output
  claude stop 7c5dcf5d      stop this session
```

<h4 id="run-a-shell-command">
  Ejecutar un comando de shell
</h4>

Para ejecutar un comando de shell como un trabajo en segundo plano en lugar de una sesión de Claude, escriba `!` como el primer carácter de la entrada de distribución de la vista de agentes. El `!` se muestra como un prefijo y todo lo que escriba después de él es el comando. El siguiente ejemplo distribuye `pytest -x` desde el cuadro de entrada de la vista de agentes:

```text theme={null}
! pytest -x
```

Presione `Enter` para iniciar el trabajo. El mismo trabajo también se puede lanzar directamente desde su shell con `--exec`:

```bash theme={null}
claude --bg --exec 'pytest -x'
```

El comando se ejecuta como un trabajo respaldado por PTY y aparece como una fila en la vista de agentes, con la línea de salida más reciente como su estado. Un trabajo de shell ejecuta el comando en lugar de Claude, por lo que no se invoca ningún modelo y la salida no se envía a ninguna sesión.

Para ver la salida, conéctese a la fila, presione `Space` para echar un vistazo sin conectarse, o ejecute `claude logs <id>` desde su shell. La salida capturada permanece en la memoria y no se escribe en el disco. La fila y su salida se limpian automáticamente aproximadamente cinco minutos después de que el comando salga, así que léalo antes si necesita el resultado.

<h3 id="how-file-edits-are-isolated">
  Cómo se aíslan las ediciones de archivos
</h3>

Cada sesión en segundo plano, ya sea iniciada desde la vista de agentes, `/bg` o `claude --bg`, comienza en su directorio de trabajo. Antes de editar archivos, Claude mueve la sesión a un [git worktree](/docs/es/worktrees) aislado bajo `.claude/worktrees/`, de modo que las sesiones paralelas pueden leer el mismo checkout pero cada una escribe en la suya propia.

Claude omite el worktree cuando:

* La sesión ya está dentro de un git worktree vinculado, ya sea que Claude lo haya creado bajo `.claude/worktrees/` o que lo haya creado con `git worktree add` en otro lugar
* El directorio de trabajo no es un repositorio git y no hay ningún hook [`WorktreeCreate`](/docs/es/hooks#worktreecreate) configurado
* La escritura está fuera del directorio de trabajo

Para desactivar el aislamiento de worktree para un repositorio donde los git worktrees no son prácticos, establezca [`worktree.bgIsolation`](/docs/es/settings#worktree-settings) en `"none"`. Las sesiones en segundo plano editarán su copia de trabajo directamente sin moverse a un worktree primero. Agregue la configuración al archivo `.claude/settings.json` del proyecto:

```json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

Fuera de un repositorio git, las sesiones escriben en el directorio de trabajo directamente y no están aisladas entre sí, por lo que evite distribuir sesiones paralelas que editen los mismos archivos. Si utiliza un sistema de control de versiones diferente, configure un hook [`WorktreeCreate`](/docs/es/worktrees#non-git-version-control) y Claude aísla las ediciones de la misma manera que lo hace para git.

Cuando el hook falla en un directorio que no es un repositorio git, la sesión omite el aislamiento para ese directorio y edita el directorio de trabajo en su lugar. Dentro de un repositorio git, las escrituras permanecen bloqueadas hasta que la sesión se aísle. Antes de v2.1.203, una sesión en segundo plano en ese estado no podía editar ningún archivo: cada escritura se rechazaba hasta que se aislaba, y el hook nunca podía aislar ese directorio.

Eliminar una sesión elimina o mantiene el worktree que Claude creó para ella, dependiendo de cómo lo elimine y qué contenga el worktree:

* Eliminar en la vista de agentes con `Ctrl+X` dos veces elimina el worktree, incluidos los cambios sin confirmar, por lo que confirme los cambios que desee mantener primero.
* Eliminar desde el shell con [`claude rm`](#manage-sessions-from-the-shell) mantiene un worktree que tiene cambios sin confirmar, junto con su fila de sesión.
* Ninguna ruta elimina un worktree con commits que no se han enviado a ningún lugar: el worktree se [mantiene junto con su sesión](#organize-the-list) y la salida nombra la ruta mantenida y la razón.
* Un worktree que creó usted mismo e inició la sesión dentro se deja en su lugar de cualquier forma.

Para encontrar la ruta del worktree de una sesión, eche un vistazo a la sesión o conéctese y verifique su directorio de trabajo.

Un [subagente](/docs/es/sub-agents) que la sesión en segundo plano genera hereda el directorio de trabajo de la sesión, por lo que sus ediciones de archivos se realizan en el worktree de la sesión en lugar de su copia de trabajo. Para darle a un subagente su propio worktree separado en su lugar, establezca [`isolation: worktree`](/docs/es/sub-agents#supported-frontmatter-fields) en su frontmatter o pase `isolation: "worktree"` al generarlo.

A partir de v2.1.198, una sesión en segundo plano que aisló sus cambios de código en un worktree también confirma, envía su propia rama y abre una solicitud de extracción en borrador sin detenerse a preguntar. La etiqueta [`#N`](#pull-request-status) aparece en su fila cuando se abre la solicitud de extracción. Nunca envía a `main` o `master`, nunca fuerza-envía o fusiona, y omite la solicitud de extracción cuando le dijo que no abriera una o el repositorio no tiene remoto.

Una sesión que edita un checkout que no aisló a sí misma aún pregunta antes de confirmar o cambiar de rama. Esto se aplica cuando el aislamiento se establece en `"none"`, cuando el movimiento del worktree falló, o cuando la sesión comenzó dentro de un worktree que ya existía.

<h3 id="set-the-model">
  Establecer el modelo
</h3>

El nombre del modelo mostrado en el encabezado de la vista de agentes es el valor predeterminado de distribución. Las nuevas sesiones que inicia desde la entrada utilizan este modelo, que proviene de la configuración [`model`](/docs/es/settings#available-settings) en su configuración de usuario. Establézcalo seleccionando un modelo en el selector [`/model`](/docs/es/model-config), o edite la configuración directamente.

Para anularlo para toda la sesión de vista de agentes, pase `--model` al abrir la vista de agentes. Consulte [Modo de permiso, modelo y esfuerzo](#permission-mode-model-and-effort).

Para cambiar el valor predeterminado de distribución desde dentro de la vista de agentes, escriba `/model` seguido de un nombre de modelo en la entrada de distribución y presione `Enter`. El encabezado se actualiza para mostrar ese modelo con un marcador `(session)`, y las sesiones que distribuya después lo utilizan. Escriba `/model default` para borrar la anulación y volver al valor predeterminado de distribución. Esta anulación dura el resto de la ejecución actual de `claude agents` y no se escribe en su archivo de configuración. El siguiente ejemplo distribuye una sesión en Opus y la siguiente en Sonnet:

```text theme={null}
/model opus
refactor auth
/model sonnet
run the test suite
```

Cada sesión en segundo plano puede ejecutarse en un modelo diferente. Para anularlo para una sesión:

* Desde el shell, pase `--model` con `claude --bg`.
* Conéctese a una sesión en ejecución y ejecute `/model` para cambiar: una selección del selector, o un `/model <name>` escrito, se guarda como su valor predeterminado para nuevas sesiones a menos que presione `s` en el selector para un cambio solo de sesión. Un cambio solo de sesión persiste si la sesión se reinicia.
* Distribuya un [subagente](/docs/es/sub-agents) cuyo frontmatter establezca un campo `model`.

<h3 id="permission-mode-model-and-effort">
  Modo de permiso, modelo y esfuerzo
</h3>

Una sesión en segundo plano lee su [configuración](/docs/es/settings) desde el directorio en el que se ejecuta, igual que si hubiera iniciado `claude` allí. Esto incluye valores [`env`](/docs/es/settings#available-settings) en la configuración del proyecto, por lo que una variable `ANTHROPIC_MODEL` o de proveedor establecida allí se aplica a las sesiones en segundo plano en ese directorio.

La selección del proveedor en la nube, como `CLAUDE_CODE_USE_BEDROCK` o `CLAUDE_CODE_USE_VERTEX`, y los alias `ANTHROPIC_DEFAULT_*_MODEL` siguen el shell que distribuyó la sesión. Si exporta una anulación de cuerpo de solicitud [`CLAUDE_CODE_EXTRA_BODY`](/docs/es/env-vars) en ese shell, llega a la sesión de la misma manera. Antes de v2.1.206, los trabajadores en segundo plano ignoraban un `CLAUDE_CODE_EXTRA_BODY` exportado por shell.

Si exporta una puerta de enlace `ANTHROPIC_BASE_URL` en el shell de distribución, también llega a la sesión, junto con `ANTHROPIC_CUSTOM_HEADERS`, cuando el supervisor se ejecuta con el mismo entorno de puerta de enlace y la sesión se ejecuta en el directorio desde el que distribuye o es su propia sesión enviada al segundo plano con `←` o `/background`. Ese es el caso normal cuando el primer shell en abrir la vista de agentes o distribuir una sesión en segundo plano es el shell de puerta de enlace. Distribuir a un directorio diferente con `@repo` o `--cwd` no lleva la puerta de enlace del shell; la [configuración](/docs/es/settings) de ese proyecto suministra el punto final. Consulte [el proceso supervisor](#the-supervisor-process) para ver cómo las sesiones en segundo plano obtienen la configuración del proveedor y las credenciales.

El [modo de permiso](/docs/es/permissions) depende de cómo inició la sesión. Enviar al segundo plano una sesión existente con `/bg` o `←` mantiene el modo de permiso actual, por lo que una sesión que cambió a `acceptEdits` o `auto` permanece en ese modo después de desconectarse. Distribuir desde la entrada de la vista de agentes o ejecutar `claude --bg` desde su shell utiliza el `defaultMode` de la configuración de ese directorio, o el `permissionMode` del [frontmatter del subagente distribuido](/docs/es/sub-agents#supported-frontmatter-fields).

El modo de permiso, modelo y esfuerzo con el que inicia una sesión en segundo plano, junto con las [banderas de configuración que lleva](#from-inside-a-session), todos persisten cuando el supervisor posteriormente [detiene y reinicia](#the-supervisor-process) su proceso. Una sesión que lanzó con `claude --bg --dangerously-skip-permissions` o `claude --bg --permission-mode bypassPermissions` permanece en `bypassPermissions` después de ese reinicio en lugar de volver al `defaultMode` del directorio, y un modelo o esfuerzo que cambió a mitad de sesión con `/model` o `/effort` se mantiene.

Un esfuerzo que la sesión tomó de la configuración [`effortLevel`](/docs/es/settings#available-settings) en lugar de `--effort` o `/effort` no se fija en la distribución: cada proceso iniciado para la sesión lee la configuración nuevamente, por lo que editar `effortLevel` en `settings.json` llega a las sesiones que envía al segundo plano con `←` o `/bg` y sus reinicios posteriores. Antes de v2.1.203, enviar al segundo plano una sesión registraba su esfuerzo derivado de configuración como si hubiera pasado `--effort`, por lo que ediciones posteriores de `effortLevel` nunca lo alcanzaban.

Un nombre que estableció con [`/rename`](/docs/es/commands) o `Ctrl+R` también persiste en ese reinicio, por lo que [`claude --resume <name>`](/docs/es/sessions#name-your-sessions) aún resuelve la sesión. Antes de v2.1.202, el reinicio revertía la sesión al nombre con el que se distribuyó y el nuevo nombre dejaba de resolverse.

Para establecer valores predeterminados para cada sesión que distribuya desde la vista de agentes, pase cualquiera de `--permission-mode`, `--model`, `--effort` o `--agent` al abrirla:

```bash theme={null}
claude agents --permission-mode plan --model opus --effort high
```

`--agent` establece el [subagente](/docs/es/sub-agents) utilizado cuando un mensaje de distribución no nombra uno, ya sea con `@name` o como la primera palabra. Por defecto es la configuración [`agent`](/docs/es/settings#available-settings) si se establece una, de lo contrario el agente integrado `claude` que lo captura todo. Nombrar un subagente en la entrada de distribución anula ambos.

`claude agents` también acepta `--dangerously-skip-permissions` como abreviatura de `--permission-mode bypassPermissions`, y `--allow-dangerously-skip-permissions` para hacer que `bypassPermissions` esté disponible en el ciclo `Shift+Tab` de cada sesión distribuida sin comenzar en ese modo. Ambos coinciden con las [banderas CLI de nivel superior](/docs/es/cli-reference).

Los valores predeterminados activos aparecen en el pie de página debajo de la entrada de distribución.

Sin estas banderas, la sesión utiliza el `defaultMode` de la configuración de ese directorio o el `permissionMode` del [frontmatter del subagente distribuido](/docs/es/sub-agents#supported-frontmatter-fields), y el modelo mostrado en el encabezado de la vista de agentes.

El uso de `bypassPermissions` con `claude --bg --permission-mode` se rechaza hasta que haya aceptado el descargo de responsabilidad de bypass ejecutando `claude --dangerously-skip-permissions` una vez de forma interactiva, ya que ese modo permite que una sesión que no está viendo actúe sin aprobación. Pasar `--dangerously-skip-permissions` o `--permission-mode bypassPermissions` a `claude agents` muestra el mismo descargo de responsabilidad cuando no lo ha aceptado antes, y aceptar aplica `bypassPermissions` a las sesiones que lanza desde la vista. Pasar `--allow-dangerously-skip-permissions` muestra el mismo descargo de responsabilidad también, y aceptar hace que `bypassPermissions` esté disponible en el ciclo `Shift+Tab` de esas sesiones sin iniciarlas en él.

<h3 id="settings-plugins-and-mcp-servers">
  Configuración, plugins y servidores MCP
</h3>

La vista de agentes acepta las mismas banderas de configuración que `claude` para cargar configuración, plugins, servidores MCP y directorios adicionales. Cada bandera se aplica a la vista de agentes en sí y se pasa a cada sesión que distribuya desde ella, por lo que un plugin o servidor MCP que cargue de esta manera está disponible en esas sesiones también.

| Bandera                                                                                          | Efecto                                                                             |
| :----------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------- |
| [`--settings <file-or-json>`](/docs/es/settings)                                                      | Anule la configuración para la vista de agentes y las sesiones distribuidas        |
| [`--add-dir <path>`](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) | Otorgue acceso a archivos a un directorio adicional                                |
| [`--plugin-dir <path>`](/docs/es/plugins)                                                             | Cargue un plugin desde un directorio local                                         |
| [`--mcp-config <file-or-json>`](/docs/es/mcp)                                                         | Cargue servidores MCP desde un archivo de configuración o cadena JSON              |
| `--strict-mcp-config`                                                                            | Use solo los servidores MCP de `--mcp-config`, ignorando otra configuración de MCP |

Repita `--add-dir`, `--plugin-dir` o `--mcp-config` una vez por valor. La forma separada por espacios, como `--add-dir a b c`, no es compatible con `claude agents`.

El siguiente ejemplo abre la vista de agentes con una anulación de configuración y un directorio adicional:

```bash theme={null}
claude agents --settings ./ci-settings.json --add-dir ../shared-lib
```

<h2 id="manage-sessions-from-the-shell">
  Gestionar sesiones desde el shell
</h2>

Cada sesión en segundo plano tiene un ID corto que puede usar desde el shell. El ID se imprime cuando inicia una sesión con `claude --bg`, y el ID de cada sesión es su nombre de directorio bajo `~/.claude/jobs/`. Estos comandos son útiles para scripting o cuando no desea abrir la vista de agentes.

| Comando                      | Propósito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude agents`              | Abrir la vista de agentes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `claude agents --cwd <path>` | Abrir la vista de agentes limitada a sesiones iniciadas bajo `<path>`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `claude agents --json`       | Imprimir sesiones activas como un array JSON y salir: cada sesión activa, más sesiones en segundo plano que aún están funcionando o bloqueadas incluso cuando su proceso ha salido. Agregue `--all` para incluir también sesiones en segundo plano completadas. Cada entrada tiene `cwd`, `kind` y `startedAt`. Las entradas en segundo plano también tienen `id`, utilizable con `claude attach`/`logs`/`stop`, y `state`: uno de `working`, `blocked`, `done`, `failed` o `stopped`. `pid` y `status` están presentes solo mientras el proceso está activo, más `waitingFor` cuando el estado es `waiting`, que indica en qué está bloqueada la sesión, como `permission prompt` o `input needed`; `sessionId` y `name` aparecen cuando están configurados. Una entrada interactiva que nunca nombró lleva un nombre predeterminado construido a partir del nombre del directorio de trabajo más un sufijo de dos caracteres, como `my-app-3f`. Combinar con `--cwd <path>` para filtrar |
| `claude attach <id>`         | Conectarse a una sesión en esta terminal                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `claude logs <id>`           | Imprimir la salida reciente de la sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `claude stop <id>`           | Detener una sesión. También acepta `claude kill`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `claude respawn <id>`        | Reiniciar una sesión, en ejecución o detenida, con su conversación intacta, por ejemplo para usar un binario Claude Code actualizado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `claude respawn --all`       | Reiniciar cada sesión en ejecución, por ejemplo para mover todas las sesiones a un binario Claude Code actualizado a la vez                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `claude rm <id>`             | Eliminar una sesión de la lista. Elimina un worktree que Claude creó para la sesión si no tiene cambios sin confirmar y sin commits que no estén enviados a ningún lugar; de lo contrario la sesión se mantiene también, y el comando imprime la ruta del worktree y la razón para que pueda resolverlo y ejecutar `claude rm` nuevamente. Deja un worktree que creó usted mismo en su lugar. La transcripción de la conversación permanece en su máquina local y sigue disponible a través de `claude --resume`                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `claude daemon status`       | Imprimir el estado del [supervisor](#the-supervisor-process), versión, directorio de socket y número de workers                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `claude daemon stop --any`   | Detener el proceso supervisor y las sesiones en segundo plano que aloja. Pase `--keep-workers` para dejar las sesiones en segundo plano en ejecución de modo que el siguiente supervisor se reconecte a ellas. El siguiente `claude agents` o `claude --bg` inicia un nuevo supervisor                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

<h2 id="how-background-sessions-are-hosted">
  Cómo se alojan las sesiones en segundo plano
</h2>

Cada sesión listada en la vista de agentes se considera una sesión en segundo plano, independientemente de si está actualmente conectado a ella. Por el contrario, una sesión iniciada ejecutando `claude` directamente está vinculada a esa terminal y finaliza cuando se cierra, a menos que la [envíe al segundo plano](#from-inside-a-session).

<h3 id="the-supervisor-process">
  El proceso supervisor
</h3>

Las sesiones en segundo plano se alojan mediante un proceso supervisor por usuario, separado de su terminal y de la vista de agentes. Se inicia automáticamente la primera vez que envía una sesión al segundo plano o abre la vista de agentes, y no lo administra directamente.

Cuando una actualización ha reemplazado o eliminado el binario desde el cual se lanzó un proceso de Claude Code en ejecución, ese proceso inicia el supervisor desde otra copia instalada, como el lanzador `claude` instalado o la versión más nueva en el disco.

El supervisor mantiene un proceso de trabajo precalentado listo para que un envío desde la vista de agentes o `claude --bg` se inicie sin la demora de un lanzamiento en frío. Cuando envía, el supervisor asigna el trabajador precalentado a su sesión, aplica el directorio, la configuración y las credenciales de esa sesión a él, y luego inicia un reemplazo para el próximo envío. Si no hay un trabajador precalentado saludable disponible, el supervisor lanza un proceso nuevo en su lugar.

El supervisor y sus sesiones se autentican con las mismas credenciales almacenadas que sus sesiones interactivas y no realizan conexiones de red adicionales más allá de la API del modelo. Las variables de selección de proveedor como `CLAUDE_CODE_USE_BEDROCK` y los alias `ANTHROPIC_DEFAULT_*_MODEL` se leen del shell que envió cada sesión y se aplican a su trabajador.

El `PATH` del shell de envío se aplica al trabajador de la misma manera, por lo que los comandos de shell que ejecuta la sesión encuentran las mismas herramientas que su terminal. Antes de v2.1.203, una sesión en segundo plano mantenía el `PATH` del shell que inició primero el supervisor, por lo que las herramientas agregadas a su `PATH` desde entonces podrían faltar, más a menudo en Windows.

Una sesión en segundo plano no hereda variables de punto final de puerta de enlace como `ANTHROPIC_BASE_URL` o las variables de URL base equivalentes de Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry del shell que inició el supervisor. Sin una puerta de enlace exportada en el shell desde el que envía, la sesión utiliza sus credenciales almacenadas y cualquier valor `env` en la [configuración](/docs/es/settings) del directorio del proyecto. Para apuntar cada sesión en un proyecto a una [puerta de enlace LLM](/docs/es/llm-gateway), establezca `ANTHROPIC_BASE_URL` en el bloque `env` de `settings.json` de ese proyecto en `.claude/`.

Si exporta una puerta de enlace `ANTHROPIC_BASE_URL` en el shell desde el que envía, llega al trabajador de esa sesión. `ANTHROPIC_CUSTOM_HEADERS` y la credencial exportada junto a ellos se reenvían con él. Esto sucede cuando el supervisor se inició desde un entorno con la misma puerta de enlace. El supervisor captura su entorno desde el primer shell que abre la vista de agentes o envía una sesión en segundo plano, por lo que comenzar desde el shell de la puerta de enlace le da ese entorno. El reenvío también se aplica solo a las sesiones enviadas al directorio desde el que está enviando, o enviadas al segundo plano desde su propia sesión con `←` o `/background`: enviar a un directorio diferente con `@repo` o `--cwd` no lleva la puerta de enlace del shell, y el bloque `env` de `settings.json` de ese proyecto proporciona el punto final en su lugar. Cuando el entorno del supervisor lleva una puerta de enlace diferente o ninguna, el trabajador mantiene sus credenciales almacenadas contra el punto final predeterminado en lugar de mezclar la credencial de un entorno con el punto final de otro. Antes de v2.1.203, el `ANTHROPIC_BASE_URL` del shell de envío se descartaba mientras que el `ANTHROPIC_API_KEY` exportado junto a él se mantenía, por lo que la clave de la puerta de enlace se enviaba al punto final predeterminado y cada solicitud fallaba con un 401.

El punto final reenviado se aplica solo a ese proceso activo y nunca se escribe en el disco. Cuando el supervisor detiene una sesión inactiva y luego la reinicia, el proceso reiniciado lee su punto final desde su configuración nuevamente: con un `ANTHROPIC_AUTH_TOKEN` de puerta de enlace vuelve a sus credenciales almacenadas, y con un `ANTHROPIC_API_KEY` emitido por la puerta de enlace puede fallar al autenticarse hasta que la puerta de enlace se establezca en la configuración.

Cada sesión en segundo plano es su propio proceso de Claude Code, administrado por el supervisor en lugar de estar vinculado a su terminal. Una sesión que está funcionando activamente, esperando su entrada o tiene una terminal conectada mantiene su proceso ejecutándose. Un comando de shell en segundo plano en ejecución, subagente, flujo de trabajo dinámico o monitor cuenta como trabajo activo, por lo que un proceso de larga duración como un servidor de desarrollo mantiene la sesión activa.

Una vez que una sesión finaliza y permanece sin conectar durante aproximadamente una hora, el supervisor detiene su proceso para liberar recursos. Una sesión que ha [fijado](#organize-the-list) con `Ctrl+T` está exenta y mantiene su proceso ejecutándose mientras está inactiva. La transcripción y el estado permanecen en el disco de cualquier forma, y la próxima vez que se conecte, eche un vistazo o responda a una sesión detenida, el supervisor inicia un proceso nuevo desde donde se quedó. Cuando cada sesión ha finalizado y no hay terminal conectada, el supervisor mismo sale e inicia nuevamente la próxima vez que lo necesite.

El trabajo en segundo plano que la sesión inició en el nivel superior se entrega cuando su proceso se detiene, se reinicia o se actualiza, incluso en Windows. El siguiente proceso iniciado para esa sesión lo recoge:

* Un comando de shell en segundo plano que terminó en el ínterin se reporta como completado con su salida
* Un flujo de trabajo dinámico se reanuda desde donde se quedó
* Un [subagente en segundo plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background) se reanuda desde su propia transcripción

A partir de v2.1.198 la entrega cubre los tres. Antes de v2.1.198 cubría solo comandos de shell y flujos de trabajo, por lo que un subagente en segundo plano se detenía con el proceso y se reportaba como fallido en el próximo despertar.

El trabajo cuyo estado vive solo dentro del proceso mismo se detiene con él en lugar de ser entregado. Eso son comandos de shell que un subagente inició, que el subagente reanudado puede iniciar nuevamente, y [monitores](/docs/es/tools-reference#monitor-tool) en ejecución, cuya secuencia de eventos no se puede mover a otro proceso.

Eliminar la sesión detiene todo lo que entregó. Para detener todo el trabajo en segundo plano de la sesión con el proceso en lugar de entregarlo, establezca la variable de entorno [`CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF`](/docs/es/env-vars#variables) en `1`.

Un proceso reiniciado encuentra la conversación de una sesión que [se movió a un worktree](#how-file-edits-are-isolated) a mitad de la tarea: cuando la transcripción no está donde la sesión comenzó, Claude Code también busca bajo los worktrees registrados del repositorio. Antes de v2.1.207, reabriendo esa sesión desde la vista de agentes después de que su proceso se había detenido podría mostrar una conversación vacía con solo su mensaje original, con la transcripción aún intacta en el disco; abrir la sesión nuevamente en v2.1.207 o posterior la recupera.

Si una sesión reiniciada vuelve mostrando solo su mensaje original porque Claude Code malinterpretó su transcripción como vacía, la transcripción de conversación se renombra con un sufijo `.orphaned-` en lugar de eliminarse, por lo que permanece en su máquina.

Una fila vacía dejada por presionar `←` que nunca recibió un mensaje se elimina completamente después de aproximadamente cinco minutos para que la lista se limpie por sí sola. Las sesiones iniciadas con `claude --bg` y las sesiones que esperan un mensaje de configuración como un diálogo de confianza no se eliminan de esta manera.

Cuando el host tiene poca memoria, el supervisor detiene primero las sesiones inactivas no fijadas y detiene las fijadas inactivas solo si eso no liberó nada.

El supervisor observa el binario de Claude Code instalado en el disco y se reinicia en la nueva versión después de que el [actualizador automático](/docs/es/setup#auto-updates) regular lo reemplace. Esta es una observación de archivo local, no una verificación de red. Las sesiones en segundo plano son procesos desconectados, por lo que siguen ejecutándose durante el reinicio y el nuevo supervisor se reconecta a ellas. Una sesión fijada inactiva también se reinicia en su lugar en la nueva versión para que recoja la actualización sin que usted se reconecte.

Una vez que el nuevo supervisor toma el control, también reinicia las sesiones inactivas restantes en la nueva versión, algunas a la vez en segundo plano, después de un breve retraso que permite que las terminales conectadas durante el reinicio se reconecten primero. Una sesión que está funcionando, esperando su entrada o tiene una terminal conectada no se interrumpe; se mueve a la nueva versión la próxima vez que su proceso se reinicia. Antes de v2.1.206, el supervisor movía solo algunas sesiones inactivas por minuto a una nueva versión, por lo que las sesiones podrían seguir ejecutando la antigua durante un tiempo después de una actualización.

Estos reinicios solo mueven una sesión a una versión más nueva. Un supervisor que ejecuta una versión anterior de Claude Code que la que se inició el proceso de la sesión deja ese proceso solo; la sesión sigue ejecutando la versión más nueva hasta que un supervisor más nuevo toma el control.

Ejecutar `claude attach` mientras el supervisor está reiniciando una sesión, ya sea para una actualización, un estancamiento o una migración, espera el proceso de reemplazo en lugar de fallar. Una línea de estado como `Agent is updating to the new Claude Code…` nombra lo que está esperando y cuenta los segundos transcurridos, y el comando se conecta tan pronto como la sesión esté lista. Después de aproximadamente 60 segundos deja de esperar e informa un error. Antes de v2.1.205, `claude attach` dejaba de reintentar después de unos pocos segundos e imprimía un error mientras la sesión aún se estaba reiniciando.

<h3 id="where-state-is-stored">
  Dónde se almacena el estado
</h3>

El estado de la sesión se almacena en su directorio de configuración de Claude Code. Si establece [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars), el supervisor usa ese directorio en lugar de `~/.claude` y se ejecuta como una instancia separada con sus propias sesiones.

| Ruta                             | Contenidos                                                                                                                  |
| :------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| `~/.claude/daemon.log`           | Registro del supervisor                                                                                                     |
| `~/.claude/daemon/roster.json`   | Lista de sesiones en segundo plano en ejecución, utilizada para reconectarse después de un reinicio                         |
| `~/.claude/jobs/<id>/state.json` | Estado por sesión mostrado en la vista de agentes                                                                           |
| `~/.claude/jobs/<id>/tmp/`       | Directorio de trabajo temporal por sesión. Las escrituras aquí no solicitan permiso. Se elimina cuando se elimina la sesión |

Cada sesión en segundo plano tiene la variable de entorno `CLAUDE_JOB_DIR` establecida en su directorio `~/.claude/jobs/<id>`, por lo que los comandos de shell que ejecuta la sesión pueden escribir archivos temporales en `$CLAUDE_JOB_DIR/tmp` sin colisionar con sesiones paralelas.

Para inspeccionar este estado sin leer los archivos directamente, ejecute `claude daemon status`. Informa si el supervisor es accesible, su ID de proceso y versión, el directorio de socket y cuántas sesiones en segundo plano están activas.

El comando también advierte cuando el supervisor en ejecución está en una versión diferente a la de `claude` que invocó, lo que sucede después de una actualización en la que el supervisor aún no se ha reiniciado. La advertencia muestra ambas versiones y le indica que ejecute `claude daemon stop --any` para adoptar la nueva versión. Cuando Claude Code se instala como un servicio del sistema operativo, el comando sugerido es `claude daemon stop` sin la bandera.

Las sesiones sobreviven a ese desajuste de versión intactas: una versión anterior de Claude Code que actualiza el `state.json` de una sesión preserva campos que no reconoce y mantiene la sesión listada. La lista de sesiones en `roster.json` sigue la misma regla: una versión anterior que la reescribe preserva campos que una versión más nueva escribió, por lo que las sesiones iniciadas por la versión más nueva permanecen accesibles y continúan aceptando entrada después de que el supervisor se reinicia. Antes de v2.1.200, las versiones anteriores podrían descartar esos campos al reescribir.

En Windows, `claude daemon status` expone el error de archivo subyacente cuando el archivo de clave de tubería del daemon está bloqueado o no es legible en lugar de informar una falla de conexión genérica.

<h3 id="turn-off-agent-view">
  Desactivar la vista de agentes
</h3>

Para desactivar completamente los agentes en segundo plano y la vista de agentes, establezca la configuración `disableAgentView` [setting](/docs/es/settings) en `true` o establezca la variable de entorno `CLAUDE_CODE_DISABLE_AGENT_VIEW`. Los administradores pueden aplicar esto a través de [configuraciones administradas](/docs/es/permissions#managed-settings).

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="claude-agents-lists-subagents-instead-of-opening-agent-view">
  `claude agents` enumera subagentes en lugar de abrir la vista de agentes
</h3>

Si `claude agents` imprime un recuento seguido de sus subagentes configurados y luego sale, la vista de agentes no está disponible en su entorno. Ejecute `claude update` para instalar la versión más reciente.

Si la vista de agentes aún no se abre después de actualizar, verifique si ha sido [desactivada](#turn-off-agent-view) por una configuración o variable de entorno.

<h3 id="agent-view-opens-with-no-sessions">
  La vista de agentes se abre sin sesiones
</h3>

Antes de distribuir su primera sesión, la vista de agentes muestra las secciones de encabezado vacías con una descripción debajo de cada una, más una explicación de una línea encima de la entrada, en lugar de la lista de sesiones. Escriba un mensaje en la entrada en la parte inferior y presione `Enter` para distribuir su primera sesión.

<h3 id="backgrounding-shows-a-background-this-session-dialog">
  Backgrounding muestra un diálogo `Background this session?`
</h3>

Si presionar `←` para enviar al segundo plano la sesión actual muestra un diálogo `Background this session?`, la sesión tiene trabajo en vuelo que no puede trasladarse a la sesión en segundo plano, como un [monitor](/docs/es/tools-reference#monitor-tool) en ejecución, y Claude Code no lo detendrá silenciosamente. El diálogo nombra el trabajo que se detendrá y, por separado, cuenta las tareas que se trasladan. Ejecute `/tasks` para ver todo lo que se está ejecutando, luego confirme para enviar al segundo plano de todas formas o elija `Stay` para dejar que el trabajo termine primero. Consulte [Desde dentro de una sesión](#from-inside-a-session) para saber qué tipos de tareas se trasladan y cuáles se detienen.

<h3 id="prompt-rejected-as-too-short">
  Mensaje rechazado por ser demasiado corto
</h3>

La entrada de distribución espera una descripción de tarea, no un abridor conversacional. Un mensaje más corto de cuatro caracteres se rechaza con una sugerencia `Too short` para que una pulsación de tecla extraviada no inicie una sesión. Describa lo que desea que haga la sesión, como `investigate the flaky checkout test`.

<h3 id="sessions-show-as-failed-after-shutdown">
  Las sesiones se muestran como fallidas después del apagado
</h3>

Apagar o reiniciar su máquina detiene las sesiones en segundo plano en ejecución, por lo que se muestran como fallidas cuando abre la vista de agentes la próxima vez. Conéctese, eche un vistazo o responda a cualquiera de ellas y la sesión se reiniciará desde donde se quedó.

El sueño solo no causa esto. Las sesiones se conservan durante el sueño y el supervisor se reconecta a ellas al despertar.

<h3 id="opening-a-session-says-the-conversation-is-already-open">
  Abrir una sesión dice que la conversación ya está abierta
</h3>

Abrir una fila detenida cuya conversación también está siendo mantenida abierta por otro proceso Claude Code interactivo en ejecución, por ejemplo un worker en segundo plano para la misma conversación que aún se está cerrando, muestra `This conversation is already open in another running Claude session` en lugar de iniciar el proceso de la fila, porque dos procesos no pueden escribir en la misma transcripción. Responda en la sesión que ya tiene la conversación abierta, o salga de ella y abra la fila nuevamente. Una respuesta que escribió con el intento rechazado no se pierde; se envía la próxima vez que se inicia la sesión.

Antes de v2.1.203, este estado iniciaba un segundo proceso de todas formas. Ese proceso salía con un error `currently running as a background agent` y la fila se mostraba como fallida.

<h3 id="a-session-fails-before-starting-with-a-possibly-low-memory-note">
  Una sesión falla antes de iniciarse con una nota `possibly low memory`
</h3>

A partir de v2.1.199, cuando el proceso de una sesión en segundo plano sale antes de terminar de iniciarse y el host tiene poca memoria, el estado de la fila nombra la salida y añade `possibly low memory — free some up and retry`. Las versiones anteriores mostraban solo la razón de salida desnuda para este fallo.

La nota es una hipótesis, no una causa confirmada. Claude Code la añade solo cuando el proceso salió silenciosamente, sin escribir un error y sin ser detenido por una señal, y el host reportó poca memoria en ese momento. Cuando el proceso escribió un error antes de salir, la fila muestra ese error en su lugar.

Libere memoria en la máquina, luego conéctese, eche un vistazo o responda a la fila y el supervisor inicia un proceso nuevo para la sesión. Cuando la memoria se mantiene baja, el supervisor también [detiene las sesiones inactivas](#the-supervisor-process) para liberar recursos por su cuenta.

<h3 id="agent-view-says-the-background-service-did-not-respond">
  La vista de agentes dice que el servicio en segundo plano no respondió
</h3>

Si conectarse, echar un vistazo o `claude logs` reporta que el servicio en segundo plano no respondió, el proceso supervisor probablemente se ha estancado. Deténgalo y deje que el siguiente `claude agents` inicie uno nuevo. Para mantener sus sesiones en segundo plano ejecutándose durante el reinicio, pase `--keep-workers`:

```bash theme={null}
claude daemon stop --any --keep-workers
```

El nuevo supervisor se reconecta a las sesiones en ejecución. Sin `--keep-workers`, el comando también termina las sesiones en segundo plano. La bandera `--any` confirma que desea detener un supervisor que se inició bajo demanda en lugar de como un servicio instalado, que es el predeterminado.

Un supervisor que se inicia pero no puede aceptar conexiones sale y libera su bloqueo por sí solo, por lo que el siguiente `claude agents` inicia uno nuevo sin este detención manual. Los pasos anteriores se aplican cuando un supervisor en ejecución se estanca.

En Windows, si el supervisor no responde a la solicitud de detención, el comando imprime su ID de proceso. Termine ese proceso con `taskkill /PID <pid>` para finalizar la recuperación. Las sesiones en segundo plano aún se conservan cuando pasó `--keep-workers`.

<h3 id="dispatch-fails-with-could-not-resolve-authentication-method">
  La distribución falla con `Could not resolve authentication method`
</h3>

Si una distribución en segundo plano falla con `Could not resolve authentication method` mientras las sesiones interactivas se autentican normalmente, el worker que recibió la distribución no recogió las credenciales. El supervisor proporciona una instantánea de credencial nueva cuando asigna un [worker precalentado](#the-supervisor-process), por lo que este error significa que no había credencial almacenada disponible para el proceso supervisor en sí. Confirme que ha ejecutado `/login` o ha configurado una clave API, luego detenga el supervisor:

```bash theme={null}
claude daemon stop --any --keep-workers
```

El siguiente `claude agents` o `claude --bg` inicia un supervisor nuevo que lee sus credenciales almacenadas. Si se autentica con una variable de entorno como `ANTHROPIC_API_KEY` en lugar de `/login`, ejecute ese comando siguiente desde un shell donde la variable esté configurada.

Vea la [referencia de errores](/docs/es/errors#could-not-resolve-authentication-method) para la lista completa de causas y soluciones.

<h3 id="background-sessions-can’t-read-desktop-documents-or-downloads-on-macos">
  Las sesiones en segundo plano no pueden leer Desktop, Documents o Downloads en macOS
</h3>

En macOS, el host de sesión en segundo plano se ejecuta como su propio proceso y solicita acceso a carpetas protegidas por separado desde su terminal. Si una sesión en segundo plano reporta `Operation not permitted` al leer `~/Desktop`, `~/Documents`, `~/Downloads` u otra ubicación protegida, otorgue acceso en Configuración del Sistema bajo Privacidad y Seguridad > Archivos y Carpetas, o habilite Acceso Total al Disco para la entrada.

Con el instalador nativo, la entrada aparece como Claude Code y la concesión persiste en las actualizaciones. Con otros métodos de instalación como Homebrew o npm, la entrada muestra la ruta del binario y puede necesitar ser otorgada nuevamente después de actualizar.

<h3 id="background-sessions-can’t-reach-local-network-hosts-on-macos">
  Las sesiones en segundo plano no pueden alcanzar hosts de red local en macOS
</h3>

En macOS 15 y posterior, el sistema bloquea un proceso para que no alcance dispositivos en su red local hasta que otorgue permiso de Red Local. Antes de v2.1.198, el host de sesión en segundo plano nunca solicitó ese permiso, por lo que los comandos dirigidos a una dirección LAN fallaban con `connect: no route to host` aunque el mismo comando funcionaba en una terminal en primer plano. A partir de v2.1.198, el primer comando en una sesión en segundo plano que se conecta a una dirección de red local activa la solicitud de permiso de Red Local de macOS para Claude Code. Otórguelo una vez y esos comandos alcanzarán hosts LAN de la misma manera que lo hacen en una terminal en primer plano.

<h3 id="a-session-is-slow-to-respond-after-attaching">
  Una sesión es lenta para responder después de conectarse
</h3>

Una vez que una sesión ha terminado y se ha quedado sin conectar durante aproximadamente una hora, el supervisor detiene su proceso para liberar recursos. Conectarse inicia un proceso nuevo desde donde se quedó y cambia a la sesión inmediatamente mientras el proceso se reinicia. Las sesiones que están funcionando, esperando su entrada o [fijadas](#organize-the-list) no se detienen de esta manera, así que fije una sesión con `Ctrl+T` para mantenerla receptiva.

Mientras el proceso se inicia, se muestra la última pantalla de la transcripción de la sesión con una nota `Session is starting` debajo, y la sesión en vivo la reemplaza tan pronto como esté lista.

<h3 id="claude/worktrees/-is-filling-up">
  `.claude/worktrees/` se está llenando
</h3>

Eliminar una sesión en la vista de agentes elimina el worktree que Claude creó para ella, y un worktree que no se puede eliminar de forma segura [mantiene su fila de sesión](#organize-the-list) para que no quede huérfano. `claude rm` mantiene un worktree que tiene cambios sin confirmar, y su fila de sesión, e imprime la ruta mantenida. Enumere las entradas sobrantes con `git worktree list` en el directorio del proyecto y elimine cada una con `git worktree remove <path>`. Vea [Limpiar worktrees](/docs/es/worktrees#clean-up-worktrees).

<h2 id="limitations">
  Limitaciones
</h2>

La vista de agentes está en vista previa de investigación con las siguientes limitaciones:

* **Se aplican límites de velocidad**: las sesiones en segundo plano consumen el uso de su suscripción igual que las sesiones interactivas, por lo que ejecutar diez agentes en paralelo usa cuota aproximadamente diez veces más rápido que ejecutar uno.
* **Las sesiones son locales**: las sesiones en segundo plano se ejecutan en su máquina. Se conservan durante el modo de suspensión pero se detienen si la máquina se apaga.
* **Los worktrees creados por Claude se eliminan con la sesión en la vista de agentes**: confirme los cambios antes de eliminar una sesión que editó archivos en su propio worktree. Un worktree con confirmaciones que no se han enviado a ningún lugar se mantiene junto con la sesión. `claude rm` también mantiene un worktree que tiene cambios sin confirmar junto con su sesión, y un worktree que usted creó se deja en su lugar.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Para otras formas de ejecutar Claude en paralelo, consulte:

* [Ejecutar agentes en paralelo](/docs/es/agents): compare la vista de agentes con subagentes, equipos de agentes y worktrees
* [Equipos de agentes](/docs/es/agent-teams): coordine múltiples sesiones que se envíen mensajes entre sí
* [Claude Code en la web](/docs/es/claude-code-on-the-web): ejecute sesiones en un entorno en la nube administrado en lugar de localmente

<h2 id="version-history">
  Historial de versiones
</h2>

La vista de agentes ha evolucionado rápidamente durante la vista previa de investigación. Si está en una versión anterior de Claude Code, algunos comportamientos en esta página pueden diferir; en particular, `claude agents` rechaza banderas que aún no admite con un error `unknown option`. La tabla a continuación enumera cuándo se agregó cada bandera y comportamiento.

| Versión  | Cambio                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| v2.1.208 | Conectarse a una sesión cuyo proceso se ha detenido muestra la última pantalla de su transcripción mientras el proceso se inicia, en lugar de solo una nota `Session is starting`. Una respuesta que no se puede entregar porque el servicio de fondo es inaccesible o el envío falla se guarda y se envía como el siguiente prompt de la sesión cuando su proceso se inicia nuevamente; antes de esta versión, una respuesta perdida mientras el servicio de fondo era inaccesible se descartaba. Un proceso cuyo propio binario fue reemplazado por una actualización aún puede iniciar el supervisor, desde el lanzador `claude` instalado o la versión más nueva en disco, en lugar de fallar hasta que Claude Code se reiniciara. Un supervisor que ejecuta una versión anterior nunca reinicia una sesión inactiva iniciada por una versión más nueva en su propio binario más antiguo. Eliminar una sesión elimina su worktree incluso después de que la sesión movió el worktree a una rama diferente, y mantiene el worktree junto con la fila de sesión cuando el worktree tiene commits que no se han enviado a ningún lugar u otra sesión lo reclama, en lugar de destruir los commits u orfandar el worktree. `/install-github-app` y la lista de configuración `/mcp` y sus acciones de autenticación se rechazan en una sesión de fondo con un mensaje que nombra la alternativa; en v2.1.208 solamente, el selector `/model` fue rechazado de la misma manera y un `/model <name>` escrito cambió solo esa sesión en lugar de también guardar su modelo predeterminado. |
| v2.1.207 | El panel de vista previa se abre con la oración que la fila trunca, como la pregunta exacta para una sesión que lo espera a usted, y muestra cuánto tiempo una sesión bloqueada ha estado esperando como una única línea `waiting 3m` en lugar de prefijar la misma marca de tiempo a la oración de estado y la pregunta. Pegar el mismo texto nuevamente en la entrada de distribución expande el marcador de posición colapsado `[Pasted text #N]` en lugar de agregar uno segundo. Una sesión en segundo plano nombrada al aceptar un plan muestra ese nombre en su fila. Una sesión en segundo plano que se movió a un worktree mantiene su conversación cuando su proceso se reinicia desde la vista de agentes.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| v2.1.206 | Los resúmenes de filas llenan el ancho restante de la fila y se truncan solo en el borde derecho de la terminal en lugar de en 64 columnas. Después de que el supervisor se reinicia en una nueva versión de Claude Code, reinicia las sesiones en segundo plano inactivas restantes en esa versión en el fondo en lugar de algunas por minuto. Eliminar una sesión con `Ctrl+X` o `claude rm` también la borra de la lista de sesiones del supervisor, por lo que la fila ya no reaparece después de un reinicio del supervisor.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| v2.1.205 | Los resúmenes de filas muestran el informe de una línea propia de la sesión, truncado en 64 columnas, en lugar de una invocación de herramienta sin procesar o un recuento `done/total`; las filas agrupadas por directorio se abren con una palabra de estado coloreada. El panel de vista previa se abre con la oración de estado completa y, para una sesión que espera por usted, su pregunta exacta encima de la entrada de respuesta. Las sesiones que editan, comentan, cierran o marcan una solicitud de extracción como lista con `gh` están vinculadas a ella, no solo las que crean o verifican una solicitud de extracción, un push vincula una solicitud de extracción incluso cuando el nombre de rama local no coincide, y una solicitud de extracción cuya salida del comando de creación excedió el límite en línea también está vinculada. Un turno sin texto legible mantiene el estado anterior de la sesión en lugar de voltearlo de nuevo a `Working`. `claude attach` espera hasta aproximadamente 60 segundos una sesión que se está reiniciando, con una línea de estado que indica por qué, en lugar de fallar.                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| v2.1.203 | Una puerta de enlace `ANTHROPIC_BASE_URL` exportada en el shell de distribución llega a las sesiones distribuidas desde él en ese mismo directorio cuando el supervisor comparte ese entorno de puerta de enlace, en lugar de ser descartada mientras la clave API exportada junto a ella se mantenía. El `PATH` del shell de distribución se aplica al worker de cada sesión. Presionar `←` mientras los subagentes se están ejecutando espera a que terminen en lugar de reiniciarlos después de diez segundos. La lista vacía siempre muestra los encabezados de sección con una descripción bajo cada uno. Escribir `@` en la entrada de distribución también enumera los worktrees de git registrados del repositorio de lanzamiento que viven dentro de su árbol de directorios. Un esfuerzo heredado de la configuración `effortLevel` sigue ediciones posteriores a esa configuración en lugar de ser fijado en la distribución. Abrir una sesión detenida cuya conversación ya está abierta en otra sesión en ejecución se rechaza con un mensaje en lugar de fallar la fila. Un comando que no está disponible en la vista de agentes deja el texto escrito en la entrada. Un hook `WorktreeCreate` que falla fuera de un repositorio de git ya no bloquea la sesión de editar archivos.                                                                                                                                                                                                                                                                                      |
| v2.1.202 | Un nombre establecido con `/rename` o `Ctrl+R` en una sesión en segundo plano persiste cuando el supervisor detiene y reinicia su proceso, en lugar de revertir al nombre con el que se distribuyó la sesión.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| v2.1.200 | Una versión anterior de Claude Code que reescribe la lista de sesiones en `roster.json` preserva campos escritos por una versión más nueva, coincidiendo con la garantía existente de `state.json`, por lo que las sesiones iniciadas por la versión más nueva continúan aceptando entrada después de que el supervisor se reinicia. Cuando abre una sesión que ha dejado de responder, el supervisor reinicia su proceso y la sesión continúa la respuesta interrumpida desde donde se quedó.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| v2.1.199 | Una sesión en segundo plano cuyo proceso se cierra antes de terminar de iniciarse en un host con poca memoria muestra `possibly low memory — free some up and retry` en su estado de fila en lugar de solo la razón de salida desnuda. Enviar al segundo plano una sesión con `←` o `/background` lleva su `/color` a la nueva fila.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| v2.1.198 | La vista de agentes envía una notificación a través de `preferredNotifChannel` cuando una sesión en segundo plano necesita entrada, finaliza o falla, y dispara el hook `Notification` con el tipo `agent_needs_input` o `agent_completed`. `←` y `/exit` dentro de `claude attach <id>` regresan a la vista de agentes en lugar de salir al shell; `Ctrl+Z` regresa al shell. Una sesión en segundo plano que aisló su trabajo en un worktree confirma, envía su propia rama aislada, nunca `main` o `master`, y abre una solicitud de extracción en borrador cuando finaliza en lugar de preguntar primero. `/login` se ejecuta en la vista de agentes y abre el diálogo de inicio de sesión. El diálogo de salida `Background work is running` ofrece `Move to background and exit`. La entrega de salida también cubre subagentes en segundo plano, que se reanudan desde su transcripción en el próximo despertar en lugar de ser reportados como fallidos. `claude --bg` combinado con `-p` o `--print` se rechaza con un error.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| v2.1.196 | Un único presionar `←` envía al segundo plano una sesión en primer plano; las versiones anteriores requerían dos presiones, con una sugerencia de pie de página y una confirmación. `--dangerously-skip-permissions` pasado a `claude agents` muestra el descargo de responsabilidad de bypass en lugar de ser silenciosamente descartado. Las sesiones interactivas que nunca nombró llevan un nombre predeterminado como `my-app-3f` en listados de sesiones y `claude agents --json`. Los comandos shell en segundo plano y flujos de trabajo dinámicos sobreviven al proceso de la sesión siendo detenido, reiniciado o actualizado, incluso en Windows; establezca `CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF=1` para desactivar la entrega. Una transcripción malinterpretada como vacía al reiniciar se renombra con un sufijo `.orphaned-` en lugar de eliminarse.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| v2.1.195 | El trabajo en vuelo se traslada cuando envía al segundo plano una sesión en Windows también; establezca `CLAUDE_DISABLE_ADOPT=1` para detenerlo en su lugar. El grupo `Completed` llena el espacio vertical restante y el encabezado se compacta en terminales cortas. Una versión anterior de Claude Code ya no descarta campos `state.json` más nuevos de sesiones o oculta esas sesiones de `claude agents`. Conectarse a una sesión detenida cambia inmediatamente en lugar de mostrar una pantalla en blanco durante hasta cinco segundos. Un supervisor que no puede aceptar conexiones sale y libera su bloqueo por sí solo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| v2.1.174 | Las sesiones en segundo plano ya no heredan variables de punto final de puerta de enlace como `ANTHROPIC_BASE_URL` del shell de lanzamiento del supervisor; el supervisor proporciona una instantánea de credencial nueva a workers precalentados, corrigiendo errores espurios de `Could not resolve authentication method`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| v2.1.172 | `/model` en la entrada de distribución establece una anulación de modelo de distribución con alcance de sesión.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| v2.1.161 | Los resúmenes de filas muestran un recuento `done/total` para elementos de trabajo paralelos; el panel de vista previa nombra el elemento de trabajo paralelo que más tiempo lleva ejecutándose.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| v2.1.157 | `claude agents` acepta `--agent`; las sesiones distribuidas honran la configuración `agent`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| v2.1.145 | Dictado de voz compatible en la entrada de respuesta del panel de vista previa y la entrada de distribución.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| v2.1.143 | Se agregó la configuración `worktree.bgIsolation`; `claude agents` acepta `--allow-dangerously-skip-permissions`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| v2.1.142 | `claude agents` acepta `--permission-mode`, `--model`, `--effort`, `--dangerously-skip-permissions`, `--settings`, `--add-dir`, `--plugin-dir`, `--mcp-config` y `--strict-mcp-config`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| v2.1.141 | `claude agents` acepta `--cwd` para limitar la lista a un proyecto.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| v2.1.139 | La vista de agentes se introdujo como una vista previa de investigación.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
