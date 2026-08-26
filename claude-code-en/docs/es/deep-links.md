> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Iniciar sesiones desde enlaces

> Abra una sesión de terminal de Claude Code desde una URL. Incruste enlaces `claude-cli://` en runbooks, alertas y paneles para que un clic abra Claude Code en el repositorio correcto con el mensaje correcto.

Un enlace profundo es una URL `claude-cli://` que abre Claude Code en una nueva ventana de terminal. La URL puede llevar un directorio de trabajo y un mensaje para rellenar previamente.

Esto le permite compartir un punto de partida de un clic para una tarea: cualquiera que tenga Claude Code instalado y haga clic en el enlace verá una sesión abierta con el mensaje ya escrito. El mensaje se rellena pero no se envía hasta que presione Intro.

Debido a que un enlace profundo es una URL, puede colocarlo en cualquier lugar donde pueda ir un enlace:

* Un paso de runbook de incidentes que abre el repositorio del servicio afectado con un mensaje de diagnóstico
* Una alerta de monitoreo o panel que vincula a un mensaje de investigación para una métrica específica
* Una página README o wiki que abre el proyecto con un mensaje de incorporación
* Una notificación de fallo de CI que rellena previamente el nombre del trabajo fallido

Esta página cubre cómo [construir un enlace](#build-a-link), [incrustar uno en un runbook o activarlo desde el shell](#examples), y [administrar o deshabilitar el registro del controlador](#registration-and-supported-platforms) en cada plataforma.

<h2 id="how-it-works">
  Cómo funciona
</h2>

El prefijo `claude-cli://` es un esquema de URL personalizado que Claude Code registra con su sistema operativo, similar a cómo los enlaces `mailto:` abren su cliente de correo electrónico. El enlace puede estar en una página web, en una wiki, en un mensaje de Slack o en cualquier aplicación que represente enlaces. Cuando hace clic en uno:

1. El navegador o la aplicación entrega la URL a su sistema operativo.
2. El sistema operativo reconoce el prefijo `claude-cli://` e inicia Claude Code en su máquina.
3. Se abre una nueva ventana de terminal con Claude Code ejecutándose en el directorio que especificó el enlace, y el texto del mensaje del enlace ya está en el cuadro de entrada.
4. Lee el mensaje, lo edita si lo desea y presiona Intro para enviarlo.

El enlace en sí puede estar alojado en cualquier lugar, pero la sesión siempre se abre localmente en la computadora donde hizo clic. Consulte [Registro y plataformas compatibles](#registration-and-supported-platforms) para ver qué emulador de terminal se abre en cada sistema operativo.

<Note>
  La plataforma que muestra el enlace debe permitir esquemas de URL personalizados. El Markdown representado por GitHub permite `http` y `https` pero elimina esquemas como `claude-cli://` en READMEs, problemas, solicitudes de extracción y wikis. Solo se muestra el texto del enlace, sin enlace detrás y la URL oculta. Consulte [Solución de problemas](#the-link-renders-as-plain-text-instead-of-being-clickable) para obtener una solución alternativa.
</Note>

<h3 id="what-a-launched-session-shows">
  Qué muestra una sesión iniciada
</h3>

Un enlace profundo nunca ejecuta nada por sí solo. El enlace solo elige un directorio y rellena el cuadro de mensaje. Si hace clic en un enlace de una página en la que no confía, el mensaje sigue siendo inerte: nada llega al modelo hasta que lea lo que se rellenó y presione Intro.

Cuando se abre la sesión, una línea de advertencia debajo del cuadro de entrada lee `Prompt from an external link` y permanece visible hasta que envíe o borre el mensaje. Para mensajes de más de 1.000 caracteres, la advertencia incluye el recuento de caracteres y le indica que desplace y revise el texto completo antes de presionar Intro, ya que los mensajes largos pueden empujar las instrucciones fuera de la pantalla. Las reglas de permisos, `CLAUDE.md` y los mensajes de confianza para el directorio seleccionado se aplican de la misma manera que para cualquier otra sesión.

<h2 id="build-a-link">
  Construir un enlace
</h2>

Cada enlace profundo comienza con `claude-cli://open`, que es la única ruta que acepta el controlador, seguida de parámetros de consulta opcionales. La forma mínima abre Claude Code en su directorio de inicio con un mensaje vacío:

```text theme={null}
claude-cli://open
```

Agregue parámetros para controlar dónde comienza la sesión y qué contiene el cuadro de mensaje:

| Parámetro | Descripción                                                                                                                                                                                                                                                                      |
| --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `q`       | Texto para rellenar previamente en el cuadro de mensaje. [Codifique en URL](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) el valor. Use `%0A` para saltos de línea en mensajes de varias líneas. Máximo 5.000 caracteres. |
| `cwd`     | Ruta absoluta para usar como directorio de trabajo. Las rutas de red y UNC se rechazan, al igual que las rutas que contienen caracteres de control invisibles o bidireccionales.                                                                                                 |
| `repo`    | Un slug `owner/name` de GitHub. Claude Code lo resuelve a un clon local que ha visto antes e inicia allí. Si no tiene un clon coincidente, la sesión se abre en su directorio de inicio.                                                                                         |

`cwd` y `repo` son [dos formas de establecer el directorio de trabajo](#choose-between-cwd-and-repo). Si pasa ambos, `cwd` tiene prioridad e `repo` se ignora, incluso si la ruta `cwd` no existe.

El siguiente enlace apunta a un repositorio llamado `acme/payments` con un mensaje de diagnóstico de dos líneas. Reemplace `acme/payments` con el slug `owner/name` de su repositorio cuando construya el suyo:

```text theme={null}
claude-cli://open?repo=acme/payments&q=Investigate%20the%20failed%20deploy%20of%20payments-api.%0ACheck%20recent%20commits%20to%20main%20and%20the%20last%20successful%20build.
```

Al hacer clic en él, se abre una nueva ventana de terminal, se inicia Claude Code en su clon local de `acme/payments` y se rellena el cuadro de mensaje con el texto decodificado:

```text theme={null}
Investigate the failed deploy of payments-api.
Check recent commits to main and the last successful build.
```

Puede editar el mensaje antes de presionar Intro para enviarlo. Si no tiene un clon local del repositorio, la sesión se abre en su directorio de inicio. Consulte [Elegir entre `cwd` y `repo`](#choose-between-cwd-and-repo) para saber cómo se selecciona la ruta local cuando tiene múltiples clones o worktrees.

<h3 id="choose-between-cwd-and-repo">
  Elegir entre `cwd` y `repo`
</h3>

Use `cwd` cuando todos los que hagan clic en el enlace tengan el proyecto en la misma ruta absoluta, como un devcontainer estandarizado o una imagen de VM.

Use `repo` cuando el enlace se comparte y cada persona clona en una ubicación diferente. Claude Code resuelve el slug a una ruta local de la siguiente manera:

* Cada vez que ejecuta `claude` en un repositorio de Git, la ruta del sistema de archivos de ese directorio se registra contra el slug `owner/name` de GitHub del repositorio.
* Cuando llega un enlace profundo, `repo` abre cualquier ruta coincidente que haya usado más recientemente. Los múltiples clones y worktrees se rastrean por separado, por lo que elige el que usó por última vez.
* La búsqueda solo encuentra rutas donde ya ha ejecutado Claude Code al menos una vez.
* El enlace no cambia qué rama está desprotegida. La sesión se abre en el estado actual de ese directorio.

El encabezado de bienvenida muestra qué ruta eligió para que pueda confirmar que se abrió el clon correcto.

<h2 id="examples">
  Ejemplos
</h2>

Las secciones a continuación muestran dos formas comunes de usar un enlace profundo: como un enlace Markdown en un documento y como un comando en un script o alias de shell.

<h3 id="embed-a-link-in-a-runbook">
  Incrustar un enlace en un runbook
</h3>

Un enlace profundo en un runbook le da a quien está triando una forma de un clic para comenzar a investigar en el repositorio correcto con un mensaje preparado. La plataforma que representa el runbook debe permitir esquemas de URL personalizados. El Markdown representado por GitHub no permite `claude-cli://`, por lo que un enlace profundo en un README, problema o wiki de GitHub muestra solo su etiqueta sin enlace clickeable. Consulte [la nota de solución de problemas](#the-link-renders-as-plain-text-instead-of-being-clickable) para obtener una solución alternativa.

El mensaje es parte de la URL y debe estar codificado en URL. Para producir el valor codificado, pase el texto del mensaje a través de `encodeURIComponent` en una consola del navegador o cualquier codificador de URL.

El ejemplo a continuación agrega un punto de entrada de investigación a un runbook de incidentes para un servicio llamado `web-gateway`:

```markdown theme={null}
## High 5xx rate on web-gateway

1. Acknowledge the page in PagerDuty.
2. [Open Claude Code in the gateway repo](claude-cli://open?repo=acme/web-gateway&q=5xx%20rate%20is%20elevated%20on%20web-gateway.%20Check%20recent%20deploys%2C%20error%20logs%20from%20the%20last%2030%20minutes%2C%20and%20open%20incidents%20in%20Linear.)
3. Post initial findings in #incident.
```

Para usar esto en su propio runbook, reemplace `acme/web-gateway` con el slug del repositorio de su servicio. Esto permite que los ingenieros con Claude Code instalado y un clon local de ese repositorio hagan clic en el paso 2 e inicien la investigación con el mensaje listo para enviar.

<h3 id="open-a-link-from-the-shell">
  Abrir un enlace desde el shell
</h3>

También puede abrir un enlace profundo desde un script de shell, alias o automatización en lugar de hacer clic en él. Llame al comando de apertura de URL de su sistema operativo con el enlace como argumento.

<Tabs>
  <Tab title="macOS">
    El comando `open` integrado pasa la URL al controlador `claude-cli://` registrado:

    ```bash theme={null}
    open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Linux">
    La mayoría de los entornos de escritorio proporcionan `xdg-open`, que pasa la URL al controlador registrado:

    ```bash theme={null}
    xdg-open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Windows">
    En PowerShell, `Start-Process` pasa la URL al controlador registrado:

    ```powershell theme={null}
    Start-Process "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```

    En `cmd.exe`, `start` trata su primer argumento entrecomillado como un título de ventana, así que pase un título vacío antes de la URL:

    ```cmd theme={null}
    start "" "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>
</Tabs>

<h2 id="registration-and-supported-platforms">
  Registro y plataformas compatibles
</h2>

Claude Code registra el controlador `claude-cli://` con su sistema operativo la primera vez que inicia una sesión interactiva en macOS, Linux y Windows. No ejecuta un comando de instalación separado. El registro escribe solo en ubicaciones a nivel de usuario:

| Plataforma | Ubicación del controlador                                                                                       |
| ---------- | --------------------------------------------------------------------------------------------------------------- |
| macOS      | `~/Applications/Claude Code URL Handler.app`                                                                    |
| Linux      | `claude-code-url-handler.desktop` bajo `$XDG_DATA_HOME/applications`, por defecto `~/.local/share/applications` |
| Windows    | `HKEY_CURRENT_USER\Software\Classes\claude-cli`                                                                 |

El controlador inicia Claude Code en un emulador de terminal detectado. En macOS, Claude Code recuerda la terminal de su sesión interactiva más reciente y la reutiliza, compatible con iTerm2, Ghostty, kitty, Alacritty, WezTerm y Terminal.app. En Linux, respeta la variable de entorno `$TERMINAL`, luego `x-terminal-emulator`, luego una lista de emuladores comunes. En Windows, prefiere Windows Terminal, luego PowerShell, luego `cmd.exe`.

Para evitar el registro por completo, establezca [`disableDeepLinkRegistration`](/docs/es/settings) en `"disable"` en `settings.json`. Para aplicar esto en toda una organización para que los usuarios no puedan volver a habilitarlo, establézcalo en [configuración administrada](/docs/es/server-managed-settings) en su lugar.

<h2 id="open-a-vs-code-tab-instead-of-a-terminal">
  Abrir una pestaña de VS Code en lugar de una terminal
</h2>

La extensión de VS Code registra su propio controlador en `vscode://anthropic.claude-code/open`, que abre una pestaña del editor de Claude Code en lugar de una ventana de terminal. Consulte [Iniciar una pestaña de VS Code desde otras herramientas](/docs/es/vs-code#launch-a-vs-code-tab-from-other-tools) para los parámetros de esa URL.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="clicking-the-link-does-nothing">
  Hacer clic en el enlace no hace nada
</h3>

El controlador probablemente aún no está registrado. Inicie una sesión `claude` interactiva una vez en esa máquina, salga e intente el enlace nuevamente. Si está en Linux sin un entorno de escritorio, `xdg-open` puede no tener nada a lo que enviar.

<h3 id="the-link-renders-as-plain-text-instead-of-being-clickable">
  El enlace se representa como texto sin formato en lugar de ser clickeable
</h3>

Algunos representadores de Markdown solo permiten enlaces `http` y `https` y eliminan otros esquemas de URL. GitHub hace esto en READMEs, problemas, solicitudes de extracción y wikis: `[label](claude-cli://...)` se representa como solo `label`, sin enlace y la URL eliminada. En estas plataformas, coloque el enlace profundo en un bloque de código para que los lectores puedan ver la URL y pegarla en la barra de direcciones de su navegador.

<h3 id="the-session-opens-in-my-home-directory-instead-of-the-repo">
  La sesión se abre en mi directorio de inicio en lugar del repositorio
</h3>

El parámetro `repo` solo se resuelve a clones que Claude Code ya ha visto. Ejecute `claude` dentro del clon una vez para que su ruta se registre, o cambie el enlace para usar `cwd` con una ruta absoluta.

<h3 id="the-link-opens-the-wrong-terminal">
  El enlace abre la terminal incorrecta
</h3>

En macOS, inicie `claude` en su terminal preferida una vez y el siguiente enlace profundo la usará. En Linux, establezca la variable de entorno `$TERMINAL` en el nombre del comando de su emulador preferido. En Windows, el orden es fijo: instale Windows Terminal si desea que los enlaces se abran allí en lugar de en una ventana de PowerShell o `cmd.exe`.

<h2 id="learn-more">
  Más información
</h2>

Estas páginas cubren formas relacionadas de iniciar o extender sesiones de Claude Code:

* [Skills](/docs/es/skills): almacene un mensaje de runbook largo como un `/skill` en el repositorio para que el parámetro `q` del enlace profundo solo tenga que nombrarlo
* [Modo no interactivo](/docs/es/headless): ejecute Claude desde un script y capture la salida sin abrir una terminal
