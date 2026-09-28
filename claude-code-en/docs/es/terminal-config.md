> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Configura tu terminal para Claude Code

> Corrige Shift+Enter para saltos de línea, obtén una campana de terminal cuando Claude termine, configura tmux, coincide con el tema de color y habilita el modo Vim en la CLI de Claude Code.

Claude Code funciona en cualquier terminal sin configuración. Esta página es para cuando algo específico no se comporta como se espera. Encuentre su síntoma a continuación. Si todo ya se siente bien, no necesita esta página.

* [Shift+Enter envía en lugar de insertar una salta de línea](#enter-multiline-prompts)
* [Los atajos de tecla Option no hacen nada en macOS](#enable-option-key-shortcuts-on-macos)
* [Sin sonido ni alerta cuando Claude termina](#get-a-terminal-bell-or-notification)
* [Ejecuta Claude Code dentro de tmux](#configure-tmux)
* [Retroceso elimina una palabra completa en Windows](#fix-backspace-deleting-a-whole-word-on-windows)
* [La pantalla parpadea o el desplazamiento hacia atrás salta](#switch-to-fullscreen-rendering)
* [Deseas teclas Vim en el símbolo del sistema](#edit-prompts-with-vim-keybindings)

Esta página trata sobre cómo hacer que tu terminal envíe las señales correctas a Claude Code. Para cambiar qué teclas responde Claude Code en sí, consulta [atajos de teclado](/docs/es/keybindings) en su lugar.

<h2 id="enter-multiline-prompts">
  Ingrese indicaciones de varias líneas
</h2>

Presionar Enter envía su mensaje. Para agregar un salto de línea sin enviar, presione Ctrl+J, o escriba `\` y luego presione Enter. Ambos funcionan en cada terminal sin necesidad de configuración.

En la mayoría de las terminales también puede presionar Mayús+Enter, pero la compatibilidad varía según el emulador de terminal:

| Terminal                                                                                           | Mayús+Enter para nueva línea                                          |
| :------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------- |
| Ghostty, Kitty, iTerm2, WezTerm, Warp, Apple Terminal, Windows Terminal                            | Funciona sin configuración                                            |
| Otras terminales que admiten el protocolo de teclado kitty, como foot y Alacritty 0.16 o posterior | Funciona sin configuración. Requiere Claude Code v2.1.269 o posterior |
| VS Code, Cursor, Devin Desktop, Alacritty anterior a 0.16, Zed                                     | Ejecute `/terminal-setup` una vez                                     |
| gnome-terminal, IDEs de JetBrains como PyCharm y Android Studio                                    | No disponible; use Ctrl+J o `\` luego Enter                           |

Para VS Code, Cursor, Devin Desktop, Alacritty anterior a 0.16 y Zed, `/terminal-setup` escribe un atajo de teclado Mayús+Enter en el archivo de configuración de la terminal. En la primera ejecución verá una confirmación como `Installed VSCode terminal Shift+Enter key binding`. Los enlaces existentes se dejan en su lugar; si ve un mensaje como `VSCode terminal Shift+Enter key binding already configured`, no se realizó ningún cambio. Ejecute `/terminal-setup` directamente en la terminal del host en lugar de dentro de tmux o screen, ya que necesita escribir en la configuración de la terminal del host.

En VS Code, Cursor y Devin Desktop, `/terminal-setup` también actualiza dos configuraciones del editor: establece `terminal.integrated.gpuAcceleration` en `"off"` para evitar texto distorsionado en la terminal integrada, y establece `terminal.integrated.mouseWheelScrollSensitivity` para un desplazamiento más suave en [modo de pantalla completa](/docs/es/fullscreen). Para deshacer el cambio de aceleración de GPU, establézcalo nuevamente en `"auto"` y recargue la ventana del editor.

En Zed, `/terminal-setup` actualiza su `keymap.json` en su lugar:

* Si el mapa de teclas ya tiene enlaces y ninguno de ellos es un `shift-enter` de Terminal, Claude Code primero lo respalda en una copia en el mismo directorio, como `keymap.json.1a2b3c4d.bak`, luego fusiona el enlace Mayús+Enter en su mapa de teclas, manteniendo sus otros atajos de teclado y comentarios
* Si Claude Code no puede leer o analizar el mapa de teclas, no puede hacer una copia de seguridad, o no puede verificar el resultado fusionado, [deja el archivo sin cambios e imprime el bloque de enlace para que lo agregue usted mismo](/docs/es/errors#terminal-setup-left-your-zed-keymap-unchanged)

Si está ejecutando dentro de tmux, Mayús+Enter también requiere la [configuración de tmux a continuación](#configure-tmux) incluso cuando la terminal externa la admite.

Para vincular una nueva línea a una tecla diferente, o para intercambiar el comportamiento de modo que Enter inserte una nueva línea y Mayús+Enter envíe, asigne las acciones `chat:newline` y `chat:submit` en su [archivo de atajos de teclado](/docs/es/keybindings).

<h2 id="enable-option-key-shortcuts-on-macos">
  Habilitar atajos de teclado con la tecla Opción en macOS
</h2>

Algunos atajos de teclado de Claude Code utilizan la tecla Opción, como Opción+Intro para una nueva línea u Opción+P para cambiar modelos. En macOS, la mayoría de los terminales no envían Opción como modificador de forma predeterminada, por lo que estos atajos no funcionan hasta que los habilite. La configuración del terminal para esto generalmente se etiqueta como "Use Option as Meta Key"; Meta es el nombre histórico de Unix para la tecla ahora etiquetada como Opción o Alt.

<Tabs>
  <Tab title="Apple Terminal">
    Abra Configuración → Perfiles → Teclado y marque "Use Option as Meta Key".

    Si aceptó la solicitud de configuración del terminal de primera ejecución de Claude Code, esto ya está hecho. Esa solicitud ejecuta `/terminal-setup` para usted, que habilita Opción como Meta y desactiva la campana audible en su perfil de Apple Terminal.

    En [modo lector de pantalla](/docs/es/accessibility), `/terminal-setup` deja la configuración de campana sin cambios para que la campana del terminal permanezca audible. Antes de v2.1.211, `/terminal-setup` desactivaba la campana incluso en modo lector de pantalla. Si una ejecución anterior desactivó la campana, vuelva a activarla en Configuración → Perfiles → Avanzado → "Audible bell".
  </Tab>

  <Tab title="iTerm2">
    Abra Configuración → Perfiles → Teclas → General y establezca Left Option key y Right Option key en "Esc+".

    Ejecutar `/terminal-setup` en iTerm2 habilita "Applications in terminal may access clipboard" en Configuración → General → Selection para que el comando `/copy` pueda escribir en su portapapeles del sistema. El comando detecta iTerm2 incluso cuando se ejecuta desde dentro de tmux. Reinicie iTerm2 para que el cambio surta efecto.
  </Tab>

  <Tab title="VS Code">
    Agregue `"terminal.integrated.macOptionIsMeta": true` a su configuración de VS Code.
  </Tab>
</Tabs>

Para Ghostty, Kitty y otros terminales, busque una configuración de Option-as-Alt u Option-as-Meta en el archivo de configuración del terminal.

<h2 id="get-a-terminal-bell-or-notification">
  Obtener una campana de terminal o notificación
</h2>

Cuando Claude termina una tarea o se pausa para solicitar un permiso, y parece que está alejado de la terminal, se dispara un evento de notificación. Consulte [cuándo se dispara cada tipo de notificación](/docs/es/hooks#notification) para conocer el momento exacto. Mostrar esto como una campana de terminal o notificación de escritorio le permite cambiar a otro trabajo mientras se ejecuta una tarea larga.

De forma predeterminada, Claude Code envía una notificación de escritorio solo en Ghostty, Kitty e iTerm2. En otras terminales, establezca [`preferredNotifChannel`](/docs/es/settings-reference#preferrednotifchannel) en `"terminal_bell"` para sonar la campana de terminal en su lugar, o configure un [hook de Notification](#play-a-sound-with-a-notification-hook) para un sonido o comando personalizado. La siguiente entrada de configuración activa la campana de terminal:

```json ~/.claude/settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

La notificación de escritorio llega a su máquina local a través de SSH, por lo que una sesión remota aún puede alertarle. Ghostty y Kitty la reenvían a su centro de notificaciones del SO sin configuración adicional. iTerm2 requiere que habilite el reenvío:

<Steps>
  <Step title="Abrir la configuración de notificaciones de iTerm2">
    Vaya a Configuración → Perfiles → Terminal.
  </Step>

  <Step title="Habilitar alertas">
    Marque "Notification Center Alerts", luego haga clic en "Filter Alerts" y habilite "Send escape sequence-generated alerts".
  </Step>
</Steps>

Si las notificaciones aún no aparecen, confirme que su aplicación de terminal tiene permiso de notificación en la configuración de su SO, y si está ejecutando dentro de tmux, [habilite passthrough](#configure-tmux).

<h3 id="play-a-sound-with-a-notification-hook">
  Reproducir un sonido con un hook de Notification
</h3>

En cualquier terminal puede configurar un [hook de Notification](/docs/es/hooks-guide#get-notified-when-claude-needs-input) para reproducir un sonido o ejecutar un comando personalizado cuando Claude necesite su atención. Los hooks se ejecutan junto con la notificación integrada en lugar de reemplazarla, por lo que las terminales que no reciben una notificación de escritorio, como Warp o la terminal integrada de VS Code, pueden usar un hook o establecer `preferredNotifChannel` en `"terminal_bell"` en su lugar.

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
  Configurar tmux
</h2>

Cuando Claude Code se ejecuta dentro de tmux, por defecto Shift+Enter envía en lugar de insertar una nueva línea, y las notificaciones de escritorio y la [barra de progreso](/docs/es/settings-reference#terminalprogressbarenabled) nunca llegan a la terminal externa. Agregue estas líneas a `~/.tmux.conf`, luego ejecute `tmux source-file ~/.tmux.conf` para aplicarlas al servidor en ejecución:

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

La línea `allow-passthrough` permite que las notificaciones y actualizaciones de progreso lleguen a la terminal externa en lugar de ser absorbidas por tmux. Las líneas `extended-keys` permiten que tmux distinga Shift+Enter de Enter simple para que el atajo de nueva línea funcione.

<h2 id="fix-backspace-deleting-a-whole-word-on-windows">
  Corregir que Retroceso elimine una palabra completa en Windows
</h2>

En Windows, Claude Code lee un Retroceso que llega como `^H` como Ctrl+Retroceso, que [elimina la palabra anterior](/docs/es/interactive-mode#text-editing), excepto cuando `TERM_PROGRAM` es `mintty` o `TERM` es `cygwin`. En macOS y Linux, Claude Code lo lee como Retroceso simple.

Si cada pulsación de Retroceso elimina una palabra completa, su terminal envía `^H` para Retroceso simple. Establezca [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE=0`](/docs/es/env-vars). Retroceso y Ctrl+H entonces borran un carácter cada uno. Si Ctrl+Retroceso borra solo un carácter en macOS o Linux porque su terminal envía `^H` para ello, establezca la variable a `1` en su lugar.

<h2 id="match-the-color-theme">
  Coincidir con el tema de color
</h2>

Utilice el comando `/theme`, o el selector de temas en `/config`, para elegir un tema de Claude Code que coincida con su terminal. Al seleccionar la opción automática, se detecta el fondo claro u oscuro de su terminal, por lo que el tema sigue los cambios de apariencia del sistema operativo siempre que lo haga su terminal. Claude Code no controla el esquema de colores de la terminal en sí, que se establece mediante la aplicación de terminal.

Para personalizar lo que aparece en la parte inferior de la interfaz, configure una [línea de estado personalizada](/docs/es/statusline) que muestre el modelo actual, el directorio de trabajo, la rama de git u otro contexto.

<h3 id="create-a-custom-theme">
  Crear un tema personalizado
</h3>

Además de los ajustes preestablecidos integrados, `/theme` enumera cualquier tema personalizado que haya definido y cualquier tema contribuido por [plugins](/docs/es/plugins/components#themes-and-output-styles) instalados. Seleccione **Nuevo tema personalizado…** al final de la lista para crear uno de forma interactiva: usted nombra el tema y luego elige tokens de color individuales para anular. Presione `Ctrl+E` mientras un tema personalizado está resaltado para editarlo.

Cada tema personalizado es un archivo JSON en `~/.claude/themes/`. El nombre de archivo sin la extensión `.json` es el slug del tema, y seleccionar el tema almacena `custom:<slug>` como su preferencia de tema. El archivo tiene tres campos opcionales:

| Campo       | Tipo   | Descripción                                                                                                                                                               |
| :---------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`      | string | Etiqueta de visualización mostrada en `/theme`. Por defecto es el slug del nombre de archivo                                                                              |
| `base`      | string | Ajuste preestablecido integrado desde el que comienza el tema: `dark`, `light`, `dark-daltonized`, `light-daltonized`, `dark-ansi`, o `light-ansi`. Por defecto es `dark` |
| `overrides` | object | Mapa de nombres de tokens de color a valores de color. Los tokens no enumerados aquí se heredan del ajuste preestablecido base                                            |

Los valores de color aceptan `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, o `ansi:<name>` donde `<name>` es uno de los 16 nombres de color ANSI estándar como `red` o `cyanBright`. Los tokens desconocidos y los valores de color inválidos se ignoran, por lo que un error tipográfico no puede romper la representación.

El siguiente ejemplo define un tema que mantiene el ajuste preestablecido oscuro pero recolora el acento del prompt, el texto de error y el texto de éxito:

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

Claude Code observa `~/.claude/themes/` y se recarga cuando se agrega o cambia un archivo, por lo que las ediciones realizadas en su editor se aplican a una sesión en ejecución sin necesidad de reiniciar. Si la carpeta `~/.claude/themes/` en sí no existía cuando Claude Code se inició, reinicie una vez después de crear su primer archivo de tema. Después de eso, los cambios se aplican sin necesidad de reiniciar.

La referencia a continuación cubre los tokens que puede establecer en `overrides`. El editor interactivo en `/theme` muestra los mismos tokens con una vista previa en vivo, además de algunos acentos de propósito único como colores de pantalla de incorporación que se omiten aquí.

<Accordion title="Referencia de tokens de color">
  El siguiente ejemplo combina tokens de varios de los grupos a continuación: el acento de marca, el borde del modo de plan, los fondos de diff y el fondo del mensaje.

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

  Controle el acento de marca principal y los matices de texto de primer plano utilizados en toda la interfaz.

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

  Señale estados de éxito, fallo y advertencia en mensajes e indicadores.

  | Token     | Controla                                                                |
  | :-------- | :---------------------------------------------------------------------- |
  | `success` | Mensajes de éxito y comprobaciones aprobadas                            |
  | `error`   | Mensajes de error y fallos                                              |
  | `warning` | Advertencias, mensajes de precaución y el indicador del modo automático |
  | `merged`  | Estado de solicitud de extracción fusionada                             |

  <h4 id="input-box-and-mode-indicators">
    Cuadro de entrada e indicadores de modo
  </h4>

  Establezca el color del borde del cuadro de entrada y el acento mostrado mientras un modo de permiso o indicador está activo.

  | Token          | Controla                                                                                                                                                                                                        |
  | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
  | `promptBorder` | Borde del cuadro de entrada                                                                                                                                                                                     |
  | `planMode`     | Acento del modo de plan, mensajes del modo de plan y diálogos del modo de plan                                                                                                                                  |
  | `autoAccept`   | Acento del modo de aceptación de ediciones                                                                                                                                                                      |
  | `bashBorder`   | Borde del cuadro de entrada al ingresar un comando de shell `!`                                                                                                                                                 |
  | `ide`          | Indicador de conexión IDE                                                                                                                                                                                       |
  | `fastMode`     | Indicador de modo rápido                                                                                                                                                                                        |
  | `effortUltra`  | La etiqueta `ultracode` en el borde del cuadro de entrada mientras [ultracode](/docs/es/model-config#adjust-effort-level) está activado. Su anulación de este color tiene efecto en Claude Code v2.1.239 o posterior |

  <h4 id="diff-rendering">
    Representación de diff
  </h4>

  Coloree el código agregado y eliminado en ediciones y revisiones de archivos.

  | Token               | Controla                                                                                |
  | :------------------ | :-------------------------------------------------------------------------------------- |
  | `diffAdded`         | Fondo de líneas agregadas                                                               |
  | `diffRemoved`       | Fondo de líneas eliminadas                                                              |
  | `diffAddedDimmed`   | Fondo de líneas agregadas en el diff atenuado mostrado después de rechazar una edición  |
  | `diffRemovedDimmed` | Fondo de líneas eliminadas en el diff atenuado mostrado después de rechazar una edición |
  | `diffAddedWord`     | Resaltado a nivel de palabra dentro de una línea agregada                               |
  | `diffRemovedWord`   | Resaltado a nivel de palabra dentro de una línea eliminada                              |

  <h4 id="fullscreen-mode">
    Modo de pantalla completa
  </h4>

  Claude Code pinta `userMessageBackground`, `bashMessageBackgroundColor` y `memoryBackgroundColor` en los renderizadores predeterminado y de pantalla completa. Utiliza `userMessageBackgroundHover` y `selectionBg` solo en [modo de representación de pantalla completa](/docs/es/fullscreen).

  | Token                        | Controla                                                             |
  | :--------------------------- | :------------------------------------------------------------------- |
  | `userMessageBackground`      | Fondo detrás de sus mensajes en la transcripción                     |
  | `userMessageBackgroundHover` | Fondo detrás de un mensaje mientras se desplaza o se expande         |
  | `bashMessageBackgroundColor` | Fondo detrás de entradas de comando de shell `!` en la transcripción |
  | `memoryBackgroundColor`      | Fondo detrás de entradas de memoria `#` en la transcripción          |
  | `selectionBg`                | Fondo del texto seleccionado con el ratón                            |

  <h4 id="usage-meter-and-speaker-labels">
    Medidor de uso y etiquetas de altavoz
  </h4>

  Ajuste la barra mostrada en la vista `/usage` y las etiquetas que distinguen sus mensajes de los de Claude.

  | Token              | Controla                                                |
  | :----------------- | :------------------------------------------------------ |
  | `rate_limit_fill`  | Porción llena del medidor de uso                        |
  | `rate_limit_empty` | Porción no llena del medidor de uso                     |
  | `briefLabelYou`    | Color de la etiqueta `You` en sus mensajes              |
  | `briefLabelClaude` | Color de la etiqueta `Claude` en mensajes del asistente |

  <h4 id="shimmer-variants-and-subagent-colors">
    Variantes de shimmer y colores de subagentes
  </h4>

  Varios tokens tienen una variante de shimmer emparejada que proporciona el color más claro utilizado en el gradiente animado del spinner. Anule el shimmer junto con su token base si la animación se ve desajustada.

  * `claude` y `claudeShimmer`
  * `warning` y `warningShimmer`
  * `permission` y `permissionShimmer`
  * `promptBorder` y `promptBorderShimmer`
  * `inactive` e `inactiveShimmer`
  * `fastMode` y `fastModeShimmer`

  Cada [subagente](/docs/es/sub-agents) y tarea paralela se muestra en uno de ocho colores nombrados para que pueda distinguirlos en la transcripción. Los nombres de tokens siguen el patrón `<color>_FOR_SUBAGENTS_ONLY`, donde `<color>` es `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, o `cyan`. Anule estos para cambiar el aspecto de cada color nombrado. Por ejemplo, un subagente con `color: blue` en su definición se dibuja usando el valor `blue_FOR_SUBAGENTS_ONLY`.

  Claude Code representa la palabra clave [`ultrathink`](/docs/es/model-config#use-ultrathink-for-one-off-deep-reasoning) en la entrada del prompt con un gradiente arcoíris de siete colores. Los nombres de tokens siguen el patrón `rainbow_<color>` y `rainbow_<color>_shimmer`, donde `<color>` es `red`, `orange`, `yellow`, `green`, `blue`, `indigo`, o `violet`.
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  Cambiar a renderizado a pantalla completa
</h2>

En [modo de lector de pantalla](/docs/es/accessibility), esta sección no aplica. Claude Code siempre se renderiza como texto de desplazamiento simple excepto en [sesiones de fondo](/docs/es/agent-view) adjuntas, y si ejecuta `/tui fullscreen` en cualquier otra sesión, Claude Code imprime una explicación en lugar de cambiar.

Si la pantalla parpadea o la posición de desplazamiento salta mientras Claude está trabajando, cambie a [modo de renderizado a pantalla completa](/docs/es/fullscreen). En este modo se desplaza con el ratón o AvPág dentro de Claude Code en lugar de con el desplazamiento nativo de su terminal; consulte la [página de pantalla completa](/docs/es/fullscreen#search-and-review-the-conversation) para saber cómo buscar y copiar.

Si el parpadeo es el único problema y su terminal admite salida sincronizada pero no se detecta automáticamente, como Emacs `eat`, establezca [`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/es/env-vars) para detener el parpadeo sin cambiar renderizadores.

Ejecute `/tui fullscreen` para cambiar y guardar la preferencia. Su conversación se reinicia intacta y las sesiones futuras comienzan en pantalla completa a menos que [falle el inicio de pantalla completa](/docs/es/fullscreen#fullscreen-renderer-didnt-finish-starting). También puede establecer la variable de entorno `CLAUDE_CODE_NO_FLICKER` antes de iniciar Claude Code:

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
  Pegar contenido grande
</h2>

Cuando pega más de 800 caracteres o más de tres líneas en el prompt, Claude Code contrae la entrada a un marcador de posición como `[Pasted text #1 +120 lines]` para que la caja de entrada siga siendo utilizable, y sigue enviando el contenido completo cuando envía. Para entradas muy grandes como archivos completos o registros largos, escriba el contenido en un archivo y pida a Claude que lo lea en lugar de pegarlo. La transcripción de la conversación permanece legible y Claude puede hacer referencia al archivo por ruta en turnos posteriores. La terminal integrada de VS Code también puede descartar caracteres de pegados muy grandes antes de que lleguen a Claude Code, por lo que use un archivo allí.

Si el pegado contiene [caracteres Unicode invisibles](/docs/es/interactive-mode#invisible-characters-in-prompts), Claude Code los elimina cuando presiona Intro y vuelve a poner el prompt limpio en la caja de entrada para que lo envíe con otro Intro.

<h3 id="how-claude-treats-pasted-text">
  Cómo Claude trata el texto pegado
</h3>

Cuando envía, Claude ve el contenido detrás de cada marcador de posición `[Pasted text #N]` marcado como texto que pegó desde otro lugar en lugar de escribir. Claude recibe la instrucción de que un pegado puede contener instrucciones que no escribió, y de seguir instrucciones dentro de él solo donde el mensaje que escribió lo solicita. En sesiones que no [obtienen banderas de características](/docs/es/env-vars#features-that-need-feature-flag-fetching), los pegados no se marcan.

<h3 id="delete-and-restore-a-collapsed-paste">
  Eliminar y restaurar un pegado contraído
</h3>

Cuando elimina con un atajo de palabra o línea como `Ctrl+W` o `Ctrl+K`, o con una eliminación vim a través de un movimiento `f`/`t` como `df]`, y el rango eliminado llega dentro de un marcador de posición `[Pasted text #N]`, Claude Code elimina el marcador de posición completo. Para restaurarlo, pegue la eliminación nuevamente con [`Ctrl+Y`](/docs/es/interactive-mode#text-editing) después de un atajo de palabra o línea, o con [`p` en NORMAL mode](/docs/es/interactive-mode#editing-normal-mode) después de una eliminación vim.

<h3 id="recall-a-prompt-that-had-pasted-text">
  Recuperar un prompt que tenía texto pegado
</h3>

Claude Code mantiene el contenido detrás de cada marcador de posición `[Pasted text #N]` bajo `~/.claude/paste-cache/`, por lo que cuando recupera un prompt del [historial de comandos](/docs/es/interactive-mode#command-history) y lo reenvía, el contenido pegado completo se envía nuevamente, incluso en una sesión posterior.

Los archivos de caché más antiguos que [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays) se eliminan bajo las [reglas de barrido de retención](/docs/es/claude-directory#cleaned-up-automatically), por lo que un prompt recuperado puede hacer referencia a texto pegado que ya no existe. Cuando envía tal prompt, Claude Code nunca envía la cadena literal `[Pasted text #N]`, y muestra una notificación nombrando el pegado faltante:

* En un prompt simple con texto restante, Claude Code elimina el marcador de posición y envía el texto restante.
* En un comando de [shell mode](/docs/es/interactive-mode#shell-mode-with-prefix) o un comando `/`, donde la eliminación cambiaría lo que se ejecuta, y en cualquier prompt donde la eliminación deja vacío, Claude Code cancela el envío y mantiene el texto original en la entrada, con el marcador de posición aún en él. Elimine el marcador de posición o edite el comando, luego reenvíe.

<h2 id="edit-prompts-with-vim-keybindings">
  Editar prompts con atajos de teclado Vim
</h2>

Claude Code incluye un modo de edición estilo Vim para la entrada de prompts. Actívelo a través de `/config` → Editor mode, o configurando [`editorMode`](/docs/es/settings-reference#editormode) a `"vim"` en `~/.claude/settings.json`. Configure el modo Editor nuevamente a `normal` para desactivarlo.

El modo Vim admite un subconjunto de motions y operadores de modo NORMAL y VISUAL, como navegación `hjkl`, selección `v`/`V`, y `d`/`c`/`y` con objetos de texto. Consulte la [referencia del modo editor Vim](/docs/es/interactive-mode#vim-editor-mode) para la tabla de teclas completa.

Los motions de Vim no se pueden remapear a través del archivo de atajos de teclado. Para mapear una secuencia de modo INSERT de dos teclas como `jj` a Escape, configure [`vimInsertModeRemaps`](/docs/es/interactive-mode#remap-insert-mode-key-sequences) en su configuración de usuario.

Presionar Enter sigue enviando su prompt en modo INSERT, a diferencia del Vim estándar. Use `o` u `O` en modo NORMAL, o Ctrl+J, para insertar una nueva línea en su lugar.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Modo interactivo](/docs/es/interactive-mode): referencia completa de atajos de teclado y tabla de teclas Vim
* [Atajos de teclado](/docs/es/keybindings): remapea cualquier atajo de Claude Code, incluyendo Enter y Shift+Enter
* [Renderizado a pantalla completa](/docs/es/fullscreen): detalles sobre desplazamiento, búsqueda y copia en modo pantalla completa
* [Guía de ganchos](/docs/es/hooks-guide): más ejemplos de ganchos de Notificación para Linux y Windows
* [Solución de problemas](/docs/es/troubleshooting): correcciones para problemas fuera de la configuración de terminal
