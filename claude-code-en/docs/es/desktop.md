> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Aplicación de escritorio

> Aproveche al máximo Claude Code Desktop: sesiones paralelas con aislamiento de Git, diseño de panel de arrastrar y soltar, terminal integrada y editor de archivos, chats laterales, uso de computadora, envíe sesiones desde su teléfono, revisión visual de diferencias, vistas previas de aplicaciones, monitoreo de PR, conectores y configuración empresarial.

La aplicación Claude Desktop tiene tres pestañas: **Chat** para conversaciones, **Cowork** para [Dispatch y trabajo agentico más largo](https://claude.com/product/cowork), y **Code** para desarrollo de software. Esta página es la referencia para la pestaña Code.

<CardGroup cols={3}>
  <Card title="Download for macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Universal build for Intel and Apple Silicon
  </Card>

  <Card title="Download for Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    For x64 processors
  </Card>

  <Card title="Get Claude for Linux (beta)" icon="linux" href="/docs/en/desktop-linux">
    apt or .deb for Ubuntu and Debian
  </Card>
</CardGroup>

For Windows ARM64, download the [ARM64 installer](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). On Linux, install with apt; see [Claude Desktop on Linux](/docs/en/desktop-linux).

Después de instalar, inicie Claude, inicie sesión y haga clic en la pestaña **Code**. La primera vez que la abra en Windows, necesita tener [Git for Windows](https://git-scm.com/downloads/win) instalado; reinicie la aplicación después de instalarlo. Para un recorrido de su primera sesión, consulte la [guía de introducción](/docs/es/desktop-quickstart).

En la pestaña Code, cada conversación es una **sesión**: tiene su propio historial de chat, carpeta de proyecto y cambios de código, independiente de cualquier otra sesión. La barra lateral enumera sus sesiones y le permite ejecutar varias en paralelo. Dentro de una sesión puede:

* [Revisar y comentar en diffs](#review-changes-with-diff-view), luego [monitorear el PR resultante a través de CI](#monitor-pull-request-status)
* [Obtener una vista previa de su aplicación en ejecución](#preview-your-app) en el panel Navegador mientras Claude verifica sus propios cambios, y [abrir sitios externos](#browse-external-sites) junto a él
* [Organizar paneles](#arrange-your-workspace) para el chat, diff, navegador, terminal y editor de archivos lado a lado
* Hacer una [pregunta lateral](#ask-a-side-question-without-derailing-the-session) que use el contexto de la sesión sin desviarse
* [Conectar herramientas externas](#connect-external-tools) como GitHub, Slack y Linear
* Permitir que Claude [abra aplicaciones y controle su pantalla](#let-claude-use-your-computer)
* Ejecutar en su máquina, en la [nube](#run-long-running-tasks-remotely), o sobre [SSH](#ssh-sessions)

Para [trabajo recurrente programado](/docs/es/desktop-scheduled-tasks), [atajos de teclado](#keyboard-shortcuts), o [enviar tareas desde su teléfono](#sessions-from-dispatch), consulte las páginas y secciones vinculadas. Si ya usa la CLI basada en terminal, consulte la [comparación de CLI](#coming-from-the-cli) para ver qué se transfiere.

<h2 id="start-a-session">
  Iniciar una sesión
</h2>

Antes de enviar su primer mensaje, configure cuatro cosas en el área de solicitud:

* **Entorno**: elija dónde se ejecuta Claude. Seleccione **Local** para su máquina, **Remote** para sesiones en la nube alojadas por Anthropic, una [**conexión SSH**](#ssh-sessions) para una máquina remota que usted administra, o en Windows una [**distribución WSL**](/docs/es/desktop-wsl). Consulte [configuración del entorno](#environment-configuration).
* **Carpeta del proyecto**: seleccione la carpeta o repositorio en el que Claude trabaja. Para sesiones remotas, puede agregar [múltiples repositorios](#run-long-running-tasks-remotely).
* **Modelo**: elija un [modelo](/docs/es/model-config#available-models) del menú desplegable junto al botón de envío. Puede cambiar esto durante la sesión.
* **Modo de permisos**: elija cuánta autonomía tiene Claude desde el [selector de modo](#choose-a-permission-mode). Puede cambiar esto durante la sesión.

Escriba su tarea y presione **Enter** para comenzar. Cada sesión rastrea su propio contexto y cambios de forma independiente.

<h2 id="work-with-code">
  Trabajar con código
</h2>

Proporcione a Claude el contexto correcto, controle cuánto hace por su cuenta y revise lo que cambió.

<h3 id="use-the-prompt-box">
  Usar el cuadro de solicitud
</h3>

Escriba lo que desea que Claude haga y presione **Enter** para enviar. Claude lee los archivos de su proyecto, realiza cambios y ejecuta comandos según su [modo de permisos](#choose-a-permission-mode). Puede redirigir a Claude en cualquier momento: haga clic en el botón de parada para interrumpir inmediatamente, o escriba una corrección y presione **Enter** para enviarla sin detener la acción en ejecución. Claude lee la corrección tan pronto como se completa la acción actual y se ajusta antes de su siguiente paso.

El botón **+** junto al cuadro de solicitud le da acceso a archivos adjuntos, [skills](#use-skills), [conectores](#connect-external-tools) y [plugins](#install-plugins).

<h3 id="add-files-and-context-to-prompts">
  Agregar archivos y contexto a las solicitudes
</h3>

El cuadro de solicitud admite dos formas de traer contexto externo:

* **Archivos @mention**: escriba `@` seguido de un nombre de archivo para agregar un archivo al contexto de la conversación. Claude puede entonces leer y hacer referencia a ese archivo. @mention no está disponible en sesiones en la nube o WSL.
* **Adjuntar archivos**: adjunte imágenes, PDF y otros archivos a su solicitud usando el botón de adjuntos, o arrastre y suelte archivos directamente en la solicitud. Esto es útil para compartir capturas de pantalla de errores, maquetas de diseño o documentos de referencia.

<h3 id="choose-a-permission-mode">
  Elegir un modo de permisos
</h3>

Los modos de permisos controlan cuánta autonomía tiene Claude durante una sesión: si pregunta antes de editar archivos, ejecutar comandos o ambos. Puede cambiar de modo en cualquier momento usando el selector de modo junto al botón de envío. Comience con Manual para ver exactamente qué hace Claude, luego pase a Accept edits o Plan a medida que se sienta cómodo.

Para establecer un modo predeterminado para nuevas sesiones locales, agregue `permissions.defaultMode` a su [archivo de configuración](/docs/es/settings#settings-files). La aplicación de escritorio lee los mismos archivos de configuración que la CLI. Un modo que selecciona en el selector se recuerda por carpeta y tiene prioridad sobre `defaultMode` para esa carpeta, excepto Plan, que se aplica solo a la sesión actual.

| Modo                   | Clave de configuración | Comportamiento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ---------------------- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Manual**             | `default`              | Claude pregunta antes de editar archivos o ejecutar comandos. Usted ve una diferencia y puede aceptar o rechazar cada cambio. Recomendado para nuevos usuarios.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| **Accept edits**       | `acceptEdits`          | Claude acepta automáticamente ediciones de archivos y comandos comunes del sistema de archivos como `mkdir`, `touch` y `mv`, pero aún pregunta antes de ejecutar otros comandos de terminal. Use esto cuando confíe en cambios de archivos y desee una iteración más rápida.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| **Plan**               | `plan`                 | Claude lee archivos y ejecuta comandos para explorar, luego propone un plan sin editar su código fuente. Bueno para tareas complejas donde desea revisar el enfoque primero.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| **Auto**               | `auto`                 | Claude ejecuta todas las acciones con verificaciones de seguridad en segundo plano que verifican la alineación con su solicitud. Reduce solicitudes de permisos mientras mantiene supervisión. Aparece cuando su cuenta cumple con los [requisitos de disponibilidad](#auto-mode-availability) a continuación; no hay un control deslizante de Configuración separado para esto.                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| **Bypass permissions** | `bypassPermissions`    | Claude se ejecuta sin avisos de permisos, excepto aquellos forzados por [reglas de solicitud](/docs/es/permissions#manage-permissions) explícitas, herramientas de conectores [que su organización estableció en `ask`](/docs/es/mcp#organization-controls-on-connector-tools), herramientas MCP marcadas [`requiresUserInteraction`](/docs/es/mcp#require-approval-for-a-specific-tool), o clasificadores de seguridad cuando Claude [actúa en sitios externos](#browse-external-sites); equivalente a `--dangerously-skip-permissions` en la CLI. En planes Pro y Max, habilítelo en su Configuración → Claude Code bajo "Allow bypass permissions mode"; en planes Team y Enterprise no hay un control deslizante de Configuración, y la política de la organización lo controla en su lugar. Use solo en contenedores o máquinas virtuales sandboxed. |

Las versiones anteriores de la pestaña Code etiquetaban estos modos como Ask permissions, Auto accept edits y Plan mode.

El modo de permisos `dontAsk` está disponible solo en la [CLI](/docs/es/permission-modes#allow-only-pre-approved-tools-with-dontask-mode).

<span id="auto-mode-availability" />

Auto mode está disponible para todos los usuarios en la API de Anthropic y requiere Claude Opus 4.6 o posterior, u Sonnet 4.6 o posterior. Los administradores de la organización pueden desactivar auto mode con la clave `disableAutoMode` en [configuración administrada](#managed-settings).

En implementaciones empresariales que enrutan Desktop a Google Cloud's Agent Platform, auto mode está [disponible de forma predeterminada](/docs/es/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry), y solo Claude Sonnet 5, Opus 4.7 y Opus 4.8 son compatibles allí. Antes de Claude Code v2.1.207, las implementaciones empresariales en Google Cloud's Agent Platform tenían que establecer `CLAUDE_CODE_ENABLE_AUTO_MODE` para habilitar auto mode.

<Tip title="Mejor práctica">
  Comience tareas complejas en Plan para que Claude mapee un enfoque antes de realizar cambios. Una vez que apruebe el plan, cambie a Accept edits o Manual para ejecutarlo. Consulte [explorar primero, luego planificar, luego codificar](/docs/es/best-practices#explore-first-then-plan-then-code) para obtener más información sobre este flujo de trabajo.
</Tip>

Las sesiones en la nube admiten Accept edits, Plan y Auto. Accept edits corresponde al modo `default`: las sesiones en la nube aceptan automáticamente ediciones de archivos, por lo que el selector muestra Accept edits en lugar de Manual. Bypass permissions no está disponible porque el entorno en la nube ya está sandboxed.

Los administradores empresariales pueden restringir qué modos de permisos están disponibles. Consulte [configuración empresarial](#enterprise-configuration) para obtener detalles.

<h3 id="preview-your-app">
  Vista previa de su aplicación
</h3>

Claude puede iniciar un servidor de desarrollo y abrirlo en el panel del navegador para verificar sus cambios. Esto funciona tanto para aplicaciones web frontend como para servidores backend: Claude puede probar puntos finales de API, ver registros del servidor e iterar sobre problemas que encuentra. En la mayoría de los casos, Claude inicia el servidor automáticamente después de editar archivos del proyecto. También puede pedirle a Claude que haga una vista previa en cualquier momento. De forma predeterminada, Claude [verifica automáticamente](#auto-verify-changes) cambios después de cada edición.

El panel del navegador también puede abrir archivos HTML estáticos, PDF, imágenes y videos de su proyecto. Haga clic en una ruta HTML, PDF, imagen o video en el chat para abrirla allí.

Desde el panel del navegador, puede:

* Interactuar con su aplicación en ejecución directamente en el panel del navegador
* Ver a Claude verificar sus propios cambios automáticamente: toma capturas de pantalla, inspecciona el DOM, hace clic en elementos, completa formularios y corrige problemas que encuentra
* Iniciar o detener servidores desde el menú desplegable de servidores en la barra de herramientas de la sesión
* Persistir cookies y almacenamiento local en reinicios del servidor seleccionando **Persist sessions** en el menú desplegable, para que no tenga que volver a iniciar sesión durante el desarrollo
* Editar la configuración del servidor o detener todos los servidores a la vez

Claude crea la configuración inicial del servidor basada en su proyecto. Si su aplicación usa un comando de desarrollo personalizado, edite `.claude/launch.json` para que coincida con su configuración. Consulte [Configurar servidores de vista previa](#configure-preview-servers) para la referencia completa.

Para borrar datos de sesión guardados, o para desactivar el navegador por completo, use los controles deslizantes en Configuración → Claude Code.

<h3 id="browse-external-sites">
  Examinar sitios externos
</h3>

El panel del navegador es un navegador con pestañas, por lo que puede abrir documentación, rastreadores de problemas o cualquier otro sitio junto a su aplicación en ejecución. Para abrir el navegador, presione **Cmd+Shift+B** en macOS o **Ctrl+Shift+B** en Windows, o selecciónelo del menú **Views**. Cuando hace clic en un enlace externo en el chat, un selector ofrece **Open in app** para usar el panel del navegador u **Default browser** para usar el suyo propio; **Cmd**-clic en macOS o **Ctrl**-clic en Windows abre un enlace en su navegador del sistema directamente. Puede iniciar sesión en sitios en el panel, incluidos flujos de inicio de sesión emergentes como Google OAuth.

Claude puede leer e interactuar con páginas externas usando las mismas herramientas que usa para [verificar su aplicación](#preview-your-app), con dos verificaciones de seguridad adicionales:

* Los clasificadores de seguridad revisan las acciones de escritura de Claude en páginas externas, como hacer clic y escribir, en cada modo de permisos. Estos son los mismos clasificadores que [auto mode](#choose-a-permission-mode) usa, y cuando marcan una acción, obtiene un aviso de permisos independientemente del modo.
* En modos de permisos distintos de Auto y Bypass permissions, una verificación de lista de permitidos de dominio también se aplica antes de que Claude navegue a un nuevo sitio.

<h4 id="approve-claude’s-actions-on-a-site">
  Aprobar las acciones de Claude en un sitio
</h4>

La primera vez que Claude actúa en un sitio externo, aparece una tarjeta de permisos y Claude espera su elección: **Allow once**, **Always allow** o **Deny**. **Allow once** aprueba la acción sin guardar nada. **Always allow** guarda la aprobación para ese sitio en su dispositivo, y puede revocarla en Configuración. Cada sitio necesita su propia aprobación, incluidos los subdominios. Sus servidores de desarrollo local y archivos de proyecto no necesitan aprobación, por lo que [auto-verify](#auto-verify-changes) sigue funcionando sin avisos.

Incluso en un sitio aprobado, Claude no comprará artículos, creará cuentas ni omitirá CAPTCHAs sin su entrada. Navegar en el panel del navegador usa el mismo modelo de seguridad que la [extensión Claude en Chrome](/docs/es/chrome). Consulte [Usar Claude en Chrome de forma segura](https://support.claude.com/en/articles/12902428-using-claude-in-chrome-safely) para saber cómo Claude maneja sitios sensibles y acciones riesgosas.

<h4 id="choose-between-the-browser-and-the-chrome-extension">
  Elegir entre el panel del navegador y la extensión de Chrome
</h4>

El panel del navegador usa un perfil de navegador limpio, separado de su navegador personal, sin ninguno de sus inicios de sesión guardados o historial. Úselo para construir y probar su aplicación y para sitios que no necesitan su identidad. Cuando desee que Claude actúe como usted en sus sesiones iniciadas, use la [extensión Claude en Chrome](/docs/es/chrome) en su lugar, que comparte el estado de inicio de sesión de su navegador.

<h4 id="restrict-external-browsing-for-your-organization">
  Restringir la navegación externa para su organización
</h4>

El navegador sigue los mismos [controles de lista de permitidos y lista de bloqueos de sitios](https://support.claude.com/en/articles/13065128-claude-in-chrome-admin-controls) que la extensión Claude en Chrome. Si su organización ya configuró esas listas para la extensión, el navegador las respeta automáticamente. Los administradores también pueden desactivar las herramientas de Claude en páginas externas con la configuración administrada [`browserExternalPageTools`](#managed-settings). Con las herramientas desactivadas, los usuarios aún pueden navegar a sitios externos; las herramientas de Claude no pueden leerlos ni actuar sobre ellos.

Para desactivar la navegación externa por completo, establezca la configuración administrada [`disableBrowserExternalNavigation`](#managed-settings) en `true`. Esto bloquea toda la navegación externa en el navegador, incluidos los sitios en la lista de permitidos de su organización; los servidores de desarrollo localhost y las vistas previas de archivos siguen funcionando. Use `browserExternalPageTools` para permitir que los usuarios sigan navegando por sitios externos sin las herramientas de Claude, y `disableBrowserExternalNavigation` para bloquear sitios externos tanto para usuarios como para Claude.

<h3 id="review-changes-with-diff-view">
  Revisar cambios con vista de diferencias
</h3>

Después de que Claude realiza cambios en su código, la vista de diferencias le permite revisar modificaciones archivo por archivo antes de crear una solicitud de extracción.

Cuando Claude cambia archivos, aparece un indicador de estadísticas de diferencias que muestra el número de líneas agregadas y eliminadas, como `+12 -1`. Haga clic en este indicador para abrir el visor de diferencias, que muestra una lista de archivos a la izquierda y los cambios para cada archivo a la derecha.

Para comentar en líneas específicas, haga clic en cualquier línea en la diferencia para abrir un cuadro de comentarios. Escriba su comentario y presione **Enter** para agregar el comentario. Después de agregar comentarios a varias líneas, envíe todos los comentarios a la vez:

* **macOS**: presione **Cmd+Enter**
* **Windows**: presione **Ctrl+Enter**

Claude lee sus comentarios y realiza los cambios solicitados, que aparecen como una nueva diferencia que puede revisar.

<h3 id="review-your-code">
  Revisar su código
</h3>

En la vista de diferencias, haga clic en **Review code** en la barra de herramientas superior derecha para pedirle a Claude que evalúe los cambios antes de confirmar. Claude examina las diferencias actuales y deja comentarios directamente en la vista de diferencias. Puede responder a cualquier comentario o pedirle a Claude que revise.

La revisión se enfoca en problemas de alta señal: errores de compilación, errores de lógica definitivos, vulnerabilidades de seguridad y errores obvios. No marca estilo, formato, problemas preexistentes o nada que un linter detectaría.

<h3 id="monitor-pull-request-status">
  Monitorear el estado de la solicitud de extracción
</h3>

Después de abrir una solicitud de extracción, aparece una barra de estado de CI en la sesión. Claude Code usa la CLI de GitHub para sondear resultados de verificación y mostrar fallas.

* **Auto-fix**: cuando está habilitado, Claude intenta automáticamente corregir verificaciones de CI fallidas leyendo la salida de falla e iterando.
* **Auto-merge**: cuando está habilitado, Claude fusiona el PR una vez que todas las verificaciones pasan. El método de fusión es squash. Auto-merge debe estar [habilitado en la configuración de su repositorio de GitHub](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-auto-merge-for-pull-requests-in-your-repository) para que esto funcione.

Use los controles deslizantes **Auto-fix** y **Auto-merge** en la barra de estado de CI para habilitar cualquiera de las opciones. Claude Code también envía una notificación de escritorio cuando CI finaliza. Para archivar la sesión automáticamente una vez que el PR se fusiona o cierra, active [auto-archive](#work-in-parallel-with-sessions) en Configuración → Claude Code.

<Note>
  El monitoreo de PR requiere que la [CLI de GitHub (`gh`)](https://cli.github.com/) esté instalada y autenticada en su máquina. Si `gh` no está instalado, Desktop le solicita que lo instale la primera vez que intente crear un PR.
</Note>

<h2 id="arrange-your-workspace">
  Organizar su espacio de trabajo
</h2>

La pestaña Code está construida alrededor de paneles que puede organizar en cualquier diseño: chat, diff, navegador, terminal, archivo, plan, tareas y subagente. Arrastre un panel por su encabezado para reposicionarlo, o arrastre un borde de panel para redimensionarlo. Presione **Cmd+\\** en macOS o **Ctrl+\\** en Windows para cerrar el panel enfocado. Abra paneles adicionales desde el menú **Views** en la barra de herramientas de la sesión.

<Note>
  El diseño del panel, terminal, editor de archivos y modos de vista en esta sección requieren Claude Desktop v1.2581.0 o posterior. Abra **Claude → Check for Updates** en macOS o **Help → Check for Updates** en Windows para actualizar.
</Note>

<h3 id="run-commands-in-the-terminal">
  Ejecutar comandos en la terminal
</h3>

La terminal integrada le permite ejecutar comandos junto a su sesión sin cambiar a otra aplicación. Ábrala desde el menú **Views** o presione **Ctrl+\`** en macOS o Windows. La terminal se abre en el directorio de trabajo de su sesión y comparte el mismo entorno que Claude, por lo que comandos como `npm test` o `git status` ven los mismos archivos que Claude está editando. Para abrir una segunda pestaña de terminal, haga clic en **+** en el encabezado del panel de terminal o haga clic con el botón derecho en una carpeta en el chat para elegir **Open in terminal**. La terminal está disponible solo en sesiones locales.

<h3 id="open-and-edit-files">
  Abrir y editar archivos
</h3>

Haga clic en una ruta de archivo en el chat o visor de diferencias para abrirlo en el panel de archivos. Las rutas HTML, PDF, imagen y vídeo se abren en el [panel de navegador](#preview-your-app) en su lugar. Realice ediciones puntuales y haga clic en **Save** para escribirlas de vuelta. Si el archivo cambió en el disco desde que lo abrió, el panel le advierte y le permite anular o descartar. Haga clic en **Discard** para revertir sus ediciones, o haga clic en la ruta en el encabezado del panel para copiar la ruta absoluta.

El panel de archivos está disponible en sesiones locales y SSH. Para sesiones remotas, pídale a Claude que realice el cambio.

<h3 id="open-files-in-other-apps">
  Abrir archivos en otras aplicaciones
</h3>

Haga clic con el botón derecho en cualquier ruta de archivo en el chat, visor de diferencias o panel de archivos para abrir un menú contextual:

* **Attach as context**: agregue el archivo a su siguiente solicitud
* **Open in**: abra el archivo en un editor instalado como VS Code, Cursor o Zed
* **Show in Finder** en macOS, **Show in Explorer** en Windows: abra la carpeta contenedora
* **Copy path**: copie la ruta absoluta a su portapapeles

<h3 id="switch-view-modes">
  Cambiar modos de vista
</h3>

Los modos de vista controlan cuánto detalle aparece en la transcripción del chat. Cambie de modo desde el menú desplegable **Transcript view** junto al botón de envío, o presione **Ctrl+O** en macOS o Windows para ciclar a través de ellos.

| Modo        | Lo que muestra                                                                     |
| ----------- | ---------------------------------------------------------------------------------- |
| **Normal**  | Llamadas de herramientas contraídas en resúmenes, con respuestas de texto completo |
| **Verbose** | Cada llamada de herramienta, lectura de archivo y paso intermedio que Claude toma  |
| **Summary** | Solo las respuestas finales de Claude y los cambios que realizó                    |

Use Verbose cuando depure por qué Claude tomó una acción particular. Use Summary cuando esté ejecutando múltiples sesiones y desee escanear resultados rápidamente.

<h3 id="keyboard-shortcuts">
  Atajos de teclado
</h3>

Presione **Cmd+/** en macOS o **Ctrl+/** en Windows para ver todos los atajos disponibles en la pestaña Code. En Windows, use **Ctrl** en lugar de **Cmd** para los atajos a continuación. El ciclismo de sesiones, el alternador de terminal y el alternador de modo de vista usan **Ctrl** en todas las plataformas.

| Atajo                                 | Acción                                  |
| ------------------------------------- | --------------------------------------- |
| `Cmd` `/`                             | Mostrar atajos de teclado               |
| `Cmd` `N`                             | Nueva sesión                            |
| `Cmd` `W`                             | Cerrar sesión                           |
| `Ctrl` `Tab` / `Ctrl` `Shift` `Tab`   | Siguiente o sesión anterior             |
| `Cmd` `Shift` `]` / `Cmd` `Shift` `[` | Siguiente o sesión anterior             |
| `Esc`                                 | Detener respuesta de Claude             |
| `Cmd` `Shift` `D`                     | Alternar panel de diferencias           |
| `Cmd` `Shift` `B`                     | Alternar panel de navegador             |
| `Cmd` `Shift` `S`                     | Seleccionar un elemento en el navegador |
| `Ctrl` `` ` ``                        | Alternar panel de terminal              |
| `Cmd` `\`                             | Cerrar panel enfocado                   |
| `Cmd` `;`                             | Abrir chat lateral                      |
| `Ctrl` `O`                            | Ciclar modos de vista                   |
| `Cmd` `Shift` `M`                     | Abrir menú de modo de permisos          |
| `Cmd` `Shift` `I`                     | Abrir menú de modelo                    |
| `Cmd` `Shift` `E`                     | Abrir menú de esfuerzo                  |
| `1`–`9`                               | Seleccionar elemento en un menú abierto |

Estos atajos se aplican solo a la pestaña Code. Los [atajos de modo interactivo](/docs/es/interactive-mode#keyboard-shortcuts) basados en terminal, como `Shift+Tab` para ciclar modos, no se aplican en Desktop.

<h3 id="check-usage">
  Verificar uso
</h3>

Haga clic en el anillo de uso junto al selector de modelo para ver su uso actual de la ventana de contexto y su uso del plan para el período. El uso de contexto es por sesión; el uso del plan se comparte en todas sus superficies de Claude Code.

<h2 id="let-claude-use-your-computer">
  Permitir que Claude use su computadora
</h2>

El uso de computadora permite que Claude abra sus aplicaciones, controle su pantalla y trabaje directamente en su máquina de la manera que lo haría. Pídale a Claude que pruebe una aplicación nativa en un simulador móvil, interactúe con una herramienta de escritorio que no tiene CLI, o automatice algo que solo funciona a través de una GUI.

<Note>
  El uso de computadora es una vista previa de investigación en macOS y Windows que requiere un plan Pro o Max. No está disponible en planes Team o Enterprise. La aplicación Claude Desktop debe estar en ejecución.
</Note>

El uso de computadora está deshabilitado de forma predeterminada. [Habilítelo en Configuración](#enable-computer-use) antes de que Claude pueda controlar su pantalla. En macOS, también necesita otorgar permisos de Accesibilidad y Grabación de pantalla.

<Warning>
  A diferencia de la [herramienta Bash sandboxed](/docs/es/sandboxing), el uso de computadora se ejecuta en su escritorio real con acceso a lo que apruebe. Claude verifica cada acción e identifica posibles inyecciones de solicitud desde contenido en pantalla, pero el límite de confianza es diferente. Consulte la [guía de seguridad de uso de computadora](https://support.claude.com/en/articles/14128542) para obtener mejores prácticas.
</Warning>

<h3 id="when-computer-use-applies">
  Cuándo se aplica el uso de computadora
</h3>

Claude tiene varias formas de interactuar con una aplicación o servicio, y el uso de computadora es la más amplia y lenta. Intenta la herramienta más precisa primero:

* Si tiene un [conector](#connect-external-tools) para un servicio, Claude usa el conector.
* Si la tarea es un comando de shell, Claude usa Bash.
* Si la tarea es trabajo en navegador y tiene [Claude en Chrome](/docs/es/chrome) configurado, Claude usa eso.
* Si ninguno de esos se aplica, Claude usa el uso de computadora.

Los [niveles de acceso por aplicación](#app-permissions) refuerzan esto: los navegadores están limitados a solo lectura, y las terminales e IDE a solo clic, dirigiendo a Claude hacia la herramienta dedicada incluso cuando el uso de computadora está activo. El control de pantalla se reserva para cosas que nada más puede alcanzar, como aplicaciones nativas, paneles de control de hardware, simuladores móviles o herramientas propietarias sin una API.

<h3 id="enable-computer-use">
  Habilitar el uso de computadora
</h3>

El uso de computadora está deshabilitado de forma predeterminada. Si le pide a Claude que haga algo que lo necesita mientras está deshabilitado, Claude le dice que podría hacer la tarea si habilita el uso de computadora en Configuración.

<Steps>
  <Step title="Actualizar la aplicación de escritorio">
    Asegúrese de tener la última versión de Claude Desktop. En macOS y Windows, descargue o actualice en [claude.com/download](https://claude.com/download); en Linux, actualice a través de su gestor de paquetes ([instrucciones](/docs/es/desktop-linux)). Luego reinicie la aplicación.
  </Step>

  <Step title="Activar el control deslizante">
    En la aplicación de escritorio, vaya a **Configuración > General** (bajo **Aplicación de escritorio**). Encuentre el control deslizante **Computer use** y actívelo. En Windows, el control deslizante surte efecto inmediatamente y la configuración está completa. En macOS, continúe con el siguiente paso.

    Si no ve el control deslizante, confirme que está en macOS o Windows con un plan Pro o Max, luego actualice y reinicie la aplicación.
  </Step>

  <Step title="Otorgar permisos de macOS">
    En macOS, otorgue dos permisos del sistema antes de que el control deslizante surta efecto:

    * **Accessibility**: permite que Claude haga clic, escriba y desplace
    * **Screen Recording**: permite que Claude vea lo que hay en su pantalla

    La página de Configuración muestra el estado actual de cada permiso. Si alguno se deniega, haga clic en la insignia para abrir el panel de Configuración del Sistema relevante.
  </Step>
</Steps>

<h3 id="app-permissions">
  Permisos de aplicación
</h3>

La primera vez que Claude necesita usar una aplicación, aparece un aviso en su sesión. Haga clic en **Allow for this session** o **Deny**. Las aprobaciones duran para la sesión actual, o 30 minutos en [sesiones generadas por Dispatch](#sessions-from-dispatch).

El aviso también muestra qué nivel de control obtiene Claude para esa aplicación. Estos niveles se fijan por categoría de aplicación y no se pueden cambiar:

| Nivel        | Lo que Claude puede hacer                                            | Se aplica a                         |
| :----------- | :------------------------------------------------------------------- | :---------------------------------- |
| View only    | Ver la aplicación en capturas de pantalla                            | Navegadores, plataformas de trading |
| Click only   | Hacer clic y desplazarse, pero no escribir ni usar atajos de teclado | Terminales, IDE                     |
| Full control | Hacer clic, escribir, arrastrar y usar atajos de teclado             | Todo lo demás                       |

Las aplicaciones con amplio alcance como terminales, Finder o File Explorer y Configuración del Sistema o Settings muestran una advertencia adicional en el aviso para que sepa qué aprobación les otorga.

Puede configurar dos configuraciones en **Configuración > General** (bajo **Aplicación de escritorio**):

* **Denied apps**: agregue aplicaciones aquí para rechazarlas sin solicitar. Claude aún puede afectar una aplicación denegada indirectamente a través de acciones en una aplicación permitida, pero no puede interactuar directamente con la aplicación denegada.
* **Unhide apps when Claude finishes**: mientras Claude está trabajando, sus otras ventanas se ocultan para que interactúe solo con la aplicación aprobada. Cuando Claude termina, las ventanas ocultas se restauran a menos que desactive esta configuración.

<h2 id="manage-sessions">
  Gestionar sesiones
</h2>

Cada sesión es una conversación independiente con su propio contexto y cambios. Puede ejecutar múltiples sesiones en paralelo, ramificar chats laterales, enviar trabajo a la nube o permitir que Dispatch inicie sesiones para usted desde su teléfono.

<h3 id="work-in-parallel-with-sessions">
  Trabajar en paralelo con sesiones
</h3>

Haga clic en **+ New session** en la barra lateral, o presione **Cmd+N** en macOS o **Ctrl+N** en Windows, para trabajar en múltiples tareas en paralelo. Presione **Ctrl+Tab** y **Ctrl+Shift+Tab** para ciclar a través de sesiones en la barra lateral. Para repositorios de Git, cada sesión obtiene su propia copia aislada de su proyecto usando [Git worktrees](/docs/es/worktrees), por lo que los cambios en una sesión no afectan otras sesiones hasta que los confirme.

Para ver dos sesiones a la vez, mantenga presionado **Cmd** en macOS o **Ctrl** en Windows y haga clic en una sesión en la barra lateral. La sesión se abre en un segundo panel junto al que ya tiene abierto. Mientras la división está activa, hacer clic en otra sesión de la barra lateral reemplaza el panel que tenga el foco. Presione **Cmd+\\** en macOS o **Ctrl+\\** en Windows para cerrar el panel enfocado y volver a una única sesión.

Los worktrees se almacenan en `<project-root>/.claude/worktrees/` de forma predeterminada. Puede cambiar esto a un directorio personalizado en Configuración → Claude Code bajo "Worktree location". También puede establecer un prefijo de rama que se antepone a cada nombre de rama de worktree, lo que es útil para mantener las ramas creadas por Claude organizadas. Para eliminar un worktree cuando haya terminado, pase el cursor sobre la sesión en la barra lateral y haga clic en el icono de archivo. Para que las sesiones se archiven automáticamente cuando su solicitud de extracción se fusiona o cierra, active **Auto-archive after PR merge or close** en Configuración → Claude Code. Auto-archive solo se aplica a sesiones locales que han terminado de ejecutarse.

Para incluir archivos ignorados por git como `.env` en nuevos worktrees, cree un [archivo `.worktreeinclude`](/docs/es/worktrees#copy-gitignored-files-into-worktrees) en la raíz de su proyecto.

<Note>
  El aislamiento de sesión requiere [Git](https://git-scm.com/downloads). La mayoría de las Macs incluyen Git de forma predeterminada. Ejecute `git --version` en Terminal para verificar. En Windows, Git es necesario para que la pestaña Code funcione: [descargue Git para Windows](https://git-scm.com/downloads/win), instálelo y reinicie la aplicación. Si encuentra errores de Git, pida ayuda a Claude en la [pestaña Cowork](https://claude.com/product/cowork) para solucionar problemas de su configuración.
</Note>

Use los controles en la parte superior de la barra lateral para filtrar sesiones por estado, proyecto o entorno, y para agrupar sesiones por proyecto. Para renombrar una sesión, haga clic en el título de la sesión en la barra de herramientas en la parte superior de la sesión activa. Para verificar el uso del contexto, consulte [Verificar uso](#check-usage). Cuando el contexto se llena, Claude resume automáticamente la conversación y continúa trabajando. También puede escribir `/compact` para activar la compresión antes y liberar espacio de contexto. Consulte [la ventana de contexto](/docs/es/how-claude-code-works#the-context-window) para obtener detalles sobre cómo funciona la compresión.

La aplicación de escritorio envía una notificación del sistema operativo cuando una sesión de Code termina una tarea y usted no está viendo actualmente esa sesión.

<h3 id="ask-a-side-question-without-derailing-the-session">
  Hacer una pregunta lateral sin descarrilar la sesión
</h3>

Un chat lateral le permite hacer una pregunta a Claude que usa el contexto de su sesión pero no agrega nada de vuelta a la conversación principal. Úselo cuando desee entender un fragmento de código, verificar una suposición o explorar una idea sin dirigir la sesión fuera de curso.

Presione **Cmd+;** en macOS o **Ctrl+;** en Windows para abrir un chat lateral, o escriba `/btw` en el cuadro de solicitud. El chat lateral puede leer todo en el hilo principal hasta ese punto. Cuando haya terminado, cierre el chat lateral y continúe la sesión principal donde la dejó. Los chats laterales están disponibles en sesiones locales, SSH y WSL.

<h3 id="watch-background-tasks">
  Ver tareas en segundo plano
</h3>

El panel de tareas muestra el trabajo en segundo plano que se ejecuta dentro de la sesión actual: subagentes, comandos de shell en segundo plano y [flujos de trabajo dinámicos](/docs/es/workflows). Ábralo desde el menú **Views** o arrástrelo a su diseño.

Haga clic en cualquier entrada para ver su salida en el panel de subagente o detenerla. Para ver qué están haciendo otras sesiones, use la [barra lateral](#work-in-parallel-with-sessions).

<h3 id="run-long-running-tasks-remotely">
  Ejecutar tareas de larga duración de forma remota
</h3>

Para refactorizaciones grandes, suites de pruebas, migraciones u otras tareas de larga duración, seleccione **Remote** en lugar de **Local** al iniciar una sesión. Las sesiones remotas se ejecutan en la infraestructura en la nube de Anthropic y continúan incluso si cierra la aplicación o apaga su computadora. Regrese en cualquier momento para ver el progreso o dirigir a Claude en una dirección diferente. También puede monitorear sesiones remotas desde [claude.ai/code](https://claude.ai/code) o la aplicación Claude iOS.

Las sesiones remotas también admiten múltiples repositorios. Después de seleccionar un entorno en la nube, haga clic en el botón **+** junto a la píldora de repositorio para agregar repositorios adicionales a la sesión. Cada repositorio obtiene su propio selector de rama. Esto es útil para tareas que abarcan múltiples bases de código, como actualizar una biblioteca compartida y sus consumidores.

Consulte [Claude Code en la web](/docs/es/claude-code-on-the-web) para obtener más información sobre cómo funcionan las sesiones remotas.

<h3 id="continue-in-another-surface">
  Continuar en otra superficie
</h3>

El menú **Continue in**, accesible desde el icono de VS Code en la esquina inferior derecha de la barra de herramientas de la sesión, le permite mover su sesión a otra superficie:

* **Claude Code on the Web**: envía su sesión local para continuar ejecutándose de forma remota. Desktop empuja su rama, genera un resumen de la conversación y crea una nueva sesión remota con el contexto completo. Luego puede elegir archivar la sesión local o mantenerla. Esto requiere un árbol de trabajo limpio y no está disponible para sesiones SSH.
* **Your IDE**: abre su proyecto en un IDE compatible en el directorio de trabajo actual.

<h3 id="sessions-from-dispatch">
  Sesiones desde Dispatch
</h3>

[Dispatch](https://support.claude.com/en/articles/13947068) es una conversación persistente con Claude que vive en la pestaña [Cowork](https://claude.com/product/cowork). Usted envía un mensaje a Dispatch con una tarea, y decide cómo manejarla.

Una tarea puede terminar como una sesión de Code de dos formas: usted solicita una directamente, como "abra una sesión de Claude Code y corrija el error de inicio de sesión", o Dispatch decide que la tarea es trabajo de desarrollo e inicia una por su cuenta. Las tareas que típicamente se enrutan a Code incluyen corregir errores, actualizar dependencias, ejecutar pruebas o abrir solicitudes de extracción. La investigación, edición de documentos y trabajo con hojas de cálculo permanecen en Cowork.

De cualquier forma, la sesión de Code aparece en la barra lateral de la pestaña Code con una insignia **Dispatch**. Obtiene una notificación push en su teléfono cuando termina o necesita su aprobación.

Si tiene [uso de computadora](#let-claude-use-your-computer) habilitado, las sesiones de Code generadas por Dispatch también pueden usarlo. Las aprobaciones de aplicaciones en esas sesiones expiran después de 30 minutos y vuelven a solicitar, en lugar de durar la sesión completa como las sesiones de Code regulares.

Para configuración, emparejamiento y configuración de Dispatch, consulte el [artículo de ayuda de Dispatch](https://support.claude.com/en/articles/13947068). Dispatch requiere un plan Pro o Max y no está disponible en planes Team o Enterprise.

Dispatch es una de varias formas de trabajar con Claude cuando está lejos de su terminal. Consulte [Plataformas e integraciones](/docs/es/platforms#work-when-you-are-away-from-your-terminal) para compararlo con Remote Control, Channels, Slack y tareas programadas.

<h2 id="extend-claude-code">
  Extender Claude Code
</h2>

Conecte servicios externos, agregue flujos de trabajo reutilizables, personalice el comportamiento de Claude y configure servidores de vista previa. Para administrar conectores, skills y plugins en un solo lugar, haga clic en **Customize** en la barra lateral.

<h3 id="connect-external-tools">
  Conectar herramientas externas
</h3>

Para sesiones locales y [SSH](#ssh-sessions), haga clic en el botón **+** junto al cuadro de solicitud y seleccione **Connectors** para agregar integraciones como Google Calendar, Slack, GitHub, Linear, Notion y más. Puede agregar conectores antes o durante una sesión. El botón **+** no está disponible en sesiones en la nube o WSL, pero [routines](/docs/es/routines) configuran conectores en el momento de la creación de la rutina.

Para administrar o desconectar conectores, vaya a Configuración → Connectors en la aplicación de escritorio, o seleccione **Manage connectors** desde el menú Connectors en el cuadro de solicitud.

Una vez conectado, Claude puede leer su calendario, enviar mensajes, crear problemas e interactuar con sus herramientas directamente. Puede preguntarle a Claude qué conectores están configurados en su sesión.

Los conectores son [MCP servers](/docs/es/mcp) con un flujo de configuración gráfico. Úselos para integración rápida con servicios compatibles. Para integraciones no listadas en Connectors, agregue MCP servers manualmente a través de [archivos de configuración](/docs/es/mcp#installing-mcp-servers). También puede [crear conectores personalizados](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp).

<h3 id="use-skills">
  Usar skills
</h3>

[Skills](/docs/es/skills) extienden lo que Claude puede hacer. Claude los carga automáticamente cuando son relevantes, o puede invocar uno directamente: escriba `/` en el cuadro de solicitud o haga clic en el botón **+** y seleccione **Slash commands** para ver lo que está disponible. Esto incluye [comandos integrados](/docs/es/commands), sus [skills personalizados](/docs/es/skills#create-your-first-skill), skills del proyecto desde su base de código y skills de cualquier [plugin instalado](/docs/es/plugins). Seleccione uno y aparecerá resaltado en el campo de entrada. Escriba su tarea después de él y envíe como de costumbre.

Puede enviar un comando mientras Claude está trabajando, igual que cualquier otro mensaje, y la sesión vuelve a estar inactiva una vez que se completa el turno. Antes de v2.1.206, un comando enviado a mitad de turno podría dejar la sesión mostrándose como en ejecución y los mensajes que envió después no se entregaban.

<h3 id="install-plugins">
  Instalar plugins
</h3>

[Plugins](/docs/es/plugins) son paquetes reutilizables que agregan skills, agentes, hooks, MCP servers y configuraciones LSP a Claude Code. Puede instalar plugins desde la aplicación de escritorio sin usar la terminal.

Para sesiones locales y [SSH](#ssh-sessions), haga clic en el botón **+** junto al cuadro de solicitud y seleccione **Plugins** para ver sus plugins instalados y sus skills. Para agregar un plugin, seleccione **Add plugin** del submenú para abrir el navegador de plugins, que muestra plugins disponibles desde sus [marketplaces](/docs/es/plugin-marketplaces) configurados incluyendo el marketplace oficial de Anthropic. Seleccione **Manage plugins** para habilitar, deshabilitar o desinstalar plugins.

Los plugins pueden estar limitados a su cuenta de usuario, un proyecto específico o solo locales. Si su organización gestiona plugins centralmente, esos plugins están disponibles en sesiones de escritorio de la misma manera que en la CLI. Los plugins no están disponibles para sesiones en la nube o WSL. Para la referencia completa de plugins incluyendo crear sus propios plugins, consulte [plugins](/docs/es/plugins).

<h3 id="configure-preview-servers">
  Configurar servidores de vista previa
</h3>

Claude detecta automáticamente su configuración de servidor de desarrollo y almacena la configuración en `.claude/launch.json` en la raíz de la carpeta que seleccionó al iniciar la sesión. Preview usa esta carpeta como su directorio de trabajo, por lo que si seleccionó una carpeta principal, las subcarpetas con sus propios servidores de desarrollo no se detectarán automáticamente. Para trabajar con el servidor de una subcarpeta, inicie una sesión en esa carpeta directamente o agregue una configuración manualmente.

Para personalizar cómo se inicia su servidor, por ejemplo para usar `yarn dev` en lugar de `npm run dev` o para cambiar el puerto, edite el archivo manualmente o haga clic en **Edit configuration** en el menú desplegable del servidor para abrirlo en su editor de código. El archivo admite JSON con comentarios.

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

Puede definir múltiples configuraciones para ejecutar diferentes servidores desde el mismo proyecto, como un frontend y una API. Consulte los [ejemplos](#examples) a continuación.

<h4 id="auto-verify-changes">
  Verificación automática de cambios
</h4>

Cuando `autoVerify` está habilitado, Claude verifica automáticamente cambios de código después de editar archivos. Toma capturas de pantalla, verifica errores y confirma que los cambios funcionan antes de completar su respuesta.

Auto-verify está habilitado de forma predeterminada. Desactívelo por proyecto agregando `"autoVerify": false` a `.claude/launch.json`, o alterne desde el menú desplegable del servidor.

```json theme={null}
{
  "version": "0.0.1",
  "autoVerify": false,
  "configurations": [...]
}
```

Cuando está deshabilitado, las herramientas de vista previa aún están disponibles y puede pedirle a Claude que verifique en cualquier momento. Auto-verify lo hace automático después de cada edición.

<h4 id="configuration-fields">
  Campos de configuración
</h4>

Cada entrada en el array `configurations` acepta los siguientes campos:

| Campo               | Tipo      | Descripción                                                                                                                                                                                                                                                                                           |
| ------------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`              | string    | Un identificador único para este servidor                                                                                                                                                                                                                                                             |
| `runtimeExecutable` | string    | El comando a ejecutar, como `npm`, `yarn` o `node`                                                                                                                                                                                                                                                    |
| `runtimeArgs`       | string\[] | Argumentos pasados a `runtimeExecutable`, como `["run", "dev"]`                                                                                                                                                                                                                                       |
| `port`              | number    | El puerto en el que escucha su servidor. Por defecto es 3000                                                                                                                                                                                                                                          |
| `cwd`               | string    | Directorio de trabajo relativo a la raíz de su proyecto. Por defecto es la raíz del proyecto. Use `${workspaceFolder}` para hacer referencia a la raíz del proyecto explícitamente                                                                                                                    |
| `env`               | object    | Variables de entorno adicionales como pares clave-valor, como `{ "NODE_ENV": "development" }`. No ponga secretos aquí ya que este archivo se confirma en su repositorio. Para pasar secretos a su servidor de desarrollo, establézcalos en el [editor de entorno local](#local-sessions) en su lugar. |
| `autoPort`          | boolean   | Cómo manejar conflictos de puerto. Consulte a continuación                                                                                                                                                                                                                                            |
| `program`           | string    | Un script a ejecutar con `node`. Consulte [cuándo usar `program` vs `runtimeExecutable`](#when-to-use-program-vs-runtimeexecutable)                                                                                                                                                                   |
| `args`              | string\[] | Argumentos pasados a `program`. Solo se usa cuando `program` está establecido                                                                                                                                                                                                                         |

<a id="when-to-use-program-vs-runtimeexecutable" />

<h5 id="when-to-use-program-vs-runtimeexecutable">
  Cuándo usar `program` vs `runtimeExecutable`
</h5>

Use `runtimeExecutable` con `runtimeArgs` para iniciar un servidor de desarrollo a través de un administrador de paquetes. Por ejemplo, `"runtimeExecutable": "npm"` con `"runtimeArgs": ["run", "dev"]` ejecuta `npm run dev`.

Use `program` cuando tenga un script independiente que desee ejecutar con `node` directamente. Por ejemplo, `"program": "server.js"` ejecuta `node server.js`. Pase banderas adicionales con `args`.

<h4 id="port-conflicts">
  Conflictos de puerto
</h4>

El campo `autoPort` controla qué sucede cuando su puerto preferido ya está en uso:

* **`true`**: Claude encuentra y usa un puerto libre automáticamente. Adecuado para la mayoría de servidores de desarrollo.
* **`false`**: Claude falla con un error. Use esto cuando su servidor debe usar un puerto específico, como para devoluciones de llamada OAuth o listas de permitidos CORS.
* **No establecido (predeterminado)**: Claude pregunta si el servidor necesita ese puerto exacto, luego guarda su respuesta.

Cuando Claude elige un puerto diferente, pasa el puerto asignado a su servidor a través de la variable de entorno `PORT`.

<h4 id="examples">
  Ejemplos
</h4>

Estas configuraciones muestran configuraciones comunes para diferentes tipos de proyectos:

<Tabs>
  <Tab title="Next.js">
    Esta configuración ejecuta una aplicación Next.js usando Yarn en el puerto 3000:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "web",
          "runtimeExecutable": "yarn",
          "runtimeArgs": ["dev"],
          "port": 3000
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Multiple servers">
    Para un monorepo con un servidor frontend y API, defina múltiples configuraciones. El frontend usa `autoPort: true` para que elija un puerto libre si 3000 está ocupado, mientras que el servidor API requiere el puerto 8080 exactamente:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "frontend",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "dev"],
          "cwd": "apps/web",
          "port": 3000,
          "autoPort": true
        },
        {
          "name": "api",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "start"],
          "cwd": "server",
          "port": 8080,
          "env": { "NODE_ENV": "development" },
          "autoPort": false
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Node.js script">
    Para ejecutar un script Node.js directamente en lugar de usar un comando del administrador de paquetes, use el campo `program`:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "server",
          "program": "server.js",
          "args": ["--verbose"],
          "port": 4000
        }
      ]
    }
    ```
  </Tab>
</Tabs>

<h2 id="environment-configuration">
  Configuración del entorno
</h2>

El entorno que elige al [iniciar una sesión](#start-a-session) determina dónde Claude se ejecuta y cómo se conecta:

* **Local**: se ejecuta en su máquina con acceso directo a sus archivos
* **Remote**: se ejecuta en la infraestructura en la nube de Anthropic. Las sesiones continúan incluso si cierra la aplicación.
* **SSH**: se ejecuta en una máquina remota a la que se conecta a través de SSH, como sus propios servidores, máquinas virtuales en la nube o contenedores de desarrollo
* **WSL** (Windows): se ejecuta dentro de una [distribución WSL 2](/docs/es/desktop-wsl) en su máquina, utilizando su cadena de herramientas Linux y rutas nativas

<h3 id="local-sessions">
  Sesiones locales
</h3>

La aplicación de escritorio no siempre hereda su entorno de shell completo. En macOS, cuando inicia la aplicación desde el Dock o Finder, lee su perfil de shell, como `~/.zshrc` o `~/.bashrc`, para extraer `PATH` y un conjunto fijo de variables de Claude Code, pero otras variables que exporta allí no se recogen. En Windows, la aplicación hereda variables de entorno de usuario y sistema pero no lee perfiles de PowerShell.

Para establecer variables de entorno para sesiones locales y servidores de desarrollo en cualquier plataforma, abra el menú desplegable de entorno en el cuadro de solicitud, pase el cursor sobre **Local** y haga clic en el icono de engranaje para abrir el editor de entorno local. Las variables que guarda aquí se almacenan cifradas en su máquina y se aplican a cada sesión local y servidor de vista previa que inicia. También puede agregar variables a la clave `env` en su archivo `~/.claude/settings.json`, aunque estas llegan solo a sesiones de Claude y no a servidores de desarrollo. Consulte [variables de entorno](/docs/es/env-vars) para la lista completa de variables compatibles.

[Extended thinking](/docs/es/model-config#extended-thinking) está habilitado de forma predeterminada, lo que mejora el rendimiento en tareas de razonamiento complejo pero usa tokens adicionales. Para deshabilitar el pensamiento, establezca `MAX_THINKING_TOKENS` en `0` en el editor de entorno local; esto no tiene efecto en Fable 5, que siempre usa extended thinking. En [proveedores de terceros](/docs/es/third-party-integrations), `0` omite el parámetro `thinking` en su lugar, y los modelos de razonamiento adaptativo aún pueden pensar. En modelos con [razonamiento adaptativo](/docs/es/model-config#adjust-effort-level), cualquier otro valor de `MAX_THINKING_TOKENS` se ignora porque el razonamiento adaptativo controla la profundidad del pensamiento en su lugar. En Opus 4.6 y Sonnet 4.6, establezca `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` en `1` para usar un presupuesto de pensamiento fijo; Fable 5, Sonnet 5 y Opus 4.7 y posteriores siempre usan razonamiento adaptativo y no tienen modo de presupuesto fijo.

<h3 id="cloud-sessions">
  Sesiones remotas
</h3>

Las sesiones remotas continúan en segundo plano incluso si cierra la aplicación. El uso cuenta hacia los límites de su [plan de suscripción](/docs/es/costs) sin cargos de computación separados.

Puede crear entornos en la nube personalizados con diferentes niveles de acceso a la red y variables de entorno. Seleccione el menú desplegable de entorno al iniciar una sesión remota y elija **Add environment**. Consulte [el entorno en la nube](/docs/es/claude-code-on-the-web#the-cloud-environment) para obtener detalles sobre la configuración del acceso a la red y variables de entorno.

<h3 id="ssh-sessions">
  Sesiones SSH
</h3>

Las sesiones SSH le permiten ejecutar Claude Code en una máquina remota mientras usa la aplicación de escritorio como su interfaz. Esto es útil para trabajar con bases de código que viven en máquinas virtuales en la nube, contenedores de desarrollo o servidores con hardware o dependencias específicas.

Para agregar una conexión SSH, haga clic en el menú desplegable de entorno antes de iniciar una sesión y seleccione **+ Add SSH connection**. El diálogo solicita:

* **Name**: una etiqueta amigable para esta conexión
* **SSH Host**: `user@hostname` o un host definido en `~/.ssh/config`
* **SSH Port**: por defecto es 22 si se deja vacío, o usa el puerto de su configuración SSH
* **Identity File**: ruta a su clave privada, como `~/.ssh/id_rsa`. Déjelo vacío para usar la clave predeterminada o su configuración SSH.

Una vez agregada, la conexión aparece en el menú desplegable de entorno. Selecciónela para iniciar una sesión en esa máquina. Claude se ejecuta en la máquina remota con acceso a sus archivos y herramientas.

La máquina remota debe ejecutar Linux o macOS. La aplicación de escritorio instala Claude Code en la máquina remota automáticamente la primera vez que se conecta. Una vez conectado, las sesiones SSH admiten modos de permisos, conectores, plugins y servidores MCP.

<h4 id="pre-configure-ssh-connections-for-your-team">
  Pre-configurar conexiones SSH para su equipo
</h4>

Los administradores pueden distribuir conexiones SSH a los miembros del equipo agregando `sshConfigs` a un archivo de [configuración administrada](/docs/es/settings#settings-precedence). Las conexiones definidas de esta manera aparecen en el menú desplegable de entorno de cada usuario automáticamente y se muestran como administradas, por lo que los usuarios pueden seleccionarlas pero no pueden editarlas ni eliminarlas en la aplicación.

El siguiente ejemplo pre-configura una única conexión que se abre en `~/projects` en el host remoto:

```json theme={null}
{
  "sshConfigs": [
    {
      "id": "shared-dev-vm",
      "name": "Shared Dev VM",
      "sshHost": "user@dev.example.com",
      "sshPort": 22,
      "sshIdentityFile": "~/.ssh/id_ed25519",
      "startDirectory": "~/projects"
    }
  ]
}
```

Cada entrada requiere `id`, `name` y `sshHost`. Los campos `sshPort`, `sshIdentityFile` y `startDirectory` son opcionales. Los usuarios también pueden agregar `sshConfigs` a su propio `~/.claude/settings.json`, que es donde se almacenan las conexiones agregadas a través del diálogo.

<h4 id="restrict-which-ssh-hosts-users-can-connect-to">
  Restringir a qué hosts SSH pueden conectarse los usuarios
</h4>

Los administradores pueden limitar las sesiones SSH de Desktop a un conjunto aprobado de hosts agregando `sshHostAllowlist` a un archivo de [configuración administrada](/docs/es/settings#settings-precedence). Cuando se establece, los usuarios solo pueden conectarse a hosts cuyo nombre de host resuelto coincida con uno de los patrones. Establézcalo en una matriz vacía para deshabilitar completamente las sesiones SSH.

El siguiente ejemplo permite conexiones a cualquier host bajo `devboxes.example.com` y a un único host bastión nombrado:

```json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

Los patrones no distinguen entre mayúsculas y minúsculas. `*` coincide con cualquier host, y `*.example.com` coincide con `example.com` y cualquier subdominio. Cualquier otra cosa es una coincidencia exacta. La verificación se ejecuta contra el nombre de host después de la resolución de `~/.ssh/config` a través de `ssh -G`, por lo que se permiten entradas de alias `Host` y `ProxyCommand`/`ProxyJump` siempre que el `HostName` resuelto coincida.

`sshHostAllowlist` se lee solo desde la configuración administrada; los valores en la configuración de usuario o proyecto se ignoran. Solo la aplicación Claude Desktop honra esta configuración; la CLI de Claude Code y las extensiones IDE no la leen, y no restringe los comandos `ssh` ejecutados a través de la herramienta Bash. Rige a qué hosts se conecta la aplicación Desktop, no la salida de red, por lo que emparéjelo con los controles de red de su organización o de confianza cero si necesita un límite duro.

<h2 id="enterprise-configuration">
  Configuración empresarial
</h2>

Las organizaciones en planes Team o Enterprise pueden gestionar el comportamiento de la aplicación de escritorio a través de controles de consola de administración, archivos de configuración administrados y políticas de gestión de dispositivos.

<h3 id="admin-console-controls">
  Controles de consola de administración
</h3>

Estas configuraciones se configuran a través de la [consola de configuración de administración](https://claude.ai/admin-settings/claude-code):

* **Code in the desktop**: controle si los usuarios en su organización pueden acceder a Claude Code en la aplicación de escritorio
* **Code in the web**: habilite o deshabilite [sesiones web](/docs/es/claude-code-on-the-web) para su organización
* **Remote Control**: habilite o deshabilite [Remote Control](/docs/es/remote-control) para su organización
* **Disable Bypass permissions mode**: evite que los usuarios en su organización habiliten el modo bypass permissions

<h3 id="managed-settings">
  Configuración administrada
</h3>

La configuración administrada anula la configuración del proyecto y usuario y se aplica a las sesiones de Claude Code en Desktop. Puede establecer estas claves en el archivo de [configuración administrada](/docs/es/settings#settings-precedence) de su organización o enviarlas de forma remota a través de la consola de administración.

| Clave                                      | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissions.disableBypassPermissionsMode` | establezca en `"disable"` para evitar que los usuarios habiliten el modo bypass permissions.                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `disableAutoMode`                          | establezca en `"disable"` para evitar que los usuarios habiliten el modo [Auto](/docs/es/permission-modes#eliminate-prompts-with-auto-mode). Elimina Auto del selector de modo. También aceptado bajo `permissions`.                                                                                                                                                                                                                                                                                                                                             |
| `autoMode`                                 | personalice lo que el clasificador de modo auto confía y bloquea en toda su organización. Consulte [Configurar el modo auto](/docs/es/auto-mode-config).                                                                                                                                                                                                                                                                                                                                                                                                         |
| `browserExternalPageTools`                 | establezca en `"disabled"` para evitar que Claude use herramientas para leer o actuar en páginas externas en el [panel del navegador](#browse-external-sites). Los usuarios aún pueden navegar a sitios externos por sí mismos, y las vistas previas del servidor de desarrollo local no se ven afectadas.                                                                                                                                                                                                                                                  |
| `disableBrowserExternalNavigation`         | establezca en `true` para desactivar completamente la navegación externa en el [panel del navegador](#browse-external-sites). Ni los usuarios ni Claude pueden navegar a sitios externos, y las vistas previas del servidor de desarrollo localhost no se ven afectadas. El valor debe ser el booleano JSON `true`; la cadena `"true"` se ignora.                                                                                                                                                                                                           |
| `sshConfigs`                               | pre-configure [conexiones SSH](#pre-configure-ssh-connections-for-your-team) que aparecen en el menú desplegable de entorno. Los usuarios no pueden editar ni eliminar conexiones administradas.                                                                                                                                                                                                                                                                                                                                                            |
| `sshHostAllowlist`                         | restrinja [sesiones SSH](#restrict-which-ssh-hosts-users-can-connect-to) a hosts cuyo nombre de host resuelto coincida con uno de estos patrones. Una matriz vacía deshabilita las sesiones SSH. Se lee solo desde configuración administrada.                                                                                                                                                                                                                                                                                                              |
| `managedMcpServers`                        | envíe configuraciones de servidor MCP a todos los usuarios en una implementación de terceros. Cada entrada especifica un transporte de `"http"`, `"sse"` o `"stdio"`, detalles de conexión y opcionalmente un mapa `toolPolicy` que restringe qué herramientas en ese servidor pueden invocar los usuarios. Disponible solo en implementaciones de Desktop de terceros (3P). Entregue esta clave a través del archivo de configuración administrada o MDM, ya que las implementaciones de terceros no reciben configuraciones de consola de administración. |

Las restricciones de modelo como [`availableModels`](/docs/es/model-config#restrict-model-selection) se aplican en las sesiones de Claude Code de Desktop de la misma manera que en la CLI de terminal; consulte [cobertura de superficie](/docs/es/model-config#surface-coverage).

* **Sesiones locales en esta máquina**: se aplica un archivo de configuración administrada implementado en disco. La configuración administrada enviada de forma remota a través de la consola de administración también llega a estas sesiones en la API de Anthropic cuando la sesión se autentica con un inicio de sesión de organización o una clave API configurada directamente, siguiendo la misma [precedencia de configuración](/docs/es/settings#settings-precedence) que la CLI de terminal.
* **[Sesiones en la nube](#cloud-sessions)**: se ejecutan en máquinas virtuales administradas por Anthropic y reciben solo [configuración administrada por servidor](/docs/es/server-managed-settings).
* **[Sesiones SSH](#ssh-sessions)**: la sesión lee el archivo de configuración administrada desde el host remoto. Desktop mismo lee `sshConfigs` y `sshHostAllowlist` desde la configuración administrada de la máquina local al crear la conexión.

`permissions.disableBypassPermissionsMode` y `disableAutoMode` también funcionan en configuración de usuario y proyecto, pero colocarlos en configuración administrada evita que los usuarios los anulen.

Claude Code lee `autoMode` desde configuración de usuario, la bandera `--settings` y configuración administrada, pero no desde `.claude/settings.json` o `.claude/settings.local.json`: ambos archivos viven en el directorio del repositorio, por lo que un repositorio clonado o paso de compilación no puede inyectar sus propias reglas de clasificador. Antes de v2.1.207, Claude Code también leía `.claude/settings.local.json`.

Para la lista completa de configuraciones solo administradas incluyendo `allowManagedPermissionRulesOnly` y `allowManagedHooksOnly`, consulte [configuraciones solo administradas](/docs/es/permissions#managed-only-settings).

<h3 id="device-management-policies">
  Políticas de gestión de dispositivos
</h3>

Los equipos de TI pueden gestionar la aplicación de escritorio a través de MDM en macOS o política de grupo en Windows. Las políticas disponibles incluyen habilitar o deshabilitar la función Claude Code, controlar actualizaciones automáticas y establecer una URL de implementación personalizada.

* **macOS**: configure a través del dominio de preferencia `com.anthropic.claudefordesktop` usando herramientas como Jamf o Kandji
* **Windows**: configure a través del registro en `SOFTWARE\Policies\Claude`

<h3 id="network-access-requirements">
  Requisitos de acceso a la red
</h3>

Desktop carga su código de aplicación y contenido de usuario desde hosts de CDN de Anthropic.

```text theme={null}
anthropic.com
*.anthropic.com
claude.ai
*.claude.ai
claude.com
*.claude.com
claude.app
*.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

El tráfico es HTTPS en el puerto 443 a menos que configure un puerto personalizado para [OTLP](/docs/es/monitoring-usage), una puerta de enlace LLM o un servidor MCP.

Para servidores proxy, autoridades de certificados personalizadas, mTLS y los dominios que necesita la CLI independiente, consulte [configuración de red](/docs/es/network-config).

Para reducir el número de caracteres comodín de firewall, permita estos hosts de Anthropic en su lugar. Ciertos subdominios se generan dinámicamente y deben permanecer como caracteres comodín.

```text theme={null}
anthropic.com
api.anthropic.com
a-api.anthropic.com
a-cdn.anthropic.com
s-cdn.anthropic.com
assets-proxy.anthropic.com
claude.ai
a.claude.ai
a-cdn.claude.ai
assets.claude.ai
downloads.claude.ai
*.livepreview.claude.ai
claude.com
platform.claude.com
*.livepreview.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

<h3 id="authentication-and-sso">
  Autenticación y SSO
</h3>

Las organizaciones empresariales pueden requerir SSO para todos los usuarios. Consulte [autenticación](/docs/es/authentication) para obtener detalles a nivel de plan y [Configurar SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso) para la configuración de SAML; la configuración de OIDC se cubre en la [Guía del administrador empresarial de Claude](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide).

<h3 id="data-handling">
  Manejo de datos
</h3>

Claude Code procesa su código localmente en sesiones locales o en la infraestructura en la nube de Anthropic en sesiones en la nube. Las conversaciones y el contexto del código se envían a la API de Anthropic para procesamiento. Consulte [manejo de datos](/docs/es/data-usage) para obtener detalles sobre retención de datos, privacidad y cumplimiento.

<h3 id="deployment">
  Implementación
</h3>

Desktop se puede distribuir a través de herramientas de implementación empresarial:

* **macOS**: distribuya a través de MDM como Jamf o Kandji usando el instalador `.dmg`
* **Windows**: implemente a través del paquete MSIX. Consulte [Deploy Claude Desktop for Windows](https://support.claude.com/en/articles/12622703-deploy-claude-desktop-for-windows) para opciones de implementación empresarial incluyendo instalación silenciosa

Para los dominios a permitir en su firewall, consulte [requisitos de acceso a la red](#network-access-requirements) arriba. Para configuración de proxy, autoridades de certificados personalizadas y puertas de enlace LLM, consulte [configuración de red](/docs/es/network-config).

Para la referencia completa de configuración empresarial, consulte la [guía de configuración empresarial](https://support.claude.com/en/articles/12622667-enterprise-configuration).

<h2 id="coming-from-the-cli">
  ¿Viene de la CLI?
</h2>

Si ya usa la CLI de Claude Code, Desktop ejecuta el mismo motor subyacente con una interfaz gráfica. Puede ejecutar ambos simultáneamente en la misma máquina, incluso en el mismo proyecto. Cada uno mantiene historial de sesión separado, pero comparten configuración y memoria del proyecto a través de archivos CLAUDE.md.

Para mover una sesión de CLI a Desktop, ejecute `/desktop` en la terminal. Claude guarda su sesión y la abre en la aplicación de escritorio, luego sale de la CLI. Este comando está disponible en macOS y Windows cuando inicia sesión con una suscripción de Claude. No está disponible con autenticación de clave API ni en Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry.

<Tip>
  Cuándo usar Desktop vs CLI: use Desktop cuando desee gestionar sesiones paralelas en una ventana, organizar paneles lado a lado o revisar cambios visualmente. Use la CLI cuando necesite scripting, automatización o prefiera un flujo de trabajo de terminal.
</Tip>

<h3 id="cli-flag-equivalents">
  Equivalentes de banderas de CLI
</h3>

Esta tabla muestra el equivalente de la aplicación de escritorio para banderas de CLI comunes. Las banderas no listadas no tienen equivalente de escritorio porque están diseñadas para scripting o automatización.

| CLI                                       | Equivalente de Desktop                                                                                                                                                                             |
| ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--model sonnet`                          | Menú desplegable de modelo junto al botón de envío                                                                                                                                                 |
| `--resume`, `--continue`                  | Haga clic en una sesión en la barra lateral                                                                                                                                                        |
| `--permission-mode`                       | Selector de modo junto al botón de envío                                                                                                                                                           |
| `--dangerously-skip-permissions`          | Modo Bypass permissions. En planes Pro y Max, habilítelo en Configuración → Claude Code → "Allow bypass permissions mode"; en planes Team y Enterprise, la política de la organización lo controla |
| `--add-dir`                               | Agregue múltiples repositorios con el botón **+** en sesiones remotas                                                                                                                              |
| `--allowedTools`, `--disallowedTools`     | Sin equivalente por sesión. Las reglas de permisos en [archivos de configuración](/docs/es/settings) aún se aplican.                                                                                    |
| `--verbose`                               | Modo de vista [Verbose](#switch-view-modes) en el menú desplegable de vista de transcripción                                                                                                       |
| `--print`, `--output-format`              | No disponible. Desktop es solo interactivo.                                                                                                                                                        |
| Variable de entorno `ANTHROPIC_MODEL`     | Menú desplegable de modelo junto al botón de envío                                                                                                                                                 |
| Variable de entorno `MAX_THINKING_TOKENS` | Establezca en el editor de entorno local. Consulte [configuración del entorno](#environment-configuration).                                                                                        |

<h3 id="shared-configuration">
  Configuración compartida
</h3>

Desktop y CLI leen los mismos archivos de configuración, por lo que su configuración se transfiere:

* Los archivos **[CLAUDE.md](/docs/es/memory)** y `CLAUDE.local.md` en su proyecto son utilizados por ambos
* Los **[MCP servers](/docs/es/mcp)** configurados en `~/.claude.json` o `.mcp.json` funcionan en ambos
* Los **[Hooks](/docs/es/hooks)** y **[skills](/docs/es/skills)** definidos en configuración se aplican a ambos
* La **[Configuración](/docs/es/settings)** en `~/.claude.json` y `~/.claude/settings.json` se comparte. Las reglas de permisos, herramientas permitidas y otras configuraciones en `settings.json` se aplican a sesiones de Desktop.
* **Modelos**: los mismos [modelos](/docs/es/model-config#available-models) están disponibles en ambos. En Desktop, seleccione el modelo del menú desplegable junto al botón de envío. Puede cambiar el modelo durante la sesión desde el mismo menú desplegable.

<Note>
  **MCP servers de la aplicación de chat de Claude Desktop**: la aplicación Desktop carga MCP servers de `claude_desktop_config.json` en sesiones de la pestaña Code, junto con servidores de `~/.claude.json` y `.mcp.json`. Un servidor definido en `claude_desktop_config.json` está disponible tanto en la superficie de chat de Desktop como en la pestaña Code.

  La CLI independiente no lee `claude_desktop_config.json`. En macOS y WSL, ejecute `claude mcp add-from-claude-desktop` para copiar esos servidores en `~/.claude.json`. Consulte [Importar MCP servers desde Claude Desktop](/docs/es/mcp#import-mcp-servers-from-claude-desktop) para el flujo de importación y opciones de alcance.
</Note>

<h3 id="feature-comparison">
  Comparación de características
</h3>

Esta tabla compara capacidades principales entre la CLI y Desktop. Para una lista completa de banderas de CLI, consulte la [referencia de CLI](/docs/es/cli-reference).

| Característica                                          | CLI                                                              | Desktop                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------------------------------- | ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Modos de permisos                                       | Todos los modos incluyendo `dontAsk`                             | Manual, Aceptar ediciones, Plan y Auto. Bypass permissions aparece en el selector de modo una vez habilitado: a través del botón de alternancia de Configuración en planes Pro y Max, o a través de la política de la organización en planes Team y Enterprise                                                                                                                                                                          |
| `--dangerously-skip-permissions`                        | Bandera de CLI                                                   | Modo Bypass permissions. En planes Pro y Max, habilítelo en Configuración → Claude Code → "Allow bypass permissions mode"; en planes Team y Enterprise, la política de la organización lo controla                                                                                                                                                                                                                                      |
| [Proveedores de terceros](/docs/es/third-party-integrations) | Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry | API de Anthropic de forma predeterminada. Para enrutamiento de puerta de enlace, consulte [conectar la aplicación de escritorio a una puerta de enlace](/docs/es/llm-gateway-connect#desktop-app). Para ejecutar la pestaña Code en Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o una puerta de enlace LLM autohospedada, consulte [Claude Desktop en 3P](https://claude.com/docs/third-party/claude-desktop/overview). |
| [MCP servers](/docs/es/mcp)                                  | Configurar en archivos de configuración                          | UI de Connectors para sesiones locales y SSH, o archivos de configuración                                                                                                                                                                                                                                                                                                                                                               |
| [Plugins](/docs/es/plugins)                                  | Comando `/plugin`                                                | UI del administrador de plugins                                                                                                                                                                                                                                                                                                                                                                                                         |
| Archivos @mention                                       | Basado en texto                                                  | Con autocompletado; sesiones locales y SSH solamente                                                                                                                                                                                                                                                                                                                                                                                    |
| Archivos adjuntos                                       | No disponible                                                    | Imágenes, PDF                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Aislamiento de sesión                                   | Bandera [`--worktree`](/docs/es/cli-reference)                        | Worktrees automáticos                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Múltiples sesiones                                      | Terminales separadas                                             | Pestañas de barra lateral                                                                                                                                                                                                                                                                                                                                                                                                               |
| Tareas recurrentes                                      | Trabajos cron, tuberías de CI                                    | [Tareas programadas](/docs/es/desktop-scheduled-tasks)                                                                                                                                                                                                                                                                                                                                                                                       |
| Uso de computadora                                      | [Habilitar a través de `/mcp`](/docs/es/computer-use) en macOS        | [Control de aplicaciones y pantalla](#let-claude-use-your-computer) en macOS y Windows                                                                                                                                                                                                                                                                                                                                                  |
| Integración de Dispatch                                 | No disponible                                                    | [Sesiones de Dispatch](#sessions-from-dispatch) en la barra lateral                                                                                                                                                                                                                                                                                                                                                                     |
| Scripting y automatización                              | [`--print`](/docs/es/cli-reference), [Agent SDK](/docs/es/headless)        | No disponible                                                                                                                                                                                                                                                                                                                                                                                                                           |

<h3 id="what’s-not-available-in-desktop">
  Lo que no está disponible en Desktop
</h3>

Las siguientes características están disponibles solo en la CLI o extensión de VS Code, excepto donde se indique:

* **Proveedores de terceros**: Desktop se conecta a la API de Anthropic de forma predeterminada. Para enrutar Desktop a través de una puerta de enlace, consulte [conectar la aplicación de escritorio a una puerta de enlace](/docs/es/llm-gateway-connect#desktop-app). Las implementaciones empresariales pueden configurar Google Cloud's Agent Platform y proveedores de puerta de enlace a través de [configuración administrada](https://claude.com/docs/third-party/claude-desktop/configuration). Para Amazon Bedrock o Microsoft Foundry en la CLI, consulte el [inicio rápido](/docs/es/quickstart). Como excepción a la sección anterior, [Claude Desktop en 3P](https://claude.com/docs/third-party/claude-desktop/overview) ejecuta la pestaña Code en Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry o una puerta de enlace LLM autohospedada.
* **Linux (beta)**: Computer Use aún no está disponible en la aplicación de escritorio Linux. Consulte [Claude Desktop en Linux](/docs/es/desktop-linux).
* **Sugerencias de código en línea**: Desktop no proporciona sugerencias de estilo autocompletado. Funciona a través de solicitudes conversacionales y cambios de código explícitos.
* **Equipos de agentes**: sesiones paralelas de Claude Code que se comunican entre sí están disponibles en la [CLI](/docs/es/agent-teams), no en Desktop. Para trabajo multiagente dentro de una sesión, use [flujos de trabajo dinámicos](/docs/es/workflows), que se ejecutan en Desktop.
* **Comandos de diálogo de terminal**: comandos integrados que abren un panel interactivo en la terminal se comportan de manera diferente en la pestaña Code. Edite [archivos de configuración](/docs/es/settings) directamente para gestionar reglas de permisos y configuración, o ejecute los comandos desde la CLI independiente.
  * Los comandos sin forma de argumento, como `/permissions`, responden con `isn't available in this environment`.
  * `/config` abre Configuración → Claude Code. El texto después del comando se ignora, por lo que `/config theme=dark` no establece el tema.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Las secciones a continuación cubren problemas específicos de la aplicación de escritorio. Para errores de API en tiempo de ejecución que aparecen en el chat como `API Error: 500`, `529 Overloaded`, `429` o `Prompt is too long`, consulte la [referencia de errores](/docs/es/errors). Esos errores y sus soluciones son los mismos en la CLI, escritorio y web.

<h3 id="check-your-version">
  Verificar su versión
</h3>

Para ver qué versión de la aplicación de escritorio está ejecutando:

* **macOS**: haga clic en **Claude** en la barra de menú, luego **About Claude**
* **Windows**: haga clic en **Help**, luego **About**

Haga clic en el número de versión para copiarlo a su portapapeles.

<h3 id="403-or-authentication-errors-in-the-code-tab">
  Errores 403 o de autenticación en la pestaña Code
</h3>

Si ve `Error 403: Forbidden` u otros fallos de autenticación al usar la pestaña Code:

1. Cierre sesión e inicie sesión nuevamente desde el menú de la aplicación. Esta es la solución más común.
2. Verifique que tenga una suscripción de pago activa: Pro, Max, Team o Enterprise.
3. Si la CLI funciona pero Desktop no, cierre completamente la aplicación de escritorio, no solo cierre la ventana, luego reabrala e inicie sesión nuevamente.
4. Verifique su conexión a Internet y configuración de proxy.

<h3 id="blank-or-stuck-screen-on-launch">
  Pantalla en blanco o atascada al iniciar
</h3>

Si la aplicación se abre pero muestra una pantalla en blanco o sin respuesta:

1. Reinicie la aplicación.
2. Verifique si hay actualizaciones pendientes. En macOS y Windows la aplicación se actualiza automáticamente al iniciar; en Linux, actualice a través de apt como se describe en [Claude Desktop en Linux](/docs/es/desktop-linux).
3. En una red administrada, confirme que su firewall permite los hosts de CDN en [requisitos de acceso a la red](#network-access-requirements).
4. En Windows, verifique Event Viewer para registros de bloqueo bajo **Windows Logs → Application**.

<h3 id="failed-to-load-session">
  "Failed to load session"
</h3>

Si ve `Failed to load session`, la carpeta seleccionada puede no existir más, un repositorio de Git puede requerir Git LFS que no está instalado, o los permisos de archivo pueden impedir el acceso. Intente seleccionar una carpeta diferente o reinicie la aplicación.

<h3 id="session-not-finding-installed-tools">
  La sesión no encuentra herramientas instaladas
</h3>

Si Claude no puede encontrar herramientas como `npm`, `node` u otros comandos de CLI, verifique que las herramientas funcionen en su terminal regular, verifique que su perfil de shell configure correctamente PATH y reinicie la aplicación de escritorio para recargar variables de entorno.

<h3 id="git-and-git-lfs-errors">
  Errores de Git y Git LFS
</h3>

En Windows, Git es necesario para que la pestaña Code inicie sesiones locales. Si ve "Git is required," instale [Git para Windows](https://git-scm.com/downloads/win) y reinicie la aplicación.

Si ve "Git LFS is required by this repository but is not installed," instale Git LFS desde [git-lfs.com](https://git-lfs.com/), ejecute `git lfs install` y reinicie la aplicación.

<h3 id="mcp-servers-not-working-on-windows">
  Los MCP servers no funcionan en Windows
</h3>

Si los controles deslizantes de MCP server no responden o los servidores no se conectan en Windows, verifique que el servidor esté configurado correctamente en su configuración, reinicie la aplicación, verifique que el proceso del servidor se esté ejecutando en Task Manager y revise los registros del servidor para errores de conexión.

<h3 id="app-won’t-quit">
  La aplicación no se cierra
</h3>

* **macOS**: presione Cmd+Q. Si la aplicación no responde, use Force Quit con Cmd+Option+Esc, seleccione Claude y haga clic en Force Quit.
* **Windows**: use Task Manager con Ctrl+Shift+Esc para finalizar el proceso de Claude.

<h3 id="windows-specific-issues">
  Problemas específicos de Windows
</h3>

* **PATH no actualizado después de instalar**: abra una nueva ventana de terminal. Las actualizaciones de PATH solo se aplican a nuevas sesiones de terminal.
* **Error de instalación concurrente**: si ve un error sobre otra instalación en progreso pero no la hay, intente ejecutar el instalador como Administrador.

<h3 id="branch-doesn’t-exist-yet-when-opening-in-cli">
  "Branch doesn't exist yet" al abrir en CLI
</h3>

Las sesiones en la nube pueden crear ramas que no existen en su máquina local. Haga clic en el nombre de la rama en la barra de herramientas de la sesión para copiarlo, luego obténgalo localmente:

```bash theme={null}
git fetch origin <branch-name>
git checkout <branch-name>
```

<h3 id="still-stuck">
  ¿Aún atascado?
</h3>

* Abra Help → Get Support en la aplicación de escritorio, o visite el [centro de soporte de Claude](https://support.claude.com/) directamente
* Para problemas que también se reproducen en la CLI independiente `claude`, busque o presente un error en [GitHub Issues](https://github.com/anthropics/claude-code/issues)

Al presentar un problema, incluya la versión de su aplicación de escritorio, su sistema operativo, el mensaje de error exacto y registros relevantes. En macOS, verifique Console.app. En Windows, verifique Event Viewer → Windows Logs → Application.
