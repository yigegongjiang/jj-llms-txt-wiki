> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modo interactivo

> Referencia completa de atajos de teclado, modos de entrada y características interactivas en sesiones de Claude Code.

<h2 id="keyboard-shortcuts">
  Atajos de teclado
</h2>

<Note>
  Los atajos de teclado pueden variar según la plataforma y la terminal. En [renderizado a pantalla completa](/docs/es/fullscreen), presione `?` en el visor de transcripción para ver los atajos disponibles allí.

  **Usuarios de macOS**: Los atajos de la tecla Option/Alt (`Alt+B`, `Alt+F`, `Alt+D`, `Alt+Y`, `Alt+P`) requieren configurar Option como Meta en su terminal. Consulte [Habilitar atajos de tecla Option en macOS](/docs/es/terminal-config#enable-option-key-shortcuts-on-macos) para la configuración en cada terminal.
</Note>

<h3 id="general-controls">
  Controles generales
</h3>

| Atajo                                                                                                           | Descripción                                                                                                                                                                                                                                                                                                | Contexto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :-------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+C`                                                                                                        | Interrumpir o borrar entrada                                                                                                                                                                                                                                                                               | Interrumpe una operación en ejecución. Si nada se está ejecutando, la primera pulsación borra la entrada del símbolo del sistema y una segunda pulsación cierra Claude Code                                                                                                                                                                                                                                                                                                                                                                                            |
| `Ctrl+X Ctrl+K`                                                                                                 | Detener todos los [subagentes en segundo plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background) en esta sesión y desactivar [respuestas automáticas de artefactos](/docs/es/artifacts#let-claude-reply-to-comments-on-its-own) para el resto de ella. Presione dos veces en 3 segundos para confirmar | Control de subagentes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `Ctrl+D`                                                                                                        | Salir de la sesión de Claude Code                                                                                                                                                                                                                                                                          | La primera pulsación muestra una sugerencia de confirmación y una segunda pulsación dentro de 800 ms cierra. Cuando el símbolo del sistema tiene texto, `Ctrl+D` elimina el carácter después del cursor en su lugar                                                                                                                                                                                                                                                                                                                                                    |
| `Ctrl+G` o `Ctrl+X Ctrl+E`                                                                                      | Abrir en el editor de texto predeterminado                                                                                                                                                                                                                                                                 | Edite su símbolo del sistema o respuesta personalizada en su editor de texto predeterminado. `Ctrl+X Ctrl+E` es el enlace nativo de readline. Active **Mostrar última respuesta en editor externo** en `/config` para anteponer la respuesta anterior de Claude como contexto comentado con `#` arriba de su símbolo del sistema; Claude Code elimina el bloque de comentarios cuando guarda                                                                                                                                                                           |
| `Ctrl+L`                                                                                                        | Redibujar la pantalla                                                                                                                                                                                                                                                                                      | Fuerza un redibujado completo de la terminal, manteniendo la entrada y el historial de conversación. Úselo para recuperarse si la pantalla se vuelve ilegible o parcialmente en blanco. Consulte [Borrar la conversación](/docs/es/fullscreen#clear-the-conversation) para renderizado a pantalla completa                                                                                                                                                                                                                                                                  |
| `Ctrl+O`                                                                                                        | Alternar visor de transcripción                                                                                                                                                                                                                                                                            | Muestra el uso detallado de herramientas y ejecución, con una marca de tiempo y el modelo utilizado en cada mensaje del asistente. También expande líneas que se contraen de forma predeterminada, como llamadas MCP, mostradas como una sola línea `Called slack 3 times`, y [mensajes de sus otras sesiones](/docs/es/cross-session-messaging#what-a-message-looks-like), mostrados como una vista previa de una línea `Message from @<sender>`                                                                                                                           |
| `Ctrl+R`                                                                                                        | Búsqueda inversa del historial de comandos                                                                                                                                                                                                                                                                 | Busque a través de comandos anteriores de forma interactiva                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `Ctrl+V` o `Cmd+V` (iTerm2) o `Alt+V` (Windows y WSL)                                                           | Pegar imagen desde el portapapeles                                                                                                                                                                                                                                                                         | Inserta un chip `[Image #N]` en el cursor para que pueda hacer referencia a él posicionalmente en su símbolo del sistema. En WSL, tanto `Ctrl+V` como `Alt+V` están vinculados; use `Alt+V` si su terminal intercepta `Ctrl+V`                                                                                                                                                                                                                                                                                                                                         |
| `Ctrl+B`                                                                                                        | Tareas en ejecución en segundo plano                                                                                                                                                                                                                                                                       | Coloca en segundo plano comandos Bash y agentes. Los usuarios de Tmux presionan dos veces                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `Ctrl+T`                                                                                                        | Alternar lista de tareas de Claude                                                                                                                                                                                                                                                                         | Mostrar u ocultar [lista de tareas pendientes de Claude](#task-list) en el área de estado. Esta no es la vista de tareas en segundo plano; use [`/tasks`](/docs/es/commands) para ver shells y subagentes en ejecución                                                                                                                                                                                                                                                                                                                                                      |
| `Ctrl+S`                                                                                                        | Guardar o restaurar símbolo del sistema                                                                                                                                                                                                                                                                    | Con texto en la entrada, lo guarda y borra el símbolo del sistema. Presionado nuevamente en un símbolo del sistema vacío, restaura el texto guardado, la posición del cursor, el contenido pegado y el modo de entrada, por lo que un `!` guardado [comando shell](#shell-mode-with-prefix) vuelve en modo shell                                                                                                                                                                                                                                                       |
| `Ctrl+Z`                                                                                                        | Suspender Claude Code                                                                                                                                                                                                                                                                                      | Solo Unix. Suspende el proceso a su shell; ejecute `fg` para reanudar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `Flechas izquierda/derecha`                                                                                     | Ciclar a través de pestañas de diálogo                                                                                                                                                                                                                                                                     | Navegue entre pestañas en diálogos de permisos y menús                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `Tab`                                                                                                           | Aceptar una sugerencia de autocompletado o agregar un comentario a una respuesta de permiso                                                                                                                                                                                                                | Mientras se muestran sugerencias de autocompletado en la entrada del símbolo del sistema, acepta la sugerencia seleccionada. En la mayoría de los avisos de permiso, con **Sí** o **No** enfocado, abre un campo de comentario en esa opción, y presionarlo nuevamente cierra el campo. Consulte [agregar un comentario cuando responde a un aviso de permiso](/docs/es/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                                                                                      |
| `Flechas arriba/abajo` o `Ctrl+P`/`Ctrl+N`                                                                      | Mover cursor o navegar por el historial de comandos                                                                                                                                                                                                                                                        | Cuando la entrada abarca más de una fila visual, ya sea envuelta o multilínea, primero mueve el cursor dentro del símbolo del sistema. Una vez que el cursor está en la primera o última fila visual, presionar nuevamente navega por el historial de comandos. Mientras tiene mensajes en cola, `Arriba` desde la primera fila en su lugar [los retira](#take-back-what-you-queued)                                                                                                                                                                                   |
| `Esc`                                                                                                           | Interrumpir Claude o cerrar un diálogo                                                                                                                                                                                                                                                                     | Detiene la respuesta actual o la llamada de herramienta a mitad de turno para que pueda redirigir. Claude mantiene el trabajo realizado hasta ahora. Si tiene [mensajes en cola](#queue-messages-while-claude-works), Claude Code los envía a continuación. Cuando un diálogo está abierto, `Esc` cierra el diálogo. En un aviso de permiso, `Esc` rechaza la acción, lo mismo que [**No** sin un comentario](/docs/es/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                                       |
| `Esc` + `Esc`                                                                                                   | Borrar borrador de entrada o rebobinar                                                                                                                                                                                                                                                                     | Cuando la entrada del símbolo del sistema contiene texto, doble `Esc` lo borra y guarda el borrador en el historial para que `Arriba` lo recupere. Cuando la entrada está vacía, doble `Esc` abre el [menú de rebobinado](/docs/es/checkpointing) para restaurar o resumir código y conversación desde un punto anterior                                                                                                                                                                                                                                                    |
| `Ctrl+Enter` o `Ctrl+X Ctrl+S`                                                                                  | Enviar mensajes en cola ahora                                                                                                                                                                                                                                                                              | Envía sus [mensajes en cola](#queue-messages-while-claude-works) y su borrador con ellos de inmediato. [Cuando Claude Code envía lo que puso en cola](#when-claude-code-sends-what-you-queued) cubre lo que sucede con el turno en el que Claude está trabajando. En [modo shell](#shell-mode-with-prefix), la tecla solo pone en cola su comando. En terminales que no reportan teclas extendidas, `Ctrl+Enter` llega como `Enter` simple; `Ctrl+X Ctrl+S` funciona en cualquier terminal. Requiere Claude Code v2.1.275 o posterior                                  |
| `Shift+Tab`, o `Alt+M` en Windows cuando el tiempo de ejecución de Node o Bun no habilita el modo de entrada VT | Ciclar modos de permiso                                                                                                                                                                                                                                                                                    | Cicle a través de `default` (etiquetado como Manual en el indicador de modo), `acceptEdits`, `plan` y, cuando esté disponible, `bypassPermissions` y luego `auto`. Desde `auto`, la primera pulsación cambia a `default`. Consulte [modos de permiso](/docs/es/permission-modes). En un aviso de permiso de archivo, la misma tecla cierra un [campo de comentario](/docs/es/permissions#add-a-comment-when-you-answer-a-permission-prompt) abierto. Sin campo abierto, selecciona la opción que permite la acción para el resto de la sesión, cuando el aviso ofrece esa opción |
| `Option+P` (macOS) o `Alt+P` (Windows/Linux)                                                                    | Cambiar modelo                                                                                                                                                                                                                                                                                             | Cambie modelos sin borrar su símbolo del sistema                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `Option+T` (macOS) o `Alt+T` (Windows/Linux)                                                                    | Alternar pensamiento extendido                                                                                                                                                                                                                                                                             | Habilitar o deshabilitar el modo de pensamiento extendido. No tiene efecto en Opus 5.5 o los modelos Fable, que siempre utilizan pensamiento extendido. Funciona en macOS sin configurar Option como Meta                                                                                                                                                                                                                                                                                                                                                              |
| `Option+O` (macOS) o `Alt+O` (Windows/Linux)                                                                    | Alternar modo rápido                                                                                                                                                                                                                                                                                       | Habilitar o deshabilitar [modo rápido](/docs/es/fast-mode)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h3 id="text-editing">
  Edición de texto
</h3>

| Atajo                         | Descripción                                              | Contexto                                                                                                                                                                                                                       |
| :---------------------------- | :------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+A`                      | Mover cursor al inicio de la línea actual                | En entrada multilínea, mueve al inicio de la línea lógica actual                                                                                                                                                               |
| `Ctrl+E`                      | Mover cursor al final de la línea actual                 | En entrada multilínea, mueve al final de la línea lógica actual                                                                                                                                                                |
| `Ctrl+K`                      | Eliminar hasta el final de la línea                      | Almacena el texto eliminado para pegarlo                                                                                                                                                                                       |
| `Ctrl+U`                      | Eliminar desde el cursor hasta el inicio de la línea     | Almacena el texto eliminado para pegarlo. Repita para borrar en varias líneas en entrada multilínea. En macOS, los emuladores de terminal incluyendo iTerm2 y Terminal.app asignan `Cmd+Backspace` a este atajo                |
| `Ctrl+W`                      | Eliminar hacia atrás hasta el espacio en blanco anterior | Almacena el texto eliminado para pegarlo. Una pulsación elimina una ruta completa o `--flag=value`. Para eliminar solo la palabra anterior, presione `Option+Delete` en macOS o `Ctrl+Backspace` en Windows                    |
| `Ctrl+Y`                      | Pegar texto eliminado                                    | Pega el texto que eliminó por última vez con uno de los atajos de eliminación de palabra o línea, como `Ctrl+K`, `Ctrl+U` o `Ctrl+W`                                                                                           |
| `Alt+Y` (después de `Ctrl+Y`) | Ciclar historial de pegado                               | Después de pegar, cicle a través del texto eliminado anteriormente. Requiere [Option como Meta](#keyboard-shortcuts) en macOS                                                                                                  |
| `Alt+B`                       | Mover cursor hacia atrás una palabra                     | Navegación de palabras. Requiere [Option como Meta](#keyboard-shortcuts) en macOS                                                                                                                                              |
| `Alt+F`                       | Mover cursor hacia adelante una palabra                  | Se mueve al final de la palabra actual o al final de la siguiente palabra cuando el cursor está entre palabras. Requiere [Option como Meta](#keyboard-shortcuts) en macOS                                                      |
| `Alt+D`                       | Eliminar hasta el final de la palabra                    | Elimina hasta el final de la palabra actual o hasta el final de la siguiente palabra cuando el cursor está entre palabras. Almacena el texto eliminado para pegarlo. Requiere [Option como Meta](#keyboard-shortcuts) en macOS |
| `Ctrl+_` o `Ctrl+Shift+-`     | Deshacer última edición de entrada                       | Restaura el texto de entrada anterior y la posición del cursor                                                                                                                                                                 |

<h3 id="make-ctrl-w-delete-back-to-whitespace">
  Límites de palabras en atajos de edición
</h3>

Los atajos de palabras `Alt+B`, `Alt+F`, `Alt+D`, `Option+Delete` y `Ctrl+Backspace` tratan una palabra como una secuencia de letras y dígitos, por lo que la puntuación como `_`, `.` y `/` separa palabras. Con `src/utils/foo.ts` en el símbolo del sistema, las pulsaciones repetidas de `Alt+B` se detienen al inicio de `ts`, `foo`, `utils` y `src`.

`Ctrl+W` es diferente: ignora la puntuación y elimina hacia atrás hasta el espacio en blanco anterior, por lo que una pulsación elimina todo `src/utils/foo.ts`.

En texto escrito sin espacios, como chino o japonés, los atajos de palabras aún se mueven o eliminan una palabra a la vez.

Estas convenciones de readline se aplican en Claude Code v2.1.261 y posterior. La configuración [`keybindingFlavor`](/docs/es/settings-reference#keybindingflavor) que las activó en versiones anteriores está deprecada y no tiene efecto.

No puede remapear estos atajos en el [archivo de configuración de atajos de teclado](/docs/es/keybindings), que no tiene acciones para ellos.

<h3 id="theme-and-display">
  Tema y pantalla
</h3>

| Atajo    | Descripción                                           | Contexto                                                                                                                           |
| :------- | :---------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+T` | Alternar resaltado de sintaxis para bloques de código | Solo funciona dentro del menú del selector `/theme`. Controla si el código en las respuestas de Claude utiliza colores de sintaxis |

<h3 id="multiline-input">
  Entrada multilínea
</h3>

| Método               | Atajo              | Contexto                                                                                                                                                                                                   |
| :------------------- | :----------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Escape rápido        | `\` + `Enter`      | Funciona en todas las terminales                                                                                                                                                                           |
| Tecla Option         | `Option+Enter`     | Después de habilitar [Option como Meta](/docs/es/terminal-config#enable-option-key-shortcuts-on-macos) en macOS                                                                                                 |
| Shift+Enter          | `Shift+Enter`      | Nativo en iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal, Windows Terminal. Para otras terminales, consulte [Ingresar símbolos del sistema multilínea](/docs/es/terminal-config#enter-multiline-prompts) |
| Secuencia de control | `Ctrl+J`           | Funciona en cualquier terminal sin configuración                                                                                                                                                           |
| Modo de pegado       | Pegar directamente | Para bloques de código, registros                                                                                                                                                                          |

<h3 id="quick-commands">
  Comandos rápidos
</h3>

| Atajo                | Descripción                       | Notas                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------- | :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/` al inicio        | Comando o skill                   | Consulte [comandos](#commands) y [skills](/docs/es/skills)                                                                                                                                                                                                                                                                                                                                 |
| `!` al inicio        | Modo shell                        | Ejecute un comando directamente, agregue su salida a la sesión y haga que Claude responda a ella                                                                                                                                                                                                                                                                                      |
| `@`                  | Mención de ruta de archivo        | Activar autocompletado de ruta de archivo. En sesiones con [mensajería entre sesiones](/docs/es/cross-session-messaging#message-another-session), cuando escribe al menos una letra después de `@`, Claude Code también sugiere sus otras sesiones activas en esta máquina, para que pueda decirle a Claude que envíe un mensaje a la que elija. Requiere Claude Code v2.1.232 o posterior |
| `:`                  | Código abreviado de emoji         | Escriba un `:name:` completo para insertar el emoji, o dos o más caracteres para sugerencias. Consulte [Códigos abreviados de emoji](#emoji-shortcodes). Requiere Claude Code v2.1.217 o posterior                                                                                                                                                                                    |
| `?` en entrada vacía | Alternar panel de ayuda de atajos | Escribir `?` cuando la entrada ya contiene texto inserta el carácter                                                                                                                                                                                                                                                                                                                  |

<h3 id="transcript-viewer">
  Visor de transcripción
</h3>

Cuando el visor de transcripción está abierto (alternado con `Ctrl+O`), estos atajos están disponibles. Ejecute `/tui` sin argumento para verificar qué renderizador está activo. `Ctrl+E` puede ser remapeado a través de [`transcript:toggleShowAll`](/docs/es/keybindings).

| Atajo                | Descripción                                                                                                                                                                                                                                                 |
| :------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Alternar el panel de ayuda de atajos de teclado. Requiere [renderizado a pantalla completa](/docs/es/fullscreen)                                                                                                                                                 |
| `{` / `}`            | Saltar al símbolo del sistema del usuario anterior o siguiente, como movimiento de párrafo de vim. Requiere [renderizado a pantalla completa](/docs/es/fullscreen)                                                                                               |
| `Ctrl+E`             | Alternar mostrar todo el contenido. Disponible solo en el renderizador clásico, no en [renderizado a pantalla completa](/docs/es/fullscreen)                                                                                                                     |
| `[`                  | Escriba la conversación completa en el scrollback nativo de su terminal para que `Cmd+F`, modo de copia de tmux y otras herramientas nativas puedan buscarla. Requiere [renderizado a pantalla completa](/docs/es/fullscreen#search-and-review-the-conversation) |
| `v`                  | Escriba la conversación en un archivo temporal y ábralo en `$VISUAL` o `$EDITOR`. Requiere [renderizado a pantalla completa](/docs/es/fullscreen)                                                                                                                |
| `q`, `Ctrl+C`, `Esc` | Salir de la vista de transcripción. Los tres pueden ser remapeados a través de [`transcript:exit`](/docs/es/keybindings)                                                                                                                                         |

<h3 id="voice-input">
  Entrada de voz
</h3>

| Atajo                    | Descripción    | Notas                                                                                                                                                                                                             |
| :----------------------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Mantener o tocar `Space` | Dictado de voz | Requiere que [dictado de voz](/docs/es/voice-dictation) esté habilitado. Mantenga presionado para grabar, o ejecute `/voice tap` para alternancia de toque. [Remapeable](/docs/es/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Comandos
</h2>

Escriba `/` en Claude Code para ver los comandos disponibles, o escriba `/` seguido de cualquier letra para filtrar. El menú `/` enumera comandos integrados, [skills](/docs/es/skills) agrupadas y creadas por usuarios, y comandos contribuidos por [plugins](/docs/es/plugins/overview) y [servidores MCP](/docs/es/mcp#use-mcp-prompts-as-commands). No todos los comandos integrados son visibles para todos los usuarios, ya que algunos dependen de su plataforma o plan, y [algunos comandos disponibles están ocultos en el menú por diseño](/docs/es/commands#how-the-command-menu-matches-what-you-type) y se ejecutan cuando escribe su nombre completo.

En [renderizado a pantalla completa](/docs/es/fullscreen#use-the-mouse), el comando `/` y las listas de sugerencias de archivos `@` también responden al ratón: pasar el cursor destaca una fila y hacer clic la acepta.

Consulte la [referencia de comandos](/docs/es/commands) para obtener la lista completa de comandos incluidos en Claude Code.

<h3 id="complete-a-command-mid-prompt">
  Completar un comando a mitad de la solicitud
</h3>

La finalización de comandos también funciona a mitad de una solicitud: escriba `/` después de un espacio, luego las primeras letras de un nombre, como en `ejecutar las pruebas, luego /com`. Solo los comandos cuyos nombres comienzan con esas letras coinciden, por lo que una ruta de archivo como `/tmp/notes.md` no mantiene una lista abierta. Claude Code ejecuta un comando solo cuando el comando [inicia su mensaje](/docs/es/commands).

* **En [renderizado a pantalla completa](/docs/es/fullscreen)**: las coincidencias se abren como una lista mientras escribe, sin ninguna fila resaltada, por lo que `Enter` sigue enviando su solicitud tal como está escrita. Presione `Tab` para insertar la coincidencia superior, o seleccione una fila con las teclas de flecha y `Enter`.
* **Fuera del renderizado a pantalla completa**: el resto de la coincidencia superior aparece como texto fantasma en su cursor, con un recuento como `+2` cuando hay más comandos que coinciden. Presione `Tab` para insertar la única coincidencia, o para abrir la lista cuando varias coinciden, luego seleccione una fila con las teclas de flecha y `Enter`.

En ambos renderizadores, presione `Tab` en un `/` desnudo a mitad de la solicitud para enumerar todos los comandos.

Una skill de plugin coincide también con su nombre desnudo, por lo que `/deploy` encuentra una skill denominada `myplugin:deploy-app`. Cuando inserta la coincidencia, Claude Code escribe el `/myplugin:deploy-app` completo.

<h2 id="vim-editor-mode">
  Modo editor Vim
</h2>

Habilite la edición de estilo vim a través de `/config` → Editor mode.

Claude Code mantiene su modo vim y la posición del cursor cuando alterna el [visor de transcripción](#transcript-viewer) con `Ctrl+O` o abre y cierra un panel como `/config`. Si deja el prompt en modo NORMAL, sigue en modo NORMAL cuando regresa, con el cursor donde lo dejó.

<h3 id="mode-switching">
  Cambio de modo
</h3>

| Comando          | Acción                                                                                                                  | Desde el modo  |
| :--------------- | :---------------------------------------------------------------------------------------------------------------------- | :------------- |
| `Esc` o `Ctrl+[` | Entrar en modo NORMAL. En terminales que utilizan el protocolo de teclado Kitty, `Ctrl+[` requiere v2.1.242 o posterior | INSERT, VISUAL |
| `i`              | Insertar antes del cursor                                                                                               | NORMAL         |
| `I`              | Insertar al principio de la línea                                                                                       | NORMAL         |
| `a`              | Insertar después del cursor                                                                                             | NORMAL         |
| `A`              | Insertar al final de la línea                                                                                           | NORMAL         |
| `o`              | Abrir línea debajo                                                                                                      | NORMAL         |
| `O`              | Abrir línea arriba                                                                                                      | NORMAL         |
| `v`              | Iniciar selección visual por carácter                                                                                   | NORMAL         |
| `V`              | Iniciar selección visual por línea                                                                                      | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  Remapear secuencias de teclas en modo INSERT
</h3>

La configuración [`vimInsertModeRemaps`](/docs/es/settings-reference#viminsertmoderemaps) mapea una secuencia de dos teclas en modo INSERT a Escape, por lo que un mapeo como `jj` lo devuelve al modo NORMAL. Requiere Claude Code v2.1.208 o posterior.

El siguiente ejemplo de `~/.claude/settings.json` activa el modo vim y mapea `jj` a Escape:

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Cada clave es exactamente dos caracteres imprimibles escritos en secuencia, y `"<Esc>"` es el único destino admitido. Las entradas con una longitud o destino diferente se ignoran.

Escribir el primer carácter de una secuencia lo inserta normalmente. Presionar el segundo carácter dentro de un segundo elimina ese carácter pendiente y cambia al modo NORMAL, sin dejar ningún carácter en su entrada. Después de la ventana de un segundo, o si sigue una tecla diferente, ambos caracteres permanecen como texto literal, por lo que aún puede escribir una palabra que contenga la secuencia haciendo una pausa entre las dos teclas.

Claude Code lee esta configuración desde su archivo de configuración de usuario, la bandera `--settings` y [configuración administrada](/docs/es/managed-settings) solamente. Las entradas en el archivo `.claude/settings.json` o `.claude/settings.local.json` de un proyecto se ignoran, por lo que un repositorio extraído no puede remapear sus pulsaciones de teclas.

<h3 id="navigation-normal-mode">
  Navegación (modo NORMAL)
</h3>

| Comando         | Acción                                                                                                                                                                                     |
| :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `h`/`j`/`k`/`l` | Mover izquierda/abajo/arriba/derecha                                                                                                                                                       |
| `Space`         | Mover a la derecha                                                                                                                                                                         |
| `w`             | Siguiente palabra                                                                                                                                                                          |
| `e`             | Final de palabra                                                                                                                                                                           |
| `b`             | Palabra anterior                                                                                                                                                                           |
| `0`             | Principio de línea                                                                                                                                                                         |
| `$`             | Final de línea                                                                                                                                                                             |
| `^`             | Primer carácter no en blanco                                                                                                                                                               |
| `gg`            | Principio de entrada                                                                                                                                                                       |
| `G`             | Final de entrada                                                                                                                                                                           |
| `f{char}`       | Saltar a la siguiente ocurrencia del carácter                                                                                                                                              |
| `F{char}`       | Saltar a la ocurrencia anterior del carácter                                                                                                                                               |
| `t{char}`       | Saltar justo antes de la siguiente ocurrencia del carácter                                                                                                                                 |
| `T{char}`       | Saltar justo después de la ocurrencia anterior del carácter                                                                                                                                |
| `;`             | Repetir último movimiento f/F/t/T                                                                                                                                                          |
| `,`             | Repetir último movimiento f/F/t/T en orden inverso                                                                                                                                         |
| `/`             | Abrir búsqueda de historial inverso, igual que `Ctrl+R`. El prompt de búsqueda vacío muestra una sugerencia: presione `Esc` luego `i` luego `/` para abrir el menú de comandos en su lugar |

<Note>
  En modo NORMAL de vim, si el cursor está al principio o al final de la entrada y no puede moverse más, `j`/`k` y `↑`/`↓` navegan por el historial de comandos en su lugar. `←` en un prompt vacío abre [vista de agente](/docs/es/agent-view) desde modo NORMAL así como INSERT; antes de v2.1.219, `←` en un prompt vacío no hacía nada en modo NORMAL.
</Note>

<h3 id="editing-normal-mode">
  Edición (modo NORMAL)
</h3>

| Comando               | Acción                                                                                                                    |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `x`                   | Eliminar carácter                                                                                                         |
| `dd`                  | Eliminar línea                                                                                                            |
| `D`                   | Eliminar hasta el final de la línea                                                                                       |
| `dw`/`de`/`db`        | Eliminar palabra/hasta el final/hacia atrás                                                                               |
| `df{char}`/`dt{char}` | Eliminar hasta e incluyendo, o hasta, la siguiente ocurrencia de un carácter                                              |
| `cc`                  | Cambiar línea                                                                                                             |
| `C`                   | Cambiar hasta el final de la línea                                                                                        |
| `cw`/`ce`/`cb`        | Cambiar palabra/hasta el final/hacia atrás                                                                                |
| `s`                   | Sustituir carácter: eliminar el carácter bajo el cursor e ingresar modo INSERT. Requiere Claude Code v2.1.211 o posterior |
| `S`                   | Sustituir línea: limpiar la línea e ingresar modo INSERT. Requiere Claude Code v2.1.211 o posterior                       |
| `yy`/`Y`              | Yanquear (copiar) línea                                                                                                   |
| `yw`/`ye`/`yb`        | Yanquear palabra/hasta el final/hacia atrás                                                                               |
| `p`                   | Pegar después del cursor                                                                                                  |
| `P`                   | Pegar antes del cursor                                                                                                    |
| `>>`                  | Indentar línea                                                                                                            |
| `<<`                  | Desindentación de línea                                                                                                   |
| `J`                   | Unir líneas                                                                                                               |
| `u`                   | Deshacer                                                                                                                  |
| `.`                   | Repetir último cambio                                                                                                     |

<h3 id="text-objects-normal-mode">
  Objetos de texto (modo NORMAL)
</h3>

Los objetos de texto funcionan con operadores como `d`, `c` e `y`:

| Comando   | Acción                                                         |
| :-------- | :------------------------------------------------------------- |
| `iw`/`aw` | Palabra interior/alrededor                                     |
| `iW`/`aW` | PALABRA interior/alrededor (delimitada por espacios en blanco) |
| `i"`/`a"` | Interior/alrededor de comillas dobles                          |
| `i'`/`a'` | Interior/alrededor de comillas simples                         |
| `i(`/`a(` | Interior/alrededor de paréntesis                               |
| `i[`/`a[` | Interior/alrededor de corchetes                                |
| `i{`/`a{` | Interior/alrededor de llaves                                   |

<h3 id="visual-mode">
  Modo visual
</h3>

Presione `v` para selección por carácter o `V` para selección por línea. Los movimientos extienden la selección, y los operadores actúan sobre ella directamente.

| Comando          | Acción                                                      |
| :--------------- | :---------------------------------------------------------- |
| `d`/`x`          | Eliminar selección                                          |
| `y`              | Yanquear selección                                          |
| `c`/`s`          | Cambiar selección                                           |
| `p`              | Reemplazar selección con contenido del registro             |
| `r{char}`        | Reemplazar cada carácter seleccionado con `{char}`          |
| `~`/`u`/`U`      | Alternar, minúsculas o mayúsculas de selección              |
| `>`/`<`          | Indentar o desindentación de líneas seleccionadas           |
| `J`              | Unir líneas seleccionadas                                   |
| `o`              | Intercambiar cursor y ancla                                 |
| `iw`/`aw`/`i"`/… | Seleccionar un objeto de texto                              |
| `v`/`V`          | Alternar entre carácter a carácter y línea a línea, o salir |

El modo visual por bloques con `Ctrl+V` no es compatible.

<h2 id="command-history">
  Historial de comandos
</h2>

Claude Code mantiene un historial de los prompts que escribe, y la recuperación con la flecha hacia arriba accede a prompts de sesiones anteriores del mismo proyecto:

* El historial de entrada se almacena por directorio de trabajo
* Ejecutar `/clear` inicia una nueva sesión: la recuperación luego enumera los prompts de la nueva sesión primero, con los prompts de sesiones anteriores después de ellos. La conversación de la sesión anterior se conserva y se puede reanudar.
* Enviar el mismo prompt dos veces seguidas registra una entrada de historial, por lo que presionar Arriba va al prompt anterior distinto
* Cuando recupera un prompt que incluía texto pegado, Claude Code envía el contenido pegado completo nuevamente cuando lo reenvía. Si el contenido ha sido [limpiado](/docs/es/claude-directory#cleaned-up-automatically), Claude Code no envía la cadena literal `[Pasted text #N]`; consulte [Pegar contenido grande](/docs/es/terminal-config#paste-large-content) para ver qué sucede con el prompt
* La expansión del historial con `!` está deshabilitada de forma predeterminada

<h3 id="reverse-search-with-ctrl-r">
  Búsqueda inversa con Ctrl+R
</h3>

Presione `Ctrl+R` para buscar interactivamente en su historial de comandos. En [renderizado a pantalla completa](/docs/es/fullscreen), `Ctrl+R` abre un diálogo de búsqueda en su lugar: escriba para filtrar, presione `Arriba` y `Abajo` para moverse entre coincidencias, y presione `Ctrl+S` para cambiar el alcance entre esta sesión, este proyecto y todos los proyectos. Presione `Intro` o `Tab` para colocar una coincidencia en la entrada del prompt, o `Esc` para cancelar. Los pasos a continuación describen la búsqueda en línea del renderizador clásico:

1. **Iniciar búsqueda**: presione `Ctrl+R` para activar la búsqueda inversa del historial
2. **Escribir consulta**: ingrese texto para buscar en comandos anteriores. El término de búsqueda se resalta en los resultados coincidentes
3. **Navegar coincidencias**: presione `Ctrl+R` nuevamente para recorrer coincidencias más antiguas
4. **Alcance de búsqueda**: la búsqueda en línea siempre busca prompts de todos los proyectos
5. **Aceptar coincidencia**:
   * Presione `Tab` o `Esc` para aceptar la coincidencia actual y continuar editando
   * Presione `Intro` para aceptar y ejecutar el comando inmediatamente
6. **Cancelar búsqueda**:
   * Presione `Ctrl+C` para cancelar y restaurar su entrada original
   * Presione `Retroceso` en búsqueda vacía para cancelar

La búsqueda en línea escanea su historial de prompts completo, el más reciente primero, con duplicados contraídos a la ocurrencia más reciente. El diálogo a pantalla completa busca en todo su historial de prompts en el alcance seleccionado, el más reciente primero, con duplicados contraídos a la ocurrencia más reciente: los prompts más recientes aparecen inmediatamente, y las coincidencias de prompts más antiguos se rellenan a medida que Claude Code carga el resto. Los prompts coincidentes se muestran con el término de búsqueda resaltado, para que pueda encontrar y reutilizar entradas anteriores.

Aceptar una coincidencia o cancelar la búsqueda tiene efecto inmediatamente, incluso mientras Claude Code aún está cargando el historial.

<h2 id="background-bash-commands">
  Comandos Bash en segundo plano
</h2>

Claude Code admite la ejecución de comandos Bash en segundo plano, lo que le permite continuar trabajando mientras se ejecutan procesos de larga duración.

<h3 id="how-backgrounding-works">
  Cómo funciona la ejecución en segundo plano
</h3>

Cuando Claude Code ejecuta un comando en segundo plano, lo ejecuta de forma asincrónica y devuelve inmediatamente un ID de tarea en segundo plano. Claude Code puede responder a nuevas indicaciones mientras el comando continúa ejecutándose en segundo plano.

Para ejecutar comandos en segundo plano, puede:

* Indicar a Claude Code que ejecute un comando en segundo plano
* Presionar `Ctrl+B` para mover una invocación regular de la herramienta Bash al segundo plano. Los usuarios de Tmux deben presionar `Ctrl+B` dos veces debido a la tecla de prefijo de tmux.

**Características clave:**

* La salida se escribe en un archivo y Claude puede recuperarla usando la herramienta Read
* Las tareas en segundo plano tienen ID únicos para el seguimiento y la recuperación de salida
* Las tareas en segundo plano se limpian automáticamente cuando Claude Code se cierra. En macOS y Linux, cuando detiene una tarea en segundo plano desde [`/tasks`](/docs/es/commands) o Claude Code la detiene al salir, los procesos que se desvincularon del shell de la tarea, como los iniciados bajo `setsid` o `timeout`, también se detienen
* Si coloca la sesión en segundo plano en lugar de salir, sus tareas en segundo plano continúan ejecutándose en la sesión en segundo plano. Consulte [colocar una sesión en ejecución en segundo plano](/docs/es/agent-view#from-inside-a-session)
* Las tareas en segundo plano se terminan automáticamente si la salida excede 5GB, con una nota en stderr explicando por qué
* En macOS y Linux, Claude Code detiene sus tareas en segundo plano en ejecución cuando el sistema operativo señala presión de memoria crítica, siempre que la sesión haya estado inactiva durante al menos 30 minutos y no se esté ejecutando ningún turno o subagente. Requiere Claude Code v2.1.193 o posterior
  * El [registro de depuración](/docs/es/debug-your-config) indica por qué se detuvieron las tareas, o por qué un evento de presión las dejó en ejecución
  * Establezca [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/es/env-vars) en `1` para desactivar las detenciones por presión de memoria
* Los comandos en segundo plano propiedad de un [subagente](/docs/es/sub-agents) no tienen límite de tiempo, excepto que un comando propiedad de un subagente que se ejecuta en primer plano termina cuando ese subagente da su respuesta final; consulte [Comandos en segundo plano](/docs/es/tools-reference#background-commands) en la referencia de herramientas. Antes de v2.1.218, ni la recolección de presión de memoria ni el límite anterior de 60 minutos en comandos de subagente cubrían comandos movidos al segundo plano con `Ctrl+B`

Para desactivar toda la funcionalidad de tareas en segundo plano, establezca la variable de entorno `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` en `1`. Consulte [Variables de entorno](/docs/es/env-vars) para obtener más detalles.

**Comandos comúnmente colocados en segundo plano:**

* Herramientas de compilación (webpack, vite, make)
* Gestores de paquetes (npm, yarn, pnpm)
* Ejecutores de pruebas (jest, pytest)
* Servidores de desarrollo
* Procesos de larga duración (docker, terraform)

<h3 id="shell-mode-with-prefix">
  Modo shell con prefijo `!`
</h3>

Ejecute comandos shell directamente sin pasar por Claude prefijando su entrada con `!`:

```bash theme={null}
! npm test
! git status
! ls -la
```

Modo shell:

* Agrega el comando y su salida al contexto de la conversación
* Muestra el progreso y la salida en tiempo real
* Admite el mismo backgrounding `Ctrl+B` para comandos de larga duración
* No requiere que Claude interprete o apruebe el comando
* Admite autocompletado basado en historial: escriba un comando parcial y presione `Tab` para completar desde comandos `!` anteriores en el proyecto actual
* Admite autocompletado de ruta de archivo en vivo a partir de v2.1.193 en todas las plataformas: escriba un token que contenga una barra diagonal, como `./src/` o `~/`, para ver una lista desplegable de archivos y directorios coincidentes, luego presione `Tab` para aceptar. Use barras diagonales en Windows también; la lista desplegable se activa con `/`, no con `\`
* Salga con `Escape`, `Backspace`, o `Ctrl+U` en un indicador vacío
* Pegar texto que comienza con `!` en un indicador vacío entra en modo shell automáticamente, coincidiendo con el comportamiento de `!` escrito

A menos que su sesión sea una de las enumeradas en [modo sandbox estricto](/docs/es/sandboxing#the-unsandboxed-retry-escape-hatch), los comandos que escribe en modo shell se ejecutan fuera del [sandbox](/docs/es/sandboxing) incluso cuando ha habilitado sandboxing, porque el sandbox se aplica a los comandos que ejecuta Claude.

Claude responde automáticamente a la salida del comando una vez que llega a la transcripción, por lo que puede ejecutar `! npm test` y obtener una explicación de los errores sin un segundo indicador. La respuesta cuesta lo mismo que enviar un indicador normal. Para restaurar el comportamiento anterior donde la salida se agrega al contexto sin una respuesta, establezca [`respondToBashCommands`](/docs/es/settings-reference#respondtobashcommands) en `false` en `settings.json`. Antes de v2.1.186, el modo shell siempre agregaba salida al contexto sin una respuesta.

<h2 id="queue-messages-while-claude-works">
  Encolar mensajes mientras Claude trabaja
</h2>

Escriba un mensaje y presione `Enter` mientras Claude está trabajando. Claude Code encola el mensaje en lugar de interrumpir el turno, y lista las entradas encoladas encima del cuadro de entrada hasta que las envía. Puede encolar comandos `!` [shell](#shell-mode-with-prefix) y la mayoría de [comandos](/docs/es/commands) de la misma manera, aparte de los comandos, como `/status`, que Claude Code ejecuta tan pronto como los envía.

Los mensajes enviados y encolados se muestran en gris hasta que Claude comienza a responder a ellos, por lo que puede saber qué mensajes Claude aún no ha comenzado.

<h3 id="when-claude-code-sends-what-you-queued">
  Cuándo Claude Code envía lo que encoló
</h3>

Cuándo una entrada encolada llega a Claude depende de lo que encoló.

* Mensajes: si encola un mensaje mientras Claude está ejecutando llamadas de herramientas, Claude Code lo pasa a Claude tan pronto como esas llamadas de herramientas terminen, dentro del mismo turno. Cuando el turno termina con mensajes aún encolados, salen sin otra pulsación de tecla, en el orden en que los escribió
* Comandos y comandos shell: Claude Code los retiene hasta que el turno termina, luego los ejecuta uno a la vez, manteniendo el orden en que los encoló

Para enviar lo que encoló sin esperar, presione `Ctrl+Enter`. Sus mensajes encolados salen de inmediato, con su borrador encolado detrás si había escrito uno. Requiere Claude Code v2.1.275 o posterior.

Si encoló un comando shell `!` antes de sus mensajes, la tecla interrumpe el turno. De lo contrario, lo que sucede con el turno depende de lo que Claude esté haciendo cuando presione la tecla:

* Ejecutando comandos shell, subagentes u otro trabajo que pueda moverse al [fondo](#background-bash-commands): ese trabajo se mueve al fondo y sigue ejecutándose, y Claude lee sus mensajes en el mismo turno
* Solo escribiendo una respuesta, o ejecutando algo que no pueda moverse al fondo: Claude Code interrumpe el turno y envía sus mensajes a continuación. Antes de v2.1.281, la tecla interrumpía el turno en ambos casos

En [modo shell](#shell-mode-with-prefix), la tecla solo encola su comando. En terminales que no reportan teclas extendidas, `Ctrl+Enter` llega como `Enter` simple y encola el borrador en su lugar; `Ctrl+X Ctrl+S` funciona en cualquier terminal. Ambas teclas son vinculaciones de la [acción `chat:sendNow`](/docs/es/keybindings#chat-actions).

Presione `Esc` para interrumpir el turno sin enviar su borrador. Claude Code mantiene lo que encoló y lo envía de inmediato.

Claude Code ejecuta algunos comandos tan pronto como los envía en lugar de encolarlos, entre ellos `/model`, `/effort`, y `/fast`. Cada uno de los tres cambia una configuración: el modelo, el nivel de esfuerzo, o el modo rápido. Si Claude Code aplica la nueva configuración al turno en el que Claude ya está trabajando, o solo desde su siguiente turno, difiere por comando:

* [`/model`](/docs/es/model-config#setting-your-model): una vez que confirme la [advertencia de caché](/docs/es/prompt-caching#switching-models), si Claude Code muestra una, Claude Code aplica su cambio a la siguiente solicitud que realiza en ese turno
* [`/effort`](/docs/es/model-config#adjust-effort-level): una vez que confirme la [advertencia de caché](/docs/es/prompt-caching#changing-effort-level), si Claude Code muestra una, Claude Code aplica su cambio a la siguiente solicitud que realiza en ese turno
* [`/fast`](/docs/es/fast-mode#toggle-fast-mode): Claude Code mantiene la configuración de modo rápido que estaba activa cuando el turno comenzó, por lo que su cambio de velocidad se aplica desde su siguiente turno. Si su modelo actual no admite modo rápido, activarlo también [cambia su modelo](/docs/es/prompt-caching#turning-on-fast-mode), y Claude Code usa el nuevo modelo desde su siguiente solicitud en ese turno

<h3 id="take-back-what-you-queued">
  Recupere lo que encoló
</h3>

Presione `Up` desde la primera línea del cuadro de entrada para recuperar los mensajes y comandos encolados. Claude Code los elimina de la cola y los coloca en el cuadro de entrada, uno por línea, antes de cualquier texto que haya escrito. Edite el texto y presione `Enter` para encolarlo nuevamente como una entrada, o borre el cuadro de entrada para descartarlo.

Claude Code recupera comandos shell encolados solo cuando el cuadro de entrada está vacío y no tiene nada más encolado, y cambia el cuadro de entrada al modo shell cuando lo hace. De lo contrario, los deja en la cola, listados con su prefijo `!`, y los ejecuta después de que el turno termina.

<h2 id="prompt-suggestions">
  Sugerencias de prompt
</h2>

Cuando abre una sesión por primera vez, Claude Code muestra un comando de ejemplo atenuado en la entrada del prompt para ayudarle a comenzar. Lo selecciona del historial de git de su proyecto, por lo que el ejemplo refleja los archivos en los que ha estado trabajando recientemente.

Después de que Claude responda, Claude Code puede sugerir su próximo prompt basándose en el historial de su conversación, como un paso de seguimiento de una solicitud de varias partes o una continuación natural de su flujo de trabajo.

* Presione `Tab` o `Flecha derecha` para colocar la sugerencia en la entrada del prompt, luego `Intro` para enviar
* Comience a escribir para descartar la sugerencia

Claude Code genera cada una de estas sugerencias de próximo prompt con una solicitud de fondo al mismo modelo que su sesión está utilizando. La solicitud cuenta hacia los límites de uso de su plan o sus costos de API. Debido a que reutiliza la caché de prompts de la conversación, es principalmente lecturas de caché más algunos tokens de salida, por lo que el costo adicional es pequeño.

<h3 id="when-claude-code-skips-suggestions">
  Cuándo Claude Code omite sugerencias
</h3>

En modo interactivo, Claude Code deja las sugerencias de prompt desactivadas de forma predeterminada y oculta el botón de alternancia **Prompt suggestions** en `/config` en una [sesión que no obtiene indicadores de características](/docs/es/env-vars#features-that-need-feature-flag-fetching), como una en un proveedor de terceros o a través de una puerta de enlace de aplicaciones Claude, y en una [primera sesión después de una instalación o actualización](/docs/es/env-vars#first-session-after-an-install-or-upgrade) cuyos indicadores aún no han llegado.

Claude Code también omite sugerencias individuales en varias situaciones, incluyendo:

* La caché de prompts está fría, para evitar costos innecesarios
* Después del primer turno de una conversación, en algunas sesiones
* La respuesta anterior terminó en un error
* Mientras está en Plan Mode
* Su cuenta está cerca o en su límite de uso. Para mantener las sugerencias activadas hasta que alcance el límite, establezca [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/es/env-vars) en `true`. Antes de v2.1.238, Claude Code las omitía cerca del límite incluso con la variable establecida en `true`
* En un [equipo de agentes](/docs/es/agent-teams), en las sesiones de compañeros de equipo de forma predeterminada. La sesión del líder muestra sugerencias

En modo de impresión, Claude Code no genera sugerencias de forma predeterminada. Pase [`--prompt-suggestions`](/docs/es/cli-reference#cli-flags) con `-p "<prompt>" --output-format stream-json --verbose` para que Claude Code emita un mensaje `prompt_suggestion` después de cada turno que genere uno. El generador también omite conversaciones muy cortas y cachés de prompts frías aquí, por lo que una única consulta `-p` corta puede no emitir ninguna.

<h3 id="turn-prompt-suggestions-off">
  Desactivar sugerencias de prompt
</h3>

Para desactivar completamente las sugerencias de prompt, use cualquiera de las siguientes opciones:

* Desactive **Prompt suggestions** en `/config`
* Establezca [`promptSuggestionEnabled`](/docs/es/settings-reference#promptsuggestionenabled) en `false` en su archivo de configuración
* Establezca la variable de entorno [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/es/env-vars) en `false`, que tiene prioridad sobre la configuración:
  ```bash theme={null}
  export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
  ```

Para desactivar las sugerencias de prompt en toda una organización, establezca `promptSuggestionEnabled` en `false` en [configuración administrada](/docs/es/managed-settings). También establezca `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION` en `false` bajo la clave [`env`](/docs/es/settings-reference#env) administrada para que los usuarios no puedan reactivarlas con su propia variable de entorno.

<h2 id="emoji-shortcodes">
  Códigos de emoji
</h2>

Escriba un `:` seguido de un código de emoji en la entrada del prompt para insertar el emoji. Requiere Claude Code v2.1.217 o posterior.

* Escriba un código completo como `:heart:` y Claude Code lo reemplaza con ❤️ tan pronto como escriba el `:` de cierre
* Escriba `:` más al menos dos caracteres de un nombre, como `:hea`, para abrir una ventana emergente de sugerencias, luego presione `Tab` o `Enter` para insertar el emoji resaltado

El código debe comenzar la entrada o seguir un espacio, por lo que un `:` dentro de una palabra o URL no abre sugerencias.

Para desactivar la función, establezca [`emojiCompletionEnabled`](/docs/es/settings-reference#emojicompletionenabled) en `false` en `settings.json`. Esto desactiva tanto la ventana emergente de sugerencias como el reemplazo en línea.

<h2 id="check-spelling-as-you-type">
  Verificar la ortografía mientras escribe
</h2>

Claude Code puede subrayar palabras mal escritas en la entrada del símbolo del sistema mientras escribe. Solo verifica el texto en el cuadro de entrada, nunca las respuestas de Claude ni sus archivos. Tampoco verifica nada mientras el cuadro de entrada está en [modo shell](#shell-mode-with-prefix), búsqueda del historial `Ctrl+R`, o [dictado de voz](/docs/es/voice-dictation).

La verificación ortográfica está desactivada de forma predeterminada, y Claude Code no verifica nada en [modo lector de pantalla](/docs/es/accessibility). Requiere Claude Code v2.1.235 o posterior.

<h3 id="prerequisites">
  Requisitos previos
</h3>

* Instale [aspell](https://github.com/GNUAspell/aspell), [hunspell](https://github.com/hunspell/hunspell), o [ispell](https://en.wikipedia.org/wiki/Ispell) y asegúrese de que esté en su `PATH`. Claude Code ejecuta el primero de los tres que encuentra, en ese orden, en todas las plataformas, incluido un shim `.cmd` que un gestor de paquetes instala en Windows.
* Para verificar que el programa está en su `PATH`, ejecute `aspell --version`, `hunspell --version`, o `ispell -v` en su terminal. Un error "command not found" significa que aún no está en su `PATH`.

<h3 id="turn-spell-checking-on-or-off">
  Activar o desactivar la verificación ortográfica
</h3>

Claude Code lee la configuración [`spellcheck`](/docs/es/settings-reference#spellcheck) desde tres lugares e la ignora en el archivo `.claude/settings.json` y `.claude/settings.local.json` de un proyecto. Actívela desde el que utilice:

<Tabs>
  <Tab title="Configuración de usuario">
    Agregue `spellcheck` a `~/.claude/settings.json`. Se aplica en cada proyecto que abre, como el resto de su [configuración de usuario](/docs/es/settings#where-settings-live):

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>

  <Tab title="Línea de comandos">
    Guarde `spellcheck` en un archivo JSON, como `spellcheck.json`:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```

    Luego pase el archivo a `--settings`. Se aplica solo a esa sesión:

    ```bash theme={null}
    claude --settings spellcheck.json
    ```
  </Tab>

  <Tab title="Configuración administrada">
    Agregue `spellcheck` a una de las [fuentes de configuración administrada](/docs/es/permissions#managed-settings) de su organización. Se aplica a cada usuario que recibe esa configuración, y no pueden desactivarla:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>
</Tabs>

Para verificar que la verificación ortográfica está activada, escriba una palabra mal escrita y un espacio. Claude Code subraya la palabra. Si no lo hace, consulte [Cuando Claude Code no subraya nada](#when-claude-code-underlines-nothing). Para desactivar la verificación ortográfica nuevamente, establezca `enabled` en `false` en el mismo lugar, o elimine `spellcheck`.

Para elegir cuál de los tres programas ejecuta Claude Code, qué diccionario utiliza, o el color del subrayado, agregue cualquiera de estos campos junto a `enabled`, en el mismo lugar:

* `checker`: `aspell`, `hunspell`, o `ispell`. Claude Code no retrocede desde un verificador que nombre, y trata cualquier otro valor como `auto`.
* `language`: un nombre de diccionario en la forma de su verificador, como `en_GB`. Claude Code ignora cualquier valor que no sea un nombre de diccionario simple, como una ruta o un nombre con espacios, y el verificador utiliza su diccionario predeterminado.
* `color`: un nombre de color como `yellow`, o un valor `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, o `ansi:<name>`. Claude Code utiliza el color de error de su tema de forma predeterminada y para cualquier valor que no reconozca.

Por ejemplo, esta configuración `spellcheck` ejecuta hunspell con su diccionario `en_GB` y subraya palabras en amarillo. Funciona igual en `~/.claude/settings.json`, en el archivo que pasa a `--settings`, y en la configuración administrada:

```json theme={null}
{
  "spellcheck": {
    "enabled": true,
    "checker": "hunspell",
    "language": "en_GB",
    "color": "yellow"
  }
}
```

Si más de uno de los tres lugares tiene una configuración `spellcheck`, Claude Code utiliza solo uno de ellos: primero la configuración administrada, luego `--settings`, luego la configuración de usuario. No combina campos de dos lugares. Por ejemplo, cuando `--settings` establece `spellcheck`, un `language` en su configuración de usuario no tiene efecto.

<h3 id="what-claude-code-underlines">
  Qué subraya Claude Code
</h3>

Poco después de que deje de escribir, Claude Code subraya las palabras que el diccionario no conoce. Deja sola la palabra que aún está escribiendo hasta que se mueva más allá de ella, y nunca cambia su texto. También omite el texto que parece código:

* Comandos como `/help`, menciones `@`, URLs, rutas de archivo, y banderas como `--verbose`
* Palabras con dígitos, guiones bajos, o una letra mayúscula después de la primera, y texto entre comillas invertidas

Claude Code también omite texto en chino, japonés, coreano, tailandés, lao, jemer y birmano.

Claude Code no tiene su propia lista de palabras: una palabra está mal escrita cuando su verificador dice que lo está. Para evitar que Claude Code subraye una palabra, agregue la palabra al diccionario personal de su verificador, siguiendo la documentación del verificador. Claude Code recoge la palabra nueva después de reiniciarlo.

<h3 id="when-claude-code-underlines-nothing">
  Cuando Claude Code no subraya nada
</h3>

Claude Code no subraya nada cuando no puede mantener un verificador en ejecución:

* No hay verificador instalado, o el que nombró en `checker` falta
* El verificador falla dos veces seguidas, al inicio o más tarde en la sesión. Claude Code lo reinicia después del primer fallo y deja de verificar después del segundo, hasta que reinicie Claude Code
* El verificador tarda más de 15 segundos en responder, tres veces. Cada vez, Claude Code deja sin marcar las palabras en las que estaba esperando; después de la tercera, deja de verificar hasta que reinicie Claude Code

Para averiguar cuál de estos sucedió, inicie `claude --debug` con la verificación ortográfica activada y escriba una palabra. Luego busque las líneas `[spellcheck]` en el registro de depuración en `~/.claude/debug/<session-id>.txt`. Una línea nombra el programa que Claude Code inició, o enumera los que buscó y no encontró. Las líneas posteriores dicen por qué se detuvo. Un error de diccionario faltante allí significa que el verificador no tiene diccionario para su valor `language`, o ninguno predeterminado cuando `language` no está establecido. Instale uno, o establezca `language` en un diccionario que tenga.

<h2 id="invisible-characters-in-prompts">
  Caracteres invisibles en indicaciones
</h2>

El texto pegado puede contener caracteres Unicode que una terminal dibuja como nada en absoluto, como caracteres de etiqueta, controles bidireccionales y espacios de ancho cero, por lo que una indicación puede contener texto que nunca ve. Para evitar que el texto copiado lleve instrucciones que su terminal no dibuja, Claude Code elimina esos caracteres cuando presiona Intro, antes de enviar nada. Limpia tanto la indicación como el contenido de cualquier [referencia de texto pegado](/docs/es/terminal-config#paste-large-content) que la indicación incluya. Claude Code mantiene los conectores que escriben los scripts persa e índico y los selectores dentro de secuencias de emoji.

Si Claude Code eliminó algo, ese Intro no envía nada. La indicación limpia vuelve al cuadro de entrada con un aviso como `Se eliminaron 3 caracteres invisibles · revise y presione Intro para enviar`, y presionar Intro nuevamente envía el texto tal como se muestra.

Cuando pasa una indicación en la línea de comandos, como en `claude "fix the login bug"`, o canaliza una en una sesión interactiva, Claude Code no espera un segundo Intro. Elimina los caracteres, muestra un aviso y envía la indicación limpia. Si la indicación limpia comenzaría con `/`, Claude Code la coloca en el cuadro de entrada para que la revise y envíe.

<h2 id="review-changes-with-/diff">
  Revise cambios con /diff
</h2>

Ejecute `/diff` para revisar los cambios en su árbol de trabajo sin salir de Claude Code. Verá las ediciones que Claude ha realizado hasta ahora junto con cualquier otra cosa que no haya confirmado.

En los cambios que `/diff` lee de git, un submódulo aparece como una única entrada, y solo cuando cambia la confirmación a la que apunta; las ediciones a archivos dentro del submódulo no aparecen allí.

En [renderizado a pantalla completa](/docs/es/fullscreen), `/diff` abre el [panel de diff](#diff-panel) junto a la conversación, que permanece abierto y se actualiza mientras continúa trabajando. En el renderizador clásico, `/diff` abre el [visor de diff](#diff-viewer) en lugar del mensaje, y lo cierra cuando termine de leer.

<h3 id="diff-panel">
  Panel de diff
</h3>

El panel de diff enumera los archivos modificados con sus recuentos de líneas añadidas y eliminadas, y muestra el diff de cada archivo bajo la lista. Claude Code lo actualiza cada vez que Claude edita un archivo o ejecuta un comando de shell. Para cerrarlo, ejecute `/diff` nuevamente o haga clic en la `✕` en su encabezado.

Para usar el panel necesita:

* [Renderizado a pantalla completa](/docs/es/fullscreen)
* Un repositorio git
* Una terminal con al menos 110 columnas de ancho
* Claude Code v2.1.260 o posterior

Cuando el panel no puede abrirse, `/diff` abre el visor de diff en su lugar o le indica por qué.

El panel también se abre por sí solo una vez que Claude comienza a editar archivos, si su terminal tiene al menos 144 columnas de ancho. Después de haberlo abierto usted mismo con `/diff`, las sesiones posteriores lo abren tan pronto como Claude edita un archivo en cualquier terminal lo suficientemente ancha para contenerlo. Cierre el panel y permanecerá cerrado, en esta sesión y en las posteriores, hasta que ejecute `/diff` nuevamente.

Mientras el panel está abierto, puede:

* **Saltar a un archivo**: haga clic en su fila en la lista. Desplace el panel con la rueda del ratón. Cuando la lista de archivos en sí es demasiado larga para caber, desplácela con `Alt+Arriba` y `Alt+Abajo`, o `Ctrl+Arriba` y `Ctrl+Abajo`.
* **Pregunte a Claude sobre líneas específicas**: selecciónelas en el panel con el ratón. Claude Code adjunta la selección a su siguiente mensaje y muestra un recuento de líneas en la entrada hasta que lo envíe.
  * Para enviar el mensaje sin la selección, mueva el cursor justo después del indicador de recuento de líneas y presione `Retroceso` para eliminarlo. Requiere Claude Code v2.1.271 o posterior.
* **Muestre los archivos que el panel omite**: la lista omite archivos de prueba y archivos generados, y colapsa los cambios anteriores a esta sesión en una línea en la parte inferior. Haga clic en cualquier línea de recuento para expandirla.
* **Cambie con qué compara el panel**: presione `Ctrl+X B` para cambiar entre los cambios de esta sesión, sus cambios sin confirmar como una lista, y todo desde que su rama se dividió de la rama predeterminada. Claude Code recuerda la opción para cada proyecto.

Para vincular teclas a estas acciones, consulte [Acciones del panel de diff](/docs/es/keybindings#diff-panel-actions).

<h3 id="diff-viewer">
  Visor de diff
</h3>

El visor de diff ocupa el lugar del mensaje hasta que lo cierre. Su vista **Actual** muestra sus cambios sin confirmar de git, o, cuando no hay ninguno, lo que su rama añade en la rama predeterminada. El visor también tiene una vista de turno para cada mensaje después del cual Claude editó archivos, mostrando solo esas ediciones. Claude Code construye las vistas de turno a partir de las ediciones de archivos de Claude en lugar de git, por lo que un cambio que Claude realiza a través de un comando de shell aparece solo bajo Actual.

Use estas teclas en el visor:

* **Izquierda y Derecha**: moverse entre Actual y las vistas de turno.
* **Arriba y Abajo**: seleccionar un archivo.
* **Intro**: abrir el diff del archivo seleccionado. Desplácelo con Arriba y Abajo, o AvPág y RePág.
* **Esc**: volver del diff de un archivo a la lista, o cerrar el visor desde la lista.

Para reasignar estas teclas, consulte [Acciones de diff](/docs/es/keybindings#diff-actions).

<h2 id="side-questions-with-/btw">
  Preguntas laterales con /btw
</h2>

Use `/btw` para hacer una pregunta sobre su trabajo actual sin agregar al historial de conversación.

```
/btw what was the name of that config file again?
```

Claude responde una pregunta lateral a partir de lo que ya está en la conversación: sus mensajes, sus respuestas y los resultados de herramientas que ha recopilado. Puede preguntar sobre código que Claude ya ha leído, decisiones que tomó anteriormente, o cualquier otra cosa de la sesión. Una pregunta lateral posterior también ve sus preguntas laterales anteriores: Claude Code reproduce los 20 intercambios más recientes con cada pregunta, hasta que los borre. La pregunta y la respuesta nunca entran en el historial de conversación. En la terminal, aparecen en una superposición descartable. La terminal mantiene el hilo en memoria: presione `x` para borrar los intercambios anteriores, y desaparece cuando sale de Claude Code.

En el [panel de chat de la extensión de VS Code](/docs/es/vs-code#use-the-prompt-box), `/btw` abre un panel en lugar de la superposición que describe esta sección, y hace preguntas de seguimiento directamente en el panel. El hilo del panel sobrevive a las recargas de ventana, según el cronograma de retención que describe esa página. Necesita la extensión en la versión 2.1.227 o posterior. Las versiones anteriores de la extensión no ofrecen `/btw`.

* **Disponible mientras Claude está trabajando**: puede ejecutar `/btw` incluso mientras Claude está procesando una respuesta. La pregunta lateral se ejecuta de forma independiente y no interrumpe el turno principal. Ve todo en la conversación hasta ahora, excepto la respuesta que Claude aún está escribiendo.
* **Sin acceso a herramientas**: las preguntas laterales responden solo a partir de lo que ya está en contexto. Claude no puede leer archivos, ejecutar comandos o buscar al responder una pregunta lateral. Si Claude escribe llamadas de herramientas como texto de todas formas, la respuesta termina con una nota de que nada fue ejecutado.
* **Respuesta única**: no hay turnos de seguimiento en la superposición. Para continuar el hilo, haga otra pregunta `/btw`. Para continuar con acceso completo a herramientas en una sesión local, presione `f` para bifurcar esta pregunta y respuesta en un [subagente de fondo](/docs/es/sub-agents#fork-the-current-conversation).
* **Bajo costo**: mientras el [almacenamiento en caché de indicaciones](/docs/es/prompt-caching) de la conversación esté activo, una pregunta lateral cuesta poco más allá de la respuesta en sí.

Sus cinco preguntas laterales anteriores más recientes aparecen como una lista atenuada encima de la respuesta actual, con un recuento de las más antiguas. Se mantienen fuera del historial de conversación.

Para volver a la superposición después de descartarla, ejecute `/btw` sin pregunta. La superposición se reabre en su intercambio más reciente. Antes de la versión 2.1.212, `/btw` sin pregunta imprimía un mensaje de uso en su lugar.

Una vez que aparece la respuesta, la superposición acepta estas teclas.

| Tecla                        | Acción                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Space`, `Enter`, `Escape`   | Descartar la respuesta y volver al indicador                                                                                                                                                                                                                                                                                                                                                                                    |
| `Up` / `Down`                | Desplazarse por la respuesta                                                                                                                                                                                                                                                                                                                                                                                                    |
| `Shift+Left` / `Shift+Right` | Pasar entre esta respuesta y sus respuestas anteriores de `/btw`. `Shift+Left` se mueve a respuestas más antiguas y `Shift+Right` regresa hacia la actual. `[` y `]` hacen lo mismo, para terminales que no reportan `Shift` con teclas de flecha. `Tab` / `Shift+Tab` ciclan a través de las mismas respuestas. Requiere Claude Code v2.1.257 o posterior. Entre v2.1.187 y v2.1.256, las teclas eran `Left` / `Right` simples |
| `c`                          | Copiar la respuesta al portapapeles como Markdown sin formato. Use esto en lugar de la selección del ratón, que captura la representación terminal con ajuste duro en lugar del texto fuente                                                                                                                                                                                                                                    |
| `f`                          | Iniciar un [subagente bifurcado](/docs/es/sub-agents#fork-the-current-conversation) que hereda la conversación principal más esta pregunta y respuesta, para que pueda continuar con acceso completo a herramientas. Permanece en la sesión actual y encuentra la bifurcación en el [panel debajo de su indicador](/docs/es/sub-agents#observe-and-steer-running-forks). Disponible solo en sesiones locales                              |
| `x`                          | Borrar la lista de intercambios anteriores de `/btw` mostrados encima de la respuesta actual                                                                                                                                                                                                                                                                                                                                    |

En una [sesión de fondo](/docs/es/agent-view#attach-to-a-session) adjunta, `Left` lo desconecta y lo devuelve a la vista de agente, incluso mientras la respuesta aún está llegando. La pregunta lateral sigue ejecutándose mientras está fuera. La próxima vez que se adjunte a la sesión, la superposición se reabre con la pregunta lateral, o con su respuesta. Antes de v2.1.257, `Left` no se desconectaba allí.

`/btw` ve su conversación completa pero no tiene herramientas. Un [subagente](/docs/es/sub-agents) tiene herramientas y comienza a partir del indicador que recibe, o, para una [bifurcación](/docs/es/sub-agents#fork-the-current-conversation), a partir de una copia de esta conversación. Use `/btw` para preguntar sobre lo que Claude ya sabe de esta sesión; use un subagente para ir a descubrir algo nuevo.

<h2 id="task-list">
  Lista de tareas
</h2>

La lista de tareas es la lista de verificación de Claude: elementos que Claude creó para planificar trabajo de múltiples pasos, con indicadores que muestran qué está pendiente, en progreso o completado. Es independiente de la vista de tarea en segundo plano. Para ver shells en ejecución y subagentes, use [`/tasks`](/docs/es/commands) en su lugar.

La lista se completa solo en sesiones que tienen las herramientas de seguimiento de tareas, que Claude Code proporciona de forma predeterminada en [modelos Claude 3.x, Opus 4 a través de 4.7, Sonnet 4 a través de 4.6 y Haiku 4.5](/docs/es/tools-reference#task-tool-availability). En cualquier otro modelo, incluido un ID de modelo que Claude Code no reconoce, la lista permanece vacía a menos que opte por participar con `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` u una de las otras formas en [Disponibilidad de herramientas de tareas](/docs/es/tools-reference#task-tool-availability). Cuando la sesión tiene las herramientas, la lista de tareas funciona de la siguiente manera:

* Presione `Ctrl+T` para alternar la vista de la lista de tareas. La pantalla muestra hasta cinco tareas a la vez. Cuando Claude aún no ha creado ningún elemento de lista de verificación, el botón de alternancia no tiene efecto visible porque no hay nada que mostrar
* Si deja la lista expandida, Claude Code restaura la vista expandida la próxima vez que inicie una sesión que aún tenga tareas, como con `--resume` o `--continue`. Cuando la lista de tareas está vacía, Claude Code la inicia contraída
* Para ver todas las tareas o borrarlas, pregúntele directamente a Claude: "muéstrame todas las tareas" o "borra todas las tareas"
* Las tareas persisten en las compactaciones de contexto, ayudando a Claude a mantenerse organizado en proyectos más grandes
* Para compartir una lista de tareas entre sesiones, establezca `CLAUDE_CODE_TASK_LIST_ID` para usar un directorio nombrado en `~/.claude/tasks/`: `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Resumen de sesión
</h2>

Cuando regresa a la terminal después de alejarse, Claude Code muestra un resumen de una línea de lo que ha sucedido en la sesión hasta el momento. El resumen se genera en segundo plano una vez que han pasado al menos tres minutos desde el último turno completado y la terminal no está enfocada, por lo que está listo cuando vuelve a cambiar. Los resúmenes solo aparecen una vez que la sesión tiene al menos tres turnos, y nunca dos seguidas.

Ejecute `/recap` para generar un resumen bajo demanda. Claude Code limita tanto los resúmenes automáticos como la salida de `/recap` a 400 caracteres. Para desactivar los resúmenes automáticos, abra `/config` y desactive **Resumen de sesión**.

El resumen de sesión está activado de forma predeterminada para todos los planes y proveedores. El resumen siempre se omite en modo no interactivo.

<h2 id="wait-for-a-usage-limit-to-reset">
  Esperar a que se reinicie un límite de uso
</h2>

Cuando un [límite de uso](/docs/es/errors#youve-hit-your-session-limit) de claude.ai detiene a Claude a mitad de una tarea, Claude Code espera en la sesión abierta y continúa la tarea automáticamente después de que se reinicie el límite. La continuación automática está activada de forma predeterminada en sesiones interactivas con sesión iniciada con una suscripción de claude.ai. Requiere Claude Code v2.1.234 o posterior.

Mientras Claude Code espera, una línea en la parte inferior de la sesión muestra cuándo continuará:

```text theme={null}
Usage limit reached · continuing automatically at 3:45pm · esc to cancel
```

Mantenga la sesión abierta. Lo que sucede a continuación depende de cómo termine la espera:

* **En el reinicio**: la línea lee `continuing shortly`, luego `Usage limit reset · continuing automatically`, y Claude Code envía a Claude un mensaje fijo para retomar la tarea donde se detuvo. No reenvía su último mensaje.
* **Después de que su computadora se durmió**: si se durmió durante más de aproximadamente 30 minutos y el límite se reinició mientras dormía, la línea lee `Your usage limit has reset · press enter to continue`. Presione `Enter` para continuar. Después de un sueño más corto, Claude Code continúa automáticamente.
* **Temprano**: cuando termina de agregar [créditos de uso](/docs/es/costs#add-usage-credits-to-your-subscription) con `/usage-credits`, inicia sesión nuevamente después de `/upgrade`, o cambia modelos con `/model` durante la espera, Claude Code verifica si el uso está disponible nuevamente y continúa de inmediato si es así. No verifica después de una actualización o compra que realice en un navegador por su cuenta. En [`opusplan`](/docs/es/model-config#opusplan-model-setting) y otras configuraciones de modelo que ejecutan el modo de plan en un modelo diferente, Claude Code espera el reinicio en su lugar.

La tarea continuada se ejecuta como cualquier otro turno. Claude Code aún solicita [permisos](/docs/es/permissions) como de costumbre, por lo que la tarea puede detenerse en un mensaje mientras está fuera. Si vuelve a alcanzar el límite, Claude Code reactiva la espera automáticamente como máximo dos veces seguidas, luego se detiene y muestra `Automatic continue stopped after repeated usage-limit hits · /rate-limit-options to try again`.

<h3 id="cancel-the-wait">
  Cancelar la espera
</h3>

Presione `Esc` en un mensaje vacío, o `Ctrl+C`, mientras se muestra la línea, o ejecute [`/rate-limit-options`](/docs/es/commands#all-commands) y seleccione **Don't continue automatically**. Claude Code confirma con una línea que comienza con `Automatic continue cancelled`.

Después de una cancelación, nada continúa hasta que envíe un mensaje o seleccione la fila que comienza con **Wait here, then continue automatically** de `/rate-limit-options` nuevamente. Claude Code no inicia una espera por su cuenta nuevamente para esa ventana de reinicio; la siguiente ventana de reinicio comienza de nuevo.

La espera también termina sin continuar la tarea en estos casos:

* **Envía un mensaje**: Claude Code ejecuta su mensaje en lugar de esperar.
* **Sale de Claude Code**: la espera no se reinicia cuando reanuda la sesión.
* **La conversación cambia de manos**: cambia de cuenta con `/login`, borra o rebobina la conversación, `/resume` otra sesión, extrae una con `/teleport`, relanza con `/tui`, o entrega la sesión a Claude Desktop, una sesión de fondo o la nube.
* **La configuración se desactiva, o el reinicio se mueve más allá de 24 horas**: esto solo termina una espera que Claude Code inició por su cuenta. Una espera que seleccionó de `/rate-limit-options` sigue contando hacia atrás.
* **La continuación está bloqueada**: un hook [`UserPromptSubmit`](/docs/es/hooks#userpromptsubmit) que bloquea el mensaje de continuación, o una falla antes de que llegue al modelo, termina la espera. Claude Code le indica que la continuación no se ejecutó. Envíe un mensaje para continuar.

<h3 id="start-a-wait-yourself">
  Inicie una espera usted mismo
</h3>

Claude Code no inicia la espera por su cuenta en estos casos:

* **Sesiones de Control Remoto y compañero de equipo de agentes**: una persona en esa terminal aún puede iniciar una.
* **Un reinicio a más de 24 horas de distancia**: un límite semanal puede reiniciarse días después.
* **Un límite de Opus o Sonnet mientras ejecuta un modelo fuera de esa familia**: su próximo turno puede no alcanzar ese límite. [`opusplan`](/docs/es/model-config#opusplan-model-setting) y otras configuraciones de modelo que ejecutan el modo de plan en la familia limitada no obtienen esta excepción.

En esos casos, y siempre que la continuación automática esté desactivada, Claude Code abre el menú de opciones de límite de uso una vez por ventana de reinicio cuando alcanza un límite en su propia terminal. Seleccione la fila que comienza con **Wait here, then continue automatically** para iniciar la espera. En una sesión de [Control Remoto](/docs/es/remote-control) o compañero de [equipo de agentes](/docs/es/agent-teams), ejecute `/rate-limit-options` usted mismo para abrir el menú.

Claude Code no ofrece la espera en absoluto en estos casos:

* **Sesiones de fondo y ejecuciones `-p`**: la fila del menú no está disponible.
* **Claves API, proveedores en la nube y facturación basada en el uso**: el uso allí se mide por solicitud, por lo que no hay reinicio para esperar.
* **Un [gateway LLM](/docs/es/llm-gateway#subscriptions-and-gateways) sin un inicio de sesión de claude.ai guardado**: Claude Code ofrece la espera solo mientras un inicio de sesión de claude.ai guardado es la credencial activa.

<h3 id="turn-automatic-continue-off">
  Desactivar la continuación automática
</h3>

En `/config`, desactive **Continue automatically at usage limit**, o establezca [`autoContinueAtUsageLimit`](/docs/es/settings-reference#autocontinueatusagelimit) en `false` en su configuración de usuario. `/config autoContinueAtUsageLimit=false` también funciona, incluso con `-p`, pero la forma `key=value` no puede volver a activarlo, porque la configuración otorga ejecución desatendida. Qué archivos de configuración lee Claude Code para esta clave se encuentra en la [referencia de configuración](/docs/es/settings-reference#autocontinueatusagelimit).

<h2 id="pr-review-status">
  Estado de revisión de PR
</h2>

Cuando trabaja en una rama con una solicitud de extracción abierta, Claude Code muestra un enlace de PR en el pie de página, como "PR #446". El enlace tiene un subrayado de color que indica el estado de revisión:

* Verde: aprobado
* Amarillo: revisión pendiente
* Rojo: cambios solicitados
* Gris: borrador

La insignia desaparece una vez que la solicitud de extracción se fusiona o se cierra.

`Cmd+click` (macOS) o `Ctrl+click` (Windows/Linux) en el enlace para abrir la solicitud de extracción en su navegador.

El estado se actualiza tan pronto como un `git push`, o un comando `gh pr` que cambie la solicitud de extracción, como `gh pr create` o `gh pr merge`, se ejecuta correctamente en la sesión.

Claude Code representa la insignia como un hipervínculo incluso cuando no puede detectar soporte de hipervínculo en su terminal, lo que comúnmente ocurre a través de SSH o en tmux. Establezca [`FORCE_HYPERLINK=0`](/docs/es/env-vars) para representar la insignia como texto sin formato.

Cuando establece [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/es/env-vars), Claude Code no verifica el estado de la solicitud de extracción o solicitud de fusión.

<Note>
  El estado de PR para repositorios de GitHub necesita un token de GitHub. Claude Code encuentra uno basado en el host remoto:

  * **github.com**: `GH_TOKEN` o `GITHUB_TOKEN`, o el token guardado por `gh auth login`. Sin uno, el pie de página muestra `install gh for PR status` cuando la CLI `gh` no está instalada, o `gh auth login for PR status` cuando lo está
  * **Un host de GitHub Enterprise establecido como `GH_HOST`**: `GH_ENTERPRISE_TOKEN` o `GITHUB_ENTERPRISE_TOKEN`, o el token guardado por `gh auth login --hostname <host>`. Sin uno, el pie de página muestra las mismas sugerencias
  * **Cualquier otro host de GitHub**: el token guardado por `gh auth login --hostname <host>`. Sin uno, Claude Code no muestra ninguna insignia ni sugerencia
</Note>

<h3 id="gitlab-merge-requests">
  Solicitudes de fusión de GitLab
</h3>

Cuando trabaja en una rama con una solicitud de fusión abierta de GitLab, Claude Code muestra una insignia `MR !N` en el espacio del pie de página que de otro modo contendría el enlace de PR de GitHub. `!N` es la propia sintaxis de referencia de GitLab para el número de solicitud de fusión N. El subrayado de color muestra el estado de la solicitud de fusión:

* Verde: GitLab reporta que la solicitud de fusión es fusionable
* Amarillo: cualquier otro estado abierto
* Gris: borrador

La insignia desaparece una vez que la solicitud de fusión se fusiona o se cierra.

Se actualiza tan pronto como un `git push`, o un comando `glab mr` que cambie la solicitud de fusión, como `glab mr create` o `glab mr merge`, se ejecuta correctamente en la sesión.

Para obtener la insignia, necesita:

* Claude Code v2.1.234 o posterior
* Un repositorio remoto que apunte a su host de GitLab, ya sea gitlab.com o una instancia autogestionada
* La [CLI `glab`](https://gitlab.com/gitlab-org/cli) en su `PATH`, autenticada con `glab auth login`

Claude Code ignora las variables de entorno del token de `glab`, como `GITLAB_TOKEN`, cuando verifica el estado, por lo que no obtiene ninguna insignia de un token exportado solo. Claude Code también busca `glab` y su inicio de sesión una vez por sesión, por lo que reinicie Claude Code después de instalar `glab` o ejecutar `glab auth login`.

<h2 id="issue-reference-links">
  Enlaces de referencia de problemas
</h2>

Cuando Claude menciona un problema como `owner/repo#123`, puede hacer clic en la referencia para abrirlo, siempre que su terminal admita hipervínculos. Si Claude Code no detecta compatibilidad con hipervínculos en su terminal, establezca [`FORCE_HYPERLINK`](/docs/es/env-vars) en `1` para activar los enlaces, o en `0` para mantener las referencias como texto sin formato.

Solo obtiene un enlace para el formulario de dos partes `owner/repo#123`. Estos permanecen como texto sin formato:

* Un `#123` desnudo
* Una ruta anidada de GitLab como `group/subgroup/project#123`
* Cualquier referencia dentro de un intervalo de código o bloque de código

Claude Code construye el enlace para el host del repositorio que identifica desde su remoto de git, no para el repositorio que nombra la referencia:

| Host del repositorio                                                                 | Dónde `owner/repo#123` enlaza                              |
| :----------------------------------------------------------------------------------- | :--------------------------------------------------------- |
| github.com, un host de GitHub Enterprise, o cualquier host no listado a continuación | `https://<host>/owner/repo/issues/123`                     |
| gitlab.com                                                                           | `https://gitlab.com/owner/repo/-/issues/123`               |
| bitbucket.org, codeberg.org, o gitea.com                                             | Sin enlace; la referencia permanece como texto sin formato |

<h2 id="see-also">
  Ver también
</h2>

* [Skills](/docs/es/skills) - Indicaciones personalizadas y flujos de trabajo
* [Checkpointing](/docs/es/checkpointing) - Rebobinar las ediciones de Claude y restaurar estados anteriores
* [Referencia de CLI](/docs/es/cli-reference) - Banderas y opciones de línea de comandos
* [Configuración](/docs/es/settings) - Opciones de configuración
* [Gestión de memoria](/docs/es/memory) - Gestión de archivos CLAUDE.md
