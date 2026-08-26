> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usar Claude Code con un lector de pantalla

> Configure Claude Code para lectores de pantalla como VoiceOver y NVDA, además de configuración para ampliadores de pantalla, movimiento reducido y temas seguros para daltónicos.

Claude Code tiene un modo lector de pantalla que reemplaza su interfaz de terminal visual con texto plano y lineal. En lugar de cuadros, animaciones de progreso y redibujados en el lugar, el modo imprime líneas etiquetadas que un lector de pantalla como VoiceOver o NVDA lee en orden, para que pueda mantener una conversación completa, aprobar permisos de herramientas y revisar la salida de principio a fin.

El modo lector de pantalla es opcional. Si utiliza un ampliador de pantalla, movimiento reducido o un tema seguro para daltónicos en lugar de un lector de pantalla, consulte [Configuración de accesibilidad más allá del modo lector de pantalla](#accessibility-settings-beyond-screen-reader-mode).

<Note>
  El modo lector de pantalla requiere Claude Code v2.1.181 o posterior. Las versiones anteriores rechazan la bandera `--ax-screen-reader` con `error: unknown option '--ax-screen-reader'`.
</Note>

<h2 id="turn-on-screen-reader-mode">
  Activar el modo lector de pantalla
</h2>

Elija el método que se ajuste a la frecuencia con la que utiliza un lector de pantalla:

* Para una sesión: ejecute `claude --ax-screen-reader`.
* Para sesiones iniciadas desde un shell: establezca la variable de entorno `CLAUDE_AX_SCREEN_READER` en `1`. En Bash o Zsh, ejecute `export CLAUDE_AX_SCREEN_READER=1`; en PowerShell, ejecute `$env:CLAUDE_AX_SCREEN_READER = "1"`. Agregue la línea a su perfil de shell para cubrir cada shell.
* Para cada sesión en la máquina: agregue `"axScreenReader": true` a su [archivo de configuración](/docs/es/settings) de usuario. Esto cubre cualquier terminal, incluida la terminal integrada de VS Code.

<Note>
  Los métodos se enumeran en orden de precedencia: la bandera [`--ax-screen-reader`](/docs/es/cli-reference#cli-flags) anula la variable de entorno [`CLAUDE_AX_SCREEN_READER`](/docs/es/env-vars), que anula la configuración [`axScreenReader`](/docs/es/settings#available-settings).
</Note>

Si utiliza Claude Code a través de SSH, establezca la variable de entorno o la configuración en la máquina remota donde se ejecuta Claude Code.

Cuando el modo está activado, lo primero que imprime Claude Code es una línea de confirmación que nombra el método que lo activó: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]` o `[Screen Reader Mode: on via settings]`. El formato de nomenclatura de método requiere Claude Code v2.1.206 o posterior. Cuando Claude Code se reinicia a sí mismo, por ejemplo para terminar de instalar una actualización, el nuevo proceso hereda el modo a través de la variable de entorno `CLAUDE_AX_SCREEN_READER`, por lo que su línea de confirmación lee `[Screen Reader Mode: on via env]` independientemente del método que utilizó.
Las versiones anteriores imprimen `[Accessible screen reader mode: on]`.

<h2 id="turn-off-screen-reader-mode">
  Desactivar el modo lector de pantalla
</h2>

Invierta el método que activó el modo: comience sin la bandera, desestablezca la variable de entorno o establezca `axScreenReader` en `false`. Establecer `CLAUDE_AX_SCREEN_READER=0` mantiene el modo desactivado incluso cuando la configuración es `true`.

<h2 id="what-your-screen-reader-hears">
  Lo que su lector de pantalla escucha
</h2>

En el modo lector de pantalla, Claude Code escribe texto plano:

* sin caracteres de dibujo de cuadros para el cromo de la interfaz
* sin señales solo de color
* sin redibujados de contenido que no ha cambiado; los spinners de progreso se renderizan como texto estático
* las tablas en las respuestas de Claude se leen como oraciones `Header: value` en lugar de una cuadrícula de caracteres de cuadro. Requiere Claude Code v2.1.198 o posterior; las versiones anteriores dibujan tablas como cuadrículas incluso en el modo lector de pantalla.

La salida se acumula en el scrollback de su terminal, para que pueda releer turnos anteriores con los comandos de revisión de su lector de pantalla o la búsqueda de su terminal.

El modo lector de pantalla se renderiza como texto plano desplazable, incluso si ha activado [renderizado a pantalla completa](/docs/es/fullscreen) con la [configuración `tui`](/docs/es/settings#available-settings); la configuración no tiene efecto mientras el modo está activo. Las sesiones de fondo adjuntas aún se renderizan a pantalla completa; consulte [Limitaciones conocidas](#known-limitations).

Cada mensaje en la transcripción comienza con una etiqueta que su lector de pantalla anuncia, nombrando lo que es: sus mensajes, las respuestas de Claude, la actividad de herramientas, errores y solicitudes. Las etiquetas también se pueden buscar, para que pueda saltar entre secciones de la transcripción buscando el scrollback de su terminal:

| Etiqueta               | Significado                                                                                        |
| :--------------------- | :------------------------------------------------------------------------------------------------- |
| `you:`                 | Sus mensajes                                                                                       |
| `claude:`              | Respuestas de Claude                                                                               |
| `tool:`                | Actividad de herramientas, como una edición de archivo o un comando ejecutado                      |
| `tool error:`          | Una herramienta que falló                                                                          |
| `error:`               | Un error en la conversación, como una solicitud de API fallida                                     |
| `Permission Required:` | Una solicitud de permiso esperando su respuesta                                                    |
| `Cost:`                | El resumen de costo de la sesión cuando Claude Code sale, si su cuenta [muestra costos](/docs/es/costs) |

El cursor de la terminal sigue el acento de entrada, por lo que el comando de lectura de línea actual de un lector de pantalla responde "dónde estoy" con la solicitud que está editando.

<h3 id="jump-between-turns">
  Saltar entre turnos
</h3>

Claude Code emite marcadores de integración de shell OSC 133 en los límites de turno, por lo que la tecla de salto a solicitud anterior de su terminal se mueve entre turnos sin leer toda la transcripción:

* iTerm2: Cmd+Shift+Up
* Terminal de VS Code: Ctrl+Up en Windows, Cmd+Up en macOS
* Windows Terminal: sin tecla por defecto; vincule la acción `scrollToMark` en su configuración
* Kitty y Ghostty: consulte la documentación de la terminal para su tecla de salto a solicitud

macOS Terminal no actúa sobre los marcadores, y Claude Code no los emite en WezTerm. En esas terminales, busque la etiqueta `you:` en el scrollback en su lugar.

<h2 id="answer-menus-and-prompts">
  Responder menús y solicitudes
</h2>

En el modo lector de pantalla, los menús que normalmente navegaría con las teclas de flecha, incluidas las solicitudes de permiso, se convierten en listas numeradas. Cada opción se anuncia como una línea numerada, seguida de una solicitud `Enter selection` que nombra el rango válido. Escriba el número de la opción que desea y presione Intro.

* Para cancelar un menú desestimable: presione Escape. Su solicitud termina con `or Escape to cancel`.
* Si escribe un número que no está en la lista: Claude Code anuncia el rango válido y le permite intentarlo de nuevo.

Las solicitudes de sí o no piden una respuesta escrita en lugar de un menú de dos opciones. Responda `y` o `n` y presione Intro. `yes` y `no` también funcionan.

<h2 id="hear-when-claude-code-needs-you">
  Escuchar cuando Claude Code lo necesita
</h2>

En el modo lector de pantalla, Claude Code suena la campana de la terminal cuando necesita su atención, para que no tenga que seguir verificando la transcripción. La campana suena cuando:

* Claude termina una respuesta
* aparece una solicitud de permiso
* una herramienta que se ejecutó durante más de 5 segundos termina

La campana es la alerta estándar de su terminal. Para silenciarla, cambie la configuración de campana en su aplicación de terminal. La campana no requiere modo lector de pantalla: fuera del modo, establezca [`preferredNotifChannel`](/docs/es/settings#available-settings) en `"terminal_bell"` para alertas similares cuando Claude lo está esperando. Consulte [Obtener una campana de terminal o notificación](/docs/es/terminal-config#get-a-terminal-bell-or-notification).

<h2 id="accessibility-settings-beyond-screen-reader-mode">
  Configuración de accesibilidad más allá del modo lector de pantalla
</h2>

Estas opciones abordan necesidades de accesibilidad fuera del modo lector de pantalla. Todas ellas funcionan junto con él.

* La [variable de entorno](/docs/es/env-vars) `CLAUDE_CODE_ACCESSIBILITY` es para ampliadores de pantalla. Establezca `CLAUDE_CODE_ACCESSIBILITY=1` para mantener visible el cursor de terminal nativo para que los ampliadores, como macOS Zoom, puedan rastrear la posición del cursor.
* La [configuración](/docs/es/settings#available-settings) `prefersReducedMotion` reduce o desactiva spinners, shimmer y otras animaciones sin cambiar el resto de la interfaz.
* La [configuración](/docs/es/settings#available-settings) `theme` selecciona los colores de la interfaz, incluidos los temas seguros para daltónicos `dark-daltonized` y `light-daltonized`.

<h2 id="known-limitations">
  Limitaciones conocidas
</h2>

Algunos comportamientos no se adaptan al modo lector de pantalla:

* El modo lector de pantalla no se activa automáticamente cuando se ejecuta un lector de pantalla.
* Los cambios de modo, como entrar en [modo plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode), aún no se anuncian.
* Adjuntarse a una [sesión de fondo](/docs/es/agent-view) con `claude attach` o desde la vista de agente entra en la pantalla alternativa de la terminal, que no tiene scrollback nativo. Este es el [mismo comportamiento que otras sesiones adjuntas](/docs/es/fullscreen). Para salir, presione la flecha izquierda en una solicitud vacía, o Ctrl+Z si un diálogo tiene el foco.
* Claude Code anuncia costos en el resumen que imprime al salir, no por turno.
* El modo lector de pantalla no cambia el [modo no interactivo](/docs/es/headless) con la bandera `-p`. El modo no interactivo ya escribe texto plano y sigue siendo una alternativa para scripting.

<h2 id="report-an-issue">
  Reportar un problema
</h2>

Si algo no funciona con su lector de pantalla, ampliador o terminal, abra un problema en el [rastreador de problemas de Claude Code](https://github.com/anthropics/claude-code/issues) y mencione su tecnología de asistencia en el título. Incluya su sistema operativo, aplicación de terminal y nombre y versión de la tecnología de asistencia en el informe.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Estas páginas contienen las entradas de referencia completa y la configuración relacionada para lo que cubre esta página:

* [Settings](/docs/es/settings#available-settings): las entradas `axScreenReader`, `prefersReducedMotion`, `theme` y `preferredNotifChannel`
* [Environment variables](/docs/es/env-vars): las entradas `CLAUDE_AX_SCREEN_READER` y `CLAUDE_CODE_ACCESSIBILITY`
* [CLI reference](/docs/es/cli-reference#cli-flags): la bandera `--ax-screen-reader`
* [Terminal configuration](/docs/es/terminal-config): campanas, notificaciones y temas fuera del modo lector de pantalla
* [Non-interactive mode](/docs/es/headless): ejecuciones de `claude -p` con script, que escriben texto plano sin modo lector de pantalla
