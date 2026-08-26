> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Renderizado a pantalla completa

> Habilite un modo de renderizado más suave y sin parpadeos con soporte de ratón y uso de memoria estable en conversaciones largas.

<Note>
  El renderizado a pantalla completa es una vista previa de investigación opcional ([research preview](#research-preview)). Ejecute `/tui fullscreen` para cambiar en su conversación actual. El comportamiento puede cambiar según los comentarios.
</Note>

El renderizado a pantalla completa es una ruta de renderizado alternativa para la CLI de Claude Code que elimina el parpadeo, mantiene el uso de memoria plano en conversaciones largas y añade soporte de ratón. Dibuja la interfaz en el búfer de pantalla alternativa de la terminal, como `vim` o `htop`, y solo renderiza los mensajes que están actualmente visibles. Esto reduce la cantidad de datos enviados a su terminal en cada actualización.

La diferencia es más notable en emuladores de terminal donde el rendimiento de renderizado es el cuello de botella, como la terminal integrada de VS Code, tmux e iTerm2. Si su posición de desplazamiento de terminal salta a la parte superior mientras Claude está trabajando, o la pantalla parpadea mientras la salida de herramientas se transmite, este modo aborda esos problemas.

<Note>
  El término pantalla completa describe cómo Claude Code se apodera de la superficie de dibujo de la terminal, de la manera que lo hace `vim`. No tiene nada que ver con maximizar su ventana de terminal, y funciona en cualquier tamaño de ventana.
</Note>

<h2 id="enable-fullscreen-rendering">
  Habilitar renderizado a pantalla completa
</h2>

Ejecute `/tui fullscreen` dentro de cualquier conversación de Claude Code. La CLI guarda la [configuración `tui`](/docs/es/settings#available-settings) y se reinicia en pantalla completa con su conversación intacta, por lo que puede cambiar a mitad de sesión sin perder contexto. Ejecute `/tui default` para volver al renderizador clásico, o `/tui` sin argumentos para imprimir qué renderizador está activo.

La sesión reiniciada mantiene la conversación tal como aparece en pantalla. Si ejecutó [`/rewind`](/docs/es/checkpointing#rewind-and-summarize) anteriormente en la sesión, el reinicio se reanuda desde el punto rebobinado en lugar de la transcripción más larga guardada en disco. Antes de v2.1.207, cambiar renderizadores después de un rebobinado restauraba la conversación que el rebobinado había eliminado.

También puede establecer la variable de entorno `CLAUDE_CODE_NO_FLICKER` antes de iniciar Claude Code:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

La configuración `tui` y la variable de entorno son equivalentes. El comando `/tui` borra `CLAUDE_CODE_NO_FLICKER` del proceso reiniciado para que la configuración que escribe tenga efecto.

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

El renderizado a pantalla completa captura eventos de ratón y los maneja dentro de Claude Code:

* **Haga clic en la entrada del indicador** para posicionar su cursor en cualquier lugar del texto que está escribiendo.
* **Haga clic en una sugerencia en la lista de comandos `/` o archivos `@`** para aceptarla. Pasar el ratón por encima resalta la fila bajo su cursor.
* **Haga clic en una opción en un menú de selección** para elegirla. Esto cubre solicitudes de permisos, `/model`, `/config` y otros diálogos que muestran una lista de opciones. Pasar el ratón por encima muestra un puntero en la fila bajo su cursor. Requiere Claude Code v2.1.187 o posterior.
* **Haga clic en una opción en un menú de selección múltiple** para alternarla, y haga clic en el botón de envío para confirmar sus opciones. Hacer clic en una fila de texto libre, como la fila `Other` en una pregunta de opción múltiple, enfoca su campo de entrada para que pueda escribir una respuesta. Requiere Claude Code v2.1.208 o posterior.
* **Haga clic en un resultado de herramienta contraído** para expandirlo y ver la salida completa. Haga clic nuevamente para contraerlo. La llamada de herramienta y su resultado se expanden juntos. Solo los mensajes que tienen más para mostrar son clicables.
* **Mantenga presionado `Cmd` en macOS, o `Ctrl` en Linux y Windows, y haga clic en una URL o ruta de archivo** para abrirla. Las rutas de archivo en la salida de herramientas, como las impresas después de una edición o escritura, se abren en su aplicación predeterminada. Las URLs simples `http://` y `https://` se abren en su navegador. A partir de v2.1.181, un clic simple sin mantener presionado `Cmd` o `Ctrl` ya no abre enlaces, coincidiendo con el comportamiento de terminal nativa. Algunos terminales de macOS reenvían `Cmd`+clic a la aplicación en ejecución en lugar de abrir el enlace ellos mismos, y el protocolo de ratón de terminal no tiene forma de codificar la tecla `Cmd`, por lo que Claude Code lo recibe como un clic simple. En Ghostty, y a partir de v2.1.198 en Warp en macOS, Claude Code detecta esto y permite que un clic simple en un enlace lo abra, y mantener presionado `Cmd` sigue funcionando. En la terminal integrada de VS Code y terminales similares basadas en xterm.js, Claude Code se remite al manejador de enlaces propio de la terminal, que utiliza el mismo gesto.
* **Haga clic y arrastre** para seleccionar texto en cualquier lugar de la conversación. El doble clic selecciona una palabra, coincidiendo con los límites de palabras de iTerm2 para que una ruta de archivo se seleccione como una unidad. A partir de v2.1.198, el doble clic en una URL selecciona la URL completa, incluido el esquema. El triple clic selecciona la línea.
* **Desplácese con la rueda del ratón** para moverse a través de la conversación.

El texto seleccionado se copia a su portapapeles automáticamente al soltar el ratón. Para desactivar esto, alterne Copiar al seleccionar en `/config`.

Con Copiar al seleccionar desactivado, presione `Ctrl+Shift+c` para copiar manualmente. En terminales que admiten el protocolo de teclado kitty, como kitty, WezTerm, Ghostty e iTerm2, `Cmd+c` también funciona. Si tiene una selección activa, `Ctrl+c` copia en lugar de cancelar.

Con una selección activa, mantenga presionada `Shift` y presione las teclas de flecha para extenderla desde el teclado. `Shift+↑` y `Shift+↓` desplazan la ventana gráfica cuando la selección alcanza el borde superior o inferior. `Shift+Home` y `Shift+End` extienden hasta el inicio o final de la línea actual.

<h2 id="scroll-the-conversation">
  Desplazarse por la conversación
</h2>

El renderizado a pantalla completa maneja el desplazamiento dentro de la aplicación. Use estos atajos de teclado para navegar:

| Atajo de teclado | Acción                                                         |
| :--------------- | :------------------------------------------------------------- |
| `PgUp` / `PgDn`  | Desplazarse hacia arriba o hacia abajo media pantalla          |
| `Ctrl+Home`      | Saltar al inicio de la conversación                            |
| `Ctrl+End`       | Saltar al último mensaje y reactivar el seguimiento automático |
| Rueda del ratón  | Desplazarse algunas líneas a la vez                            |

En teclados sin teclas dedicadas `PgUp`, `PgDn`, `Home` o `End`, como teclados de MacBook, mantenga presionada `Fn` con las teclas de flecha: `Fn+↑` envía `PgUp`, `Fn+↓` envía `PgDn`, `Fn+←` envía `Home`, y `Fn+→` envía `End`. `Ctrl+Fn+→` no llega a Claude Code en macOS, por lo que un teclado de MacBook no tiene un atajo de teclado funcional para saltar al final por defecto. En su lugar, use una de estas opciones:

* Haga clic en el [botón de saltar al final](#auto-follow).
* Desplácese hacia abajo con la rueda del ratón para reanudar el seguimiento.
* Reenlace `scroll:bottom` a un atajo de teclado que su teclado pueda enviar.

Estas acciones se pueden reenlazar. Consulte [Acciones de desplazamiento](/docs/es/keybindings#scroll-actions) para obtener la lista completa de nombres de acciones, incluidas variantes de media página y página completa que no tienen enlace predeterminado.

<h3 id="auto-follow">
  Seguimiento automático
</h3>

El desplazamiento hacia arriba pausa el seguimiento automático para que la nueva salida no lo devuelva al final. Un botón `Saltar al final` flota sobre el borde inferior de la transcripción mientras está desplazado hacia arriba, y muestra un recuento como `3 nuevos mensajes` cuando llega nueva salida. Haga clic en él, presione `Ctrl+End`, o desplácese hacia abajo para reanudar el seguimiento.

Mientras el seguimiento automático está pausado, la vista también permanece donde la desplazó cuando una respuesta termina de transmitirse. Antes de v2.1.207, la vista podría saltar por encima del inicio de la respuesta cuando una respuesta larga terminaba de transmitirse.

La sugerencia de teclado del botón refleja lo que su teclado puede enviar. En macOS sugiere hacer clic, o `Fn+↓` para desplazarse, porque `Ctrl+End` no llega a Claude Code desde un teclado Mac. Reenlace [`scroll:bottom`](/docs/es/keybindings#scroll-actions) y el botón muestra su atajo de teclado en todas las plataformas. Antes de v2.1.206, el botón sugería `Ctrl+End` en macOS.

En una terminal demasiado estrecha para la etiqueta completa, el botón acorta la sugerencia en lugar de envolver en la fila de transcripción debajo. Antes de v2.1.206, una etiqueta larga podría envolver sobre la transcripción.

Para desactivar completamente el seguimiento automático para que la vista permanezca donde la deje, abra `/config` y establezca Auto-scroll en desactivado. Con el desplazamiento automático desactivado, la vista nunca salta al final por sí sola. Los avisos de permiso y otros diálogos que necesitan una respuesta aún se desplazan a la vista independientemente de esta configuración.

<h3 id="mouse-wheel-scrolling">
  Desplazamiento de la rueda del ratón
</h3>

El desplazamiento de la rueda del ratón requiere que su terminal reenvíe eventos de ratón a Claude Code. La mayoría de terminales hacen esto siempre que una aplicación lo solicite. iTerm2 lo convierte en una configuración por perfil: si la rueda no hace nada pero `PgUp` y `PgDn` funcionan, abra Configuración → Perfiles → Terminal y active Habilitar informe de ratón. La misma configuración también es necesaria para que funcionen el clic para expandir y la selección de texto.

Si el desplazamiento de la rueda del ratón se siente lento, su terminal puede estar enviando un evento de desplazamiento por muesca física sin multiplicador. Algunas terminales, como Ghostty e iTerm2 con desplazamiento más rápido habilitado, ya amplifican eventos de rueda. Otros, incluida la terminal integrada de VS Code, envían exactamente un evento por muesca. Claude Code no puede detectar cuál.

Establezca `CLAUDE_CODE_SCROLL_SPEED` para multiplicar la distancia de desplazamiento base:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Un valor de `3` coincide con el predeterminado en `vim` y aplicaciones similares. La configuración acepta valores de 1 a 20, y valores fraccionarios por debajo de 1 como `0.5` para ralentizar el desplazamiento acelerado del trackpad y la rueda en terminales que ya amplifican eventos de rueda.

Para ajustar la velocidad de desplazamiento de forma interactiva, ejecute `/scroll-speed`. El diálogo muestra una regla que puede desplazar mientras está abierto para que pueda sentir el cambio inmediatamente. Presione `←` y `→` para ajustar, `r` para restablecer al valor predeterminado detectado automáticamente, e `Intro` para guardar.

El comando escribe el mismo valor que establece la variable de entorno `CLAUDE_CODE_SCROLL_SPEED`, persistido en `~/.claude/settings.json`. El comando no está disponible en la terminal del IDE de JetBrains.

Independientemente de la velocidad base, Claude Code acelera la velocidad de desplazamiento cuando gira la rueda rápidamente, por lo que un giro rápido cubre más distancia que el mismo número de muescas lentas. Para desactivar la aceleración y mantener una velocidad constante por muesca, establezca `wheelScrollAccelerationEnabled` en `false` en [`settings.json`](/docs/es/settings#available-settings). Esta configuración requiere Claude Code v2.1.174 o posterior.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Desplazamiento en la terminal del IDE de JetBrains
</h3>

En la terminal del IDE de JetBrains, Claude Code aplica su propio manejo de desplazamiento e ignora `CLAUDE_CODE_SCROLL_SPEED`. La terminal envía eventos de desplazamiento a una velocidad mucho más alta que otros emuladores, por lo que un multiplicador ajustado en otro lugar se excede aquí.

En 2025.2, la terminal también tiene errores de desplazamiento de rueda que producen teclas de flecha espurias y eventos de dirección incorrecta. Claude Code detecta estos en tiempo de ejecución y los mitiga automáticamente, por lo que el desplazamiento del trackpad y la rueda del ratón funcionan sin configuración. Para la mejor experiencia de desplazamiento, actualice a 2025.3 o posterior. Claude Code muestra una sugerencia la primera vez que se desplaza si detecta el error.

<h2 id="search-and-review-the-conversation">
  Buscar y revisar la conversación
</h2>

`Ctrl+o` alterna entre el indicador normal y el modo de transcripción.

Para una vista más tranquila que muestre solo su último indicador, un resumen de una línea de llamadas de herramientas con estadísticas de diferencias de edición y la respuesta final, ejecute `/focus`. La configuración persiste entre sesiones. Ejecute `/focus` nuevamente para desactivarla.

El modo de transcripción gana navegación y búsqueda de estilo `less`:

| Tecla                               | Acción                                                                                                                                    |
| :---------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `/`                                 | Abrir búsqueda. Escriba para encontrar coincidencias, `Enter` para aceptar, `Esc` para cancelar y restaurar su posición de desplazamiento |
| `n` / `N`                           | Saltar a la siguiente o anterior coincidencia. Funciona después de cerrar la barra de búsqueda                                            |
| `j` / `k` o `↑` / `↓`               | Desplazarse una línea                                                                                                                     |
| `g` / `G` o `Home` / `End`          | Saltar al inicio o final                                                                                                                  |
| `Ctrl+u` / `Ctrl+d`                 | Desplazarse media página                                                                                                                  |
| `Ctrl+b` / `Ctrl+f` o `Space` / `b` | Desplazarse una página completa                                                                                                           |
| `Ctrl+o`, `Esc`, o `q`              | Salir del modo de transcripción y volver al indicador                                                                                     |

El `Cmd+f` de su terminal y la búsqueda de tmux no ven la conversación porque vive en el búfer de pantalla alternativa, no en el desplazamiento nativo. Para devolver el contenido a su terminal, presione `Ctrl+o` para entrar en modo de transcripción primero, luego:

* **`[`**: escribe la conversación completa en el búfer de desplazamiento nativo de su terminal, con toda la salida de herramientas expandida. La conversación es ahora texto ordinario en su terminal, por lo que `Cmd+f`, modo de copia de tmux y cualquier otra herramienta nativa pueden buscar o seleccionarla. Las sesiones largas pueden pausarse por un momento mientras esto sucede. Esto dura hasta que salga del modo de transcripción con `Esc` o `q`, que lo devuelve al renderizado a pantalla completa. El siguiente `Ctrl+o` comienza de nuevo.
* **`v`**: escribe la conversación en un archivo temporal y la abre en `$VISUAL` o `$EDITOR`.

Presione `Esc` o `q` para volver al indicador.

<h2 id="clear-the-conversation">
  Limpiar la conversación
</h2>

Presione `Ctrl+L` dos veces dentro de dos segundos para ejecutar `/clear` e iniciar una nueva conversación. El primer pulso redibuja la pantalla y muestra una sugerencia; el segundo pulso borra la conversación. En macOS, presionar dos veces `Cmd+K` también ejecuta `/clear`.

<h2 id="use-with-tmux">
  Usar con tmux
</h2>

El renderizado a pantalla completa funciona dentro de tmux, con tres advertencias.

El desplazamiento de la rueda del ratón requiere el modo de ratón de tmux. Si su `~/.tmux.conf` no lo habilita ya, agregue esta línea y recargue su configuración:

```bash theme={null}
set -g mouse on
```

Sin modo de ratón, los eventos de rueda van a tmux en lugar de Claude Code. El desplazamiento de teclado con `PgUp` y `PgDn` funciona de cualquier manera. Claude Code imprime una sugerencia única al inicio si detecta tmux con modo de ratón desactivado.

El renderizado a pantalla completa es incompatible con el modo de integración de tmux de iTerm2, que es el modo en el que entra con `tmux -CC`. En modo de integración, iTerm2 renderiza cada panel de tmux como una división nativa en lugar de permitir que tmux dibuje en la terminal. El búfer de pantalla alternativa y el seguimiento de ratón no funcionan correctamente allí: la rueda del ratón no hace nada, y el doble clic puede corromper el estado de la terminal. No habilite el renderizado a pantalla completa en sesiones `tmux -CC`. El tmux regular dentro de iTerm2, sin `-CC`, funciona bien.

No todas las versiones de tmux aplican salida sincronizada desde aplicaciones, por lo que puede ver más parpadeo durante los redibujados bajo tmux que cuando ejecuta Claude Code directamente en su terminal. Si el parpadeo es notable, especialmente a través de SSH, actualice a la versión más reciente de tmux o ejecute Claude Code en su propia pestaña de terminal fuera de tmux. Verifique su versión de tmux con `tmux -V`.

Claude Code activa la salida sincronizada automáticamente cuando detecta tmux 3.4 o posterior desde la variable `TERM_PROGRAM_VERSION`, y recurre a consultar la terminal directamente para obtener soporte de salida sincronizada cuando no se puede determinar la versión. Si los redibujados realmente se vuelven atómicos depende de que su versión de tmux respete la salida sincronizada; si aún ve parpadeo bajo tmux 3.4 o posterior, actualice a la versión más reciente de tmux. Esta detección requiere Claude Code v2.1.200 o posterior.

<h2 id="keep-native-text-selection">
  Mantener la selección de texto nativa
</h2>

La captura de ratón es el punto de fricción más común, especialmente sobre SSH o dentro de tmux. Cuando Claude Code captura eventos de ratón, la copia nativa al seleccionar de su terminal deja de funcionar. La selección que realiza con clic y arrastre existe dentro de Claude Code, no en el búfer de selección de su terminal, por lo que el modo de copia de tmux, sugerencias de Kitty y herramientas similares no la ven.

Claude Code escribe la selección en su portapapeles del sistema, y la ruta que utiliza depende de su configuración. En una sesión local ejecuta una herramienta de portapapeles nativa:

* **macOS**: `pbcopy`
* **Linux**: `wl-copy` en Wayland, o `xclip` o `xsel` en X11, lo que esté instalado. Claude Code escribe tanto el portapapeles como la selección PRIMARY, por lo que el pegado con botón central funciona.
* **Windows y WSL**: PowerShell `Set-Clipboard`

Dentro de tmux también escribe en el búfer de pegado de tmux. Sobre SSH se vuelve a secuencias de escape OSC 52. Claude Code imprime un aviso después de cada copia diciéndole qué ruta utilizó.

Algunos terminales bloquean OSC 52 de forma predeterminada. iTerm2 lo bloquea hasta que active Configuración → General → Selección → Las aplicaciones en el terminal pueden acceder al portapapeles; ejecutar [`/terminal-setup`](/docs/es/terminal-config) en iTerm2 habilita esto para usted.

Para una selección nativa puntual, la tecla a utilizar depende de su terminal:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor y Devin Desktop**: `Shift`, u `Option` en macOS con la configuración `terminal.integrated.macOptionClickForcesSelection` habilitada
* **La mayoría de otros terminales**: `Shift`

Mantenga presionada esa tecla mientras hace clic y arrastra. Su terminal maneja la selección por sí mismo en lugar de pasarla a Claude Code, por lo que atajos de copia como `Cmd+C` funcionan en lo que selecciona. Claude Code también muestra la tecla correcta en su sugerencia en pantalla.

Sobre SSH o dentro de tmux, Claude Code no siempre puede detectar el terminal desde el que se está conectando, por lo que la sugerencia enumera las teclas candidatas en su lugar.

Si confía en la selección nativa todo el tiempo, establezca `CLAUDE_CODE_DISABLE_MOUSE=1` para optar por no participar en la captura de ratón mientras mantiene el renderizado sin parpadeos y la memoria plana:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Con la captura de ratón desactivada, el desplazamiento de teclado con `PgUp`, `PgDn`, `Ctrl+Home` y `Ctrl+End` aún funciona, y su terminal maneja la selección de forma nativa. Pierde clic para posicionar el cursor, clic para expandir la salida de herramientas, clic en URL y desplazamiento de rueda dentro de Claude Code.

Para mantener el desplazamiento de rueda pero desactivar el manejo de clic, arrastre y desplazamiento, establezca `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1` en su lugar. Requiere Claude Code v2.1.195 o posterior. `CLAUDE_CODE_DISABLE_MOUSE` tiene prioridad cuando ambas variables están establecidas.

Con los clics desactivados, Claude Code aún captura el ratón, por lo que la rueda y el panel táctil desplazan la conversación pero los clics izquierdos no hacen nada dentro de Claude Code. Aún necesita mantener presionada la tecla de su terminal para la selección nativa de clic y arrastre. El clic derecho y el pegado con botón central continúan funcionando en terminales que los admiten.

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

<h2 id="research-preview">
  Vista previa de investigación
</h2>

El renderizado a pantalla completa es una característica de vista previa de investigación. Ha sido probado en emuladores de terminal comunes, pero puede encontrar problemas de renderizado en terminales menos comunes o configuraciones inusuales.

Si encuentra un problema, ejecute `/feedback` dentro de Claude Code para reportarlo, o abra un problema en el [repositorio de GitHub de claude-code](https://github.com/anthropics/claude-code/issues). Incluya el nombre y la versión de su emulador de terminal.

Para desactivar el renderizado a pantalla completa, ejecute `/tui default`, o desestablezca `CLAUDE_CODE_NO_FLICKER` si la habilitó de esa manera. Para forzar el renderizador clásico independientemente de la configuración `tui` guardada, establezca `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. El renderizador clásico mantiene la conversación en el desplazamiento nativo de su terminal para que `Cmd+f` y el modo de copia de tmux funcionen como de costumbre.

Las sesiones en segundo plano abiertas desde la [vista de agente](/docs/es/agent-view) o `claude attach` siempre utilizan renderizado a pantalla completa. La terminal adjunta entra en el búfer de pantalla alternativa para mostrar la sesión, y el renderizador clásico no tiene desplazamiento ni manejo del ratón allí, por lo que la configuración `tui` y `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` no se aplican a ellas.
