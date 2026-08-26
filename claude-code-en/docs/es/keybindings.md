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
| `Footer`          | Navegación de indicador de pie de página (tareas, equipos, diff)              |
| `MessageSelector` | Selección de mensaje de diálogo de rebobinado y resumen                       |
| `DiffDialog`      | Navegación del visor de diff                                                  |
| `ModelPicker`     | Nivel de esfuerzo del selector de modelo                                      |
| `Select`          | Componentes genéricos de selección/lista                                      |
| `Plugin`          | Diálogo de plugin (examinar, descubrir, administrar)                          |
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

| Acción                 | Predeterminado | Descripción                                                                                                          |
| :--------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------- |
| `app:interrupt`        | Ctrl+C         | Cancelar operación actual                                                                                            |
| `app:exit`             | Ctrl+D         | Salir de Claude Code                                                                                                 |
| `app:redraw`           | (sin vincular) | Forzar redibujo de terminal                                                                                          |
| `app:toggleTodos`      | Ctrl+T         | Alternar visibilidad de la lista de tareas de Claude. Esta no es la vista de tarea de fondo [`/tasks`](/docs/es/commands) |
| `app:toggleTranscript` | Ctrl+O         | Alternar transcripción detallada                                                                                     |

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

| Acción                | Predeterminado                  | Descripción                                                                                                                                                                                                     |
| :-------------------- | :------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chat:cancel`         | Escape                          | Cancelar entrada actual                                                                                                                                                                                         |
| `chat:clearInput`     | Ctrl+L                          | Forzar un redibujo de pantalla completa, preservando la entrada. En [renderizado de pantalla completa](/docs/es/fullscreen#clear-the-conversation), presione dos veces dentro de dos segundos para ejecutar `/clear` |
| `chat:clearScreen`    | Cmd+K                           | En [renderizado de pantalla completa](/docs/es/fullscreen#clear-the-conversation), presione dos veces dentro de dos segundos para ejecutar `/clear`                                                                  |
| `chat:killAgents`     | Ctrl+X Ctrl+K                   | Detener todos los [subagentes de fondo](/docs/es/sub-agents#run-subagents-in-foreground-or-background) en ejecución en esta sesión                                                                                   |
| `chat:cycleMode`      | Shift+Tab\*                     | Ciclar modos de permiso                                                                                                                                                                                         |
| `chat:modelPicker`    | Meta+P                          | Abrir selector de modelo                                                                                                                                                                                        |
| `chat:fastMode`       | Meta+O                          | Alternar modo rápido                                                                                                                                                                                            |
| `chat:thinkingToggle` | Meta+T                          | Alternar pensamiento extendido                                                                                                                                                                                  |
| `chat:submit`         | Enter                           | Enviar mensaje                                                                                                                                                                                                  |
| `chat:newline`        | Ctrl+J                          | Insertar una nueva línea sin enviar                                                                                                                                                                             |
| `chat:undo`           | Ctrl+\_, Ctrl+Shift+-           | Deshacer última acción                                                                                                                                                                                          |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E           | Abrir en editor externo                                                                                                                                                                                         |
| `chat:stash`          | Ctrl+S                          | Guardar indicación actual                                                                                                                                                                                       |
| `chat:imagePaste`     | Ctrl+V (Alt+V en Windows y WSL) | Pegar imagen desde el portapapeles. En WSL, ambos atajos de teclado están vinculados de forma predeterminada                                                                                                    |

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

| Acción                      | Predeterminado | Descripción                                                                                                                                  |
| :-------------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `confirm:yes`               | Y, Enter       | Confirmar acción                                                                                                                             |
| `confirm:no`                | N, Escape      | Rechazar acción                                                                                                                              |
| `confirm:previous`          | Arriba         | Opción anterior                                                                                                                              |
| `confirm:next`              | Abajo          | Siguiente opción                                                                                                                             |
| `confirm:nextField`         | Tab            | Siguiente campo                                                                                                                              |
| `confirm:previousField`     | (sin vincular) | Campo anterior                                                                                                                               |
| `confirm:toggle`            | Espacio        | Alternar selección                                                                                                                           |
| `confirm:cycleMode`         | Shift+Tab      | Ciclar modos de permiso                                                                                                                      |
| `confirm:toggleExplanation` | Ctrl+E         | Alternar una [explicación generada por modelo del comando](/docs/es/permissions#permission-system) en indicadores de permiso de Bash y PowerShell |

<h3 id="permission-actions">
  Acciones de permiso
</h3>

Acciones disponibles en el contexto `Confirmation` para diálogos de permiso:

| Acción                   | Predeterminado | Descripción                                                                                                                                  |
| :----------------------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `permission:toggleDebug` | (sin vincular) | Alternar información de depuración de permiso. El valor predeterminado anterior de Ctrl+D se eliminó en v2.1.146 porque sombreaba `app:exit` |

<h3 id="transcript-actions">
  Acciones de transcripción
</h3>

Acciones disponibles en el contexto `Transcript`:

| Acción                     | Predeterminado    | Descripción                        |
| :------------------------- | :---------------- | :--------------------------------- |
| `transcript:toggleShowAll` | Ctrl+E            | Alternar mostrar todo el contenido |
| `transcript:exit`          | q, Ctrl+C, Escape | Salir de vista de transcripción    |

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

<h3 id="task-actions">
  Acciones de tarea
</h3>

Acciones disponibles en el contexto `Task`:

| Acción            | Predeterminado        | Descripción                                                                                                          |
| :---------------- | :-------------------- | :------------------------------------------------------------------------------------------------------------------- |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | Tarea de fondo actual. El acorde Ctrl+X Ctrl+B requiere v2.1.169 o posterior y evita el conflicto de prefijo de tmux |

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

| Acción          | Predeterminado       | Descripción       |
| :-------------- | :------------------- | :---------------- |
| `tabs:next`     | Tab, Derecha         | Siguiente pestaña |
| `tabs:previous` | Shift+Tab, Izquierda | Pestaña anterior  |

<h3 id="attachments-actions">
  Acciones de adjuntos
</h3>

Acciones disponibles en el contexto `Attachments`:

| Acción                 | Predeterminado      | Descripción                     |
| :--------------------- | :------------------ | :------------------------------ |
| `attachments:next`     | Derecha             | Siguiente adjunto               |
| `attachments:previous` | Izquierda           | Adjunto anterior                |
| `attachments:remove`   | Retroceso, Suprimir | Eliminar adjunto seleccionado   |
| `attachments:exit`     | Abajo, Escape       | Salir de navegación de adjuntos |

<h3 id="footer-actions">
  Acciones de pie de página
</h3>

Acciones disponibles en el contexto `Footer`:

| Acción                  | Predeterminado | Descripción                                                               |
| :---------------------- | :------------- | :------------------------------------------------------------------------ |
| `footer:next`           | Derecha        | Siguiente elemento de pie de página                                       |
| `footer:previous`       | Izquierda      | Elemento de pie de página anterior                                        |
| `footer:up`             | Arriba         | Navegar hacia arriba en pie de página (deselecciona en la parte superior) |
| `footer:down`           | Abajo          | Navegar hacia abajo en pie de página                                      |
| `footer:openSelected`   | Enter          | Abrir elemento de pie de página seleccionado                              |
| `footer:clearSelection` | Escape         | Limpiar selección de pie de página                                        |

<h3 id="message-selector-actions">
  Acciones del selector de mensajes
</h3>

Acciones disponibles en el contexto `MessageSelector`:

| Acción                   | Predeterminado                                  | Descripción                    |
| :----------------------- | :---------------------------------------------- | :----------------------------- |
| `messageSelector:up`     | Arriba, K, Ctrl+P                               | Mover hacia arriba en la lista |
| `messageSelector:down`   | Abajo, J, Ctrl+N                                | Mover hacia abajo en la lista  |
| `messageSelector:top`    | Ctrl+Arriba, Shift+Arriba, Meta+Arriba, Shift+K | Saltar al inicio               |
| `messageSelector:bottom` | Ctrl+Abajo, Shift+Abajo, Meta+Abajo, Shift+J    | Saltar al final                |
| `messageSelector:select` | Enter                                           | Seleccionar mensaje            |

<h3 id="diff-actions">
  Acciones de diff
</h3>

Acciones disponibles en el contexto `DiffDialog`:

| Acción                | Predeterminado | Descripción                                                                                                                                                                                   |
| :-------------------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `diff:dismiss`        | Escape         | Cerrar visor de diff; desde la vista de detalles, vuelve a la lista de archivos en su lugar                                                                                                   |
| `diff:previousSource` | Izquierda      | Fuente de diff anterior                                                                                                                                                                       |
| `diff:nextSource`     | Derecha        | Siguiente fuente de diff                                                                                                                                                                      |
| `diff:previousFile`   | Arriba, K      | Archivo anterior en la lista de archivos; desplazarse hacia arriba una línea en la vista de detalles                                                                                          |
| `diff:nextFile`       | Abajo, J       | Siguiente archivo en la lista de archivos; desplazarse hacia abajo una línea en la vista de detalles                                                                                          |
| `diff:viewDetails`    | Enter          | Ver detalles de diff                                                                                                                                                                          |
| `diff:back`           | (sin vincular) | Volver atrás en visor de diff. Escape realiza la acción de retroceso a través de `diff:dismiss`. El valor predeterminado anterior de Izquierda en la vista de detalles se eliminó en v2.1.203 |

La vista de detalles de diff también vincula atajos de teclado de estilo paginador a las [acciones de desplazamiento](#scroll-actions) estándar. Estos enlaces son parte del contexto `DiffDialog` y se aplican solo en la vista de detalles; los valores predeterminados del contexto `Scroll` enumerados en [Acciones de desplazamiento](#scroll-actions) no cambian.

| Acción                | Predeterminado   | Descripción                                              |
| :-------------------- | :--------------- | :------------------------------------------------------- |
| `scroll:pageUp`       | Av Pág           | Desplazarse hacia arriba la mitad de una ventana gráfica |
| `scroll:pageDown`     | Re Pág           | Desplazarse hacia abajo la mitad de una ventana gráfica  |
| `scroll:fullPageUp`   | Shift+Espacio, B | Desplazarse hacia arriba una ventana gráfica completa    |
| `scroll:fullPageDown` | Espacio          | Desplazarse hacia abajo una ventana gráfica completa     |
| `scroll:top`          | G, Inicio        | Saltar al inicio                                         |
| `scroll:bottom`       | Shift+G, Fin     | Saltar al final                                          |

<h3 id="model-picker-actions">
  Acciones del selector de modelo
</h3>

Acciones disponibles en el contexto `ModelPicker`:

| Acción                        | Predeterminado | Descripción                                 |
| :---------------------------- | :------------- | :------------------------------------------ |
| `modelPicker:decreaseEffort`  | Izquierda      | Disminuir nivel de esfuerzo                 |
| `modelPicker:increaseEffort`  | Derecha        | Aumentar nivel de esfuerzo                  |
| `modelPicker:thisSessionOnly` | s              | Aplicar modelo resaltado solo a esta sesión |

<h3 id="select-actions">
  Acciones de selección
</h3>

Acciones disponibles en el contexto `Select`:

| Acción            | Predeterminado    | Descripción        |
| :---------------- | :---------------- | :----------------- |
| `select:next`     | Abajo, J, Ctrl+N  | Siguiente opción   |
| `select:previous` | Arriba, K, Ctrl+P | Opción anterior    |
| `select:accept`   | Enter             | Aceptar selección  |
| `select:cancel`   | Escape            | Cancelar selección |

<h3 id="plugin-actions">
  Acciones de plugin
</h3>

Acciones disponibles en el contexto `Plugin`:

| Acción            | Predeterminado | Descripción                                                                                                       |
| :---------------- | :------------- | :---------------------------------------------------------------------------------------------------------------- |
| `plugin:toggle`   | Espacio        | Alternar selección de plugin                                                                                      |
| `plugin:install`  | I              | Instalar plugins seleccionados                                                                                    |
| `plugin:favorite` | F              | Marcar como favorito el plugin seleccionado para que se ordene cerca de la parte superior de la pestaña Instalado |

<h3 id="settings-actions">
  Acciones de configuración
</h3>

Acciones disponibles en el contexto `Settings`. Las acciones `select:accept` y `confirm:no` se reutilizan de los contextos [Selección](#select-actions) y [Confirmación](#confirmation-actions) con comportamiento específico de Configuración: los cambios se aplican a cada configuración tan pronto como la cambia, por lo que Escape cierra el panel con los cambios guardados en lugar de rechazarlos.

| Acción            | Predeterminado | Descripción                                              |
| :---------------- | :------------- | :------------------------------------------------------- |
| `settings:search` | /              | Entrar en modo de búsqueda                               |
| `settings:retry`  | R              | Reintentar carga de datos de uso en caso de error        |
| `select:accept`   | Enter, Espacio | Cambiar la configuración seleccionada o abrir su submenú |
| `confirm:no`      | Escape         | Cerrar el panel. Los cambios ya están guardados          |

<h3 id="voice-actions">
  Acciones de voz
</h3>

Acciones disponibles en el contexto `Chat` cuando [dictado de voz](/docs/es/voice-dictation) está habilitado:

| Acción             | Predeterminado | Descripción                                                               |
| :----------------- | :------------- | :------------------------------------------------------------------------ |
| `voice:pushToTalk` | Espacio        | Dictar una indicación. Mantener presionado o tocar según el modo `/voice` |

<h3 id="scroll-actions">
  Acciones de desplazamiento
</h3>

Acciones disponibles en el contexto `Scroll` cuando [renderizado de pantalla completa](/docs/es/fullscreen) está habilitado:

| Acción                      | Predeterminado       | Descripción                                                                                                                                              |
| :-------------------------- | :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scroll:lineUp`             | (sin vincular)       | Desplazarse hacia arriba una línea. El desplazamiento de rueda de ratón activa esta acción                                                               |
| `scroll:lineDown`           | (sin vincular)       | Desplazarse hacia abajo una línea. El desplazamiento de rueda de ratón activa esta acción                                                                |
| `scroll:pageUp`             | Av Pág               | Desplazarse hacia arriba la mitad de la altura de la ventana gráfica                                                                                     |
| `scroll:pageDown`           | Re Pág               | Desplazarse hacia abajo la mitad de la altura de la ventana gráfica                                                                                      |
| `scroll:top`                | Ctrl+Inicio          | Saltar al inicio de la conversación                                                                                                                      |
| `scroll:bottom`             | Ctrl+Fin             | Saltar al mensaje más reciente y reactivar el seguimiento automático                                                                                     |
| `scroll:halfPageUp`         | (sin vincular)       | Desplazarse hacia arriba la mitad de la altura de la ventana gráfica. Mismo comportamiento que `scroll:pageUp`, proporcionado para rebinds de estilo vi  |
| `scroll:halfPageDown`       | (sin vincular)       | Desplazarse hacia abajo la mitad de la altura de la ventana gráfica. Mismo comportamiento que `scroll:pageDown`, proporcionado para rebinds de estilo vi |
| `scroll:fullPageUp`         | (sin vincular)       | Desplazarse hacia arriba la altura completa de la ventana gráfica                                                                                        |
| `scroll:fullPageDown`       | (sin vincular)       | Desplazarse hacia abajo la altura completa de la ventana gráfica                                                                                         |
| `selection:copy`            | Ctrl+Shift+C / Cmd+C | Copiar el texto seleccionado al portapapeles                                                                                                             |
| `selection:clear`           | (sin vincular)       | Limpiar la selección de texto activa                                                                                                                     |
| `selection:extendLeft`      | Shift+Izquierda      | Extender la selección activa una columna hacia la izquierda                                                                                              |
| `selection:extendRight`     | Shift+Derecha        | Extender la selección activa una columna hacia la derecha                                                                                                |
| `selection:extendUp`        | Shift+Arriba         | Extender la selección activa una fila hacia arriba. Desplaza la ventana gráfica cuando la selección alcanza el borde superior                            |
| `selection:extendDown`      | Shift+Abajo          | Extender la selección activa una fila hacia abajo. Desplaza la ventana gráfica cuando la selección alcanza el borde inferior                             |
| `selection:extendLineStart` | Shift+Inicio         | Extender la selección activa al inicio de la línea                                                                                                       |
| `selection:extendLineEnd`   | Shift+Fin            | Extender la selección activa al final de la línea                                                                                                        |

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

