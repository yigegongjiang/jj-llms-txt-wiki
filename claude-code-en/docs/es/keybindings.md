> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Personalizar atajos de teclado

> Personaliza atajos de teclado en Claude Code con un archivo de configuración de keybindings.

Claude Code admite atajos de teclado personalizables. Ejecute `/keybindings` para crear o abrir su archivo de configuración en `~/.claude/keybindings.json`.

<h2 id="configuration-file">
  Archivo de configuración
</h2>

El archivo de configuración de atajos de teclado es un objeto con un array `bindings`. Cada bloque especifica un contexto y un mapa de pulsaciones de teclas a acciones.

<Note>Los cambios en el archivo de atajos de teclado se detectan y aplican automáticamente sin reiniciar Claude Code.</Note>

| Campo      | Descripción                                                 |
| :--------- | :---------------------------------------------------------- |
| `$schema`  | URL de esquema JSON opcional para autocompletado del editor |
| `$docs`    | URL de documentación opcional                               |
| `bindings` | Array de bloques de vinculación por contexto                |

Este ejemplo vincula `Ctrl+E` para abrir un editor externo en el contexto de chat, y desvincula `Ctrl+U`:

```json theme={null}
{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/es/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+u": null
      }
    }
  ]
}
```

<h2 id="contexts">
  Contextos
</h2>

Cada bloque de vinculación especifica un **contexto** donde se aplican los atajos de teclado:

