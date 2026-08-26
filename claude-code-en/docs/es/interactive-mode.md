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

  **Usuarios de macOS**: Los atajos de la tecla Option/Alt (`Alt+B`, `Alt+F`, `Alt+Y`, `Alt+M`, `Alt+P`) requieren configurar Option como Meta en su terminal:

  * **iTerm2**: Configuración → Perfiles → Teclas → General → establecer la tecla Option izquierda/derecha en "Esc+"
  * **Terminal de Apple**: Configuración → Perfiles → Teclado → marcar "Usar Option como tecla Meta"
  * **VS Code**: establecer `"terminal.integrated.macOptionIsMeta": true` en la configuración de VS Code

  Consulte [Configuración de terminal](/docs/es/terminal-config) para obtener más detalles.
</Note>

<h3 id="general-controls">
  Controles generales
</h3>

| Atajo                                                 | Descripción                                                                                                                                                                     | Contexto                                                                                                                                                                                                                                                                                                                                                              |
| :---------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+C`                                              | Interrumpir, o borrar entrada                                                                                                                                                   | Interrumpe una operación en ejecución. Si nada se está ejecutando, la primera pulsación borra la entrada del indicador y una segunda pulsación sale de Claude Code                                                                                                                                                                                                    |
| `Ctrl+X Ctrl+K`                                       | Terminar todos los [subagentes en ejecución de fondo](/docs/es/sub-agents#run-subagents-in-foreground-or-background) en esta sesión. Presione dos veces en 3 segundos para confirmar | Control de subagentes                                                                                                                                                                                                                                                                                                                                                 |
| `Ctrl+D`                                              | Salir de la sesión de Claude Code                                                                                                                                               | Señal EOF                                                                                                                                                                                                                                                                                                                                                             |
| `Ctrl+G` o `Ctrl+X Ctrl+E`                            | Abrir en el editor de texto predeterminado                                                                                                                                      | Edite su indicación o respuesta personalizada en su editor de texto predeterminado. `Ctrl+X Ctrl+E` es el enlace nativo de readline. Active Mostrar última respuesta en editor externo en `/config` para anteponer la respuesta anterior de Claude como contexto comentado con `#` encima de su indicación; el bloque de comentarios se elimina cuando guarda         |
| `Ctrl+L`                                              | Redibujar pantalla                                                                                                                                                              | Fuerza un redibujado completo de la terminal. La entrada y el historial de conversación se mantienen. Use esto para recuperarse si la pantalla se vuelve distorsionada o parcialmente en blanco                                                                                                                                                                       |
| `Ctrl+O`                                              | Alternar visor de transcripción                                                                                                                                                 | Muestra el uso y la ejecución detallada de herramientas, con una marca de tiempo y el modelo utilizado en cada mensaje del asistente. También expande las llamadas de MCP, que se contraen a una sola línea como "Llamó a slack 3 veces" de forma predeterminada                                                                                                      |
| `Ctrl+R`                                              | Búsqueda inversa del historial de comandos                                                                                                                                      | Buscar a través de comandos anteriores de forma interactiva                                                                                                                                                                                                                                                                                                           |
| `Ctrl+V` o `Cmd+V` (iTerm2) o `Alt+V` (Windows y WSL) | Pegar imagen desde el portapapeles                                                                                                                                              | Inserta un chip `[Image #N]` en el cursor para que pueda hacer referencia a él posicionalmente en su indicación. En WSL, tanto `Ctrl+V` como `Alt+V` están vinculados; use `Alt+V` si su terminal intercepta `Ctrl+V`                                                                                                                                                 |
| `Ctrl+B`                                              | Tareas en ejecución de fondo                                                                                                                                                    | Coloca comandos Bash y agentes en segundo plano. Los usuarios de Tmux presionan dos veces                                                                                                                                                                                                                                                                             |
| `Ctrl+T`                                              | Alternar lista de tareas de Claude                                                                                                                                              | Mostrar u ocultar [la lista de tareas de Claude](#task-list) en el área de estado. Esto no es la vista de tareas en segundo plano; use [`/tasks`](/docs/es/commands) para ver shells y subagentes en ejecución                                                                                                                                                             |
| `Flechas izquierda/derecha`                           | Ciclar a través de pestañas de diálogo                                                                                                                                          | Navegar entre pestañas en diálogos de permisos y menús                                                                                                                                                                                                                                                                                                                |
| `Flechas arriba/abajo` o `Ctrl+P`/`Ctrl+N`            | Mover cursor o navegar por el historial de comandos                                                                                                                             | Cuando la entrada abarca más de una fila visual, ya sea envuelta o multilínea, primero mueve el cursor dentro de la indicación. Una vez que el cursor está en la primera o última fila visual, presionar nuevamente navega por el historial de comandos. A partir de v2.1.169, la entrada de una sola línea envuelta se comporta igual que la multilínea              |
| `Esc`                                                 | Interrumpir Claude, o cerrar un diálogo                                                                                                                                         | Detener la respuesta actual o la llamada de herramienta a mitad de turno para que pueda redirigir. Claude mantiene el trabajo realizado hasta ahora. Cuando un diálogo como un indicador de permiso está abierto, `Esc` cierra el diálogo en lugar de interrumpir Claude. Antes de v2.1.202, `Esc` en algunos diálogos interrumpía Claude y dejaba el diálogo abierto |
| `Esc` + `Esc`                                         | Borrar borrador de entrada, o rebobinar                                                                                                                                         | Cuando la entrada del indicador contiene texto, doble `Esc` lo borra y guarda el borrador en el historial para que `Arriba` lo recupere. Cuando la entrada está vacía, doble `Esc` abre el [menú de rebobinado](/docs/es/checkpointing) para restaurar o resumir código y conversación desde un punto anterior                                                             |
| `Shift+Tab` o `Alt+M` (algunas configuraciones)       | Ciclar modos de permiso                                                                                                                                                         | Ciclar a través de `default` (etiquetado como Manual en el indicador de modo), `acceptEdits`, `plan` y cualquier modo que haya habilitado, como `auto` o `bypassPermissions`. Consulte [modos de permiso](/docs/es/permission-modes).                                                                                                                                      |
| `Option+P` (macOS) o `Alt+P` (Windows/Linux)          | Cambiar modelo                                                                                                                                                                  | Cambiar modelos sin borrar su indicación                                                                                                                                                                                                                                                                                                                              |
| `Option+T` (macOS) o `Alt+T` (Windows/Linux)          | Alternar pensamiento extendido                                                                                                                                                  | Habilitar o deshabilitar el modo de pensamiento extendido. No tiene efecto en Fable 5, que siempre utiliza pensamiento extendido. A partir de v2.1.132 este atajo funciona en macOS sin configurar Option como Meta                                                                                                                                                   |
| `Option+O` (macOS) o `Alt+O` (Windows/Linux)          | Alternar modo rápido                                                                                                                                                            | Habilitar o deshabilitar [modo rápido](/docs/es/fast-mode)                                                                                                                                                                                                                                                                                                                 |

<h3 id="text-editing">
  Edición de texto
</h3>

| Atajo                         | Descripción                                          | Contexto                                                                                                                                                                                                           |
| :---------------------------- | :--------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+A`                      | Mover cursor al inicio de la línea actual            | En entrada multilínea, mueve al inicio de la línea lógica actual                                                                                                                                                   |
| `Ctrl+E`                      | Mover cursor al final de la línea actual             | En entrada multilínea, mueve al final de la línea lógica actual                                                                                                                                                    |
| `Ctrl+K`                      | Eliminar hasta el final de la línea                  | Almacena el texto eliminado para pegarlo                                                                                                                                                                           |
| `Ctrl+U`                      | Eliminar desde el cursor hasta el inicio de la línea | Almacena el texto eliminado para pegarlo. Repita para borrar en múltiples líneas en entrada multilínea. En macOS, los emuladores de terminal incluyendo iTerm2 y Terminal.app asignan `Cmd+Backspace` a este atajo |
| `Ctrl+W`                      | Eliminar palabra anterior                            | Almacena el texto eliminado para pegarlo. En Windows, `Ctrl+Backspace` también elimina la palabra anterior                                                                                                         |
| `Ctrl+Y`                      | Pegar texto eliminado                                | Pegar texto eliminado con `Ctrl+K`, `Ctrl+U` o `Ctrl+W`                                                                                                                                                            |
| `Alt+Y` (después de `Ctrl+Y`) | Ciclar historial de pegado                           | Después de pegar, ciclar a través del texto eliminado anteriormente. Requiere [Option como Meta](#keyboard-shortcuts) en macOS                                                                                     |
| `Alt+B`                       | Mover cursor una palabra hacia atrás                 | Navegación de palabras. Requiere [Option como Meta](#keyboard-shortcuts) en macOS                                                                                                                                  |
| `Alt+F`                       | Mover cursor una palabra hacia adelante              | Navegación de palabras. Requiere [Option como Meta](#keyboard-shortcuts) en macOS                                                                                                                                  |

<h3 id="theme-and-display">
  Tema y visualización
</h3>

| Atajo    | Descripción                                           | Contexto                                                                                                                       |
| :------- | :---------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+T` | Alternar resaltado de sintaxis para bloques de código | Solo funciona dentro del menú del selector `/theme`. Controla si el código en las respuestas de Claude usa colores de sintaxis |

<h3 id="multiline-input">
  Entrada multilínea
</h3>

| Método               | Atajo              | Contexto                                                                                                   |
| :------------------- | :----------------- | :--------------------------------------------------------------------------------------------------------- |
| Escape rápido        | `\` + `Enter`      | Funciona en todas las terminales                                                                           |
| Tecla Option         | `Option+Enter`     | Después de habilitar [Option como Meta](/docs/es/terminal-config#enable-option-key-shortcuts-on-macos) en macOS |
| Shift+Enter          | `Shift+Enter`      | Nativo en iTerm2, WezTerm, Ghostty, Kitty, Warp, Terminal de Apple, Windows Terminal                       |
| Secuencia de control | `Ctrl+J`           | Funciona en cualquier terminal sin configuración                                                           |
| Modo de pegado       | Pegar directamente | Para bloques de código, registros                                                                          |

<Tip>
  Shift+Enter funciona sin configuración en iTerm2, WezTerm, Ghostty, Kitty, Warp, Terminal de Apple y Windows Terminal. Para VS Code, Cursor, Devin Desktop, Alacritty y Zed, ejecute `/terminal-setup` para instalar el enlace.
</Tip>

<h3 id="quick-commands">
  Comandos rápidos
</h3>

| Atajo         | Descripción                | Notas                                                                                              |
| :------------ | :------------------------- | :------------------------------------------------------------------------------------------------- |
| `/` al inicio | Comando o skill            | Consulte [comandos](#commands) y [skills](/docs/es/skills)                                              |
| `!` al inicio | Modo Bash                  | Ejecutar un comando directamente, agregar su salida a la sesión y hacer que Claude responda a ella |
| `@`           | Mención de ruta de archivo | Activar autocompletado de ruta de archivo                                                          |

<h3 id="transcript-viewer">
  Visor de transcripción
</h3>

Cuando el visor de transcripción está abierto (alternado con `Ctrl+O`), estos atajos están disponibles. En [renderizado a pantalla completa](/docs/es/fullscreen), presione `?` para mostrar el panel de referencia de atajos de teclado completo dentro del visor. `Ctrl+E` se puede reasignar a través de [`transcript:toggleShowAll`](/docs/es/keybindings).

| Atajo                | Descripción                                                                                                                                                                                                                                                     |
| :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Alternar el panel de ayuda de atajos de teclado. Requiere [renderizado a pantalla completa](/docs/es/fullscreen)                                                                                                                                                     |
| `{` / `}`            | Saltar al indicador de usuario anterior o siguiente, como el movimiento de párrafo de vim. Requiere [renderizado a pantalla completa](/docs/es/fullscreen)                                                                                                           |
| `Ctrl+E`             | Alternar mostrar todo el contenido                                                                                                                                                                                                                              |
| `[`                  | Escribir la conversación completa en el scrollback nativo de su terminal para que `Cmd+F`, el modo de copia de tmux y otras herramientas nativas puedan buscarla. Requiere [renderizado a pantalla completa](/docs/es/fullscreen#search-and-review-the-conversation) |
| `v`                  | Escribir la conversación en un archivo temporal y abrirlo en `$VISUAL` o `$EDITOR`. Requiere [renderizado a pantalla completa](/docs/es/fullscreen)                                                                                                                  |
| `q`, `Ctrl+C`, `Esc` | Salir de la vista de transcripción. Los tres se pueden reasignar a través de [`transcript:exit`](/docs/es/keybindings)                                                                                                                                               |

<h3 id="voice-input">
  Entrada de voz
</h3>

| Atajo                                  | Descripción    | Notas                                                                                                                                                                                                             |
| :------------------------------------- | :------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Mantener presionado o pulsar `Espacio` | Dictado de voz | Requiere que [dictado de voz](/docs/es/voice-dictation) esté habilitado. Mantenga presionado para grabar, o ejecute `/voice tap` para alternar con pulsar. [Reasignable](/docs/es/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Comandos
</h2>

Escriba `/` en Claude Code para ver todos los comandos disponibles, o escriba `/` seguido de cualquier letra para filtrar. El menú `/` muestra todo lo que puede invocar: comandos integrados, [skills](/docs/es/skills) incluidos y creados por el usuario, y comandos contribuidos por [plugins](/docs/es/plugins) y [servidores MCP](/docs/es/mcp#use-mcp-prompts-as-commands). No todos los comandos integrados son visibles para todos los usuarios ya que algunos dependen de su plataforma o plan.

En [renderizado a pantalla completa](/docs/es/fullscreen#use-the-mouse), el comando `/` y las listas de sugerencias de archivos `@` también responden al ratón: pasar el cursor destaca una fila y hacer clic la acepta.

Consulte la [referencia de comandos](/docs/es/commands) para obtener la lista completa de comandos incluidos en Claude Code.

<h2 id="vim-editor-mode">
  Modo editor Vim
</h2>

Habilite la edición de estilo vim a través de `/config` → Editor mode.

<h3 id="mode-switching">
  Cambio de modo
</h3>

| Comando | Acción                                         | Desde el modo  |
| :------ | :--------------------------------------------- | :------------- |
| `Esc`   | Entrar en modo NORMAL                          | INSERT, VISUAL |
| `i`     | Insertar antes del cursor                      | NORMAL         |
| `I`     | Insertar al principio de la línea              | NORMAL         |
| `a`     | Insertar después del cursor                    | NORMAL         |
| `A`     | Insertar al final de la línea                  | NORMAL         |
| `o`     | Abrir línea debajo                             | NORMAL         |
| `O`     | Abrir línea arriba                             | NORMAL         |
| `v`     | Iniciar selección visual carácter por carácter | NORMAL         |
| `V`     | Iniciar selección visual línea por línea       | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  Remapear secuencias de teclas en modo INSERT
</h3>

La configuración [`vimInsertModeRemaps`](/docs/es/settings#available-settings) mapea una secuencia de dos teclas en modo INSERT a Escape, por lo que un mapeo como `jj` lo devuelve al modo NORMAL. Requiere Claude Code v2.1.208 o posterior.

El siguiente ejemplo de `~/.claude/settings.json` activa el modo vim y mapea `jj` a Escape:

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Cada clave es exactamente dos caracteres imprimibles escritos en secuencia, y `"<Esc>"` es el único destino compatible. Las entradas con una longitud o destino diferente se ignoran.

Escribir el primer carácter de una secuencia lo inserta normalmente. Presionar el segundo carácter dentro de un segundo elimina ese carácter pendiente y cambia al modo NORMAL, sin dejar ningún carácter en su entrada. Después de la ventana de un segundo, o si una tecla diferente sigue, ambos caracteres permanecen como texto literal, por lo que aún puede escribir una palabra que contenga la secuencia haciendo una pausa entre las dos teclas.

Claude Code lee esta configuración desde su archivo de configuración de usuario, la bandera `--settings` y [configuración administrada](/docs/es/permissions#managed-settings) solamente. Las entradas en el archivo `.claude/settings.json` o `.claude/settings.local.json` de un proyecto se ignoran, por lo que un repositorio extraído no puede remapear sus pulsaciones de teclas.

<h3 id="navigation-normal-mode">
  Navegación (modo NORMAL)
</h3>

| Comando         | Acción                                                                                                                                                                                                                        |
| :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `h`/`j`/`k`/`l` | Mover izquierda/abajo/arriba/derecha                                                                                                                                                                                          |
| `Space`         | Mover a la derecha                                                                                                                                                                                                            |
| `w`             | Siguiente palabra                                                                                                                                                                                                             |
| `e`             | Final de palabra                                                                                                                                                                                                              |
| `b`             | Palabra anterior                                                                                                                                                                                                              |
| `0`             | Principio de línea                                                                                                                                                                                                            |
| `$`             | Final de línea                                                                                                                                                                                                                |
| `^`             | Primer carácter no en blanco                                                                                                                                                                                                  |
| `gg`            | Principio de entrada                                                                                                                                                                                                          |
| `G`             | Final de entrada                                                                                                                                                                                                              |
| `f{char}`       | Saltar a la siguiente ocurrencia del carácter                                                                                                                                                                                 |
| `F{char}`       | Saltar a la ocurrencia anterior del carácter                                                                                                                                                                                  |
| `t{char}`       | Saltar justo antes de la siguiente ocurrencia del carácter                                                                                                                                                                    |
| `T{char}`       | Saltar justo después de la ocurrencia anterior del carácter                                                                                                                                                                   |
| `;`             | Repetir último movimiento f/F/t/T                                                                                                                                                                                             |
| `,`             | Repetir último movimiento f/F/t/T en orden inverso                                                                                                                                                                            |
| `/`             | Abrir búsqueda de historial inverso, igual que `Ctrl+R`. A partir de v2.1.191, el símbolo del sistema de búsqueda vacío muestra una sugerencia: presione `Esc` luego `i` luego `/` para abrir el menú de comandos en su lugar |

<Note>
  En modo normal de vim, si el cursor está al principio o al final de la entrada y no puede moverse más, `j`/`k` y las teclas de flecha navegan por el historial de comandos en su lugar.
</Note>

<h3 id="editing-normal-mode">
  Edición (modo NORMAL)
</h3>

| Comando        | Acción                                      |
| :------------- | :------------------------------------------ |
| `x`            | Eliminar carácter                           |
| `dd`           | Eliminar línea                              |
| `D`            | Eliminar hasta el final de la línea         |
| `dw`/`de`/`db` | Eliminar palabra/hasta el final/hacia atrás |
| `cc`           | Cambiar línea                               |
| `C`            | Cambiar hasta el final de la línea          |
| `cw`/`ce`/`cb` | Cambiar palabra/hasta el final/hacia atrás  |
| `yy`/`Y`       | Yanquear (copiar) línea                     |
| `yw`/`ye`/`yb` | Yanquear palabra/hasta el final/hacia atrás |
| `p`            | Pegar después del cursor                    |
| `P`            | Pegar antes del cursor                      |
| `>>`           | Indentar línea                              |
| `<<`           | Desindentación de línea                     |
| `J`            | Unir líneas                                 |
| `u`            | Deshacer                                    |
| `.`            | Repetir último cambio                       |

<h3 id="text-objects-normal-mode">
  Objetos de texto (modo NORMAL)
</h3>

Los objetos de texto funcionan con operadores como `d`, `c` e `y`:

| Comando   | Acción                                                         |
| :-------- | :------------------------------------------------------------- |
| `iw`/`aw` | Palabra interior/alrededor                                     |
| `iW`/`aW` | PALABRA interior/alrededor (delimitada por espacios en blanco) |
| `i"`/`a"` | Comillas dobles interior/alrededor                             |
| `i'`/`a'` | Comillas simples interior/alrededor                            |
| `i(`/`a(` | Paréntesis interior/alrededor                                  |
| `i[`/`a[` | Corchetes interior/alrededor                                   |
| `i{`/`a{` | Llaves interior/alrededor                                      |

<h3 id="visual-mode">
  Modo visual
</h3>

Presione `v` para selección carácter por carácter o `V` para selección línea por línea. Los movimientos extienden la selección, y los operadores actúan sobre ella directamente.

| Comando          | Acción                                                          |
| :--------------- | :-------------------------------------------------------------- |
| `d`/`x`          | Eliminar selección                                              |
| `y`              | Yanquear selección                                              |
| `c`/`s`          | Cambiar selección                                               |
| `p`              | Reemplazar selección con contenido del registro                 |
| `r{char}`        | Reemplazar cada carácter seleccionado con `{char}`              |
| `~`/`u`/`U`      | Alternar, minúsculas o mayúsculas de selección                  |
| `>`/`<`          | Indentar o desindentación de líneas seleccionadas               |
| `J`              | Unir líneas seleccionadas                                       |
| `o`              | Intercambiar cursor y ancla                                     |
| `iw`/`aw`/`i"`/… | Seleccionar un objeto de texto                                  |
| `v`/`V`          | Alternar entre carácter por carácter y línea por línea, o salir |

El modo visual por bloques con `Ctrl+V` no es compatible.

<h2 id="command-history">
  Historial de comandos
</h2>

Claude Code mantiene el historial de comandos para la sesión actual:

* El historial de entrada se almacena por directorio de trabajo
* El historial de entrada se reinicia cuando ejecuta `/clear` para iniciar una nueva sesión. La conversación de la sesión anterior se conserva y se puede reanudar.
* Enviar el mismo indicador dos veces seguidas registra una entrada de historial, por lo que presionar Arriba va al indicador anterior distinto
* Use las flechas Arriba/Abajo para navegar (consulte los atajos de teclado anteriores)
* La expansión del historial con `!` está deshabilitada de forma predeterminada

<h3 id="reverse-search-with-ctrl-r">
  Búsqueda inversa con Ctrl+R
</h3>

Presione `Ctrl+R` para buscar de forma interactiva a través de su historial de comandos:

1. **Iniciar búsqueda**: presione `Ctrl+R` para activar la búsqueda de historial inverso
2. **Escribir consulta**: ingrese texto para buscar en comandos anteriores. El término de búsqueda se resalta en los resultados coincidentes
3. **Navegar coincidencias**: presione `Ctrl+R` nuevamente para ciclar a través de coincidencias más antiguas
4. **Cambiar alcance**: la búsqueda se establece de forma predeterminada en indicaciones de todos los proyectos. Presione `Ctrl+S` para ciclar el alcance entre esta sesión, este proyecto y todos los proyectos
5. **Aceptar coincidencia**:
   * Presione `Tab` o `Esc` para aceptar la coincidencia actual y continuar editando
   * Presione `Enter` para aceptar y ejecutar el comando inmediatamente
6. **Cancelar búsqueda**:
   * Presione `Ctrl+C` para cancelar y restaurar su entrada original
   * Presione `Backspace` en búsqueda vacía para cancelar

La búsqueda carga los 100 indicadores únicos más recientes en el alcance seleccionado, con duplicados contraídos a la ocurrencia más reciente. Los indicadores coincidentes se muestran con el término de búsqueda resaltado, para que pueda encontrar y reutilizar entradas anteriores.

Aceptar una coincidencia o cancelar la búsqueda surte efecto inmediatamente, incluso mientras Claude Code aún está cargando el historial. Antes de v2.1.202, aceptar o cancelar durante esa carga podría informar un error interno.

<h2 id="background-bash-commands">
  Comandos bash en segundo plano
</h2>

Claude Code admite la ejecución de comandos bash en segundo plano, lo que le permite continuar trabajando mientras se ejecutan procesos de larga duración.

<h3 id="how-backgrounding-works">
  Cómo funciona el envío a segundo plano
</h3>

Cuando Claude Code ejecuta un comando en segundo plano, ejecuta el comando de forma asincrónica e inmediatamente devuelve un ID de tarea de fondo. Claude Code puede responder a nuevas indicaciones mientras el comando continúa ejecutándose en segundo plano.

Para ejecutar comandos en segundo plano, puede:

* Indicar a Claude Code que ejecute un comando en segundo plano
* Presione `Ctrl+B` para mover una invocación regular de herramienta Bash al segundo plano. Los usuarios de Tmux deben presionar `Ctrl+B` dos veces debido a la tecla de prefijo de tmux.

**Características clave:**

* La salida se escribe en un archivo y Claude puede recuperarla usando la herramienta Read
* Las tareas de fondo tienen ID únicos para el seguimiento y la recuperación de salida
* Las tareas de fondo se limpian automáticamente cuando Claude Code sale. Enviar la sesión a segundo plano en lugar de salir las entrega a la sesión de fondo, donde continúan ejecutándose. Consulte [enviar una sesión en ejecución a segundo plano](/docs/es/agent-view#from-inside-a-session)
* Las tareas de fondo se terminan automáticamente si la salida excede 5GB, con una nota en stderr explicando por qué
* A partir de v2.1.193, en macOS y Linux, las tareas de fondo en ejecución se terminan cuando el sistema operativo señala presión de memoria, siempre que la sesión haya estado inactiva durante al menos 30 minutos sin ningún turno o subagenteejecutándose. Establezca [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/es/env-vars) en `1` para desactivar esto

Para deshabilitar toda la funcionalidad de tareas de fondo, establezca la variable de entorno `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` en `1`. Consulte [Variables de entorno](/docs/es/env-vars) para obtener más detalles.

**Comandos comúnmente enviados a segundo plano:**

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
* Admite el mismo envío a segundo plano `Ctrl+B` para comandos de larga duración
* No requiere que Claude interprete o apruebe el comando
* Admite autocompletado basado en historial: escriba un comando parcial y presione `Tab` para completar desde comandos `!` anteriores en el proyecto actual
* Admite autocompletado de ruta de archivo en vivo a partir de v2.1.193 en todas las plataformas: escriba un token que contenga una barra diagonal, como `./src/` o `~/`, para ver una lista desplegable de archivos y directorios coincidentes, luego presione `Tab` para aceptar. Use barras diagonales en Windows también; la lista desplegable se activa con `/`, no con `\`
* Salir con `Escape`, `Backspace` o `Ctrl+U` en un indicador vacío
* Pegar texto que comienza con `!` en un indicador vacío entra automáticamente en modo shell, coincidiendo con el comportamiento de `!` escrito

A partir de v2.1.186, Claude responde automáticamente a la salida del comando una vez que llega a la transcripción, por lo que puede ejecutar `! npm test` y obtener una explicación de los fallos sin un segundo indicador. La respuesta cuesta lo mismo que enviar un indicador normal. Para restaurar el comportamiento anterior donde la salida se agrega al contexto sin una respuesta, establezca [`respondToBashCommands`](/docs/es/settings#available-settings) en `false` en `settings.json`. Antes de v2.1.186, el modo shell siempre agregaba la salida al contexto sin una respuesta.

Esto es útil para operaciones rápidas de shell mientras se mantiene el contexto de la conversación.

<h2 id="prompt-suggestions">
  Sugerencias de indicación
</h2>

Cuando abre una sesión por primera vez, aparece un comando de ejemplo atenuado en la entrada de indicación para ayudarle a comenzar. Claude Code elige esto del historial de git de su proyecto, por lo que refleja archivos en los que ha estado trabajando recientemente.

Después de que Claude responde, las sugerencias continúan apareciendo según su historial de conversación, como un paso de seguimiento de una solicitud de varias partes o una continuación natural de su flujo de trabajo.

* Presione `Tab` o `Flecha derecha` para colocar la sugerencia en la entrada de indicación, luego `Intro` para enviar
* Comience a escribir para descartarla

La sugerencia se ejecuta como una solicitud de fondo que reutiliza el caché de indicación de la conversación principal, por lo que el costo adicional es mínimo. Claude Code omite la generación de sugerencias cuando el caché está frío para evitar costos innecesarios.

Las sugerencias se omiten automáticamente después del primer turno de una conversación y en plan mode. En print mode están deshabilitadas de forma predeterminada. Pase [`--prompt-suggestions`](/docs/es/cli-reference#cli-flags) con `--output-format stream-json --verbose` para emitir un mensaje `prompt_suggestion` después de cada turno en su lugar.

Para deshabilitar completamente las sugerencias de indicación, establezca la variable de entorno o alterne la configuración en `/config`:

```bash theme={null}
export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
```

<h2 id="side-questions-with-/btw">
  Preguntas laterales con /btw
</h2>

Use `/btw` para hacer una pregunta rápida sobre su trabajo actual sin agregar al historial de conversación. Esto es útil cuando desea una respuesta rápida pero no desea saturar el contexto principal o desviar a Claude de una tarea de larga duración.

```
/btw what was the name of that config file again?
```

Las preguntas laterales tienen visibilidad completa de la conversación actual, por lo que puede preguntar sobre código que Claude ya ha leído, decisiones que tomó anteriormente, o cualquier otra cosa de la sesión. La pregunta y la respuesta son efímeras: aparecen en una superposición descartable y nunca entran en el historial de conversación.

* **Disponible mientras Claude está trabajando**: puede ejecutar `/btw` incluso mientras Claude está procesando una respuesta. La pregunta lateral se ejecuta de forma independiente y no interrumpe el turno principal.
* **Sin acceso a herramientas**: las preguntas laterales responden solo desde lo que ya está en contexto. Claude no puede leer archivos, ejecutar comandos o buscar al responder una pregunta lateral.
* **Respuesta única**: no hay turnos de seguimiento en la superposición. Para continuar el hilo, divídalo en su propia sesión con `f`.
* **Bajo costo**: la pregunta lateral reutiliza el caché de indicación de la conversación principal, por lo que el costo adicional es mínimo.

Las preguntas laterales anteriores de la misma sesión aparecen como una lista atenuada encima de la respuesta actual. Se mantienen fuera del historial de conversación pero permanecen visibles en la superposición hasta que las borre.

Una vez que aparece la respuesta, la superposición acepta estas teclas.

| Tecla                      | Acción                                                                                                                                                                                                                                                                                                     |
| :------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Space`, `Enter`, `Escape` | Descartar la respuesta y volver a la indicación                                                                                                                                                                                                                                                            |
| `Up` / `Down`              | Desplazarse por la respuesta                                                                                                                                                                                                                                                                               |
| `Left` / `Right`           | Cambiar entre esta respuesta y sus respuestas anteriores de `/btw` de la sesión. `Left` se mueve a respuestas más antiguas y `Right` regresa hacia la actual. Requiere Claude Code v2.1.187 o posterior                                                                                                    |
| `c`                        | Copiar la respuesta al portapapeles como Markdown sin formato. Use esto en lugar de la selección del ratón, que captura la representación terminal con ajuste de línea duro en lugar del texto fuente                                                                                                      |
| `f`                        | Dividir en una nueva sesión. La división hereda la conversación principal más esta pregunta y respuesta como turnos de transcripción real, por lo que puede continuar con acceso completo a herramientas. La sesión original se conserva en [`/resume`](/docs/es/commands). Disponible solo en sesiones locales |
| `x`                        | Borrar la lista de intercambios anteriores de `/btw` mostrados encima de la respuesta actual                                                                                                                                                                                                               |

`/btw` es lo opuesto a un [subagent](/docs/es/sub-agents): ve su conversación completa pero no tiene herramientas, mientras que un subagent tiene herramientas completas pero comienza con un contexto vacío. Use `/btw` para preguntar sobre lo que Claude ya sabe de esta sesión; use un subagent para descubrir algo nuevo.

<h2 id="task-list">
  Lista de tareas
</h2>

La lista de tareas es la lista de verificación de Claude: elementos que Claude creó para planificar trabajo de varios pasos, con indicadores que muestran qué está pendiente, en progreso o completado. Es independiente de la vista de tareas en segundo plano. Para ver shells en ejecución y subagentes, use [`/tasks`](/docs/es/commands) en su lugar.

* Presione `Ctrl+T` para alternar la vista de la lista de tareas. La pantalla muestra hasta cinco tareas a la vez. Cuando Claude aún no ha creado ningún elemento de lista de verificación, el botón de alternancia no tiene efecto visible porque no hay nada que mostrar
* Para ver todas las tareas o borrarlas, pregunte a Claude directamente: "show me all tasks" o "clear all tasks"
* Las tareas persisten en compactaciones de contexto, ayudando a Claude a mantenerse organizado en proyectos más grandes
* Para compartir una lista de tareas entre sesiones, establezca `CLAUDE_CODE_TASK_LIST_ID` para usar un directorio nombrado en `~/.claude/tasks/`: `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Resumen de sesión
</h2>

Cuando regresa a la terminal después de alejarse, Claude Code muestra un resumen de una línea de lo que sucedió en la sesión hasta ahora. El resumen se genera en segundo plano una vez que han pasado al menos tres minutos desde el último turno completado y la terminal no está enfocada, por lo que está listo cuando vuelve a cambiar. Los resúmenes solo aparecen una vez que la sesión tiene al menos tres turnos, y nunca dos seguidas.

Ejecute `/recap` para generar un resumen bajo demanda. Para desactivar los resúmenes automáticos, abra `/config` y desactive **Session recap**.

El resumen de sesión está activado de forma predeterminada para todos los planes y proveedores. El resumen siempre se omite en modo no interactivo.

<h2 id="pr-review-status">
  Estado de revisión de PR
</h2>

Cuando trabaja en una rama con una solicitud de extracción abierta, Claude Code muestra un enlace de PR en el que se puede hacer clic en el pie de página (por ejemplo, "PR #446"). El enlace tiene un subrayado de color que indica el estado de revisión:

* Verde: aprobado
* Amarillo: revisión pendiente
* Rojo: cambios solicitados
* Gris: borrador

El distintivo desaparece una vez que la solicitud de extracción se fusiona o se cierra. `Cmd+clic` (macOS) o `Ctrl+clic` (Windows/Linux) en el enlace para abrir la solicitud de extracción en su navegador. El estado se actualiza cada 60 segundos, e inmediatamente después de que se ejecute un comando `gh pr` o `git push` en la sesión.

<Note>
  El estado de PR requiere que la CLI `gh` esté instalada y autenticada (`gh auth login`).
</Note>

<h2 id="see-also">
  Ver también
</h2>

* [Skills](/docs/es/skills) - Indicaciones personalizadas y flujos de trabajo
* [Checkpointing](/docs/es/checkpointing) - Rebobinar las ediciones de Claude y restaurar estados anteriores
* [Referencia de CLI](/docs/es/cli-reference) - Banderas y opciones de línea de comandos
* [Configuración](/docs/es/settings) - Opciones de configuración
* [Gestión de memoria](/docs/es/memory) - Gestión de archivos CLAUDE.md
