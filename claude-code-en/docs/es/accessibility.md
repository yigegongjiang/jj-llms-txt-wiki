> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usar Claude Code con un lector de pantalla

> Configure Claude Code para lectores de pantalla como VoiceOver y NVDA, además de configuración para ampliadores de pantalla, movimiento reducido y temas seguros para daltónicos.

Claude Code tiene un modo lector de pantalla que reemplaza su interfaz de terminal visual con texto plano y lineal. En lugar de cuadros, animaciones de progreso y redibujados en el lugar, Claude Code imprime líneas etiquetadas que un lector de pantalla como VoiceOver o NVDA lee en orden. Puede mantener una conversación completa, aprobar permisos de herramientas y revisar la salida de principio a fin.

El modo lector de pantalla es opcional. Si utiliza un ampliador de pantalla, movimiento reducido o un tema seguro para daltónicos en lugar de un lector de pantalla, configure `CLAUDE_CODE_ACCESSIBILITY`, `prefersReducedMotion` o `theme` desde la tabla [Configuración de accesibilidad](#accessibility-settings). El modo lector de pantalla adapta solo la interfaz de terminal, por lo que no lo necesita en el panel de chat de la extensión de VS Code. En Claude Code v2.1.236 o posterior, la extensión [anuncia la actividad de la conversación a su lector de pantalla](/docs/es/vs-code#use-a-screen-reader) allí sin ninguna configuración.

<h2 id="turn-on-screen-reader-mode">
  Activar el modo lector de pantalla
</h2>

Elija el método que se ajuste a la frecuencia con la que utiliza un lector de pantalla:

* Para una sesión: ejecute `claude --ax-screen-reader`.
* Para sesiones iniciadas desde un shell: establezca la variable de entorno `CLAUDE_AX_SCREEN_READER` en `1`. En Bash o Zsh, ejecute `export CLAUDE_AX_SCREEN_READER=1`. En PowerShell, ejecute `$env:CLAUDE_AX_SCREEN_READER = "1"`. Agregue esa línea a su perfil de shell para mantenerla en shells futuros.
* Para cada sesión en la máquina: agregue `"axScreenReader": true` a su [archivo de configuración](/docs/es/settings) de usuario. La configuración se aplica en cualquier terminal, incluida la terminal integrada de VS Code.

Si combina métodos, Claude Code aplica la bandera [`--ax-screen-reader`](/docs/es/cli-reference#cli-flags) sobre la variable de entorno [`CLAUDE_AX_SCREEN_READER`](/docs/es/env-vars#variables), y la variable sobre la configuración [`axScreenReader`](/docs/es/settings-reference#axscreenreader).

Si utiliza Claude Code a través de SSH, establezca la variable de entorno o la configuración en la máquina remota donde se ejecuta Claude Code.

La primera línea que imprime Claude Code confirma el modo: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]`, o `[Screen Reader Mode: on via settings]`.

<h2 id="turn-off-screen-reader-mode">
  Desactivar el modo lector de pantalla
</h2>

Invierta el método que activó el modo: comience sin la bandera, desestablezca la variable de entorno o establezca `axScreenReader` en `false`. Si establece `CLAUDE_AX_SCREEN_READER` en `0`, Claude Code mantiene el modo desactivado incluso cuando la configuración es `true`.

<h2 id="accessibility-settings">
  Configuración de accesibilidad
</h2>

La tabla enumera cada opción de accesibilidad, si la establece como una bandera, una variable de entorno o una configuración, y qué cambia.

| Opción                                                                  | Tipo                | Qué cambia                                                                                                                                                                                                                                                         |
| :---------------------------------------------------------------------- | :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`--ax-screen-reader`](/docs/es/cli-reference#cli-flags)                     | Bandera             | Modo lector de pantalla para una sesión.                                                                                                                                                                                                                           |
| [`CLAUDE_AX_SCREEN_READER`](/docs/es/env-vars#variables)                     | Variable de entorno | Modo lector de pantalla para sesiones iniciadas desde el shell donde la establece.                                                                                                                                                                                 |
| [`axScreenReader`](/docs/es/settings-reference#axscreenreader)               | Configuración       | Modo lector de pantalla para cada sesión cuando es `true`.                                                                                                                                                                                                         |
| [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/es/env-vars#variables)                  | Variable de entorno | Cuánto tiempo espera Claude Code después de la línea de confirmación antes de dibujar la primera solicitud en modo lector de pantalla. Requiere Claude Code v2.1.217 o posterior.                                                                                  |
| [`CLAUDE_AX_PREPARK_MS`](/docs/es/env-vars#variables)                        | Variable de entorno | Cuánto tiempo espera Claude Code, con el cursor al inicio de la línea, antes de escribir una línea nueva o modificada en modo lector de pantalla. Requiere Claude Code v2.1.233 o posterior.                                                                       |
| [`CLAUDE_CODE_ACCESSIBILITY`](/docs/es/env-vars#variables)                   | Variable de entorno | Un cursor de terminal que permanece visible para ampliadores de pantalla como macOS Zoom cuando lo establece en `1`. El cursor sigue el acento de entrada y, en Claude Code v2.1.218 o posterior, la fila resaltada en menús y paneles como `/config` y `/plugin`. |
| [`prefersReducedMotion`](/docs/es/settings-reference#prefersreducedmotion)   | Configuración       | Spinners reducidos o sin spinners, shimmer y otras animaciones cuando es `true`.                                                                                                                                                                                   |
| [`theme`](/docs/es/settings-reference#theme)                                 | Configuración       | Los colores de la interfaz, incluidos los temas amigables para daltónicos `dark-daltonized` y `light-daltonized`. También puede elegir uno con [`/theme`](/docs/es/commands#all-commands).                                                                              |
| [`preferredNotifChannel`](/docs/es/settings-reference#preferrednotifchannel) | Configuración       | Con el valor `"terminal_bell"`, una campana de terminal fuera del modo lector de pantalla cuando Claude está esperando por usted.                                                                                                                                  |

<h2 id="what-your-screen-reader-hears">
  Lo que escucha su lector de pantalla
</h2>

En modo lector de pantalla, Claude Code escribe texto plano:

* Sin caracteres de dibujo de cuadros para la interfaz
* Sin señales que dependan solo del color
* Sin redibujos de contenido que no ha cambiado. Los indicadores de progreso se renderizan como texto estático
* Las tablas en las respuestas de Claude se leen como oraciones `Encabezado: valor` en lugar de una cuadrícula con caracteres de cuadro

Claude Code deja todo lo que imprime en el historial de desplazamiento de su terminal, para que pueda releer turnos anteriores con los comandos de revisión de su lector de pantalla o la búsqueda de su terminal. Claude Code ignora la [configuración `tui`](/docs/es/settings-reference#tui) en modo lector de pantalla. Aparte de las sesiones de fondo adjuntas enumeradas en [Limitaciones conocidas](#known-limitations), imprime texto de desplazamiento en lugar de [renderizado a pantalla completa](/docs/es/fullscreen).

Claude Code también espera en dos puntos para que su lector de pantalla pueda mantenerse al día:

* Después de que Claude Code imprime la línea de confirmación, espera 3 segundos antes de dibujar el indicador, para que su lector de pantalla pueda terminar la línea. Presione cualquier tecla para finalizar la espera. Para cambiar la duración de la espera, establezca [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/es/env-vars#variables).
* Antes de que Claude Code escriba una línea nueva o modificada, como una sugerencia o más de la respuesta de Claude, mueve el cursor al inicio de la línea y espera 50 milisegundos. Su lector de pantalla luego lee la línea desde su primer carácter. Los caracteres que escribe o elimina al final de la línea de entrada aparecen inmediatamente. Para cambiar la duración de la espera, establezca [`CLAUDE_AX_PREPARK_MS`](/docs/es/env-vars#variables).

Cada mensaje en la transcripción comienza con una etiqueta que su lector de pantalla anuncia, nombrando qué es: sus mensajes, las respuestas y el pensamiento de Claude, la actividad de herramientas, errores y advertencias, e indicadores. Las etiquetas también se pueden buscar, para que pueda saltar entre secciones de la transcripción buscando en el historial de desplazamiento de su terminal:

| Etiqueta               | Significado                                                                                        |
| :--------------------- | :------------------------------------------------------------------------------------------------- |
| `you:`                 | Sus mensajes                                                                                       |
| `claude:`              | Respuestas de Claude                                                                               |
| `thinking:`            | Pensamiento de Claude                                                                              |
| `tool:`                | Actividad de herramientas, como una edición de archivo o un comando ejecutado                      |
| `tool error:`          | Una herramienta que falló                                                                          |
| `error:`               | Un error en la conversación, como una solicitud de API fallida                                     |
| `warning:`             | Una advertencia de Claude Code, como un cambio a un modelo alternativo                             |
| `Permission Required:` | Un indicador de permiso esperando su respuesta                                                     |
| `Cost:`                | El resumen de costo de la sesión cuando Claude Code sale, si su cuenta [muestra costos](/docs/es/costs) |

Claude Code mantiene el cursor de la terminal en el símbolo de intercalación de entrada, para que el comando de lectura de línea actual de su lector de pantalla lea el indicador que está editando.

Mientras escribe al final de la línea de entrada, o presiona `Backspace` allí, Claude Code escribe solo los caracteres que cambian. Su lector de pantalla repite solo esos caracteres.

Cuando elimina una palabra o una línea con uno de los [atajos de edición de texto](/docs/es/interactive-mode#text-editing), Claude Code anuncia el texto eliminado:

* Eliminar palabras con `Ctrl+W` o `Alt+D`, o con `Option+Delete` en macOS o `Ctrl+Backspace` en Windows
* Eliminar hasta el inicio de la línea con `Ctrl+U` o `Cmd+Backspace`
* Eliminar hasta el final de la línea con `Ctrl+K`

Cuando alterna [modos de permiso](/docs/es/permission-modes) con `Shift+Tab`, Claude Code anuncia el modo de permiso en el que aterriza, como `[plan mode on]` o `[accept edits on]`. Claude Code imprime el anuncio una vez y no lo repite en redibujos posteriores.

<h3 id="jump-between-turns">
  Saltar entre turnos
</h3>

Claude Code emite marcadores de integración de shell OSC 133 en los límites de turno, para que la tecla de salto a indicador anterior de su terminal se mueva entre turnos sin leer toda la transcripción:

* iTerm2: Cmd+Shift+Up
* Terminal de VS Code: Ctrl+Up en Windows, Cmd+Up en macOS
* Windows Terminal: sin tecla por defecto; vincule la acción `scrollToMark` en su configuración
* Kitty y Ghostty: consulte la documentación del terminal para su tecla de salto a indicador

macOS Terminal no actúa sobre los marcadores, y Claude Code no los emite en WezTerm. En esos terminales, busque en el historial de desplazamiento la etiqueta `you:` en su lugar.

<h2 id="answer-menus-and-prompts">
  Responder a menús y solicitudes
</h2>

En modo lector de pantalla, los menús que normalmente navegaría con las teclas de flecha, incluidas las solicitudes de permiso, se convierten en listas numeradas. Claude Code anuncia cada opción como una línea numerada, seguida de una solicitud `Enter selection` que nombra el rango válido. Escriba el número de la opción que desea y presione Enter.

* Presione Escape para cancelar un menú cuya solicitud termina con `or Escape to cancel`.
* Si escribe un número que no está en la lista, Claude Code anuncia el rango válido y le permite intentar de nuevo.

El selector [`/effort`](/docs/es/model-config#adjust-effort-level), que es un control deslizante fuera del modo lector de pantalla, se convierte en el mismo tipo de lista numerada.

Las solicitudes de sí o no piden una respuesta escrita en lugar de un menú de dos opciones. Responda `y` o `n` y presione Enter. `yes` y `no` también funcionan.

<h2 id="hear-when-claude-code-needs-you">
  Escucha cuando Claude Code te necesita
</h2>

En modo lector de pantalla, Claude Code suena la campana de la terminal cuando necesita tu atención, para que no tengas que estar verificando constantemente la transcripción. La campana suena cuando:

* Claude termina una respuesta
* Un aviso o diálogo necesita tu respuesta, como un aviso de permiso
* Una herramienta que se ejecutó más de 5 segundos termina

La campana es la alerta estándar de tu terminal. Para silenciarla, cambia la configuración de campana en tu aplicación de terminal. Fuera del modo lector de pantalla, establece [`preferredNotifChannel`](/docs/es/settings-reference#preferrednotifchannel) en `"terminal_bell"` para obtener una [campana similar](/docs/es/terminal-config#get-a-terminal-bell-or-notification) cuando Claude esté esperando tu respuesta.

<h2 id="known-limitations">
  Limitaciones conocidas
</h2>

Algunos comportamientos no se adaptan al modo lector de pantalla:

* El modo lector de pantalla no se activa automáticamente cuando se ejecuta un lector de pantalla.
* Claude Code no anuncia un cambio de modo de permiso realizado de ninguna otra forma que no sea ciclar con `Shift+Tab`, como entrar en [modo plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) desde un comando.
* Adjuntarse a una [sesión de fondo](/docs/es/agent-view) con `claude attach` o desde la vista de agente entra en la pantalla alternativa de la terminal, que no tiene scrollback nativo. Este es el [mismo comportamiento que otras sesiones adjuntas](/docs/es/fullscreen). Para salir, presione la flecha izquierda en una solicitud vacía, o Ctrl+Z si un diálogo tiene el foco.
* Claude Code anuncia costos en el resumen que imprime al salir, no por turno.
* El modo lector de pantalla no cambia el [modo no interactivo](/docs/es/headless) con la bandera `-p`. El modo no interactivo ya escribe texto plano y sigue siendo una alternativa para scripting.

<h2 id="report-an-issue">
  Reportar un problema
</h2>

Si algo no funciona con su lector de pantalla, ampliador o terminal, abra un problema en el [rastreador de problemas de Claude Code](https://github.com/anthropics/claude-code/issues) y mencione su tecnología de asistencia en el título. Incluya su sistema operativo, aplicación de terminal y nombre y versión de la tecnología de asistencia en el informe.