| Contexto          | Descripción                                                                   |
| :---------------- | :---------------------------------------------------------------------------- |
| `Global`          | Se aplica en todas partes de la aplicación                                    |
| `Chat`            | Área principal de entrada de chat                                             |
| `Autocomplete`    | Menú de autocompletado está abierto                                           |
| `Settings`        | Menú de configuración                                                         |
| `Confirmation`    | Diálogos de permiso y confirmación                                            |
| `Tabs`            | Componentes de navegación de pestañas                                         |
| `Help`            | Menú de ayuda es visible                                                      |
| `Transcript`      | Visor de transcripción                                                        |
| `HistorySearch`   | Modo de búsqueda de historial (Ctrl+R)                                        |
| `Task`            | Tarea de fondo está en ejecución                                              |
| `ThemePicker`     | Diálogo de selector de tema                                                   |
| `Attachments`     | Navegación de adjunto de imagen en diálogos de selección                      |
| `Footer`          | Navegación de indicador de pie de página (tareas, equipos, diff, artefactos)  |
| `MessageSelector` | Selección de mensaje de diálogo de rebobinado y resumen                       |
| `DiffDialog`      | Navegación del visor de diff                                                  |
| `DiffPanel`       | El [panel de diff](/docs/es/interactive-mode#diff-panel) está abierto              |
| `ModelPicker`     | Nivel de esfuerzo del selector de modelo                                      |
| `EffortSlider`    | Deslizador de esfuerzo abierto por `/effort`                                  |
| `Select`          | Componentes genéricos de selección/lista                                      |
| `Plugin`          | Diálogo de plugin (examinar, descubrir, administrar)                          |
| `Agents`          | [Vista de agente](/docs/es/agent-view) (`claude agents`)                           |
| `Scroll`          | Desplazamiento de conversación y selección de texto en modo pantalla completa |

Antes de v2.1.205, existían un contexto `Doctor` y una acción `doctor:fix` para la pantalla de diagnósticos `/doctor`.

<h2 id="available-actions">
  Acciones disponibles
</h2>

Las acciones siguen un formato `namespace:action`, como `chat:submit` para enviar un mensaje o `app:toggleTodos` para mostrar la lista de tareas. Cada contexto tiene acciones específicas disponibles.

<h3 id="app-actions">
  Acciones de aplicación
</h3>

Acciones disponibles en el contexto `Global`:

| Acción                 | Predeterminado | Descripción                                                                                                                                  |
| :--------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `app:interrupt`        | Ctrl+C         | Cancelar operación actual                                                                                                                    |
| `app:exit`             | Ctrl+D         | Salir de Claude Code. Presione dos veces dentro de 800ms para confirmar                                                                      |
| `app:redraw`           | (sin asignar)  | Forzar redibujado de terminal                                                                                                                |
| `app:toggleTodos`      | Ctrl+T         | Alternar visibilidad de la lista de verificación de tareas de Claude. Esta no es la vista de tarea en segundo plano [`/tasks`](/docs/es/commands) |
| `app:toggleTranscript` | Ctrl+O         | Alternar transcripción detallada                                                                                                             |

<h3 id="history-actions">
  Acciones de historial
</h3>

Acciones para navegar por el historial de comandos:

| Acción             | Predeterminado | Descripción                     |
| :----------------- | :------------- | :------------------------------ |
| `history:search`   | Ctrl+R         | Abrir búsqueda de historial     |
| `history:previous` | Arriba         | Elemento de historial anterior  |
| `history:next`     | Abajo          | Siguiente elemento de historial |

<h3 id="chat-actions">
  Acciones de chat
</h3>

Acciones disponibles en el contexto `Chat`:

| Acción                | Predeterminado                  | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :-------------------- | :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chat:cancel`         | Escape                          | Cancelar entrada actual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `chat:clearInput`     | Ctrl+L                          | Forzar un redibujado de pantalla completa, preservando la entrada y la conversación                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `chat:clearScreen`    | Cmd+K                           | Igual que `chat:clearInput`. Consulte [Limpiar la conversación](/docs/es/fullscreen#clear-the-conversation) para ver cómo se comporta Cmd+K en iTerm2 y Terminal.app                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `chat:killAgents`     | Ctrl+X Ctrl+K                   | Detener todos los [subagentes en segundo plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background) en esta sesión y desactivar [respuestas automáticas de artefactos](/docs/es/artifacts#let-claude-reply-to-comments-on-its-own) para el resto de ella                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `chat:cycleMode`      | Shift+Tab\*                     | Ciclar modos de permisos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `chat:modelPicker`    | Meta+P                          | Abrir selector de modelo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `chat:fastMode`       | Meta+O                          | Alternar modo rápido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `chat:thinkingToggle` | Meta+T                          | Alternar pensamiento extendido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `chat:submit`         | Enter                           | Enviar mensaje                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `chat:queueSubmit`    | Ctrl+X Enter                    | Enviar el mensaje, marcado para esperar su turno: mientras Claude está trabajando, Claude Code [lo pone en cola](/docs/es/interactive-mode#queue-messages-while-claude-works) y nunca interrumpe el turno. A diferencia de `chat:submit`, envía el borrador incluso mientras se resalta una sugerencia de autocompletado. Requiere v2.1.247 o posterior                                                                                                                                                                                                                                                                                                                            |
| `chat:sendNow`        | Ctrl+Enter, Ctrl+X Ctrl+S       | Enviar sus [mensajes en cola](/docs/es/interactive-mode#queue-messages-while-claude-works), y su borrador con ellos, de inmediato. [Cuando Claude Code envía lo que puso en cola](/docs/es/interactive-mode#when-claude-code-sends-what-you-queued) cubre lo que sucede con el turno en el que Claude está trabajando. Cuando nada se está ejecutando, la tecla envía el borrador, y en [modo shell](/docs/es/interactive-mode#shell-mode-with-prefix) solo pone en cola el comando. Las terminales que no informan de teclas extendidas entregan `Ctrl+Enter` como `Enter` simple, por lo que `Ctrl+X Ctrl+S` es el atajo que funciona en cualquier terminal. Requiere v2.1.275 o posterior |
| `chat:newline`        | Ctrl+J                          | Insertar una nueva línea sin enviar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `chat:undo`           | Ctrl+\_, Ctrl+Shift+-           | Deshacer última acción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E           | Abrir en editor externo. El [envío de entrada de vista de agente](/docs/es/agent-view#keyboard-shortcuts) también sigue los atajos de teclado de una sola pulsación de esta acción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `chat:stash`          | Ctrl+S                          | Guardar indicación actual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `chat:imagePaste`     | Ctrl+V (Alt+V en Windows y WSL) | Pegar imagen desde el portapapeles. En WSL, ambos atajos están vinculados de forma predeterminada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

\*En Windows sin modo VT (Node \<24.2.0/\<22.17.0, Bun \<1.2.23), el valor predeterminado es Meta+M.

<h3 id="autocomplete-actions">
  Acciones de autocompletado
</h3>

Acciones disponibles en el contexto `Autocomplete`:

| Acción                  | Predeterminado | Descripción          |
| :---------------------- | :------------- | :------------------- |
| `autocomplete:accept`   | Tab            | Aceptar sugerencia   |
| `autocomplete:dismiss`  | Escape         | Descartar menú       |
| `autocomplete:previous` | Arriba         | Sugerencia anterior  |
| `autocomplete:next`     | Abajo          | Siguiente sugerencia |

<h3 id="confirmation-actions">
  Acciones de confirmación
</h3>

Acciones disponibles en el contexto `Confirmation`:

| Acción                  | Predeterminado | Descripción                                                                                                                                                                                                                                                                                          |
| :---------------------- | :------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `confirm:yes`           | Enter          | Confirmar acción                                                                                                                                                                                                                                                                                     |
| `confirm:no`            | Escape         | Rechazar acción                                                                                                                                                                                                                                                                                      |
| `confirm:previous`      | Arriba         | Opción anterior                                                                                                                                                                                                                                                                                      |
| `confirm:next`          | Abajo          | Siguiente opción                                                                                                                                                                                                                                                                                     |
| `confirm:nextField`     | Tab            | Siguiente campo                                                                                                                                                                                                                                                                                      |
| `confirm:previousField` | (sin asignar)  | Campo anterior                                                                                                                                                                                                                                                                                       |
| `confirm:toggle`        | Space          | Alternar selección                                                                                                                                                                                                                                                                                   |
| `confirm:cycleMode`     | Shift+Tab\*    | Ciclar modos de permisos. En un aviso de permiso de archivo, cierra un [campo de comentario](/docs/es/permissions#add-a-comment-when-you-answer-a-permission-prompt) abierto; sin campo abierto, selecciona la opción que permite la acción para el resto de la sesión, cuando el aviso ofrece esa opción |

\*En Windows sin modo VT (Node \<24.2.0/\<22.17.0, Bun \<1.2.23), el valor predeterminado es Meta+M.

Antes de v2.1.257, una acción `confirm:toggleExplanation`, vinculada a `Ctrl+E` de forma predeterminada, mostraba una explicación generada por el modelo del comando en avisos de permisos de Bash y PowerShell.

Los diálogos utilizan `confirm:yes` y `confirm:no` para aceptar y cancelar incluso cuando no hacen una pregunta de sí o no. Si vincula una letra simple como `y` o `n` en este contexto, la letra también actúa sobre diálogos que nunca la muestran como una tecla. Un diálogo que muestra `y` y `n` como sus teclas las lee a sí mismo y no necesita vinculación.

Este ejemplo vincula `y` a `confirm:yes` y `n` a `confirm:no`:

```json theme={null}
{
  "bindings": [
    {
      "context": "Confirmation",
      "bindings": {
        "y": "confirm:yes",
        "n": "confirm:no"
      }
    }
  ]
}
```

Con estos atajos de teclado, `y` y `n` aún escriben como letras mientras un [campo de texto](#text-fields) tiene el foco.

Antes de v2.1.280, `y` también estaba vinculado a `confirm:yes` y `n` a `confirm:no` de forma predeterminada. Si creó su `keybindings.json` con `/keybindings` antes de v2.1.280, el archivo enumera ambas vinculaciones y permanecen en vigor hasta que elimine esas dos líneas.

<h3 id="permission-actions">
  Acciones de permisos
</h3>

Acciones disponibles en el contexto `Confirmation` para diálogos de permisos:

| Acción                   | Predeterminado | Descripción                                                                                                                                   |
| :----------------------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission:toggleDebug` | (sin asignar)  | Alternar información de depuración de permisos. El valor predeterminado anterior de Ctrl+D se eliminó en v2.1.146 porque sombreaba `app:exit` |

<h3 id="transcript-actions">
  Acciones de transcripción
</h3>

Acciones disponibles en el contexto `Transcript`:

| Acción                     | Predeterminado    | Descripción                        |
| :------------------------- | :---------------- | :--------------------------------- |
| `transcript:toggleShowAll` | Ctrl+E            | Alternar mostrar todo el contenido |
| `transcript:exit`          | q, Ctrl+C, Escape | Salir de la vista de transcripción |

`transcript:toggleShowAll` se aplica solo en el renderizador clásico; en [renderizado de pantalla completa](/docs/es/fullscreen), el visor de transcripción no ofrece un alternar mostrar todo.

<h3 id="history-search-actions">
  Acciones de búsqueda de historial
</h3>

Acciones disponibles en el contexto `HistorySearch`:

| Acción                     | Predeterminado | Descripción                                       |
| :------------------------- | :------------- | :------------------------------------------------ |
| `historySearch:next`       | Ctrl+R         | Siguiente coincidencia                            |
| `historySearch:accept`     | Escape, Tab    | Aceptar selección                                 |
| `historySearch:cancel`     | Ctrl+C         | Cancelar búsqueda                                 |
| `historySearch:execute`    | Enter          | Ejecutar comando seleccionado                     |
| `historySearch:cycleScope` | Ctrl+S         | Ciclar alcance: sesión, proyecto, en todas partes |

Los valores predeterminados de `historySearch:next`, `historySearch:accept`, `historySearch:cancel` y `historySearch:execute` se aplican a la búsqueda de historial en línea en el renderizador clásico, que siempre busca indicaciones de todos los proyectos. `historySearch:cycleScope` solo tiene efecto en [renderizado de pantalla completa](/docs/es/fullscreen), donde `Ctrl+R` abre un diálogo de búsqueda en su lugar y `Ctrl+S` cicla su alcance. Las otras teclas del diálogo son fijas y no se pueden reasignar: `Enter` o `Tab` coloca la coincidencia resaltada en la entrada de indicación y `Esc` cancela.

<h3 id="task-actions">
  Acciones de tarea
</h3>

Acciones disponibles en el contexto `Task`:

| Acción            | Predeterminado        | Descripción                                                                                        |
| :---------------- | :-------------------- | :------------------------------------------------------------------------------------------------- |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | Poner tarea actual en segundo plano. El acorde Ctrl+X Ctrl+B evita el conflicto de prefijo de tmux |

<h3 id="theme-actions">
  Acciones de tema
</h3>

Acciones disponibles en el contexto `ThemePicker`:

| Acción                           | Predeterminado | Descripción                    |
| :------------------------------- | :------------- | :----------------------------- |
| `theme:toggleSyntaxHighlighting` | Ctrl+T         | Alternar resaltado de sintaxis |

<h3 id="help-actions">
  Acciones de ayuda
</h3>

Acciones disponibles en el contexto `Help`:

| Acción         | Predeterminado | Descripción          |
| :------------- | :------------- | :------------------- |
| `help:dismiss` | Escape         | Cerrar menú de ayuda |

<h3 id="tabs-actions">
  Acciones de pestañas
</h3>

Acciones disponibles en el contexto `Tabs`:

| Acción          | Predeterminado  | Descripción       |
| :-------------- | :-------------- | :---------------- |
| `tabs:next`     | Tab, Right      | Siguiente pestaña |
| `tabs:previous` | Shift+Tab, Left | Pestaña anterior  |

<h3 id="attachments-actions">
  Acciones de adjuntos
</h3>

Acciones disponibles en el contexto `Attachments`:

| Acción                 | Predeterminado    | Descripción                        |
| :--------------------- | :---------------- | :--------------------------------- |
| `attachments:next`     | Right             | Siguiente adjunto                  |
| `attachments:previous` | Left              | Adjunto anterior                   |
| `attachments:remove`   | Backspace, Delete | Eliminar adjunto seleccionado      |
| `attachments:exit`     | Down, Escape      | Salir de la navegación de adjuntos |

<h3 id="footer-actions">
  Acciones de pie de página
</h3>

Acciones disponibles en el contexto `Footer`:

| Acción                  | Predeterminado    | Descripción                                                                                                                                                                                                                     |
| :---------------------- | :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `footer:next`           | Right             | Siguiente elemento de pie de página                                                                                                                                                                                             |
| `footer:previous`       | Left              | Elemento de pie de página anterior                                                                                                                                                                                              |
| `footer:up`             | Up                | Navegar hacia arriba en el pie de página (deselecciona en la parte superior)                                                                                                                                                    |
| `footer:down`           | Down              | Navegar hacia abajo en el pie de página                                                                                                                                                                                         |
| `footer:openSelected`   | Enter             | Abrir elemento de pie de página seleccionado                                                                                                                                                                                    |
| `footer:clearSelection` | Escape            | Borrar selección de pie de página                                                                                                                                                                                               |
| `footer:dismiss`        | Backspace, Delete | Descartar el enlace de [artefacto](/docs/es/artifacts) seleccionado del pie de página; el artefacto publicado en sí no se ve afectado. En otras filas de pie de página, estas teclas no tienen efecto. Requiere v2.1.217 o posterior |

Mientras se selecciona un elemento de pie de página, como una fila en el panel de agente debajo de la indicación, `Enter` lo abre incluso cuando reasigna `Enter` en el contexto `Chat` a `chat:queueSubmit` o `chat:newline`.

Los atajos de teclado de `Chat` en teclas que el contexto `Footer` no vincula, como `Shift+Tab` para `chat:cycleMode`, siguen funcionando mientras se selecciona un elemento.

<h3 id="message-selector-actions">
  Acciones del selector de mensajes
</h3>

Acciones disponibles en el contexto `MessageSelector`:

| Acción                   | Predeterminado                            | Descripción                    |
| :----------------------- | :---------------------------------------- | :----------------------------- |
| `messageSelector:up`     | Up, K, Ctrl+P                             | Mover hacia arriba en la lista |
| `messageSelector:down`   | Down, J, Ctrl+N                           | Mover hacia abajo en la lista  |
| `messageSelector:top`    | Ctrl+Up, Shift+Up, Meta+Up, Shift+K       | Saltar al principio            |
| `messageSelector:bottom` | Ctrl+Down, Shift+Down, Meta+Down, Shift+J | Saltar al final                |
| `messageSelector:select` | Enter                                     | Seleccionar mensaje            |

<h3 id="diff-actions">
  Acciones de diferencias
</h3>

Acciones disponibles en el contexto `DiffDialog`:

| Acción                | Predeterminado | Descripción                                                                                                                                                                                     |
| :-------------------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `diff:dismiss`        | Escape         | Cerrar visor de diferencias; desde la vista de detalle, vuelve a la lista de archivos en su lugar                                                                                               |
| `diff:previousSource` | Left           | Fuente de diferencia anterior                                                                                                                                                                   |
| `diff:nextSource`     | Right          | Siguiente fuente de diferencia                                                                                                                                                                  |
| `diff:previousFile`   | Up, K          | Archivo anterior en la lista de archivos; desplazarse hacia arriba una línea en la vista de detalle                                                                                             |
| `diff:nextFile`       | Down, J        | Siguiente archivo en la lista de archivos; desplazarse hacia abajo una línea en la vista de detalle                                                                                             |
| `diff:viewDetails`    | Enter          | Ver detalles de diferencias                                                                                                                                                                     |
| `diff:back`           | (sin asignar)  | Retroceder en el visor de diferencias. Escape realiza la acción de retroceso a través de `diff:dismiss`. El valor predeterminado anterior de Left en la vista de detalle se eliminó en v2.1.203 |

La vista de detalle de diferencias también vincula teclas de estilo paginador a las [acciones de desplazamiento](#scroll-actions) estándar. Estas vinculaciones son parte del contexto `DiffDialog` y se aplican solo en la vista de detalle; los valores predeterminados del contexto `Scroll` enumerados en [Acciones de desplazamiento](#scroll-actions) no cambian.

| Acción                | Predeterminado | Descripción                                           |
| :-------------------- | :------------- | :---------------------------------------------------- |
| `scroll:pageUp`       | PageUp         | Desplazarse hacia arriba media ventana gráfica        |
| `scroll:pageDown`     | PageDown       | Desplazarse hacia abajo media ventana gráfica         |
| `scroll:fullPageUp`   | Shift+Space, B | Desplazarse hacia arriba una ventana gráfica completa |
| `scroll:fullPageDown` | Space          | Desplazarse hacia abajo una ventana gráfica completa  |
| `scroll:top`          | G, Home        | Saltar al principio                                   |
| `scroll:bottom`       | Shift+G, End   | Saltar al final                                       |

<h3 id="diff-panel-actions">
  Acciones del panel de diferencias
</h3>

Acciones para el [panel de diferencias](/docs/es/interactive-mode#diff-panel) que `/diff` abre en renderizado de pantalla completa. `app:cycleDiffBase` está en el contexto `DiffPanel`, que está activo mientras el panel está abierto; los otros están en `Global`. El panel requiere Claude Code v2.1.260 o posterior.

| Acción                      | Predeterminado       | Descripción                                                                     |
| :-------------------------- | :------------------- | :------------------------------------------------------------------------------ |
| `app:toggleReplTab`         | (sin asignar)        | Abrir o cerrar el panel de diferencias, igual que ejecutar `/diff`              |
| `app:cycleDiffBase`         | Ctrl+X B             | Ciclar la base de comparación del panel: esta sesión, sin confirmar, luego rama |
| `app:diffFileListUp`        | Ctrl+Up, Meta+Up     | Desplazarse hacia arriba en la lista de archivos del panel cuando se desborda   |
| `app:diffFileListDown`      | Ctrl+Down, Meta+Down | Desplazarse hacia abajo en la lista de archivos del panel cuando se desborda    |
| `app:toggleDiffNoiseFilter` | (sin asignar)        | Mostrar u ocultar archivos de prueba y generados en el panel                    |
| `app:toggleDiffPreSession`  | (sin asignar)        | Expandir o contraer los cambios de antes de esta sesión                         |

<h3 id="model-picker-actions">
  Acciones del selector de modelo
</h3>

Acciones disponibles en el contexto `ModelPicker`:

| Acción                        | Predeterminado | Descripción                                 |
| :---------------------------- | :------------- | :------------------------------------------ |
| `modelPicker:decreaseEffort`  | Left           | Disminuir nivel de esfuerzo                 |
| `modelPicker:increaseEffort`  | Right          | Aumentar nivel de esfuerzo                  |
| `modelPicker:thisSessionOnly` | s              | Aplicar modelo resaltado solo a esta sesión |

<h3 id="effort-slider-actions">
  Acciones del control deslizante de esfuerzo
</h3>

Acciones disponibles en el contexto `EffortSlider`, el control deslizante que se abre cuando ejecuta `/effort` sin argumentos. Las teclas Left, Right, Enter y Escape del control deslizante no se pueden reasignar.

| Acción                         | Predeterminado | Descripción                                                                                                                     |
| :----------------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `effortSlider:thisSessionOnly` | s              | Aplicar el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) enfocado solo a esta sesión. Requiere v2.1.257 o posterior |

<h3 id="select-actions">
  Acciones de selección
</h3>

Acciones disponibles en el contexto `Select`:

| Acción            | Predeterminado  | Descripción                               |
| :---------------- | :-------------- | :---------------------------------------- |
| `select:next`     | Down, J, Ctrl+N | Siguiente opción                          |
| `select:previous` | Up, K, Ctrl+P   | Opción anterior                           |
| `select:pageUp`   | PageUp          | Mover hacia arriba una página de opciones |
| `select:pageDown` | PageDown        | Mover hacia abajo una página de opciones  |
| `select:first`    | Home            | Primera opción                            |
| `select:last`     | End             | Última opción                             |
| `select:accept`   | Enter           | Aceptar selección                         |
| `select:cancel`   | Escape          | Cancelar selección                        |

Claude Code aplica sus vinculaciones de `select:pageUp`, `select:pageDown`, `select:first` y `select:last` en el menú `/skills`. En la mayoría de otras listas, como el selector `/model`, se aplican sus vinculaciones de `select:first` y `select:last`. PageUp y PageDown avanzan por las opciones en esas listas independientemente de sus vinculaciones.

Antes de v2.1.280, esas otras listas ignoraban Home, End y sus vinculaciones de `select:first` y `select:last`.

<h3 id="plugin-actions">
  Acciones de plugins
</h3>

Acciones disponibles en el contexto `Plugin`:

| Acción            | Predeterminado | Descripción                                                                                                       |
| :---------------- | :------------- | :---------------------------------------------------------------------------------------------------------------- |
| `plugin:toggle`   | Space          | Alternar selección de plugin                                                                                      |
| `plugin:install`  | I              | Instalar plugins seleccionados                                                                                    |
| `plugin:favorite` | F              | Marcar el plugin seleccionado como favorito para que se ordene cerca de la parte superior de la pestaña Instalado |

<h3 id="settings-actions">
  Acciones de configuración
</h3>

Acciones disponibles en el contexto `Settings`. Las acciones `select:accept` y `confirm:no` se reutilizan de los contextos [Select](#select-actions) y [Confirmation](#confirmation-actions) con comportamiento específico de Configuración: los cambios se aplican a cada configuración tan pronto como la cambia, por lo que Escape cierra el panel con sus cambios guardados en lugar de rechazarlos.

| Acción            | Predeterminado | Descripción                                              |
| :---------------- | :------------- | :------------------------------------------------------- |
| `settings:search` | /              | Entrar en modo de búsqueda                               |
| `settings:retry`  | R              | Reintentar cargar datos de uso en caso de error          |
| `select:accept`   | Enter, Space   | Cambiar la configuración seleccionada o abrir su submenú |
| `confirm:no`      | Escape         | Cerrar el panel. Los cambios ya están guardados          |

<h3 id="agents-actions">
  Acciones de agentes
</h3>

Acciones disponibles en el contexto `Agents`, que se aplica en [vista de agente](/docs/es/agent-view), abierta con `claude agents`. Requiere v2.1.257 o posterior.

| Acción              | Predeterminado | Descripción                                                                                |
| :------------------ | :------------- | :----------------------------------------------------------------------------------------- |
| `agents:switchView` | Ctrl+S         | Cambiar [agrupación de sesión](/docs/es/agent-view#organize-the-list) entre estado y directorio |
| `agents:togglePin`  | Ctrl+T         | [Fijar o desfijar](/docs/es/agent-view#organize-the-list) la sesión seleccionada                |

Mientras la vista de agente está abierta, Claude Code utiliza la vinculación de `Agents` para cualquier tecla que el contexto `Agents` vincula, e ignora una vinculación de `Chat` o `Global` en la misma tecla. Por ejemplo, presionar Ctrl+S en la vista de agente cambia la agrupación de sesión en lugar de activar el `chat:stash` predeterminado.

El atajo de editor externo de entrada de envío no es una acción de `Agents`. La vista de agente sigue la vinculación de `chat:externalEditor` del contexto `Chat`, Ctrl+G de forma predeterminada.

Los atajos de teclado se activan en pulsaciones simples en la vista de agente, por lo que el acorde Ctrl+X Ctrl+E vinculado a `chat:externalEditor` no abre el editor allí.

<h3 id="voice-actions">
  Acciones de voz
</h3>

Acciones disponibles en el contexto `Chat` cuando [dictado de voz](/docs/es/voice-dictation) está habilitado:

| Acción             | Predeterminado | Descripción                                                               |
| :----------------- | :------------- | :------------------------------------------------------------------------ |
| `voice:pushToTalk` | Space          | Dictar una indicación. Mantenga presionado o toque según el modo `/voice` |

<h3 id="scroll-actions">
  Acciones de desplazamiento
</h3>

Acciones disponibles en el contexto `Scroll` cuando [renderizado de pantalla completa](/docs/es/fullscreen) está habilitado:

| Acción                      | Predeterminado       | Descripción                                                                                                                                         |
| :-------------------------- | :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scroll:lineUp`             | `wheelup`            | Desplazarse hacia arriba una línea. El desplazamiento de la rueda del ratón activa esta acción                                                      |
| `scroll:lineDown`           | `wheeldown`          | Desplazarse hacia abajo una línea. El desplazamiento de la rueda del ratón activa esta acción                                                       |
| `scroll:pageUp`             | PageUp               | Desplazarse hacia arriba media altura de ventana gráfica                                                                                            |
| `scroll:pageDown`           | PageDown             | Desplazarse hacia abajo media altura de ventana gráfica                                                                                             |
| `scroll:top`                | Ctrl+Home            | Saltar al inicio de la conversación                                                                                                                 |
| `scroll:bottom`             | Ctrl+End             | Saltar al mensaje más reciente y reactivar el seguimiento automático                                                                                |
| `scroll:halfPageUp`         | (sin asignar)        | Desplazarse hacia arriba media altura de ventana gráfica. Mismo comportamiento que `scroll:pageUp`, proporcionado para reasignaciones de estilo vi  |
| `scroll:halfPageDown`       | (sin asignar)        | Desplazarse hacia abajo media altura de ventana gráfica. Mismo comportamiento que `scroll:pageDown`, proporcionado para reasignaciones de estilo vi |
| `scroll:fullPageUp`         | (sin asignar)        | Desplazarse hacia arriba la altura completa de la ventana gráfica                                                                                   |
| `scroll:fullPageDown`       | (sin asignar)        | Desplazarse hacia abajo la altura completa de la ventana gráfica                                                                                    |
| `selection:copy`            | Ctrl+Shift+C / Cmd+C | Copiar el texto seleccionado al portapapeles                                                                                                        |
| `selection:clear`           | (sin asignar)        | Borrar la selección de texto activa. Requiere v2.1.234 o posterior                                                                                  |
| `selection:extendLeft`      | Shift+Left           | Extender la selección activa una columna hacia la izquierda                                                                                         |
| `selection:extendRight`     | Shift+Right          | Extender la selección activa una columna hacia la derecha                                                                                           |
| `selection:extendUp`        | Shift+Up             | Extender la selección activa una fila hacia arriba. Desplaza la ventana gráfica cuando la selección alcanza el borde superior                       |
| `selection:extendDown`      | Shift+Down           | Extender la selección activa una fila hacia abajo. Desplaza la ventana gráfica cuando la selección alcanza el borde inferior                        |
| `selection:extendLineStart` | Shift+Home           | Extender la selección activa al inicio de la línea                                                                                                  |
| `selection:extendLineEnd`   | Shift+End            | Extender la selección activa al final de la línea                                                                                                   |

<h2 id="keystroke-syntax">
  Sintaxis de pulsación de tecla
</h2>

<h3 id="modifiers">
  Modificadores
</h3>

Use teclas modificadoras con el separador `+`:

* `ctrl` o `control` - Tecla Control
* `shift` - Tecla Shift
* `alt`, `opt`, `option`, o `meta` - Tecla Alt en Windows y Linux, tecla Opción en macOS
* `cmd`, `command`, `super`, o `win` - Tecla Comando en macOS, tecla Windows en Windows, tecla Super en Linux

El grupo `cmd` solo se detecta en terminales que reportan el modificador Super, como aquellos que soportan el protocolo de teclado Kitty o el modo `modifyOtherKeys` de xterm. La mayoría de terminales no lo envían, así que use `ctrl` o `meta` para atajos de teclado que desee que funcionen en todas partes.

Por ejemplo:

```text theme={null}
ctrl+k          Ctrl + K
shift+tab       Shift + Tab
meta+p          Opción + P en macOS, Alt + P en otros lugares
ctrl+shift+c    Múltiples modificadores
```

<h3 id="uppercase-letters">
  Letras mayúsculas
</h3>

Claude Code analiza los nombres de teclas sin distinción de mayúsculas y minúsculas, por lo que `K` es el mismo atajo de teclado que `k` y `ctrl+K` es lo mismo que `ctrl+k`. Para vincular Shift y una letra, escriba `shift+k`.

<h3 id="non-us-keyboard-layouts">
  Diseños de teclado no estadounidenses
</h3>

Escriba los nombres de teclas de los atajos de teclado Ctrl como caracteres latinos incluso cuando su diseño de teclado activo escribe otros caracteres.

La forma en que Claude Code coincide con la tecla que presiona con un atajo de teclado depende del tipo de diseño:

* Bajo un diseño no latino como el cirílico, Claude Code coincide con los atajos de teclado Ctrl por la posición de la tecla en el diseño estadounidense cuando la terminal utiliza el protocolo de teclado Kitty e informa esa posición. En tal terminal, con un diseño ruso activo, presionar Ctrl y la tecla W física activa `ctrl+w`. En una terminal que no informa la posición, Claude Code coincide con lo que la terminal envía para la pulsación de tecla: un código de control ASCII activa el atajo latino, y una pulsación de tecla que llega como el carácter cirílico no coincide con ningún atajo de teclado
* Bajo diseños que reorganizan letras latinas, como AZERTY, Claude Code coincide con la letra que escribe la tecla, por lo que presionar Ctrl y la tecla etiquetada A activa `ctrl+a`

Antes de v2.1.247, presionar un atajo de teclado Ctrl bajo un diseño no latino no activaba su atajo de teclado en terminales que utilizan el protocolo de teclado Kitty, como Ghostty, Kitty, WezTerm e iTerm2.

<h3 id="chords">
  Acordes
</h3>

Los acordes son secuencias de pulsaciones de teclas separadas por espacios:

```text theme={null}
ctrl+k ctrl+s   Presione Ctrl+K, suelte, luego Ctrl+S
```

Presione cada pulsación de tecla dentro de 3 segundos de la anterior. Si espera más tiempo, Claude Code cancela el acorde y muestra un breve aviso indicándolo.

<h3 id="special-keys">
  Teclas especiales
</h3>

* `escape` o `esc` - Tecla Escape
* `enter` o `return` - Tecla Enter
* `tab` - Tecla Tab
* `space` - Barra espaciadora
* `up`, `down`, `left`, `right` - Teclas de flecha
* `pageup`, `pagedown` - Teclas Page Up y Page Down
* `home`, `end` - Teclas Home y End
* `backspace`, `delete` - Teclas de eliminación
* `wheelup`, `wheeldown` - Eventos de desplazamiento de rueda del ratón

<h2 id="unbind-default-shortcuts">
  Desvinculación de atajos predeterminados
</h2>

Establezca una acción en `null` para desvinculación de un atajo predeterminado:

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+s": null
      }
    }
  ]
}
```

Esto también funciona para vinculaciones de acordes. Desvinculación de cada acorde que comparte un prefijo libera ese prefijo para su uso como una vinculación de tecla única. Un acorde en cualquier contexto activo mantiene su prefijo reservado, por lo que debe desvinculación de cada acorde en el contexto que lo define.

Claude Code vincula estos acordes predeterminados en el prefijo `ctrl+x`: `ctrl+x ctrl+k`, `ctrl+x ctrl+e`, `ctrl+x enter`, `ctrl+x ctrl+a`, `ctrl+x ctrl+s`, y `ctrl+x tab` en `Chat`, `ctrl+x ctrl+b` en `Task`, y `ctrl+x b` en `DiffPanel`. El acorde `ctrl+x enter` requiere v2.1.247 o posterior, `ctrl+x b`, `ctrl+x ctrl+a`, y `ctrl+x tab` requieren v2.1.260 o posterior, y `ctrl+x ctrl+s` requiere v2.1.275 o posterior.

Para reclamar `ctrl+x` en sí mismo como una vinculación de tecla única, desvinculación de todos ellos:

```json theme={null}
{
  "bindings": [
    {
      "context": "Task",
      "bindings": {
        "ctrl+x ctrl+b": null
      }
    },
    {
      "context": "DiffPanel",
      "bindings": {
        "ctrl+x b": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x enter": null,
        "ctrl+x ctrl+a": null,
        "ctrl+x ctrl+s": null,
        "ctrl+x tab": null,
        "ctrl+x": "chat:newline"
      }
    }
  ]
}
```

Si desvincula algunos pero no todos los acordes en un prefijo, presionar el prefijo aún entra en modo de espera de acorde para las vinculaciones restantes.

<h2 id="reserved-shortcuts">
  Atajos reservados
</h2>

Estos atajos no se pueden reasignar:

| Atajo     | Razón                                                                                                                                                                                                                                            |
| :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ctrl+C    | Interrupción/cancelación codificada                                                                                                                                                                                                              |
| Ctrl+D    | Salida codificada                                                                                                                                                                                                                                |
| Ctrl+M    | Claude Code siempre lo recibe como Enter                                                                                                                                                                                                         |
| Ctrl+\[   | Claude Code siempre lo recibe como Escape. En terminales que utilizan el protocolo de teclado Kitty, esto requiere v2.1.242 o posterior                                                                                                          |
| Ctrl+I    | Claude Code siempre lo recibe como Tab                                                                                                                                                                                                           |
| Ctrl+H    | Envía el byte ASCII de retroceso. [Cómo Claude Code lo lee en Windows](/docs/es/terminal-config#fix-backspace-deleting-a-whole-word-on-windows) depende de su terminal y de la variable de entorno [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE`](/docs/es/env-vars) |
| Caps Lock | No se entrega a las aplicaciones de terminal                                                                                                                                                                                                     |

<h2 id="terminal-conflicts">
  Conflictos de terminal
</h2>

Algunos atajos pueden entrar en conflicto con multiplexores de terminal:

| Atajo  | Conflicto                                        |
| :----- | :----------------------------------------------- |
| Ctrl+B | Prefijo de tmux (presione dos veces para enviar) |
| Ctrl+A | Prefijo de GNU screen                            |
| Ctrl+Z | Suspensión de proceso Unix (SIGTSTP)             |

<h2 id="text-fields">
  Campos de texto
</h2>

Si vincula una letra, dígito o espacio sin modificadores, aún puede escribir ese carácter en un campo de texto dentro de un diálogo o panel. Uno de esos campos es la respuesta `Otro` a una pregunta que Claude le hace. Mientras el campo tiene el enfoque, una tecla imprimible que presiona sin Ctrl, Alt o Cmd va al campo, y Claude Code no la compara con sus atajos de teclado.

Estas teclas aún ejecutan sus atajos de teclado mientras el campo tiene el enfoque:

* Teclas que no escriben un carácter, como Enter, Escape, Tab y las teclas de flecha
* Cualquier tecla presionada con Ctrl, Alt o Cmd
* El segundo pulsación de una [combinación](#chords) ya en progreso

En el símbolo del sistema principal, Claude Code compara cada tecla con los contextos activos, como `Chat`, y escribe la tecla solo cuando ningún atajo de teclado la toma.

<h2 id="vim-mode-interaction">
  Interacción del modo Vim
</h2>

Cuando el modo vim está habilitado mediante `/config` → Editor mode, los atajos de teclado y el modo vim funcionan de forma independiente:

* **Modo Vim** maneja la entrada a nivel de entrada de texto (movimiento del cursor, modos, movimientos)
* **Atajos de teclado** manejan acciones a nivel de componente (alternar tareas, enviar, etc.)
* La tecla Escape en modo vim cambia de modo INSERT a NORMAL; no activa `chat:cancel`
* La mayoría de los atajos Ctrl+tecla pasan a través del modo vim al sistema de atajos de teclado
* Las teclas Vim no se pueden remapear a través del archivo de atajos de teclado. Para mapear una secuencia de dos teclas en modo INSERT como `jj` a Escape, use la configuración [`vimInsertModeRemaps`](/docs/es/interactive-mode#remap-insert-mode-key-sequences)
* En modo NORMAL de vim, `?` muestra el menú de ayuda (comportamiento de vim)
* En modo NORMAL de vim, `/` abre la búsqueda de historial, lo mismo que Ctrl+R en modo estándar

<h2 id="validation">
  Validación
</h2>

Claude Code valida sus atajos de teclado y muestra advertencias para:

* Errores de análisis (JSON o estructura inválida)
* Nombres de contexto inválidos
* Valores de acción inválidos, como una acción que no es una cadena o `null`
* Nombres de acción desconocidos, como un error tipográfico de una acción registrada. Claude Code omite la vinculación y mantiene cualquier vinculación predeterminada para esa tecla en vigor. Antes de v2.1.246, una vinculación con un nombre de acción desconocido deshabilitaba silenciosamente esa tecla
* Conflictos de atajos reservados
* Vinculaciones duplicadas en el mismo contexto

Claude Code reporta advertencias cuando el archivo se carga y escribe cada una en el registro de depuración. Inicie Claude Code con [`--debug`](/docs/es/cli-reference#cli-flags) para ver los detalles.
