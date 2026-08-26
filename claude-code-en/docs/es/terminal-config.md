> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configura tu terminal para Claude Code

> Corrige Shift+Enter para saltos de línea, obtén una campana de terminal cuando Claude termine, configura tmux, haz coincidir el tema de color y habilita el modo Vim en la CLI de Claude Code.

Claude Code funciona en cualquier terminal sin configuración. Esta página es para cuando algo específico no se comporta como esperas. Encuentra tu síntoma a continuación. Si todo ya se siente bien, no necesitas esta página.

* [Shift+Enter envía en lugar de insertar un salto de línea](#enter-multiline-prompts)
* [Los atajos de tecla Option no funcionan en macOS](#enable-option-key-shortcuts-on-macos)
* [Sin sonido ni alerta cuando Claude termina](#get-a-terminal-bell-or-notification)
* [Ejecutas Claude Code dentro de tmux](#configure-tmux)
* [La pantalla parpadea o el desplazamiento salta](#switch-to-fullscreen-rendering)
* [Quieres teclas Vim en el indicador](#edit-prompts-with-vim-keybindings)

Esta página trata sobre lograr que tu terminal envíe las señales correctas a Claude Code. Para cambiar qué teclas responde Claude Code, consulta [atajos de teclado](/docs/es/keybindings) en su lugar.

<h2 id="enter-multiline-prompts">
  Ingresa indicadores multilínea
</h2>

Presionar Enter envía tu mensaje. Para agregar un salto de línea sin enviar, presiona Ctrl+J, o escribe `\` y luego presiona Enter. Ambos funcionan en cada terminal sin configuración.

En la mayoría de terminales también puedes presionar Shift+Enter, pero el soporte varía según el emulador de terminal:

| Terminal                                                                | Shift+Enter para salto de línea             |
| :---------------------------------------------------------------------- | :------------------------------------------ |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal | Funciona sin configuración                  |
| VS Code, Cursor, Devin Desktop, Alacritty, Zed                          | Ejecuta `/terminal-setup` una vez           |
| gnome-terminal, IDEs de JetBrains como PyCharm y Android Studio         | No disponible; usa Ctrl+J o `\` luego Enter |

Para VS Code, Cursor, Devin Desktop, Alacritty y Zed, `/terminal-setup` escribe Shift+Enter y otros atajos de teclado en el archivo de configuración de la terminal. Los enlaces existentes se dejan en su lugar; si ves un mensaje como `VSCode terminal Shift+Enter key binding already configured`, no se realizó ningún cambio. Ejecuta `/terminal-setup` directamente en la terminal del host en lugar de dentro de tmux o screen, ya que necesita escribir en la configuración de la terminal del host.

En VS Code, Cursor y Devin Desktop, `/terminal-setup` también actualiza dos configuraciones del editor: establece `terminal.integrated.gpuAcceleration` en `"off"` para evitar texto distorsionado en la terminal integrada, y establece `terminal.integrated.mouseWheelScrollSensitivity` para un desplazamiento más suave en [modo pantalla completa](/docs/es/fullscreen). Para deshacer el cambio de aceleración de GPU, establécelo de nuevo en `"auto"` y recarga la ventana del editor.

Si estás ejecutando dentro de tmux, Shift+Enter también requiere la [configuración de tmux a continuación](#configure-tmux) incluso cuando la terminal externa la soporta.

Para vincular salto de línea a una tecla diferente, o para intercambiar el comportamiento de modo que Enter inserte un salto de línea y Shift+Enter envíe, mapea las acciones `chat:newline` y `chat:submit` en tu [archivo de atajos de teclado](/docs/es/keybindings).

<h2 id="enable-option-key-shortcuts-on-macos">
  Habilita atajos de tecla Option en macOS
</h2>

Algunos atajos de Claude Code usan la tecla Option, como Option+Enter para un salto de línea u Option+P para cambiar modelos. En macOS, la mayoría de terminales no envían Option como modificador por defecto, por lo que estos atajos no hacen nada hasta que lo habilites. La configuración de terminal para esto generalmente se etiqueta como "Use Option as Meta Key"; Meta es el nombre histórico de Unix para la tecla ahora etiquetada como Option o Alt.

<Tabs>
  <Tab title="Apple Terminal">
    Abre Configuración → Perfiles → Teclado y marca "Use Option as Meta Key".

    Si aceptaste el indicador de primera ejecución de Claude Code que ofrecía "Option+Enter para saltos de línea y campana visual", esto ya está hecho. Ese indicador ejecuta `/terminal-setup` para ti, que habilita Option como Meta y cambia la campana de audio a un destello de pantalla visual en tu perfil de Apple Terminal.
  </Tab>

  <Tab title="iTerm2">
    Abre Configuración → Perfiles → Teclas → General y establece la tecla Option Izquierda y la tecla Option Derecha en "Esc+".

    Ejecutar `/terminal-setup` en iTerm2 habilita "Applications in terminal may access clipboard" en Configuración → General → Selection para que el comando `/copy` pueda escribir en tu portapapeles del sistema. El comando detecta iTerm2 incluso cuando se ejecuta desde dentro de tmux. Reinicia iTerm2 para que el cambio surta efecto.
  </Tab>

  <Tab title="VS Code">
    Agrega `"terminal.integrated.macOptionIsMeta": true` a tu configuración de VS Code.
  </Tab>
</Tabs>

Para Ghostty, Kitty y otras terminales, busca una configuración de Option-as-Alt u Option-as-Meta en el archivo de configuración de la terminal.

<h2 id="get-a-terminal-bell-or-notification">
  Obtén una campana de terminal o notificación
</h2>

Cuando Claude termina una tarea o se pausa para un indicador de permiso, dispara un evento de notificación. Mostrar esto como una campana de terminal o notificación de escritorio te permite cambiar a otro trabajo mientras se ejecuta una tarea larga.

Por defecto, Claude Code envía una notificación de escritorio solo en Ghostty, Kitty e iTerm2. En otras terminales, establezca [`preferredNotifChannel`](/docs/es/settings#available-settings) en `"terminal_bell"` para sonar la campana de terminal en su lugar, o configure un [gancho de Notificación](#play-a-sound-with-a-notification-hook) para un sonido personalizado o comando.

La notificación de escritorio llega a su máquina local a través de SSH, por lo que una sesión remota aún puede alertarle. Ghostty y Kitty la reenvían a su centro de notificaciones del SO sin configuración adicional. iTerm2 requiere que habilite el reenvío:

<Steps>
  <Step title="Abra la configuración de notificaciones de iTerm2">
    Vaya a Configuración → Perfiles → Terminal.
  </Step>

  <Step title="Habilite alertas">
    Marque "Notification Center Alerts", luego haga clic en "Filter Alerts" y habilite "Send escape sequence-generated alerts".
  </Step>
</Steps>

Si las notificaciones aún no aparecen, confirme que su aplicación de terminal tenga permiso de notificación en su configuración del SO, y si está ejecutando dentro de tmux, [habilite passthrough](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Reproduzca un sonido con un gancho de Notificación
</h3>

En cualquier terminal puede configurar un [gancho de Notificación](/docs/es/hooks-guide#get-notified-when-claude-needs-input) para reproducir un sonido o ejecutar un comando personalizado cuando Claude necesite su atención. Los ganchos se ejecutan junto con la notificación de escritorio en lugar de reemplazarla, por lo que las terminales que no reciben una notificación de escritorio, como Warp o la terminal integrada de VS Code, pueden usar un gancho o establecer `preferredNotifChannel` en `"terminal_bell"` en su lugar.

El ejemplo a continuación reproduce un sonido del sistema en macOS. La guía vinculada tiene comandos de notificación de escritorio para macOS, Linux y Windows.

```json ~/.claude/settings.json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "hooks": [{ "type": "command", "command": "afplay /System/Library/Sounds/Glass.aiff" }]
      }
    ]
  }
}
```

<h2 id="configure-tmux">
  Configura tmux
</h2>

Cuando Claude Code se ejecuta dentro de tmux, dos cosas se rompen por defecto: Shift+Enter envía en lugar de insertar un salto de línea, y las notificaciones de escritorio y la [barra de progreso](/docs/es/settings#available-settings) nunca llegan a la terminal externa. Agrega estas líneas a `~/.tmux.conf`, luego ejecuta `tmux source-file ~/.tmux.conf` para aplicarlas al servidor en ejecución:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

La línea `allow-passthrough` permite que las notificaciones y actualizaciones de progreso lleguen a la terminal externa en lugar de ser tragadas por tmux. Las líneas `extended-keys` permiten que tmux distinga Shift+Enter de Enter simple para que el atajo de salto de línea funcione.

<h2 id="match-the-color-theme">
  Haz coincidir el tema de color
</h2>

Usa el comando `/theme`, o el selector de tema en `/config`, para elegir un tema de Claude Code que coincida con tu terminal. Seleccionar la opción auto detecta el fondo claro u oscuro de tu terminal, por lo que el tema sigue los cambios de apariencia del SO siempre que tu terminal lo haga. Claude Code no controla el esquema de color de la terminal, que se establece por la aplicación de terminal.

Para personalizar lo que aparece en la parte inferior de la interfaz, configura una [línea de estado personalizada](/docs/es/statusline) que muestre el modelo actual, directorio de trabajo, rama de git u otro contexto.

<h3 id="create-a-custom-theme">
  Crea un tema personalizado
</h3>

<Note>
  Los temas personalizados requieren Claude Code v2.1.118 o posterior.
</Note>

Además de los preajustes integrados, `/theme` enumera cualquier tema personalizado que hayas definido y cualquier tema contribuido por los [plugins](/docs/es/plugins-reference#themes) instalados. Selecciona **Nuevo tema personalizado…** al final de la lista para crear uno de forma interactiva: nombras el tema y luego seleccionas tokens de color individuales para anular. Presiona `Ctrl+E` mientras un tema personalizado está resaltado para editarlo.

Cada tema personalizado es un archivo JSON en `~/.claude/themes/`. El nombre de archivo sin la extensión `.json` es el slug del tema, y seleccionar el tema almacena `custom:<slug>` como tu preferencia de tema. El archivo tiene tres campos opcionales:

| Campo       | Tipo   | Descripción                                                                                                                                                   |
| :---------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`      | string | Etiqueta de visualización mostrada en `/theme`. Por defecto es el slug del nombre de archivo                                                                  |
| `base`      | string | Preajuste integrado desde el que comienza el tema: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi`, o `light-ansi`. Por defecto es `dark` |
| `overrides` | object | Mapa de nombres de tokens de color a valores de color. Los tokens no enumerados aquí se transfieren al preajuste base                                         |

Los valores de color aceptan `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, o `ansi:<name>` donde `<name>` es uno de los 16 nombres de color ANSI estándar como `red` o `cyanBright`. Los tokens desconocidos y los valores de color inválidos se ignoran, por lo que un error tipográfico no puede romper la representación.

El siguiente ejemplo define un tema que mantiene el preajuste oscuro pero recolora el acento del prompt, el texto de error y el texto de éxito:

```json ~/.claude/themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Claude Code observa `~/.claude/themes/` y recarga cuando un archivo cambia, por lo que las ediciones realizadas en tu editor se aplican a una sesión en ejecución sin necesidad de reiniciar.

La referencia a continuación cubre los tokens que puede establecer en `overrides`. El editor interactivo en `/theme` muestra los mismos tokens con una vista previa en vivo, además de algunos acentos de propósito único como colores de pantalla de incorporación que se omiten aquí.

<Accordion title="Referencia de tokens de color">
  El siguiente ejemplo combina tokens de varios de los grupos a continuación: el acento de marca, el borde del modo plan, los fondos de diff y el fondo del mensaje de pantalla completa.

  ```json ~/.claude/themes/midnight.json theme={null}
  {
    "name": "Midnight",
    "base": "dark",
    "overrides": {
      "claude": "#a78bfa",
      "planMode": "#38bdf8",
      "diffAdded": "#14532d",
      "diffRemoved": "#7f1d1d",
      "userMessageBackground": "#1e1b4b"
    }
  }
  ```

  <h4 id="text-and-accent-colors">
    Colores de texto y acento
  </h4>

  Controla el acento de marca principal y los matices de texto de primer plano utilizados en toda la interfaz.

  | Token         | Controla                                                                         |
  | :------------ | :------------------------------------------------------------------------------- |
  | `claude`      | Acento de marca principal, utilizado para el spinner y la etiqueta del asistente |
  | `text`        | Texto de primer plano predeterminado                                             |
  | `inverseText` | Texto dibujado sobre un fondo de color, como insignias de estado                 |
  | `inactive`    | Texto secundario como sugerencias, marcas de tiempo y elementos deshabilitados   |
  | `subtle`      | Bordes tenues y texto secundario de énfasis reducido                             |
  | `suggestion`  | Sugerencias de autocompletado y resaltado de selección en selectores             |
  | `permission`  | Bordes de diálogo, incluidas solicitudes de permiso y selectores                 |
  | `remember`    | Indicadores de memoria y `CLAUDE.md`                                             |

  <h4 id="status-colors">
    Colores de estado
  </h4>

  Señala estados de éxito, fallo y advertencia en mensajes e indicadores.

  | Token     | Controla                                                      |
  | :-------- | :------------------------------------------------------------ |
  | `success` | Mensajes de éxito y comprobaciones aprobadas                  |
  | `error`   | Mensajes de error y fallos                                    |
  | `warning` | Advertencias, mensajes de precaución y el borde del modo auto |
  | `merged`  | Estado de solicitud de extracción fusionada                   |

  <h4 id="input-box-and-mode-indicators">
    Cuadro de entrada e indicadores de modo
  </h4>

  Establece el color del borde del cuadro de entrada y el acento mostrado mientras un modo de permiso o indicador está activo.

  | Token          | Controla                                                         |
  | :------------- | :--------------------------------------------------------------- |
  | `promptBorder` | Borde del cuadro de entrada en el modo de permiso predeterminado |
  | `planMode`     | Acento y borde del modo plan                                     |
  | `autoAccept`   | Acento y borde del modo aceptar ediciones                        |
  | `bashBorder`   | Borde del cuadro de entrada al ingresar un comando de shell `!`  |
  | `ide`          | Indicador de conexión IDE                                        |
  | `fastMode`     | Indicador del modo rápido                                        |

  <h4 id="diff-rendering">
    Representación de diff
  </h4>

  Colorea el código añadido y eliminado en ediciones y revisiones de archivos.

  | Token               | Controla                                                   |
  | :------------------ | :--------------------------------------------------------- |
  | `diffAdded`         | Fondo de líneas añadidas                                   |
  | `diffRemoved`       | Fondo de líneas eliminadas                                 |
  | `diffAddedDimmed`   | Fondo de contexto sin cambios cerca de líneas añadidas     |
  | `diffRemovedDimmed` | Fondo de contexto sin cambios cerca de líneas eliminadas   |
  | `diffAddedWord`     | Resaltado a nivel de palabra dentro de una línea añadida   |
  | `diffRemovedWord`   | Resaltado a nivel de palabra dentro de una línea eliminada |

  <h4 id="fullscreen-mode">
    Modo de pantalla completa
  </h4>

  Se aplica solo en [modo de representación de pantalla completa](/docs/es/fullscreen), donde los mensajes tienen un relleno de fondo.

  | Token                        | Controla                                                                       |
  | :--------------------------- | :----------------------------------------------------------------------------- |
  | `userMessageBackground`      | Fondo detrás de tus mensajes en la transcripción                               |
  | `userMessageBackgroundHover` | Fondo detrás de un mensaje mientras está desplazado o expandido                |
  | `messageActionsBackground`   | Fondo detrás del mensaje seleccionado cuando la barra de acciones está abierta |
  | `bashMessageBackgroundColor` | Fondo detrás de entradas de comando de shell `!` en la transcripción           |
  | `memoryBackgroundColor`      | Fondo detrás de entradas de memoria `#` en la transcripción                    |
  | `selectionBg`                | Fondo del texto seleccionado con el ratón                                      |

  <h4 id="usage-meter-and-speaker-labels">
    Medidor de uso y etiquetas de altavoz
  </h4>

  Ajusta la barra mostrada en la vista `/usage` y las etiquetas que distinguen tus mensajes de los de Claude.

  | Token              | Controla                                                    |
  | :----------------- | :---------------------------------------------------------- |
  | `rate_limit_fill`  | Porción llena del medidor de uso                            |
  | `rate_limit_empty` | Porción vacía del medidor de uso                            |
  | `briefLabelYou`    | Color de la etiqueta `You` en tus mensajes                  |
  | `briefLabelClaude` | Color de la etiqueta `Claude` en los mensajes del asistente |

  <h4 id="shimmer-variants-and-subagent-colors">
    Variantes de shimmer y colores de subagentes
  </h4>

  Varios tokens tienen una variante de shimmer emparejada que proporciona el color más claro utilizado en el gradiente animado del spinner. Anula el shimmer junto con su token base si la animación se ve desajustada.

  * `claude` y `claudeShimmer`
  * `warning` y `warningShimmer`
  * `permission` y `permissionShimmer`
  * `promptBorder` y `promptBorderShimmer`
  * `inactive` e `inactiveShimmer`
  * `fastMode` y `fastModeShimmer`

  Cada [subagente](/docs/es/sub-agents) y tarea paralela se muestra en uno de ocho colores nombrados para que puedas distinguirlos en la transcripción. Los nombres de los tokens siguen el patrón `<color>_FOR_SUBAGENTS_ONLY`, donde `<color>` es `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, o `cyan`. Anula estos para cambiar el aspecto de cada color nombrado. Por ejemplo, un subagente con `color: blue` en su definición se dibuja usando el valor `blue_FOR_SUBAGENTS_ONLY`.

  Las palabras clave [`ultrathink`](/docs/es/model-config#use-ultrathink-for-one-off-deep-reasoning) y [`ultraplan`](/docs/es/ultraplan) en la entrada del prompt se representan con un gradiente arcoíris de siete colores. Los nombres de los tokens siguen el patrón `rainbow_<color>` y `rainbow_<color>_shimmer`, donde `<color>` es `red`, `orange`, `yellow`, `green`, `blue`, `indigo`, o `violet`.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Cambia a renderizado a pantalla completa
</h2>

Si la pantalla parpadea o la posición de desplazamiento salta mientras Claude está trabajando, cambia al [modo de renderizado a pantalla completa](/docs/es/fullscreen). Dibuja en una pantalla separada que la terminal reserva para aplicaciones a pantalla completa en lugar de agregar a tu desplazamiento normal, lo que mantiene el uso de memoria plano y agrega soporte de ratón para desplazamiento y selección. En este modo desplazas con el ratón o PageUp dentro de Claude Code en lugar de con el desplazamiento nativo de tu terminal; consulta la [página de pantalla completa](/docs/es/fullscreen#search-and-review-the-conversation) para saber cómo buscar y copiar.

Ejecuta `/tui fullscreen` para cambiar y guardar la preferencia. Tu conversación se reinicia intacta y las futuras sesiones comienzan en pantalla completa. También puedes establecer la variable de entorno `CLAUDE_CODE_NO_FLICKER` antes de iniciar Claude Code:

<CodeGroup>
  ```bash Bash and Zsh theme={null}
  CLAUDE_CODE_NO_FLICKER=1 claude
  ```

  ```powershell PowerShell theme={null}
  $env:CLAUDE_CODE_NO_FLICKER = "1"; claude
  ```

  ```json ~/.claude/settings.json theme={null}
  {
    "env": {
      "CLAUDE_CODE_NO_FLICKER": "1"
    }
  }
  ```
</CodeGroup>

<h2 id="paste-large-content">
  Pega contenido grande
</h2>

Cuando pegas más de 10,000 caracteres en el indicador, Claude Code colapsa la entrada a un marcador de posición `[Pasted text]` para que la caja de entrada siga siendo utilizable. El contenido completo aún se envía a Claude cuando envías.

La terminal integrada de VS Code puede soltar caracteres de pegados muy grandes antes de que lleguen a Claude Code, así que prefiere flujos de trabajo basados en archivos allí. Para entradas muy grandes como archivos completos o registros largos, escribe el contenido en un archivo y pide a Claude que lo lea en lugar de pegar. Esto mantiene la transcripción de conversación legible y permite que Claude haga referencia al archivo por ruta en turnos posteriores.

<h2 id="edit-prompts-with-vim-keybindings">
  Edita indicadores con atajos de teclado Vim
</h2>

Claude Code incluye un modo de edición de estilo Vim para la entrada del indicador. Habilítalo a través de `/config` → Editor mode, o estableciendo [`editorMode`](/docs/es/settings#available-settings) en `"vim"` en `~/.claude/settings.json`. Establece Editor mode de nuevo en `normal` para desactivarlo.

El modo Vim soporta un subconjunto de movimientos y operadores de modo NORMAL y VISUAL, como navegación `hjkl`, selección `v`/`V`, y `d`/`c`/`y` con objetos de texto. Consulta la [referencia del modo editor Vim](/docs/es/interactive-mode#vim-editor-mode) para la tabla de teclas completa. Los movimientos Vim no son remapeables a través del archivo de atajos de teclado.

Presionar Enter aún envía tu indicador en modo INSERT, a diferencia del Vim estándar. Usa `o` u `O` en modo NORMAL, o Ctrl+J, para insertar un salto de línea en su lugar.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Modo interactivo](/docs/es/interactive-mode): referencia completa de atajos de teclado y tabla de teclas Vim
* [Atajos de teclado](/docs/es/keybindings): remapea cualquier atajo de Claude Code, incluyendo Enter y Shift+Enter
* [Renderizado a pantalla completa](/docs/es/fullscreen): detalles sobre desplazamiento, búsqueda y copia en modo pantalla completa
* [Guía de ganchos](/docs/es/hooks-guide): más ejemplos de ganchos de Notificación para Linux y Windows
* [Solución de problemas](/docs/es/troubleshooting): correcciones para problemas fuera de la configuración de terminal
