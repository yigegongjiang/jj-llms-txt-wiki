> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Renderizado a pantalla completa

> Habilite un modo de renderizado más suave y sin parpadeos con soporte de ratón y uso de memoria estable en conversaciones largas.

<Note>
  El renderizado a pantalla completa es una [vista previa de investigación](#research-preview). Si [comienza en pantalla completa o en el renderizador clásico](#fullscreen-by-default) depende de su configuración. Ejecute `/tui fullscreen` o `/tui default` para cambiar en su conversación actual. El comportamiento puede cambiar según los comentarios.
</Note>

El renderizado a pantalla completa es una ruta de renderizado alternativa para la CLI de Claude Code que elimina el parpadeo, mantiene el uso de memoria plano en conversaciones largas y añade soporte de ratón. Dibuja la interfaz en el búfer de pantalla alternativa de la terminal, como `vim` o `htop`, y solo renderiza los mensajes que están actualmente visibles. Esto reduce la cantidad de datos enviados a su terminal en cada actualización.

La diferencia es más notable en emuladores de terminal donde el rendimiento de renderizado es el cuello de botella, como la terminal integrada de VS Code, tmux e iTerm2. Si su posición de desplazamiento de terminal salta a la parte superior mientras Claude está trabajando, o la pantalla parpadea mientras la salida de herramientas se transmite, este modo aborda esos problemas.

<Note>
  El término pantalla completa describe cómo Claude Code se apodera de la superficie de dibujo de la terminal, de la manera que lo hace `vim`. No tiene nada que ver con maximizar su ventana de terminal, y funciona en cualquier tamaño de ventana.
</Note>

<h2 id="enable-fullscreen-rendering">
  Habilitar renderizado a pantalla completa
</h2>

Ejecute `/tui fullscreen` dentro de cualquier conversación de Claude Code. La CLI guarda la [configuración `tui`](/docs/es/settings-reference#tui) y se reinicia en pantalla completa con su conversación intacta, por lo que puede cambiar a mitad de sesión sin perder contexto. Ejecute `/tui default` para volver al renderizador clásico, o `/tui` sin argumentos para imprimir qué renderizador está activo.

En [modo de lector de pantalla](/docs/es/accessibility), Claude Code siempre utiliza el renderizador clásico excepto en [sesiones en segundo plano](/docs/es/agent-view) adjuntas, que aún se renderizan a pantalla completa. Si ejecuta `/tui fullscreen` en cualquier otra sesión, Claude Code imprime una explicación en lugar de cambiar y no modifica la configuración `tui` guardada.

Claude Code lleva estos elementos a la sesión reiniciada:

* La conversación tal como aparece en pantalla. Después de un [`/rewind`](/docs/es/checkpointing#rewind-and-summarize), eso significa:
  * Si rebobinó anteriormente en la sesión, Claude Code se reinicia desde el punto rebobinado, no desde la transcripción más larga guardada en disco. Por ejemplo, si rebobinó pasando sus últimos tres mensajes, la sesión reiniciada se abre sin ellos
  * Si rebobinó antes de su primer mensaje, Claude Code se reinicia con una conversación vacía
* Su [modo de permisos](/docs/es/permission-modes) y [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level)
* El modelo que seleccionó por última vez con [`/model`](/docs/es/model-config#setting-your-model)
* Reglas que pasó con [`--allowed-tools` o `--disallowed-tools`](/docs/es/cli-reference#cli-flags), y sus banderas `--agent`, `--agents`, `--append-system-prompt`, y `--system-prompt-snapshot`

Claude Code rechaza reiniciarse si la sesión tiene una restricción que no puede pasar al proceso reiniciado. Las restricciones que no puede pasar incluyen:

* Banderas de lanzamiento como un reemplazo de [`--system-prompt`](/docs/es/cli-reference#cli-flags), una lista de permitidos de [`--tools`](/docs/es/cli-reference#cli-flags), o [`--setting-sources`](/docs/es/cli-reference#cli-flags)
* Reglas de denegación o consulta que una [actualización de permisos de hook o SDK](/docs/es/hooks#permission-update-entries) agregó solo para esta sesión

En ese caso, Claude Code imprime [`Cannot switch renderers in this session`](/docs/es/errors#cannot-switch-renderers-in-this-session) con las razones. No cambia ni guarda nada.

También puede establecer la variable de entorno `CLAUDE_CODE_NO_FLICKER` antes de iniciar Claude Code:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

Para saber cómo la [configuración `tui`](/docs/es/settings-reference#tui) y la variable se combinan cuando ambas están establecidas, consulte la entrada de la configuración. Después de un [inicio de pantalla completa fallido](#fullscreen-renderer-didnt-finish-starting), Claude Code aún respeta la variable pero no la configuración. El comando `/tui` borra `CLAUDE_CODE_NO_FLICKER` del proceso reiniciado para que la configuración que escribe tenga efecto.

<h3 id="fullscreen-by-default">
  Pantalla completa por defecto
</h3>

Las [sesiones en segundo plano](/docs/es/agent-view) adjuntas se renderizan a pantalla completa, y otras sesiones en [modo de lector de pantalla](/docs/es/accessibility) utilizan el renderizador clásico. De lo contrario, Claude Code lo inicia en el renderizador de la primera fila de esta tabla que coincida con su configuración:

| Su situación                                                                                                                                                                                    | Renderizador en el que inicia               |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------ |
| Estableció [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`](/docs/es/env-vars) o `CLAUDE_CODE_NO_FLICKER=0`                                                                                                | Clásico                                     |
| Estableció `CLAUDE_CODE_NO_FLICKER=1`                                                                                                                                                           | Pantalla completa                           |
| Claude Code [desactivó pantalla completa después de un inicio de pantalla completa fallido](#fullscreen-renderer-didnt-finish-starting) en esta máquina                                         | Clásico                                     |
| Está en el modo de integración [`tmux -CC`](#use-with-tmux) de iTerm2, o está conectado por SSH a Claude Code ejecutándose en Windows                                                           | Clásico                                     |
| Guardó una [configuración `tui`](/docs/es/settings-reference#tui)                                                                                                                                    | El renderizador que nombra la configuración |
| Su sesión no [obtiene banderas de características de Anthropic](/docs/es/env-vars#features-that-need-feature-flag-fetching), y Claude Code ha dejado de ofrecer el diálogo de inicio en esta máquina | Clásico                                     |
| Su sesión no obtiene banderas de características de Anthropic, y el primer lanzamiento de Claude Code en esta máquina ejecutó v2.1.239 o posterior                                              | Pantalla completa                           |
| Su sesión obtiene banderas de características de Anthropic, y utilizó Claude Code por primera vez el 6 de mayo de 2026 o posterior                                                              | Pantalla completa                           |
| Cualquier otra cosa                                                                                                                                                                             | Clásico                                     |

Las sesiones que no obtienen banderas de características incluyen las de [Amazon Bedrock](/docs/es/amazon-bedrock), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), o [Microsoft Foundry](/docs/es/microsoft-foundry), y las que tienen la telemetría desactivada.

Si inicia en el renderizador clásico y no ha guardado una configuración `tui`, Claude Code puede abrir un diálogo al inicio ofreciendo el cambio:

* Si acepta, Claude Code se reinicia de la misma manera que `/tui fullscreen`, llevando el mismo estado de sesión, y guarda la configuración una vez que la sesión reiniciada ha [iniciado correctamente](#fullscreen-renderer-didnt-finish-starting).
* Si elige **Not now**, Claude Code no lo ofrece de nuevo en esta máquina.
* Claude Code deja de ofrecer después de haber mostrado el diálogo en tres lanzamientos, respondido o no.

<h2 id="what-changes">
  Qué cambia
</h2>

El renderizado a pantalla completa cambia cómo la CLI dibuja en su terminal. El cuadro de entrada permanece fijo en la parte inferior de la pantalla en lugar de moverse mientras la salida se transmite. Si la entrada permanece en su lugar mientras Claude está trabajando, el renderizado a pantalla completa está activo. Solo los mensajes visibles se mantienen en el árbol de renderizado, por lo que la memoria permanece constante independientemente de la longitud de la conversación.

Debido a que la conversación vive en el búfer de pantalla alternativa en lugar del desplazamiento de su terminal, algunas cosas funcionan de manera diferente:

| Antes                                                           | Ahora                                                                                               | Detalles                                                                |
| :-------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------- |
| `Cmd+f` o búsqueda de tmux para encontrar texto                 | `Ctrl+o` para modo de transcripción, luego `/` para buscar o `[` para escribir en el desplazamiento | [Buscar y revisar la conversación](#search-and-review-the-conversation) |
| Clic y arrastre nativo de la terminal para seleccionar y copiar | Selección en la aplicación, se copia automáticamente al soltar el ratón                             | [Usar el ratón](#use-the-mouse)                                         |
| `Cmd`-clic para abrir una URL                                   | `Cmd`-clic en macOS, `Ctrl`-clic en otros lugares                                                   | [Usar el ratón](#use-the-mouse)                                         |

Si la captura de ratón interfiere con su flujo de trabajo, puede [desactivarla](#keep-native-text-selection) mientras mantiene el renderizado sin parpadeos.

<h2 id="use-the-mouse">
  Usar el ratón
</h2>

El renderizado a pantalla completa captura eventos del ratón y los maneja dentro de Claude Code:

* **Haga clic en la entrada de solicitud** para posicionar su cursor en cualquier lugar del texto que está escribiendo.
* **Haga clic en una sugerencia en la lista de comandos `/` o archivos `@`** para aceptarla. Pasar el ratón resalta la fila bajo su cursor.
* **Haga clic en una opción en un menú de selección** para elegirla. Esto cubre solicitudes de permisos, `/model`, `/config` y otros diálogos que muestran una lista de opciones. Pasar el ratón muestra un puntero en la fila bajo su cursor.
* **Haga clic en una opción en un menú de selección múltiple** para alternarla, y haga clic en el botón de envío para confirmar sus opciones. Hacer clic en una fila de texto libre, como la fila `Other` en una pregunta de opción múltiple, enfoca su campo de entrada para que pueda escribir una respuesta. Requiere Claude Code v2.1.208 o posterior.
* **Haga clic en un valor de configuración en el panel `/config`** para cambiarlo, y desplácese por la lista de configuración con la rueda del ratón. Requiere Claude Code v2.1.271 o posterior.
* **Desplace un menú de selección o selección múltiple con la rueda del ratón** cuando tenga más opciones de las que muestra a la vez, como la lista `/model` en una ventana de terminal corta. La rueda desplaza la lista mientras el puntero está sobre sus opciones. Requiere Claude Code v2.1.280 o posterior.
* **Haga clic en un resultado de herramienta contraído** para expandirlo y ver la salida completa. Haga clic de nuevo para contraerlo. La llamada de herramienta y su resultado se expanden juntos. Solo los mensajes que tienen más para mostrar son clicables.
  * Hacer clic también expande la salida de un comando shell `!`, ya sea un resultado truncado anterior o la fila de progreso en vivo mientras se ejecuta el comando. Requiere Claude Code v2.1.257 o posterior.
* **Mantenga presionado `Cmd` en macOS, o `Ctrl` en Linux y Windows, y haga clic en una URL o ruta de archivo** para abrirla. Las URLs simples `http://` y `https://` se abren en su navegador, y las rutas de archivo en la salida de herramientas, como las que se imprimen después de un Edit o Write, se abren en su aplicación predeterminada. Un clic simple sin el modificador no abre enlaces, coincidiendo con el comportamiento de terminal nativo.
  * Claude Code renderiza una ruta de red (UNC), como `\\server\share\file.ts`, como texto sin formato sin enlace, porque abrir una ruta de red puede enviar sus credenciales de Windows al host que nombra.
  * Algunos terminales de macOS reenvían `Cmd`+clic a la aplicación en ejecución en lugar de abrir el enlace ellos mismos, y el protocolo del ratón del terminal no tiene forma de codificar la tecla `Cmd`, por lo que Claude Code recibe un clic simple. En Ghostty, y en Warp en macOS, Claude Code detecta esto y permite que un clic simple en un enlace lo abra, y mantener presionado `Cmd` sigue funcionando.
  * En la terminal integrada de VS Code y terminales similares basados en xterm.js, Claude Code se remite al manejador de enlaces propio de la terminal, que utiliza el mismo gesto.
* **Haga clic y arrastre** para seleccionar texto en cualquier lugar de la conversación. Doble clic selecciona una palabra, coincidiendo con los límites de palabra de iTerm2 para que una ruta de archivo se seleccione como una unidad. Hacer doble clic en una URL selecciona la URL completa, incluido el esquema. Triple clic selecciona la línea.
* **Desplácese con la rueda del ratón** para moverse a través de la conversación.

El texto seleccionado se copia automáticamente en su portapapeles al soltar el ratón. Para desactivar esto, alterne Copiar al seleccionar en `/config`.

Con Copiar al seleccionar desactivado, presione `Ctrl+Shift+c` para copiar manualmente. En terminales que admiten el protocolo de teclado kitty, como kitty, WezTerm, Ghostty e iTerm2, `Cmd+c` también funciona. Si tiene una selección activa, `Ctrl+c` copia en lugar de cancelar.

Con una selección activa, mantenga presionado `Shift` y presione las teclas de flecha para extenderla desde el teclado. `Shift+↑` y `Shift+↓` desplazan la ventana gráfica cuando la selección alcanza el borde superior o inferior. `Shift+Home` y `Shift+End` extienden hasta el inicio o el final de la línea actual.

En la vista de solicitud normal, lo que sucede con una selección activa depende de la tecla que presione:

* **`Esc`**: Claude Code realiza la acción habitual de la tecla, como interrumpir la respuesta en ejecución o descartar un diálogo abierto, y la selección permanece resaltada.
* **`PgUp`, `PgDn`, `Ctrl+Home`, `Ctrl+End`, o `Shift`, `Alt` u `Option`, o `Cmd`, `Win` o `Super` con una flecha, `Home` o tecla `End`**: la selección permanece.
* **Cualquier otra tecla, incluidas las teclas de flecha simples, `Enter` y caracteres escritos**: Claude Code borra la selección.
* **Una tecla vinculada a [`selection:clear`](/docs/es/keybindings#scroll-actions)**: Claude Code borra la selección, incluso cuando la tecla es `Esc` u otra tecla que de otro modo la mantendría. La acción no tiene vinculación predeterminada.

En [modo de transcripción](#search-and-review-the-conversation), las teclas de navegación y búsqueda enumeradas allí también mantienen la selección.

<h2 id="scroll-the-conversation">
  Desplazarse por la conversación
</h2>

El renderizado a pantalla completa maneja el desplazamiento dentro de la aplicación. Utilice estos atajos de teclado para navegar:

| Atajo de teclado | Acción                                                         |
| :--------------- | :------------------------------------------------------------- |
| `PgUp` / `PgDn`  | Desplazarse hacia arriba o hacia abajo media pantalla          |
| `Ctrl+Home`      | Saltar al inicio de la conversación                            |
| `Ctrl+End`       | Saltar al último mensaje y reactivar el seguimiento automático |
| Rueda del ratón  | Desplazarse algunas líneas a la vez                            |

Puede desplazarse hacia atrás hasta el inicio de la sesión incluso después de la [compactación](/docs/es/context-window#what-survives-compaction). Claude continúa trabajando a partir del resumen de compactación, pero Claude Code mantiene todos los mensajes anteriores en el desplazamiento a pantalla completa en todas las compactaciones repetidas.

En teclados sin teclas dedicadas `PgUp`, `PgDn`, `Home` o `End`, como los teclados de MacBook, mantenga presionada `Fn` junto con las teclas de flecha: `Fn+↑` envía `PgUp`, `Fn+↓` envía `PgDn`, `Fn+←` envía `Home` y `Fn+→` envía `End`. `Ctrl+Fn+→` no llega a Claude Code en macOS, por lo que un teclado de MacBook no tiene un atajo de teclado funcional para saltar al final de forma predeterminada. En su lugar, utilice una de estas opciones:

* Haga clic en el [botón de saltar al final](#auto-follow).
* Desplácese hacia abajo con la rueda del ratón para reanudar el seguimiento.
* Reenlace `scroll:bottom` a un atajo de teclado que su teclado pueda enviar.

Estas acciones se pueden reasignar. Consulte [Acciones de desplazamiento](/docs/es/keybindings#scroll-actions) para obtener la lista completa de nombres de acciones, incluidas las variantes de media página y página completa que no tienen enlace predeterminado.

Mientras se desplaza hacia arriba, una fila de encabezado atenuada en la parte superior de la conversación muestra el mensaje más reciente que se ha desplazado fuera de la vista. Haga clic en la fila para saltar a ese mensaje.

<h3 id="auto-follow">
  Seguimiento automático
</h3>

El desplazamiento hacia arriba pausa el seguimiento automático para que la nueva salida no lo devuelva al final. Un botón `Saltar al final` flota sobre el borde inferior de la transcripción mientras se desplaza hacia arriba, y muestra un recuento como `3 nuevos mensajes` cuando llega nueva salida. Haga clic en él, presione `Ctrl+End` o desplácese hacia abajo para reanudar el seguimiento.

Mientras el seguimiento automático está pausado, la vista también permanece donde la desplazó cuando una respuesta termina de transmitirse.

La sugerencia de teclado del botón refleja lo que su teclado puede enviar. En macOS sugiere hacer clic o `Fn+↓` para desplazarse, porque `Ctrl+End` no llega a Claude Code desde un teclado Mac. Reenlace [`scroll:bottom`](/docs/es/keybindings#scroll-actions) y el botón muestra su atajo de teclado en todas las plataformas.

En una terminal demasiado estrecha para la etiqueta completa, el botón acorta la sugerencia en lugar de ajustarla a la fila de transcripción inferior.

Para desactivar completamente el seguimiento automático de modo que la vista permanezca donde la deja, abra `/config` y establezca Desplazamiento automático en desactivado. Con el desplazamiento automático desactivado, la vista nunca salta al final por sí sola. Los cuadros de diálogo de solicitud de permisos y otros que necesitan una respuesta aún se desplazan a la vista independientemente de esta configuración.

<h3 id="mouse-wheel-scrolling">
  Desplazamiento con rueda del ratón
</h3>

El desplazamiento con rueda del ratón requiere que su terminal reenvíe eventos del ratón a Claude Code. La mayoría de las terminales hacen esto siempre que una aplicación lo solicite. iTerm2 lo convierte en una configuración por perfil: si la rueda no hace nada pero `PgUp` y `PgDn` funcionan, abra Configuración → Perfiles → Terminal y active Habilitar informe del ratón. La misma configuración también es necesaria para que el clic para expandir y la selección de texto funcionen.

Si el desplazamiento con rueda del ratón se siente lento, su terminal puede estar enviando un evento de desplazamiento por muesca física sin multiplicador. Algunas terminales, como Ghostty e iTerm2 con desplazamiento más rápido habilitado, ya amplifican eventos de rueda. Otras, incluida la terminal integrada de VS Code, envían exactamente un evento por muesca. Claude Code no puede detectar cuál.

Establezca `CLAUDE_CODE_SCROLL_SPEED` para multiplicar la distancia de desplazamiento base:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Un valor de `3` coincide con el predeterminado en `vim` y aplicaciones similares. La configuración acepta cualquier valor positivo hasta 20, incluidos valores fraccionarios por debajo de 1 como `0.25` para ralentizar el desplazamiento de trackpad y rueda acelerados en terminales que ya amplifican eventos de rueda.

Para ajustar la velocidad de desplazamiento de forma interactiva, ejecute `/scroll-speed`. El cuadro de diálogo muestra una regla que puede desplazar mientras está abierto para que pueda sentir el cambio inmediatamente. Presione `←` y `→` para ajustar la velocidad, `r` para restablecer el valor predeterminado detectado automáticamente y `Intro` para guardar. El cuadro de diálogo avanza en números enteros hasta 10, y en terminales que admiten control más fino también ofrece pasos de cuarto hasta 0,25.

El comando escribe el mismo valor que establece la variable de entorno `CLAUDE_CODE_SCROLL_SPEED`, persistido en `~/.claude/settings.json`. El máximo del cuadro de diálogo es 10: si establece un valor más alto a través de la variable de entorno, el cuadro de diálogo muestra 10, y guardar desde el cuadro de diálogo persiste 10. El comando no está disponible en la terminal del IDE de JetBrains.

Independientemente de la velocidad base, Claude Code acelera la velocidad de desplazamiento cuando gira la rueda rápidamente, por lo que un giro rápido cubre más distancia que el mismo número de muescas lentas. Para desactivar la aceleración y mantener una velocidad constante por muesca, establezca `wheelScrollAccelerationEnabled` en `false` en [`settings.json`](/docs/es/settings-reference#all-settings). Esta configuración requiere Claude Code v2.1.174 o posterior.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Desplazamiento en la terminal del IDE de JetBrains
</h3>

En la terminal del IDE de JetBrains, Claude Code aplica su propio manejo de desplazamiento e ignora `CLAUDE_CODE_SCROLL_SPEED`. La terminal envía eventos de desplazamiento a una velocidad mucho más alta que otros emuladores, por lo que un multiplicador ajustado en otro lugar se excede aquí.

En 2025.2, la terminal también tiene errores de desplazamiento con rueda que producen teclas de flecha espurias y eventos de dirección incorrecta. Claude Code detecta esto en tiempo de ejecución y lo mitiga automáticamente, por lo que el desplazamiento de trackpad y rueda del ratón funcionan sin configuración. Para la mejor experiencia de desplazamiento, actualice a 2025.3 o posterior. Claude Code muestra una sugerencia la primera vez que se desplaza si detecta el error.

<h2 id="search-and-review-the-conversation">
  Buscar y revisar la conversación
</h2>

`Ctrl+o` alterna entre el modo de solicitud normal y el modo de transcripción.

Para una vista más tranquila que muestre solo su última solicitud, un resumen de una línea de llamadas de herramientas con estadísticas de edición de diferencias, y la respuesta final, ejecute `/focus`. La configuración persiste entre sesiones. Ejecute `/focus` nuevamente para desactivarlo.

El modo de transcripción obtiene navegación y búsqueda de estilo `less`:

| Tecla                               | Acción                                                                                                                                    |
| :---------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `/`                                 | Abrir búsqueda. Escriba para encontrar coincidencias, `Enter` para aceptar, `Esc` para cancelar y restaurar su posición de desplazamiento |
| `n` / `N`                           | Saltar a la siguiente o anterior coincidencia. Funciona después de haber cerrado la barra de búsqueda                                     |
| `j` / `k` o `↑` / `↓`               | Desplazarse una línea                                                                                                                     |
| `g` / `G` o `Home` / `End`          | Saltar al principio o al final                                                                                                            |
| `{` / `}`                           | Saltar a la solicitud anterior o siguiente                                                                                                |
| `Ctrl+u` / `Ctrl+d`                 | Desplazarse media página                                                                                                                  |
| `Ctrl+b` / `Ctrl+f` o `Space` / `b` | Desplazarse una página completa                                                                                                           |
| `Ctrl+o`, `Esc`, o `q`              | Salir del modo de transcripción y volver a la solicitud                                                                                   |

La búsqueda de `Cmd+f` de su terminal y la búsqueda de tmux no ven la conversación porque vive en el búfer de pantalla alternativa, no en el desplazamiento nativo. Para devolver el contenido a su terminal, presione `Ctrl+o` para entrar en modo de transcripción primero, luego:

* **`[`**: escribe la conversación completa en el búfer de desplazamiento nativo de su terminal, con toda la salida de herramientas expandida. La conversación es ahora texto ordinario en su terminal, por lo que `Cmd+f`, el modo de copia de tmux y cualquier otra herramienta nativa pueden buscar o seleccionar. Las sesiones largas pueden pausarse un momento mientras esto sucede. Esto dura hasta que salga del modo de transcripción con `Esc` o `q`, que lo devuelve a la representación a pantalla completa. El siguiente `Ctrl+o` comienza de nuevo.
* **`v`**: escribe la conversación en un archivo temporal y lo abre en `$VISUAL` o `$EDITOR`.

<h2 id="watch-your-changes-in-the-diff-panel">
  Observe sus cambios en el panel de diferencias
</h2>

En la representación a pantalla completa, [`/diff`](/docs/es/interactive-mode#review-changes-with-%2Fdiff) abre un panel junto a la conversación en lugar de un visor que tenga que cerrar, para que pueda observar cómo se acumulan los cambios mientras Claude trabaja. En una terminal ancha, el panel también puede abrirse automáticamente una vez que Claude comienza a editar archivos. [Panel de diferencias](/docs/es/interactive-mode#diff-panel) cubre lo que muestra, cómo mantenerlo cerrado y cómo cambiar con qué lo compara.

<h2 id="clear-the-conversation">
  Borrar la conversación
</h2>

Ejecute `/clear` para iniciar una nueva conversación.

Si la pantalla se ve distorsionada o parcialmente en blanco, presione `Ctrl+L` para redibujar la pantalla. El redibujado mantiene la conversación y su entrada en su lugar.

`Cmd+K` hace lo mismo que `Ctrl+L` cuando su terminal lo pasa a Claude Code. iTerm2 y Terminal.app manejan `Cmd+K` por sí mismos y borran su propia pantalla, y Claude Code detecta la pantalla borrada y repinta la conversación. Antes de v2.1.280, comenzando con v2.1.260, presionar `Ctrl+L` o `Cmd+K` donde llega a Claude Code, borraba la pantalla en renderizado a pantalla completa. Antes de v2.1.238, presionar `Ctrl+L` dos veces en dos segundos ejecutaba `/clear`.

<h2 id="use-with-tmux">
  Usar con tmux
</h2>

La representación a pantalla completa funciona dentro de tmux, con tres advertencias.

El desplazamiento con la rueda del ratón requiere el modo de ratón de tmux. Si su `~/.tmux.conf` no lo habilita ya, agregue esta línea y recargue su configuración:

```bash theme={null}
set -g mouse on
```

Sin el modo de ratón, los eventos de la rueda van a tmux en lugar de Claude Code. El desplazamiento por teclado con `PgUp` y `PgDn` funciona de cualquier forma. Claude Code imprime una sugerencia única al inicio si detecta tmux con el modo de ratón desactivado.

La representación a pantalla completa es incompatible con el modo de integración de tmux de iTerm2, que es el modo en el que entra con `tmux -CC`. En el modo de integración, iTerm2 representa cada panel de tmux como una división nativa en lugar de permitir que tmux dibuje en la terminal. El búfer de pantalla alternativa y el seguimiento del ratón no funcionan correctamente allí: la rueda del ratón no hace nada, y el doble clic puede corromper el estado de la terminal. No habilite la representación a pantalla completa en sesiones `tmux -CC`. El tmux regular dentro de iTerm2, sin `-CC`, funciona bien.

Las versiones de tmux hasta la serie 3.6 no implementan salida sincronizada, por lo que bajo esas versiones puede ver más parpadeo durante los redibujados que cuando ejecuta Claude Code directamente en su terminal. Claude Code sondea la terminal para detectar compatibilidad con salida sincronizada al inicio y la utiliza cuando la terminal lo reporta. Si ve parpadeo bajo tmux, actualice a la versión más reciente de tmux o ejecute Claude Code en su propia pestaña de terminal fuera de tmux.

<h2 id="keep-native-text-selection">
  Mantener la selección de texto nativa
</h2>

La captura del ratón es el punto de fricción más común, especialmente sobre SSH o dentro de tmux. Cuando Claude Code captura eventos del ratón, la copia nativa al seleccionar de su terminal deja de funcionar. La selección que realiza con clic y arrastre existe dentro de Claude Code, no en el búfer de selección de su terminal, por lo que el modo de copia de tmux, las sugerencias de Kitty y herramientas similares no la ven.

Claude Code escribe la selección en el portapapeles del sistema, y la ruta que utiliza depende de su configuración. En una sesión local ejecuta una herramienta de portapapeles nativa:

* **macOS**: `pbcopy`
* **Linux**: `wl-copy` en Wayland, o `xclip` o `xsel` en X11, lo que esté instalado. Claude Code escribe tanto el portapapeles como la selección PRIMARY, por lo que el pegado con botón central funciona.
* **Windows y WSL**: PowerShell `Set-Clipboard`

Dentro de tmux también escribe en el búfer de pegado de tmux. Sobre SSH recurre a secuencias de escape OSC 52. Dentro de GNU screen, Claude Code copia selecciones largas al portapapeles también. Antes de v2.1.219, si copiaba una selección más larga que aproximadamente 570 caracteres, GNU screen imprimía texto en base64 en la ventana en su lugar. Claude Code imprime una notificación después de cada copia indicándole qué ruta utilizó.

Algunos terminales bloquean OSC 52 de forma predeterminada. iTerm2 lo bloquea hasta que activa Configuración → General → Selección → Las aplicaciones en el terminal pueden acceder al portapapeles; ejecutar [`/terminal-setup`](/docs/es/terminal-config) en iTerm2 lo habilita para usted.

Para una selección nativa de una sola vez, la tecla a utilizar depende de su terminal:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor y Devin Desktop**: `Shift`, u `Option` en macOS con la configuración `terminal.integrated.macOptionClickForcesSelection` habilitada
* **La mayoría de otros terminales**: `Shift`

Mantenga esa tecla presionada mientras hace clic y arrastra. Su terminal maneja la selección en sí en lugar de pasarla a Claude Code, por lo que los atajos de teclado de copia como `Cmd+C` funcionan en lo que selecciona. Claude Code también muestra la tecla correcta en su sugerencia en pantalla.

Sobre SSH o dentro de tmux, Claude Code no siempre puede detectar el terminal desde el que se está conectando, por lo que la sugerencia enumera las teclas candidatas en su lugar.

Si confía en la selección nativa todo el tiempo, establezca `CLAUDE_CODE_DISABLE_MOUSE=1` para optar por no participar en la captura del ratón mientras mantiene la representación sin parpadeo y la memoria plana:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Con la captura del ratón deshabilitada, el desplazamiento por teclado con `PgUp`, `PgDn`, `Ctrl+Home` y `Ctrl+End` sigue funcionando, y su terminal maneja la selección de forma nativa. Pierde la capacidad de hacer clic para posicionar el cursor, hacer clic para expandir la salida de herramientas, hacer clic en URL y desplazamiento de rueda dentro de Claude Code.

Para mantener el desplazamiento de rueda pero desactivar el manejo de clic, arrastre y desplazamiento, establezca `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1` en su lugar. Requiere Claude Code v2.1.195 o posterior. `CLAUDE_CODE_DISABLE_MOUSE` tiene prioridad cuando ambas variables están establecidas.

Con los clics deshabilitados, Claude Code sigue capturando el ratón, por lo que la rueda y el panel táctil desplazan la conversación pero los clics izquierdos no hacen nada dentro de Claude Code. Aún necesita mantener presionada la tecla de su terminal para la selección nativa de clic y arrastre. El clic derecho y el pegado con botón central continúan funcionando en terminales que los admiten.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  Texto obsoleto o mal colocado en la pantalla
</h3>

El renderizado a pantalla completa envía solo las celdas que cambiaron entre fotogramas. Algunos terminales, más comúnmente Windows Terminal y otros hosts respaldados por ConPTY, coalescan estas escrituras posicionadas incorrectamente y dejan fragmentos de salida anterior en la pantalla hasta que redimensiona la ventana.

Establezca [`CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1`](/docs/es/env-vars) para repintar cada celda en cada fotograma en lugar de enviar actualizaciones incrementales.

En Windows PowerShell:

```powershell theme={null}
$env:CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT = "1"
claude
```

En macOS o Linux:

```bash theme={null}
CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1 claude
```

En Windows, Claude Code ya habilita automáticamente el repintado completo para sesiones en segundo plano y [vista de agente](/docs/es/agent-view), por lo que solo necesita establecer la variable para una sesión interactiva a pantalla completa que inició directamente.

<h3 id="fullscreen-renderer-didnt-finish-starting">
  `Claude Code's fullscreen renderer didn't finish starting last time` aparece al inicio
</h3>

Si una sesión a pantalla completa en esta máquina se bloquea antes de haber comenzado correctamente, Claude Code inicia su próxima sesión en el renderizador clásico e imprime una de dos líneas. Una sesión ha comenzado correctamente una vez que ha dibujado su primer fotograma y luego ha permanecido activa durante 10 segundos o la ha finalizado con `/exit`, Ctrl+C o Ctrl+D. La línea que ve le indica qué hace Claude Code después de esta sesión:

* Después de un inicio fallido, ve `Claude Code's fullscreen renderer didn't finish starting last time on this machine`. Claude Code intenta el renderizado a pantalla completa nuevamente en la próxima sesión que inicie
* Después de dos inicios fallidos, ve `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`. Claude Code continúa usando el renderizador clásico hasta que actualice Claude Code o ejecute `/tui fullscreen`, y no imprime nada en esas sesiones posteriores

Para confirmar que un inicio fallido es la razón por la que está en el renderizador clásico, ejecute `/tui` sin argumentos. Mientras un inicio fallido sea la razón, la línea `Current renderer` lo indica.

Para mantener el renderizador clásico, ejecute `/tui default`, que guarda la configuración `tui` sin reiniciar. Para intentar el renderizado a pantalla completa nuevamente, ejecute `/tui fullscreen`. Si esa sesión tampoco termina de iniciarse, [reporte el problema](#research-preview).

Antes de v2.1.236, Claude Code continuaba iniciando sesiones en renderizado a pantalla completa después de un inicio fallido.

<h4 id="how-claude-code-counts-failed-starts">
  Cómo Claude Code cuenta inicios fallidos
</h4>

* Sesiones que cuentan: solo sesiones que comenzaron en renderizado a pantalla completa porque su configuración `tui` lo indica, porque aceptó el [diálogo de inicio](#fullscreen-by-default), o porque Claude Code lo inicia a pantalla completa de forma predeterminada
* `CLAUDE_CODE_NO_FLICKER=1`: si lo establece, Claude Code renderiza esa sesión a pantalla completa incluso después de un inicio fallido, y no lo cuenta
* Reinicio de conteo: Claude Code cuenta inicios fallidos por versión de Claude Code, y un inicio a pantalla completa exitoso reinicia el conteo
* Diálogo de inicio: si aceptó el diálogo y la sesión reiniciada se bloqueó, Claude Code no imprime ninguna línea y no muestra el diálogo nuevamente en esta versión de Claude Code

<h2 id="research-preview">
  Vista previa de investigación
</h2>

El renderizado a pantalla completa es una característica de vista previa de investigación. Ha sido probado en emuladores de terminal comunes, pero es posible que encuentre problemas de renderizado en terminales menos comunes o configuraciones inusuales.

Si encuentra un problema, ejecute `/feedback` dentro de Claude Code para reportarlo, o abra un problema en el [repositorio de GitHub de claude-code](https://github.com/anthropics/claude-code/issues). Incluya el nombre y la versión de su emulador de terminal.

Para desactivar el renderizado a pantalla completa, ejecute `/tui default`, o desactive `CLAUDE_CODE_NO_FLICKER` si lo habilitó de esa manera. Cuando vuelva a cambiar con `/tui default`, Claude Code puede mostrar primero un mensaje de retroalimentación opcional preguntando qué lo hizo cambiar. Escriba una razón y presione `Enter` para enviarla, o presione `Esc` para omitirla. La CLI se reinicia en el renderizador clásico de cualquier manera. Para forzar el renderizador clásico independientemente de la configuración `tui` guardada, establezca `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. El renderizador clásico mantiene la conversación en el desplazamiento nativo de su terminal para que `Cmd+f` y el modo de copia de tmux funcionen como de costumbre.

Las sesiones de fondo abiertas desde la [vista de agente](/docs/es/agent-view) o `claude attach` siempre utilizan renderizado a pantalla completa. La terminal adjunta entra en el búfer de pantalla alternativa para mostrar la sesión, y el renderizador clásico no tiene desplazamiento ni manejo del ratón allí, por lo que la configuración `tui` y `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` no se aplican a ellas.