Una letra mayúscula independiente implica Shift. Por ejemplo, `K` es equivalente a `shift+k`. Esto es útil para atajos de teclado de estilo vim donde las teclas mayúsculas y minúsculas tienen significados diferentes.

Las letras mayúsculas con modificadores (por ejemplo, `ctrl+K`) se tratan como estilísticas y **no** implican Shift: `ctrl+K` es lo mismo que `ctrl+k`.

<h3 id="chords">
  Acordes
</h3>

Los acordes son secuencias de pulsaciones de teclas separadas por espacios:

```text theme={null}
ctrl+k ctrl+s   Presione Ctrl+K, suelte, luego Ctrl+S
```

<h3 id="special-keys">
  Teclas especiales
</h3>

* `escape` o `esc` - Tecla Escape
* `enter` o `return` - Tecla Enter
* `tab` - Tecla Tab
* `space` - Barra espaciadora
* `up`, `down`, `left`, `right` - Teclas de flecha
* `backspace`, `delete` - Teclas de eliminación

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

La familia predeterminada `Ctrl+X` abarca dos contextos: `ctrl+x ctrl+k` y `ctrl+x ctrl+e` en `Chat`, y `ctrl+x ctrl+b` en `Task`. Para reclamar `ctrl+x` en sí mismo como una vinculación de tecla única, desvinculación de todos ellos:

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
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
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

| Atajo     | Razón                                            |
| :-------- | :----------------------------------------------- |
| Ctrl+C    | Interrupción/cancelación codificada              |
| Ctrl+D    | Salida codificada                                |
| Ctrl+M    | Idéntico a Enter en terminales (ambos envían CR) |
| Caps Lock | No se entrega a las aplicaciones de terminal     |

<h2 id="terminal-conflicts">
  Conflictos de terminal
</h2>

Algunos atajos pueden entrar en conflicto con multiplexores de terminal:

| Atajo  | Conflicto                                        |
| :----- | :----------------------------------------------- |
| Ctrl+B | Prefijo de tmux (presione dos veces para enviar) |
| Ctrl+A | Prefijo de GNU screen                            |
| Ctrl+Z | Suspensión de proceso Unix (SIGTSTP)             |

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
* Conflictos de atajos reservados
* Conflictos de multiplexor de terminal
* Vinculaciones duplicadas en el mismo contexto

Claude Code reporta advertencias cuando el archivo se carga y escribe cada una en el registro de depuración. Inicie Claude Code con [`--debug`](/docs/es/cli-reference#cli-flags) para ver los detalles.
