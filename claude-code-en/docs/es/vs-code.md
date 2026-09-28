> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usar Claude Code en VS Code

> Instala y configura la extensión Claude Code para VS Code. Obtén asistencia de codificación con IA con diffs en línea, menciones @, revisión de planes y atajos de teclado.

<img src="https://mintcdn.com/claude-code/-YhHHmtSxwr7W8gy/images/vs-code-extension-interface.jpg?fit=max&auto=format&n=-YhHHmtSxwr7W8gy&q=85&s=300652d5678c63905e6b0ea9e50835f8" alt="Editor de VS Code con el panel de extensión Claude Code abierto en el lado derecho, mostrando una conversación con Claude" width="2500" height="1155" data-path="images/vs-code-extension-interface.jpg" />

La extensión de VS Code proporciona una interfaz gráfica nativa para Claude Code, integrada directamente en su IDE. Esta es la forma recomendada de usar Claude Code en VS Code.

Con la extensión, puede revisar y editar los planes de Claude antes de aceptarlos, aceptar automáticamente ediciones a medida que se realizan, mencionar archivos con rangos de líneas específicas de su selección, acceder al historial de conversaciones y abrir múltiples conversaciones en pestañas o ventanas separadas.

<h2 id="prerequisites">
  Requisitos previos
</h2>

Antes de instalar, asegúrate de tener:

* VS Code 1.94.0 o superior
* Una cuenta de Anthropic: cualquier suscripción pagada de Claude (Pro, Max, Team o Enterprise) o una cuenta de Claude Console funciona, y no se requiere clave API. Iniciarás sesión [con esta cuenta](/docs/es/authentication#log-in-to-claude-code) cuando abras la extensión por primera vez. Si accedes a Claude a través de un proveedor de terceros como Amazon Bedrock o Google Cloud's Agent Platform, consulta [Usar proveedores de terceros](#use-third-party-providers) para obtener instrucciones de configuración.

<Tip>
  La extensión incluye su propia copia de la CLI (interfaz de línea de comandos) para el panel de chat. Para ejecutar `claude` en la terminal integrada de VS Code, también necesitas la [instalación de CLI independiente](/docs/es/setup). Consulta [Extensión de VS Code frente a Claude Code CLI](#vs-code-extension-vs-claude-code-cli) para obtener detalles.
</Tip>

<h2 id="install-the-extension">
  Instalar la extensión
</h2>

Haz clic en el enlace de tu IDE para instalar directamente:

* [Instalar para VS Code](vscode:extension/anthropic.claude-code)
* [Instalar para Cursor](cursor:extension/anthropic.claude-code)

O en VS Code, presiona `Cmd+Shift+X` (Mac) o `Ctrl+Shift+X` (Windows/Linux) para abrir la vista Extensiones, busca "Claude Code" y haz clic en **Instalar**.

La extensión también se instala en otros forks de VS Code como Devin Desktop o Kiro. Busca "Claude Code" en la vista Extensiones del editor, o instala desde el [registro Open VSX](https://open-vsx.org/extension/Anthropic/claude-code). Si tu editor no puede instalar la extensión, [instala la CLI](/docs/es/quickstart) y ejecuta `claude` en su terminal integrada en su lugar. La CLI funciona en cualquier terminal.

<Note>Si la extensión no aparece después de la instalación, reinicia VS Code o ejecuta "Developer: Reload Window" desde la Paleta de comandos.</Note>

<h2 id="get-started">
  Comenzar
</h2>

Una vez instalado, puede comenzar a usar Claude Code a través de la interfaz de VS Code:

<Steps>
  <Step title="Abrir el panel de Claude Code">
    En todo VS Code, el icono Spark indica Claude Code: <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Icono Spark" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    La forma más rápida de abrir Claude es hacer clic en el icono Spark en la **Barra de herramientas del editor** (esquina superior derecha del editor). El icono solo aparece cuando tiene un archivo abierto.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="VS Code editor mostrando el icono Spark en la Barra de herramientas del editor" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Otras formas de abrir Claude Code:

    * **Barra de actividades**: haga clic en el icono Spark en la barra lateral izquierda para abrir la lista de sesiones. Haga clic en cualquier sesión para abrirla en su [ubicación preferida](#extension-settings), o inicie una nueva. Este icono siempre es visible en la Barra de actividades.
    * **Paleta de comandos**: `Cmd+Shift+P` (Mac) o `Ctrl+Shift+P` (Windows/Linux), escriba "Claude Code" y seleccione una opción como "Open in New Tab"
    * **Barra de estado**: si ha establecido [`preferredLocation`](#extension-settings) en `sidebar`, o abrió Claude con **Claude Code: Open in Side Bar**, haga clic en **✻ Claude Code** en la esquina inferior derecha de la ventana. Esto funciona incluso cuando no hay ningún archivo abierto.

    Puede arrastrar el panel de Claude para reposicionarlo en cualquier lugar de VS Code. Consulte [Personalizar su flujo de trabajo](#customize-your-workflow) para obtener más detalles.
  </Step>

  <Step title="Iniciar sesión">
    La primera vez que abre el panel, aparece una pantalla de inicio de sesión. Haga clic en **Sign in** y complete la autorización en su navegador.

    Si ve **Not logged in · Please run /login** más tarde, la extensión reabre la pantalla de inicio de sesión automáticamente. Si no aparece, recargue la ventana desde la Paleta de comandos con **Developer: Reload Window**.

    Si tiene `ANTHROPIC_API_KEY` establecido en su shell pero aún ve el mensaje de inicio de sesión, es posible que VS Code no haya heredado su entorno de shell. Inicie VS Code desde una terminal con `code .` para que herede sus variables de entorno, o inicie sesión con su cuenta de Claude en su lugar.

    Después de iniciar sesión, aparece una lista de verificación **Learn Claude Code**. Trabaje en cada elemento haciendo clic en **Show me**, o descártelo con la X. Para reabrirlo más tarde, desmarque **Hide Onboarding** en la configuración de VS Code en Extensions → Claude Code.
  </Step>

  <Step title="Enviar un mensaje">
    Pida a Claude que le ayude con su código o archivos, ya sea explicando cómo funciona algo, depurando un problema o realizando cambios.

    <Tip>Claude ve automáticamente el texto seleccionado. Presione `Option+K` (Mac) / `Alt+K` (Windows/Linux) para insertar también una referencia @-mention (como `@file.ts#5-10`) en su mensaje.</Tip>

    Aquí hay un ejemplo de cómo hacer una pregunta sobre una línea particular en un archivo:

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="VS Code editor con las líneas 2-3 seleccionadas en un archivo Python, y el panel de Claude Code mostrando una pregunta sobre esas líneas con una referencia @-mention" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="Revisar cambios">
    Lo que ve depende del [modo de permiso](/docs/es/permission-modes#which-mode-a-session-starts-in) que se muestra en la parte inferior del cuadro de mensaje:

    * En modo Auto o Edit automatically, Claude edita la mayoría de los archivos en su espacio de trabajo sin preguntar.
    * En modo Manual, cuando Claude quiere editar un archivo, muestra una comparación lado a lado del original y los cambios propuestos, luego solicita permiso. Puede aceptar, rechazar o decirle a Claude qué hacer en su lugar. Si edita el contenido propuesto directamente en la vista de diferencias antes de aceptar, Claude es informado de que lo modificó para que no asuma que el archivo coincide con su propuesta original.

          <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="VS Code mostrando una diferencia de los cambios propuestos por Claude con un mensaje de permiso preguntando si realizar la edición" width="3292" height="1876" data-path="images/vs-code-edits.png" />

    Para revisar una edición propuesta un cambio a la vez, use los botones **Accept this change** y **Reject this change** bajo cada cambio en la diferencia. Rechazar un cambio lo revierte en el contenido propuesto; aceptarlo lo marca como revisado. Aceptar o rechazar el archivo completo aún finaliza la revisión. Una diferencia con más de 100 cambios se abre sin los botones por cambio, así que revísela como un archivo completo. La revisión por cambio requiere Claude Code v2.1.275 o posterior.

    Las mismas acciones están disponibles en el cursor desde el menú contextual del editor y desde la Paleta de comandos como **Claude Code: Accept Change at Cursor** y **Claude Code: Reject Change at Cursor**.
  </Step>
</Steps>

Para más ideas sobre lo que puede hacer con Claude Code, consulte [Flujos de trabajo comunes](/docs/es/common-workflows).

<Tip>
  Ejecute "Claude Code: Open Walkthrough" desde la Paleta de comandos para un tour guiado de los conceptos básicos.
</Tip>

<h2 id="use-the-prompt-box">
  Usar el cuadro de solicitud
</h2>

El cuadro de solicitud admite varias características:

* **Modos de permiso**: haga clic en el indicador de modo en la parte inferior del cuadro de solicitud para cambiar los modos de permiso. En los planes Pro, Max y Team, Auto es el modo de permiso inicial integrado. Consulte [cómo la extensión elige el modo de permiso inicial](/docs/es/permission-modes#switch-permission-modes) para saber qué cambia eso y todos los modos de permiso que ofrece el indicador.
  * **Auto**: un clasificador revisa la mayoría de las acciones en lugar de pedirle permiso. Consulte [modo auto](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) para saber qué revisa y bloquea.
  * **Manual**: Claude solicita permiso antes de ediciones de archivos y la mayoría de comandos de shell.
  * **Plan**: Claude describe lo que hará y espera aprobación antes de hacer cambios. VS Code abre automáticamente el plan como un documento Markdown completo donde puede agregar comentarios en línea para dar retroalimentación antes de que Claude comience.

    También puede escribir `/plan` en el cuadro de solicitud. Requiere Claude Code v2.1.280 o posterior.

    * `/plan`: cambia al modo plan. Si ya está en modo plan, muestra el plan actual en su lugar.
    * `/plan` con una tarea, como `/plan fix the auth bug`: cambia al modo plan e inicia la planificación de esa tarea.
    * `/plan open`: cuando ya está en modo plan, abre el archivo del plan en el editor.
  * **Edit automatically**: Claude realiza ediciones sin preguntar.
* **Model**: seleccione **Switch model…** desde el menú de comandos para cambiar el modelo durante la sesión. También puede hacer clic en el nombre del modelo en la parte inferior del cuadro de solicitud para abrir el mismo selector.

  Cuando el modelo actual admite [niveles de esfuerzo](/docs/es/model-config#adjust-effort-level), el selector también muestra una fila **Effort** y el botón del nombre del modelo muestra el nivel seleccionado. Cuando elige un nivel distinto de `max`, Claude Code lo guarda para el modelo actual como su predeterminado, bajo [`modelSettings`](/docs/es/settings-reference#modelsettings) en su configuración de usuario; `max` se aplica solo a la sesión actual. El botón del nombre del modelo y la fila **Effort** requieren Claude Code v2.1.257 o posterior.
* **Command menu**: haga clic en `/` o escriba `/` para abrir el menú de comandos. Las opciones incluyen adjuntar archivos, cambiar modelos y alternar el pensamiento extendido.

  La sección Customize proporciona acceso a servidores MCP, comandos, estilos de salida, hooks, memoria, instrucciones, permisos y plugins. Los elementos con un icono de terminal se abren en la terminal integrada.

  * Para examinar comandos como `/usage` o [`/remote-control`](/docs/es/remote-control), seleccione **Slash commands** en la sección Customize. Un diálogo los enumera con un cuadro de filtro. Elija uno para ejecutarlo. Escribir `/` en el cuadro de solicitud aún sugiere comandos en línea. Requiere Claude Code v2.1.257 o posterior.

    Escribir `/skills` también abre este diálogo. Cada fila de [skill](/docs/es/skills) muestra su [visibilidad](/docs/es/skills#override-skill-visibility-from-settings), como **On** u **Name only**. Haga clic en la visibilidad para cambiarla, excepto en filas marcadas como **locked**, como skills de plugins. El atajo `/skills` y los controles de visibilidad requieren Claude Code v2.1.280 o posterior.
  * Seleccione **Output styles** en la sección Customize para elegir un [estilo de salida](/docs/es/output-styles), incluidos sus estilos personalizados. Requiere Claude Code v2.1.257 o posterior.

    Para crear un estilo personalizado en su lugar, seleccione **Build a custom style** desde el menú **Output styles**. Claude Code escribe el [archivo de estilo](/docs/es/output-styles#create-a-custom-output-style) para usted a nivel de proyecto o usuario. Requiere Claude Code v2.1.261 o posterior.
  * Seleccione **Hooks** en la sección Customize para ver los [hooks](/docs/es/hooks) cargados en la sesión, agrupados por evento. Puede agregar, editar o eliminar hooks guardados en sus archivos de configuración de usuario, proyecto y local. Los hooks de otras fuentes, como configuración administrada o plugins, son de solo lectura. Requiere Claude Code v2.1.269 o posterior.
  * Seleccione **Permissions** en la sección Customize para ver las [reglas de permiso](/docs/es/permissions) de la sesión, agrupadas en Allow, Ask y Deny. Puede agregar reglas a su configuración de usuario, proyecto o local y eliminar reglas guardadas allí. Las reglas de otras fuentes, como configuración administrada o aprobaciones realizadas solo para esta sesión, son de solo lectura. Requiere Claude Code v2.1.269 o posterior.
  * Seleccione **Memory** en la sección Customize para activar o desactivar la [memoria automática](/docs/es/memory#auto-memory). Mientras esté activada, también puede examinar las memorias que Claude ha guardado y revelar las carpetas que las almacenan en su administrador de archivos. Requiere Claude Code v2.1.274 o posterior.

    Haga clic en una memoria guardada para leerla en el diálogo, donde puede editar el texto, eliminar la memoria o abrir su archivo en el editor. Ver, editar y eliminar una memoria en el diálogo requieren Claude Code v2.1.275 o posterior.
  * Seleccione **Instructions** en la sección Customize para editar los [archivos CLAUDE.md](/docs/es/memory#claude-md-files) que Claude lee. Elija un archivo para abrirlo en el editor. Si el archivo aún no existe, Claude Code lo crea primero. Requiere Claude Code v2.1.274 o posterior.
  * Seleccione **Status** en la sección Customize, o escriba `/status`, para verificar la versión de Claude Code de la sesión, cuenta, modelo y detalles del servidor MCP. Requiere Claude Code v2.1.280 o posterior.
  * Seleccione **Sandbox** en la sección Customize, o escriba `/sandbox`, para ver si los comandos Bash de Claude se ejecutan [sandboxed](/docs/es/sandboxing). Puede cambiar el modo sandbox y agregar [comandos excluidos](/docs/es/settings-reference#sandbox-excludedcommands) allí. Requiere Claude Code v2.1.280 o posterior.
  * Seleccione **Claude in Chrome** en la sección Customize, o escriba `/chrome`, para verificar y administrar la conexión de [Claude in Chrome](/docs/es/chrome). Ambos requieren iniciar sesión con una cuenta de claude.ai. Requiere Claude Code v2.1.280 o posterior.
  * Seleccione **Export conversation** en la sección Context, o escriba `/export`, para copiar la conversación como texto sin formato o guardarla en un archivo. Agregue un nombre de archivo, como `/export notes.txt`, para omitir el diálogo y elegir dónde guardar el archivo. Requiere Claude Code v2.1.280 o posterior.
  * La sección Settings incluye **Enable Remote Control for all sessions**, que establece [`remoteControlAtStartup`](/docs/es/settings-reference#remotecontrolatstartup) para controlar si [las nuevas sesiones interactivas se conectan a Remote Control automáticamente](/docs/es/remote-control#enable-remote-control-for-all-sessions). Requiere Claude Code v2.1.203 o posterior.

    Cuando activa o desactiva el conmutador en una ventana de VS Code, el cambio se aplica a las sesiones ya abiertas en esa ventana de VS Code, no solo a las sesiones que inicia después. Si lo desactiva, las sesiones abiertas se desconectan. Con Claude Code v2.1.261 o posterior, el cambio también llega a las sesiones abiertas en sus otras ventanas de VS Code.
  * La sección Settings también incluye **Focus view**, que oculta llamadas de herramientas, resultados de herramientas y pensamiento detrás de filas expandibles, dejando sus solicitudes y las respuestas de Claude. Actívelo allí, con `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux), o desde la Paleta de comandos con **Claude Code: Toggle Focus view**. El cambio se aplica a todas las sesiones abiertas y persiste entre sesiones. Requiere Claude Code v2.1.221 o posterior.

    La lista de tareas más reciente de Claude permanece visible, al igual que el texto de una pregunta pendiente de Claude; esto requiere Claude Code v2.1.225 o posterior. Mientras Claude ejecuta [subagentes](/docs/es/sub-agents), filas de progreso en vivo con su actividad más reciente aparecen bajo el grupo de llamadas de herramientas que las inició. Esto requiere Claude Code v2.1.269 o posterior.
  * Para cerrar sesión en su cuenta de Anthropic, seleccione **Sign out** en la sección Settings, o escriba `/logout`. En un [proveedor de terceros](#use-third-party-providers), el menú no ofrece ninguno de los dos. Requiere Claude Code v2.1.277 o posterior.
  * Para reportar un error, haga clic en **Report a problem** en la parte inferior del menú, o escriba `/bug` o `/feedback` con una descripción opcional que rellene previamente el informe. Cuando envía el informe y está conectado a Anthropic en una conexión de primera parte, Claude Code lo envía a Anthropic. En un proveedor de terceros, o sin credenciales de Anthropic, el diálogo aún se abre, pero enviar muestra un error y no envía nada: a diferencia de `/bug` de la CLI, la extensión no escribe un archivo local. Requiere Claude Code v2.1.229 o posterior.

    Si la política de su organización desactiva la retroalimentación del producto, **Report a problem** no aparece en el menú, y `/bug` y `/feedback` muestran un aviso `Feedback is turned off by your organization's policy or this environment's settings.` en lugar de abrir el informe.
* **Side questions**: escriba `/btw` seguido de una pregunta para preguntar sobre su sesión [sin agregar a la conversación](/docs/es/interactive-mode#side-questions-with-%2Fbtw). La respuesta se abre en un panel junto al chat, donde puede hacer preguntas de seguimiento. El hilo sobrevive a las recargas de ventana. Claude Code mantiene los 20 intercambios más nuevos y expira los hilos almacenados en el cronograma [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays), siempre que Claude Code pueda [determinar de forma segura el período de retención](/docs/es/claude-directory#cleaned-up-automatically). Para borrar un hilo, haga clic en el icono de papelera en el panel. Requiere Claude Code v2.1.227 o posterior.
* **Copy a response**: pase el cursor sobre una respuesta y haga clic en **Copy response** para copiarla al portapapeles, o escriba `/copy` para copiar la respuesta más reciente. `/copy 2` copia la segunda más reciente. Requiere Claude Code v2.1.277 o posterior.
* **Context indicator**: el cuadro de solicitud muestra cuánta ventana de contexto de Claude está utilizando. Claude se compacta automáticamente cuando es necesario, o puede ejecutar `/compact` manualmente.
* **Prompt cache clock**: un icono de reloj junto al indicador de contexto estima cuánto tiempo le queda a la [caché de solicitud](/docs/es/prompt-caching) de la conversación antes de que expire. Cuenta regresiva desde la [vida útil](/docs/es/prompt-caching#cache-lifetime) de cinco minutos o una hora de la caché, y cada respuesta que usa la caché reinicia la cuenta regresiva. Aparte de la compactación, las [acciones que invalidan la caché](/docs/es/prompt-caching#actions-that-invalidate-the-cache) no restablecen el reloj, por lo que aún puede mostrar minutos restantes después de cambiar modelos.
  * Hasta que se agote la cuenta regresiva, el icono muestra los minutos restantes, como **12m**.
  * Cuando se agota la cuenta regresiva, los minutos desaparecen y el icono se vuelve rojo, o el color de error de su tema, hasta la siguiente respuesta. Es probable que la caché haya expirado, así que espere una respuesta más lenta y costosa a su próximo mensaje mientras se reconstruye la caché. Si la vida útil de cinco minutos sigue agotándose entre sus mensajes, consulte [Elija el TTL usted mismo](/docs/es/prompt-caching#choose-the-ttl-yourself).
  * Justo después de que la conversación se [compacte](/docs/es/prompt-caching#compacting-the-conversation), el icono también se vuelve rojo sin minutos hasta la siguiente respuesta, porque la caché aún no cubre la conversación compactada.
* **Agent map**: cuando la conversación incluye [subagentes](/docs/es/sub-agents), un recuento de agentes como **2 agents** aparece en la parte inferior del cuadro de solicitud. Su punto muestra si algún subagente está trabajando o esperando su permiso.

  Haga clic en el recuento de agentes para abrir el mapa de agentes, que dibuja los subagentes de la conversación como un árbol bajo el agente principal, cada uno con su estado, tiempo transcurrido y recuento de tokens. Haga clic en un subagente para ver su solicitud y llamadas de herramientas, abrir su transcripción de solo lectura, o detenerlo mientras se ejecuta. Requiere Claude Code v2.1.269 o posterior.

  El mapa también enumera las otras [tareas en segundo plano](/docs/es/tools-reference#background-commands) de la sesión, como comandos de shell en segundo plano y [monitores](/docs/es/tools-reference#monitor-tool), debajo de los agentes. Haga clic en una fila para abrir la tarjeta de la tarea y detenerla allí.

  Para abrir el mapa cuando no se muestra un recuento de agentes, como cuando Claude ha iniciado un shell en segundo plano pero sin subagentes, escriba `/tasks` en el cuadro de solicitud. Las tareas en segundo plano en el mapa y el `/tasks` escrito requieren Claude Code v2.1.277 o posterior.
* **Extended thinking**: permite que Claude dedique más tiempo a razonar problemas complejos. Actívelo a través del menú de comandos (`/`). El razonamiento de Claude aparece en la conversación como bloques contraídos: haga clic en un bloque para leerlo, o presione `Ctrl+O` para expandir o contraer cada bloque de pensamiento en la sesión. Consulte [Extended thinking](/docs/es/model-config#extended-thinking) para obtener detalles.
* **Multi-line input**: presione `Shift+Enter` para agregar una nueva línea sin enviar. Esto también funciona en la entrada de texto libre "Other" de los diálogos de preguntas.

<h3 id="reference-files-and-folders">
  Archivos y carpetas de referencia
</h3>

Use menciones con @ para dar a Claude contexto sobre archivos o carpetas específicos. Cuando escribe `@` seguido de un nombre de archivo o carpeta, Claude lee ese contenido y puede responder preguntas sobre él o hacer cambios en él. Claude Code admite coincidencia difusa, por lo que puede escribir nombres parciales para encontrar lo que necesita:

```text wrap theme={null}
Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
What's in @src/components/ (include a trailing slash for folders)
```

Para archivos PDF grandes, puede pedirle a Claude que lea páginas específicas en lugar del archivo completo: una sola página, un rango como páginas 1-10, o un rango abierto como página 3 en adelante.

Cuando selecciona texto en el editor, Claude puede ver su código resaltado automáticamente. El pie de página del cuadro de solicitud muestra cuántas líneas están seleccionadas. Presione `Option+K` (Mac) / `Alt+K` (Windows/Linux) para insertar una mención con @ con la ruta del archivo y números de línea (por ejemplo, `@app.ts#5-10`). Haga clic en la **X** en el indicador de selección para eliminarlo para que Claude no reciba la selección. El indicador reaparece cuando selecciona otro texto.

La extensión retiene el texto seleccionado de algunos archivos. Cuando el archivo está dentro de su espacio de trabajo y coincide con su configuración `files.exclude` o `search.exclude`, Claude recibe como máximo la ruta del archivo y no el texto que seleccionó. Lo mismo se aplica a un archivo que git ignora, siempre que la configuración `search.useIgnoreFiles` de VS Code y la configuración [`respectGitIgnore`](#extension-settings) de la extensión estén ambas activadas, que es el predeterminado. Este filtro cubre solo el panel de chat: cuando Claude Code se ejecuta en la terminal integrada, la CLI envía su texto seleccionado sea cual sea el archivo, así que agregue una [regla de denegación `Read`](#the-built-in-ide-mcp-server) para evitar que Claude vea el contenido de un archivo allí.

Claude también ve qué archivo tiene abierto en el editor, incluso cuando nada está seleccionado, y el cuadro de solicitud muestra su nombre. Para agregar solo su texto seleccionado, desactive la [configuración Attach Open File](vscode://settings/claudeCode.attachOpenFile). La configuración requiere Claude Code v2.1.271 o posterior.

También puede adjuntar imágenes y archivos a su mensaje:

* Para adjuntar una imagen, péguela desde su portapapeles en el cuadro de solicitud.
* Para adjuntar archivos, mantenga presionado `Shift` mientras arrastra archivos al cuadro de solicitud.
* Para eliminar un adjunto del contexto, haga clic en la X en él.

<h3 id="paste-text">
  Pegar texto
</h3>

El texto que pega permanece visible en el cuadro de solicitud, en lugar de contraerse a un marcador de posición como lo hace [en la terminal](/docs/es/terminal-config#paste-large-content). En sesiones donde Claude Code [marca texto pegado](/docs/es/terminal-config#how-claude-treats-pasted-text), Claude aún ve un pegado grande como texto que pegó en lugar de escribir.

Claude Code también elimina [caracteres Unicode invisibles](/docs/es/interactive-mode#invisible-characters-in-prompts) del texto que pega en el cuadro de solicitud y de cualquier otra cosa que envíe:

* Si aparece un aviso como `Removed 3 invisible characters from the pasted text` cuando pega, el texto se insertó sin esos caracteres.
* Si aparece un aviso sobre caracteres eliminados cuando envía, nada fue enviado. El texto limpio está de vuelta en el cuadro de solicitud. Envíe de nuevo para enviar el texto como se muestra.

<h3 id="resume-past-conversations">
  Reanudar conversaciones pasadas
</h3>

Haga clic en el botón **Session history** en la parte superior del panel de Claude Code para acceder a su historial de conversaciones. Puede buscar por palabra clave o examinar por tiempo.

Haga clic en cualquier conversación para reanudarla con el historial de mensajes completo. Si la conversación ya está abierta en otra pestaña de la ventana actual, hacer clic en ella cambia a esa pestaña. Para obtener más información sobre cómo reanudar sesiones, consulte [Manage sessions](/docs/es/sessions).

* **Session titles**: las nuevas sesiones reciben títulos generados por IA basados en su primer mensaje.
* **Rename and archive**: pase el cursor sobre una sesión para revelar estas acciones. Cambie el nombre para darle un título descriptivo, o archive para moverla al grupo **Archived sessions** en la parte inferior de la lista.

De forma predeterminada, una sesión sin actividad durante 14 días se mueve a **Archived sessions** automáticamente, a menos que esté abierta, no leída o en un [grupo](#organize-sessions-into-groups). El archivado automático requiere Claude Code v2.1.265 o posterior. Para cambiar el período o desactivarlo, abra la [configuración Archive Inactive Sessions](vscode://settings/claudeCode.archiveInactiveSessions) y seleccione un número de días o **Never**.

Para restaurar una sesión archivada, expanda **Archived sessions** y haga clic en **Unarchive session**. Para restaurar todas las sesiones archivadas a la vez, pase el cursor sobre el encabezado **Archived sessions** en la lista de sesiones en la Barra de actividades y haga clic en su icono de desarchivado, lo que requiere Claude Code v2.1.277 o posterior. Antes de v2.1.257, la acción era **Delete session**, que ocultaba una sesión sin forma de restaurarla. Las sesiones que eliminó aparecen bajo **Archived sessions** después de actualizar.

Cuando la conversación que reanuda terminó en modo plan, Claude Code restaura el modo plan. Requiere Claude Code v2.1.246 o posterior. Claude Code no lo restaura en dos casos:

* La extensión [elige el modo de permiso inicial](/docs/es/permission-modes#switch-permission-modes) de `claudeCode.initialPermissionMode` o una selección que se transfiere de una conversación anterior
* Tiene `claudeCode.claudeProcessWrapper` configurado

<h3 id="resume-cloud-sessions-from-claude-ai">
  Reanudar sesiones en la nube desde Claude.ai
</h3>

Si ejecuta [sesiones en la nube](/docs/es/claude-code-on-the-web), puede reanudarlas directamente en VS Code. Esto requiere iniciar sesión con **Claude.ai Subscription**, no Anthropic Console.

<Steps>
  <Step title="Open session history">
    Haga clic en el botón **Session history** en la parte superior del panel de Claude Code.
  </Step>

  <Step title="Select the Web tab">
    El diálogo muestra dos pestañas: Local y Web. Haga clic en **Web** para ver sesiones de claude.ai.
  </Step>

  <Step title="Select a session to resume">
    Examine o busque sus sesiones en la nube. Haga clic en cualquier sesión para descargarla y continuar la conversación localmente.
  </Step>
</Steps>

<Note>
  Solo las sesiones en la nube iniciadas con un repositorio de GitHub aparecen en la pestaña Web. Reanudar carga el historial de conversaciones localmente; los cambios no se sincronizan de vuelta a claude.ai.
</Note>

<h3 id="check-account-and-usage">
  Verificar cuenta y uso
</h3>

Ejecute `/usage` para abrir el diálogo Account & usage. Muestra su cuenta conectada, y el uso que reporta difiere según el inicio de sesión:

* **claude.ai plan**: barras de uso para los límites de su plan, como la sesión actual y la semana. Cada barra muestra cuánto tiempo falta para que se restablezca su límite.

  El diálogo también desglosa qué está contribuyendo a los límites de su plan. Marca comportamientos que representan el 10% o más del uso reciente, como fallos de caché, contexto largo, sesiones pesadas en subagentes o altamente paralelas, cada una con un consejo para reducirlo. Las tablas de atribución muestran cuánto uso provino de cada skill, subagente, plugin y servidor MCP.

  Use el conmutador Day y Week para cambiar entre las últimas 24 horas y los últimos 7 días. Las cifras son aproximadas y se calculan a partir de sesiones locales en esta máquina, por lo que el uso de otros dispositivos o claude.ai no se incluye.
* **Other sign-ins**: cuando los límites de plan no se aplican a su inicio de sesión, como en un [proveedor de terceros](#use-third-party-providers) o con una clave API, la sección Usage muestra el costo de la sesión y el uso de tokens en su lugar. El `/usage` de la CLI muestra los mismos totales en su [bloque Session](/docs/es/costs#track-your-costs). La lista de sesiones en la Barra de actividades también muestra los totales de la sesión activa bajo su encabezado **Account & usage**. Requiere Claude Code v2.1.277 o posterior.

Para obtener más información sobre cómo rastrear y reducir el uso, consulte [Track your costs](/docs/es/costs#track-your-costs).

<h2 id="customize-your-workflow">
  Personaliza tu flujo de trabajo
</h2>

Puedes reposicionar el panel de Claude, ejecutar múltiples conversaciones, organizar la lista de sesiones en grupos o cambiar al modo terminal.

<h3 id="choose-where-claude-lives">
  Elige dónde vive Claude
</h3>

Puedes arrastrar el panel de Claude para reposicionarlo en cualquier lugar de VS Code. Agarra la pestaña o la barra de título del panel y arrástralo a:

* **Barra lateral secundaria**: el lado derecho de la ventana. Mantiene Claude visible mientras codificas.
* **Barra lateral principal**: la barra lateral izquierda con iconos para Explorer, Search, etc.
* **Área del editor**: abre Claude como una pestaña junto a tus archivos. Útil para tareas secundarias.

Cuando Claude abre una pestaña en un nuevo grupo de editor, la extensión bloquea ese grupo, por lo que los archivos que abres mientras la pestaña de Claude está enfocada van a otro grupo en lugar de junto a ella.

Para evitar que la extensión bloquee grupos, desactiva la [configuración Lock Editor Groups](vscode://settings/claudeCode.lockEditorGroups). Los grupos que ya están bloqueados permanecen bloqueados hasta que los desbloquees. La configuración requiere Claude Code v2.1.274 o posterior.

<Tip>
  Usa la barra lateral para tu sesión principal de Claude y abre pestañas adicionales para tareas secundarias. Claude recuerda tu ubicación preferida. El icono de la lista de sesiones de la Activity Bar es independiente del panel de Claude: la lista de sesiones siempre es visible en la Activity Bar, mientras que el icono del panel de Claude solo aparece allí cuando el panel está acoplado a la barra lateral izquierda.
</Tip>

Después de ejecutar **Developer: Reload Window** o reiniciar VS Code, si una conversación vuelve con su historial depende de dónde estuviera abierta:

* **Pestaña del editor**: la conversación vuelve con su pestaña.
* **Barra lateral**: la conversación vuelve si enviaste un mensaje o Claude respondió en ella en los últimos 10 minutos. Si no vuelve, reanuda la conversación desde [Historial de sesiones](#resume-past-conversations).

Si la recarga interrumpió a Claude a mitad de un paso, Claude continúa ese paso cuando la conversación vuelve, y un aviso en el chat marca la continuación. Requiere Claude Code v2.1.274 o posterior. Si el paso fue interrumpido hace más de una hora o la sesión está abierta en otro lugar, la conversación vuelve inactiva en su lugar.

Para desactivar la continuación, abre la [configuración Continue After Reload](vscode://settings/claudeCode.continueAfterReload) y desactívala.

<h3 id="run-multiple-conversations">
  Ejecuta múltiples conversaciones
</h3>

Usa **Open in New Tab** u **Open in New Window** desde la Command Palette para iniciar conversaciones adicionales. Cada conversación mantiene su propio historial y contexto, permitiéndote trabajar en diferentes tareas en paralelo.

Cuando uses pestañas, un pequeño punto de color en el icono de spark indica el estado: azul significa que hay una solicitud de permiso pendiente, naranja significa que Claude terminó mientras la pestaña estaba oculta.

<h3 id="organize-sessions-into-groups">
  Organiza sesiones en grupos
</h3>

En la lista de sesiones de la Activity Bar, puedes recopilar sesiones relacionadas en grupos nombrados y contraíbles. Requiere Claude Code v2.1.229 o posterior.

* **Agrupar o desagrupar una sesión**: haz clic derecho en una sesión para crear un grupo a partir de ella, moverla a un grupo existente o eliminarla de su grupo. Cada sesión pertenece a un grupo a la vez, por lo que moverla a otro grupo la elimina del primero.
* **Mover varias sesiones a la vez**: `Cmd`-clic (Mac) / `Ctrl`-clic (Windows/Linux) en cada sesión, o `Shift`-clic para seleccionar un rango, luego haz clic derecho en la selección.
* **Agrupar una sesión desde su pestaña**: ejecuta **Claude Code: Add Session Tab to Group** desde la Command Palette, luego elige o crea un grupo. Requiere Claude Code v2.1.257 o posterior.
* **Renombrar o eliminar un grupo**: haz clic derecho en un encabezado de grupo. Eliminar un grupo solo elimina el grupo, y sus sesiones vuelven a la lista sin agrupar.

La extensión guarda grupos por carpeta de espacio de trabajo, por lo que sobreviven a recargas de ventana y aparecen en cada ventana donde abres la misma carpeta. Cuando buscas en la lista, la extensión muestra coincidencias en una lista plana única en todos los grupos.

<h3 id="switch-to-terminal-mode">
  Cambiar al modo terminal
</h3>

De forma predeterminada, la extensión abre un panel de chat gráfico. Si prefieres la interfaz de estilo CLI, abre la [configuración Use Terminal](vscode://settings/claudeCode.useTerminal) y marca la casilla.

También puedes abrir la configuración de VS Code (`Cmd+,` en Mac o `Ctrl+,` en Windows/Linux), ve a Extensions → Claude Code y marca **Use Terminal**.

<h2 id="manage-plugins">
  Gestionar plugins
</h2>

La extensión de VS Code incluye una interfaz gráfica para instalar y gestionar [plugins](/docs/es/plugins/overview). Escriba `/plugins` en el cuadro de solicitud para abrir la interfaz **Gestionar plugins**.

<h3 id="install-plugins">
  Instalar plugins
</h3>

El diálogo de plugins muestra dos pestañas: **Plugins** y **Marketplaces**.

En la pestaña Plugins:

* Los **plugins instalados** aparecen en la parte superior con interruptores de alternancia para habilitarlos o deshabilitarlos
* Los **plugins disponibles** de sus marketplaces configurados aparecen a continuación
* Busque para filtrar plugins por nombre o descripción
* Haga clic en **Instalar** en cualquier plugin disponible

Cuando instale un plugin, elija el alcance de la instalación:

* **Instalar para usted**: disponible en todos sus proyectos (alcance de usuario)
* **Instalar para este proyecto**: compartido con colaboradores del proyecto (alcance de proyecto)
* **Instalar localmente**: solo para usted, solo en este repositorio (alcance local)

<h3 id="share-a-plugin-install-link">
  Compartir un enlace de instalación de plugin
</h3>

Para enviar a alguien directamente a la instalación de un plugin específico, proporciónele la URL `install-plugin` de la extensión. Al abrirla, se inicia o enfoca VS Code, abre el panel Claude Code y abre el diálogo **Gestionar plugins** en la opción de alcance de ese plugin. Nada se instala hasta que la persona elige un alcance. Si el marketplace del plugin aún no está configurado en su Claude Code, el diálogo primero le pide que lo agregue.

```text theme={null}
vscode://anthropic.claude-code/install-plugin?plugin=code-review&marketplace=anthropics/claude-plugins-official
```

La URL toma dos parámetros de consulta:

| Parámetro     | Descripción                                                                                                                                                                                                 |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugin`      | El nombre del plugin tal como lo enumera su marketplace. Requerido.                                                                                                                                         |
| `marketplace` | De dónde proviene el plugin: un `owner/repo` de GitHub, una URL `https://`, o una URL de git SSH como `git@github.com:owner/repo.git`. Por defecto es `anthropics/claude-plugins-official` cuando se omite. |

Algunos valores que la [pestaña Marketplaces](#manage-marketplaces) acepta no funcionan en un enlace, como una ruta local o una dirección `http://`. Para esos, VS Code muestra un mensaje de error y el diálogo no se abre.

Dos casos terminan en un mensaje en el diálogo en lugar de la opción de alcance:

* **El marketplace no enumera un plugin con ese nombre**: el diálogo informa que el plugin no fue encontrado. Verifique el valor de `plugin` contra el listado del marketplace.
* **El plugin ya está instalado**: el diálogo lo indica, y nada cambia.

Los README de GitHub, problemas y algunos otros hosts de Markdown eliminan enlaces cuyo esquema no es `http` o `https`, por lo que un enlace `vscode://` allí se representa como texto sin formato. Coloque la URL en un bloque de código en esos hosts, como [El enlace se representa como texto sin formato en lugar de ser clickeable](/docs/es/deep-links#the-link-renders-as-plain-text-instead-of-being-clickable) describe para enlaces `claude-cli://`.

<h3 id="manage-marketplaces">
  Gestionar marketplaces
</h3>

Cambie a la pestaña **Marketplaces** para agregar o eliminar fuentes de plugins:

* Ingrese un repositorio de GitHub, URL o ruta local para agregar un nuevo marketplace
* Haga clic en el icono de actualización para actualizar la lista de plugins de un marketplace
* Haga clic en el icono de papelera para eliminar un marketplace

Los cambios de plugins que realiza en el diálogo se aplican inmediatamente a las sesiones de Claude Code abiertas en esa ventana de VS Code. Si la sesión desde la que abrió el diálogo no puede recargar sus plugins, el diálogo le ofrece intentar de nuevo o reiniciar Claude en esa sesión.

<Note>
  La gestión de plugins en VS Code utiliza los mismos comandos CLI bajo el capó. Los plugins y marketplaces que configure en la extensión también están disponibles en la CLI, y viceversa.
</Note>

Para obtener más información sobre el sistema de plugins, consulte [Plugins](/docs/es/plugins/overview) y [Plugin marketplaces](/docs/es/plugins/overview).

<h2 id="automate-browser-tasks-with-chrome">
  Automatizar tareas del navegador con Chrome
</h2>

Conecte Claude a su navegador Chrome para probar aplicaciones web, depurar con registros de consola y automatizar flujos de trabajo del navegador sin salir de VS Code. Esto requiere la [extensión Claude in Chrome](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) versión 1.0.36 o superior.

Escriba `@browser` en el cuadro de solicitud seguido de lo que desea que Claude haga:

```text wrap theme={null}
@browser go to localhost:3000 and check the console for errors
```

También puede abrir el menú de archivos adjuntos para seleccionar herramientas específicas del navegador, como abrir una nueva pestaña o leer el contenido de la página.

Claude abre nuevas pestañas para tareas del navegador y comparte el estado de inicio de sesión de su navegador, por lo que puede acceder a cualquier sitio en el que ya haya iniciado sesión.

Para obtener instrucciones de configuración, la lista completa de capacidades y solución de problemas, consulte [Usar Claude Code con Chrome](/docs/es/chrome).

<h2 id="vs-code-commands-and-shortcuts">
  Comandos y atajos de teclado de VS Code
</h2>

Abra la Paleta de Comandos (`Cmd+Shift+P` en Mac o `Ctrl+Shift+P` en Windows/Linux) y escriba "Claude Code" para ver todos los comandos disponibles de VS Code para la extensión Claude Code.

Algunos atajos de teclado dependen de qué panel esté "enfocado" (recibiendo entrada de teclado). Cuando su cursor está en un archivo de código, el editor está enfocado. Cuando su cursor está en el cuadro de solicitud de Claude, Claude está enfocado. Use `Cmd+Esc` / `Ctrl+Esc` para alternar entre ellos.

<Note>
  Estos son comandos de VS Code para controlar la extensión. No todos los comandos integrados de Claude Code están disponibles en la extensión. Consulte [Extensión de VS Code frente a CLI de Claude Code](#vs-code-extension-vs-claude-code-cli) para obtener más detalles.
</Note>

| Comando                    | Atajo de teclado                                         | Descripción                                                                                                                                                                                                                                                                              |
| -------------------------- | -------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focus Input                | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux)             | Alternar el enfoque entre el editor y Claude                                                                                                                                                                                                                                             |
| Focus last message         | -                                                        | Mover el enfoque del teclado al mensaje más reciente en la conversación, o a un aviso de permiso pendiente, para que pueda leer desde allí con el teclado o un lector de pantalla. No disponible en [modo terminal](#switch-to-terminal-mode). Requiere Claude Code v2.1.268 o posterior |
| Open in Side Bar           | -                                                        | Abrir Claude en la barra lateral                                                                                                                                                                                                                                                         |
| Open in Terminal           | -                                                        | Abrir Claude en modo terminal                                                                                                                                                                                                                                                            |
| Open in New Tab            | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | Abrir una nueva conversación como una pestaña del editor                                                                                                                                                                                                                                 |
| Open in New Window         | -                                                        | Abrir una nueva conversación en una ventana separada                                                                                                                                                                                                                                     |
| New Conversation           | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | Iniciar una nueva conversación. Requiere que Claude esté enfocado y `enableNewConversationShortcut` establecido en `true`                                                                                                                                                                |
| Reopen Closed Session      | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux)     | Reabrir la pestaña de sesión de Claude cerrada más recientemente. Se remite a la reapertura normal de editor cerrado de VS Code cuando la última pestaña cerrada no era una sesión de Claude. Deshabilitar con `enableReopenClosedSessionShortcut`                                       |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | Insertar una referencia al archivo actual y la selección (requiere que el editor esté enfocado)                                                                                                                                                                                          |
| Accept Change at Cursor    | -                                                        | Aceptar el cambio en el cursor mientras [revisa una edición propuesta](#get-started) un cambio a la vez. Requiere Claude Code v2.1.275 o posterior                                                                                                                                       |
| Reject Change at Cursor    | -                                                        | Revertir el cambio en el cursor mientras revisa una edición propuesta un cambio a la vez. Requiere Claude Code v2.1.275 o posterior                                                                                                                                                      |
| Toggle Focus view          | `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux)     | Ocultar o mostrar la actividad de herramientas en la conversación. Funciona mientras un panel de Claude o la barra lateral sea visible. Requiere Claude Code v2.1.221 o posterior                                                                                                        |
| Rename Session Tab         | -                                                        | Cambiar el nombre de la sesión en la pestaña activa de Claude. Requiere Claude Code v2.1.257 o posterior                                                                                                                                                                                 |
| Add Session Tab to Group   | -                                                        | Agregar la sesión en la pestaña activa de Claude a un [grupo de sesiones](#organize-sessions-into-groups) que elija o cree. Requiere Claude Code v2.1.257 o posterior                                                                                                                    |
| Mark Session as Unread     | -                                                        | Marcar la sesión en la pestaña activa de Claude como no leída en la lista de sesiones. Requiere Claude Code v2.1.257 o posterior                                                                                                                                                         |
| Show Logs                  | -                                                        | Ver registros de depuración de la extensión                                                                                                                                                                                                                                              |
| Logout                     | -                                                        | Cerrar sesión en su cuenta de Anthropic                                                                                                                                                                                                                                                  |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  Iniciar una pestaña de VS Code desde otras herramientas
</h3>

La extensión registra un controlador de URI en `vscode://anthropic.claude-code/open`. Úselo para abrir una nueva pestaña de Claude Code desde sus propias herramientas: un alias de shell, un marcador de navegador o cualquier script que pueda abrir una URL. Si VS Code no se está ejecutando, abrir la URL lo inicia primero. Si VS Code ya se está ejecutando, la URL se abre en la ventana que actualmente tiene el enfoque.

Invoque el controlador con el abridor de URL de su sistema operativo.

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Linux">
    ```bash theme={null}
    xdg-open "vscode://anthropic.claude-code/open"
    ```

    El comando `xdg-open` viene del paquete `xdg-utils`. Si el shell informa que no se encuentra, consulte [xdg-open is not found on Linux](/docs/es/deep-links#xdg-open-is-not-found-on-linux).
  </Tab>

  <Tab title="Windows">
    En PowerShell:

    ```powershell theme={null}
    Start-Process "vscode://anthropic.claude-code/open"
    ```

    En `cmd.exe`, `start` trata su primer argumento entrecomillado como un título de ventana, así que pase un título vacío antes de la URL:

    ```cmd theme={null}
    start "" "vscode://anthropic.claude-code/open"
    ```
  </Tab>
</Tabs>

El controlador acepta dos parámetros de consulta opcionales:

| Parámetro | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                |
| --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | Texto para rellenar previamente en el cuadro de solicitud. Debe estar codificado en URL. La solicitud se rellena previamente pero no se envía automáticamente.                                                                                                                                                                                                                                                             |
| `session` | Un ID de sesión para reanudar en lugar de iniciar una nueva conversación. La sesión debe pertenecer al espacio de trabajo actualmente abierto en VS Code. Si la sesión no se encuentra, se inicia una conversación nueva. Si la sesión ya está abierta en una pestaña, esa pestaña se enfoca. Para capturar un ID de sesión mediante programación, consulte [Continue conversations](/docs/es/headless#continue-conversations). |

Por ejemplo, para abrir una pestaña rellenada previamente con "review my changes":

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

La extensión también maneja `vscode://anthropic.claude-code/install-plugin`, que [abre el diálogo de plugins en un plugin](#share-a-plugin-install-link). Para iniciar una sesión de terminal en lugar de una pestaña de VS Code, use el controlador `claude-cli://` de la CLI. Consulte [Launch sessions from links](/docs/es/deep-links).

<h2 id="configure-settings">
  Configurar ajustes
</h2>

La extensión tiene dos tipos de ajustes:

* **Ajustes de extensión** en VS Code: controlan el comportamiento de la extensión dentro de VS Code. Abra con `Cmd+,` (Mac) o `Ctrl+,` (Windows/Linux), luego vaya a Extensiones → Claude Code. También puede escribir `/` y seleccionar **General config…** para abrir los ajustes.
* **Ajustes de Claude Code** en `~/.claude/settings.json`: compartidos entre la extensión y CLI. Úselo para comandos permitidos, variables de entorno, hooks y servidores MCP. En los planes Pro, Max y Team, también es una entrada al modo de permisos en el que comienzan las conversaciones. [Switch permission modes](/docs/es/permission-modes#switch-permission-modes) enumera el orden. Consulte [Settings](/docs/es/settings) para obtener detalles.

<Tip>
  Agregue `"$schema": "https://json.schemastore.org/claude-code-settings.json"` a su `settings.json` para obtener autocompletado y validación en línea para todos los ajustes disponibles directamente en VS Code.
</Tip>

<h3 id="extension-settings">
  Ajustes de extensión
</h3>

VS Code lee `initialPermissionMode` de sus ajustes de usuario e ignora los valores del espacio de trabajo. Antes de v2.1.225, VS Code establecía por defecto el ajuste en `default` y aplicaba los valores del espacio de trabajo.

| Ajuste                              | Predeterminado | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ----------------------------------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useTerminal`                       | `false`        | Inicie Claude en modo terminal en lugar de panel gráfico                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `initialPermissionMode`             | -              | Controla los mensajes de aprobación para nuevas conversaciones: `default`, `plan`, `acceptEdits` o `bypassPermissions`. `manual` es un alias para `default` y selecciona el modo etiquetado como **Manual** en el indicador de modo. Cuando lo deja sin establecer, la extensión elige el modo de permisos inicial como se describe en [Switch permission modes](/docs/es/permission-modes#switch-permission-modes).                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `preferredLocation`                 | `panel`        | Dónde se abre Claude: `sidebar` (derecha) o `panel` (nueva pestaña)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `lockEditorGroups`                  | `true`         | [Bloquear los grupos de editor que Claude inicia para sus pestañas](#choose-where-claude-lives), de modo que los archivos que abre mientras una pestaña de Claude está enfocada vayan a otro grupo. Cuando está desactivado, la extensión nunca bloquea un grupo de editor. Requiere Claude Code v2.1.274 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `autosave`                          | `true`         | Guardar automáticamente archivos antes de que Claude los lea o escriba                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `attachOpenFile`                    | `true`         | Agregue el archivo que está abierto en el editor a sus mensajes y muéstrelo en el cuadro de mensaje. Cuando está desactivado, solo se agrega el texto seleccionado. Requiere Claude Code v2.1.271 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `useCtrlEnterToSend`                | `false`        | Usar Ctrl/Cmd+Enter en lugar de Enter para enviar mensajes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `scrollToBottomOnSend`              | `true`         | Desplazarse a la conversación hacia el final cuando envía un mensaje. Cuando está desactivado, la conversación permanece donde la dejó. Requiere Claude Code v2.1.275 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `enableNewConversationShortcut`     | `false`        | Habilitar Cmd/Ctrl+N para iniciar una nueva conversación                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `enableReopenClosedSessionShortcut` | `true`         | Usar Cmd/Ctrl+Shift+T para reabrir la pestaña de sesión de Claude cerrada más recientemente. Cuando la última pestaña cerrada no era una sesión de Claude, el atajo ejecuta el comando normal de reapertura de editor cerrado de VS Code en su lugar.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `archiveInactiveSessions`           | `14`           | [Archivar una sesión automáticamente](#resume-past-conversations) después de estos muchos días sin actividad: `1`, `2`, `7` o `14`. Establezca `0` para desactivarlo. Requiere Claude Code v2.1.265 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `continueAfterReload`               | `true`         | Después de una recarga de ventana, Claude [continúa el paso que fue interrumpido](#choose-where-claude-lives) en la sesión restaurada. Requiere Claude Code v2.1.274 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `hideOnboarding`                    | `false`        | Ocultar la lista de verificación de incorporación (icono de gorro de graduación)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `focusView`                         | `false`        | Ocultar llamadas de herramientas, resultados de herramientas y pensamiento detrás de filas expandibles, dejando sus mensajes y las respuestas de Claude. La lista de tareas más reciente de Claude permanece visible; esto requiere Claude Code v2.1.225 o posterior. También puede alternar la vista de enfoque desde el menú de comandos. Requiere Claude Code v2.1.221 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `respectGitIgnore`                  | `true`         | Excluir patrones de .gitignore de búsquedas de archivos y de [contexto de selección](#reference-files-and-folders)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `usePythonEnvironment`              | `true`         | Activar el entorno de Python del espacio de trabajo al ejecutar Claude. Requiere la extensión de Python.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `environmentVariables`              | `[]`           | Establecer variables de entorno para el proceso de Claude. Use los ajustes de Claude Code en su lugar para configuración compartida.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `disableLoginPrompt`                | `false`        | Omitir mensajes de autenticación (para configuraciones de proveedores de terceros)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `allowDangerouslySkipPermissions`   | `false`        | Agrega Bypass permissions al selector de modo. Úselo solo en espacios aislados sin acceso a Internet.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `claudeProcessWrapper`              | -              | Ejecutable utilizado para iniciar el proceso de Claude. La ruta del binario incluido se pasa como argumento cuando está presente. Establezca esto en un binario `claude` instalado por separado si la compilación de la extensión no incluye uno para su plataforma. En una configuración envuelta, las conversaciones comienzan en modo Manual a menos que establezca `initialPermissionMode` o haya seleccionado Manual, Editar automáticamente o Auto en una conversación anterior, porque la extensión omite los ajustes y los pasos predeterminados integrados allí; consulte [Switch permission modes](/docs/es/permission-modes#switch-permission-modes). Un error "Unsupported platform" en la activación significa que no hay binario incluido para su plataforma; consulte [which platforms have prebuilt binaries](/docs/es/troubleshoot-install#native-binary-not-found-after-npm-install). |

<h2 id="use-a-screen-reader">
  Usar un lector de pantalla
</h2>

El panel de chat de la extensión funciona con lectores de pantalla. No necesita activar nada: la extensión anuncia la actividad de la conversación para cada usuario, sin cambios visuales. Esto es independiente del [modo de lector de pantalla](/docs/es/accessibility) de la CLI, que adapta la interfaz del terminal.

La compatibilidad con lectores de pantalla en el panel de chat requiere Claude Code v2.1.236 o posterior.

Durante una conversación, la extensión anuncia:

* **Respuestas de Claude**: la extensión anuncia cada respuesta una sola vez, cuando está completa, y permanece en silencio mientras el texto se transmite. Su lector de pantalla lee los bloques de código como un resumen de recuento de líneas, lee los enlaces por su etiqueta y lee las tablas celda por celda; la respuesta completa permanece legible en la transcripción.
* **Solicitudes de permiso y preguntas**: la extensión anuncia una solicitud cuando aparece su solicitud de permiso, nombrando la herramienta que Claude desea utilizar. Anuncia de la misma manera cuando Claude le hace una pregunta y cuando Claude termina un plan y espera su revisión.
* **Cambios de estado**: la extensión anuncia cuando Claude comienza a trabajar, cuando Claude está listo para su entrada y cuando Claude Code comienza a compactar la conversación.
* **Errores y solicitudes de modelo**: la extensión anuncia errores en la conversación y anuncia cuando aparece la [solicitud de consentimiento de créditos de uso](/docs/es/model-config#fable-and-usage-credits) o la [solicitud de solicitud marcada](/docs/es/model-config#ask-before-switching).

Mientras Claude trabaja, su lector de pantalla lee una etiqueta de texto en lugar de la animación del indicador de progreso.

Cuando reabre una sesión o cambia a otra, la extensión no anuncia nada: el historial restaurado, las solicitudes de permiso pendientes y el estado en progreso permanecen en silencio hasta que suceda algo nuevo.

<h3 id="use-the-chat-panel-from-the-keyboard">
  Usar el panel de chat desde el teclado
</h3>

Cada turno en la transcripción comienza con un encabezado visualmente oculto etiquetado con la solicitud que inició el turno, para que pueda saltar entre turnos con la navegación de encabezados de su lector de pantalla.

Dentro de un turno, su lector de pantalla anuncia de quién es el mensaje en el que se encuentra mientras se mueve a través de él:

* **Sus mensajes**: "Usted"
* **Mensajes de Claude**: "Claude"
* **Pasos de herramientas**: "Claude" más el nombre de la herramienta, como "Claude, Bash"
* **Bloques de pensamiento**: "Claude, pensando"

Debido a que la extensión expone la transcripción como una región etiquetada, también puede mover el foco a la transcripción misma con `Tab` y leerla a su propio ritmo. Para mover el foco al mensaje más reciente o a una solicitud de permiso en espera, ejecute **Claude Code: Focus last message** desde la [Paleta de comandos](#vs-code-commands-and-shortcuts).

Cuando una opción en una solicitud de permiso guarda una regla de permiso o acceso a directorio, su etiqueta termina nombrando dónde se guarda la aprobación, como "todos los proyectos" o "esta sesión". Con esa opción enfocada, presione la tecla de flecha `Izquierda` o `Derecha` para cambiar el destino, y la extensión anuncia cada destino cuando llega a él. También puede hacer clic en el destino en la etiqueta. Las teclas de flecha requieren Claude Code v2.1.268 o posterior.

<h2 id="vs-code-extension-vs-claude-code-cli">
  Extensión de VS Code vs. Claude Code CLI
</h2>

Claude Code está disponible tanto como una extensión de VS Code (panel gráfico) como una CLI (interfaz de línea de comandos en la terminal). Algunas características solo están disponibles en la CLI. Si necesita una característica solo de CLI, ejecute `claude` en la terminal integrada de VS Code. Esto requiere la [instalación independiente de CLI](/docs/es/setup): la extensión no agrega `claude` a su PATH. Consulte [Ejecutar CLI en VS Code](#run-cli-in-vs-code).

| Característica                 | CLI                   | Extensión de VS Code                                                                                       |
| ------------------------------ | --------------------- | ---------------------------------------------------------------------------------------------------------- |
| Comandos y skills              | [Todos](/docs/es/commands) | Subconjunto (escriba `/` para ver los disponibles)                                                         |
| Configuración del servidor MCP | Sí                    | Sí ([agregue y administre servidores](#connect-to-external-tools-with-mcp) con `/mcp` en el panel de chat) |
| Checkpoints                    | Sí                    | Sí                                                                                                         |
| Atajo bash `!`                 | Sí                    | No                                                                                                         |
| Finalización de pestañas       | Sí                    | No                                                                                                         |

<h3 id="rewind-with-checkpoints">
  Rewind con checkpoints
</h3>

La extensión de VS Code admite checkpoints, que rastrean las ediciones de archivos de Claude y le permiten revertir a un estado anterior. Pase el cursor sobre cualquier mensaje para revelar el botón de rewind, luego elija entre tres opciones:

* **Fork conversation from here**: inicie una nueva rama de conversación desde este mensaje mientras mantiene todos los cambios de código intactos
* **Rewind code to here**: revierta los cambios de archivo a este punto en la conversación mientras mantiene el historial de conversación completo
* **Fork conversation and rewind code**: inicie una nueva rama de conversación y revierta los cambios de archivo a este punto

Para obtener detalles completos sobre cómo funcionan los checkpoints y sus limitaciones, consulte [Checkpointing](/docs/es/checkpointing).

<h3 id="run-cli-in-vs-code">
  Ejecutar CLI en VS Code
</h3>

Para usar la CLI mientras permanece en VS Code, abra la terminal integrada (`` Ctrl+` `` en Windows/Linux o `` Cmd+` `` en Mac) y ejecute `claude`. La CLI se integra automáticamente con su IDE para características como visualización de diferencias y uso compartido de diagnósticos.

Instalar la extensión no coloca `claude` en su PATH de shell. La extensión incluye una copia privada de la CLI para su panel de chat, pero escribir `claude` en una terminal requiere la [instalación independiente de CLI](/docs/es/setup). Ejecute la instalación una vez y los comandos en esta página, incluidos `claude mcp add` y `claude --resume`, funcionan en cualquier terminal. Si `claude` aún no se encuentra después de instalar, [verifique su PATH](/docs/es/troubleshoot-install#verify-your-path).

Si utiliza una terminal externa, ejecute `/ide` dentro de Claude Code para conectarla a VS Code.

<h3 id="switch-between-extension-and-cli">
  Cambiar entre extensión y CLI
</h3>

La extensión y la CLI comparten el mismo historial de conversación. Para continuar una conversación de extensión en la CLI, ejecute `claude --resume` en la terminal. Esto abre un selector interactivo donde puede buscar y seleccionar su conversación.

<h3 id="include-terminal-output-in-prompts">
  Incluir salida de terminal en indicaciones
</h3>

Haga referencia a la salida de terminal en sus indicaciones usando `@terminal:name` donde `name` es el título de la terminal. Esto permite que Claude vea la salida de comandos, mensajes de error o registros sin copiar y pegar.

<h3 id="monitor-background-processes">
  Monitorear procesos en segundo plano
</h3>

Escriba `/tasks` en el cuadro de indicaciones para abrir el [mapa de agentes](#use-the-prompt-box), que enumera las tareas en segundo plano de la sesión, como un servidor de desarrollo que Claude dejó ejecutándose como un comando de shell en segundo plano. Haga clic en una tarea para abrir su tarjeta y detenerla allí. Requiere Claude Code v2.1.277 o posterior.

<h3 id="connect-to-external-tools-with-mcp">
  Conectar a herramientas externas con MCP
</h3>

MCP (Model Context Protocol) los servidores dan a Claude acceso a herramientas externas, bases de datos y API.

Para administrar servidores MCP sin salir de VS Code, escriba `/mcp` en el panel de chat. En el diálogo que se abre, puede agregar servidores, eliminar servidores guardados en el [ámbito](/docs/es/mcp#mcp-installation-scopes) local, de usuario o de proyecto, habilitar o deshabilitar servidores, reconectarse a un servidor y administrar la autenticación OAuth. Agregar y eliminar servidores en el diálogo requiere Claude Code v2.1.261 o posterior.

También puede ejecutar `claude mcp add` en la terminal integrada de VS Code (`` Ctrl+` `` o `` Cmd+` ``). El diálogo y el comando de terminal guardan en la misma configuración de MCP, y los cambios de cualquiera de los dos tienen efecto en las conversaciones que inicia después. El ejemplo a continuación agrega el servidor MCP remoto de GitHub, que se autentica con un [token de acceso personal](https://github.com/settings/personal-access-tokens) pasado como encabezado:

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

Reemplace `YOUR_GITHUB_PAT` con su token de acceso personal. El comando `claude mcp add` guarda la configuración sin validar credenciales, por lo que se acepta un valor de marcador de posición aquí pero el servidor no se conecta más tarde. Para verificar la conexión, inicie una nueva conversación, escriba `/mcp` y compruebe que el servidor muestre **Connected**. Un servidor con credenciales incorrectas muestra **Failed**.

Una vez configurado, pida a Claude que use las herramientas (por ejemplo, "Review PR #456").

Para encontrar servidores para conectar, consulte [Encontrar y construir servidores MCP](/docs/es/mcp#find-and-build-mcp-servers).

<h2 id="work-with-git">
  Trabajar con git
</h2>

Claude Code se integra con git para ayudar con flujos de trabajo de control de versiones directamente en VS Code. Pida a Claude que confirme cambios, cree solicitudes de extracción o trabaje en diferentes ramas. Para iniciar Claude en un árbol de trabajo aislado con sus propios archivos y rama, consulte [Ejecutar sesiones paralelas con worktrees](/docs/es/worktrees).

<h3 id="create-commits-and-pull-requests">
  Crear confirmaciones y solicitudes de extracción
</h3>

Claude puede preparar cambios, escribir mensajes de confirmación y crear solicitudes de extracción basadas en su trabajo:

```text wrap theme={null}
commit my changes with a descriptive message
create a pr for this feature
summarize the changes I've made to the auth module
```

Al crear solicitudes de extracción, Claude genera descripciones basadas en los cambios de código reales y puede agregar contexto sobre decisiones de prueba o implementación.

<h2 id="use-third-party-providers">
  Usar proveedores de terceros
</h2>

De forma predeterminada, Claude Code se conecta directamente a la API de Anthropic. Si su organización utiliza Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry para acceder a Claude, configure la extensión para usar su proveedor en su lugar:

<Steps>
  <Step title="Desactivar el aviso de inicio de sesión">
    Abra la [configuración Desactivar aviso de inicio de sesión](vscode://settings/claudeCode.disableLoginPrompt) y marque la casilla.

    También puede abrir la configuración de VS Code (`Cmd+,` en Mac o `Ctrl+,` en Windows/Linux), buscar "Claude Code login" y marcar **Desactivar aviso de inicio de sesión**.
  </Step>

  <Step title="Configurar su proveedor">
    Siga la guía de configuración de su proveedor:

    * [Claude Code en Amazon Bedrock](/docs/es/amazon-bedrock)
    * [Claude Code en Google Cloud's Agent Platform](/docs/es/google-vertex-ai)
    * [Claude Code en Microsoft Foundry](/docs/es/microsoft-foundry)

    Estas guías cubren la configuración de su proveedor en `~/.claude/settings.json`, lo que garantiza que su configuración se comparta entre la extensión de VS Code y la CLI.
  </Step>
</Steps>

En un proveedor de terceros, la extensión no ofrece características que requieren una cuenta de claude.ai, como barras de uso del plan, [dictado de voz](/docs/es/voice-dictation) y la pestaña Web para [sesiones en la nube](#resume-cloud-sessions-from-claude-ai). Para ver qué muestra el diálogo Cuenta y uso en estos inicios de sesión, consulte [Verificar cuenta y uso](#check-account-and-usage).

Un inicio de sesión de claude.ai que queda de un `/login` anterior permanece sin usar: la extensión no lo envía con ninguna solicitud.

<h2 id="security-and-privacy">
  Seguridad y privacidad
</h2>

Su código permanece privado. Claude Code procesa su código para proporcionar asistencia, pero no lo utiliza para entrenar modelos. Para obtener detalles sobre el manejo de datos y cómo optar por no participar en el registro, consulte [Datos y privacidad](/docs/es/data-usage).

Con los permisos de auto-edición habilitados, Claude Code puede modificar archivos de configuración de VS Code (como `settings.json` o `tasks.json`) que VS Code puede ejecutar automáticamente. Para reducir el riesgo al trabajar con código no confiable:

* Habilite [Modo restringido de VS Code](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode) para espacios de trabajo no confiables
* Utilice el modo Manual en lugar de Editar automáticamente o Auto para ediciones
* Revise los cambios cuidadosamente antes de aceptarlos

<h3 id="the-built-in-ide-mcp-server">
  El servidor MCP IDE integrado
</h3>

Cuando la extensión está activa, ejecuta un servidor MCP local al que la CLI se conecta automáticamente. Así es como la CLI abre diffs en el visor de diffs nativo de VS Code, lee su selección actual para menciones `@` y, cuando está trabajando en un cuaderno Jupyter, le pide a VS Code que ejecute celdas.

El servidor se llama `ide` y está oculto en `/mcp` porque no hay nada que configurar. Sin embargo, si su organización utiliza un hook `PreToolUse` para crear una lista de herramientas MCP permitidas, deberá saber que existe.

**Contexto de selección y archivo abierto.** Mientras está conectado, la CLI incluye su selección actual del editor y la ruta del archivo activo como contexto en cada solicitud que envía. La transcripción muestra una línea `⧉ Selected N lines from <file>` cuando esto sucede.

Para excluir un archivo sensible como `.env`, agregue una [regla de denegación `Read`](/docs/es/permissions#read-and-edit) para su ruta. Una regla de denegación coincidente evita que tanto el texto seleccionado como el aviso de archivo abierto para ese archivo lleguen a Claude.

Si desactiva la [configuración Attach Open File](#extension-settings), la CLI recibe la ruta del archivo activo solo mientras tenga texto seleccionado en él.

**Transporte y autenticación.** El servidor se vincula a `127.0.0.1` en un puerto aleatorio en el rango 10000–65535, y el puerto no es configurable. El transporte es `ws://` sin cifrar; como el socket es solo de bucle invertido, cualquier proceso que pueda capturar el tráfico también puede leer el token del archivo de bloqueo, por lo que TLS no agregaría protección. Cada activación de extensión genera un token de autenticación aleatorio nuevo, lo escribe en un archivo de bloqueo en `~/.claude/ide/<port>.lock`, y la CLI debe presentarlo como el encabezado `X-Claude-Code-Ide-Authorization` para conectarse. El archivo de bloqueo tiene permisos `0600` en un directorio `0700`, por lo que solo el usuario que ejecuta VS Code puede leerlo. Si se establece `CLAUDE_CONFIG_DIR`, el archivo de bloqueo se escribe en `$CLAUDE_CONFIG_DIR/ide/` en su lugar.

**Herramientas expuestas al modelo.** El servidor aloja una docena de herramientas, pero solo dos son visibles para el modelo. El resto son RPC internas que la CLI utiliza para su propia interfaz de usuario — abrir diffs, leer selecciones, guardar archivos — y se filtran antes de que la lista de herramientas llegue a Claude.

| Nombre de la herramienta (como se ve en los hooks) | Qué hace                                                                                                                                           | Solo lectura |
| -------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| `mcp__ide__getDiagnostics`                         | Devuelve diagnósticos del servidor de lenguaje — los errores y advertencias en el panel Problemas de VS Code. Opcionalmente limitado a un archivo. | Sí           |
| `mcp__ide__executeCode`                            | Ejecuta código Python en el kernel del cuaderno Jupyter activo. Consulte el flujo de confirmación a continuación.                                  | No           |

**La ejecución de Jupyter siempre pregunta primero.** `mcp__ide__executeCode` no puede ejecutar nada silenciosamente. En cada llamada, el código se inserta como una nueva celda al final del cuaderno activo, VS Code la desplaza a la vista, y una Quick Pick nativa le pide que **Ejecute** o **Cancele**. Cancelar — o descartar el selector con `Esc` — devuelve un error a Claude y nada se ejecuta. La herramienta también se niega rotundamente cuando no hay un cuaderno activo, cuando la extensión Jupyter (`ms-toolsai.jupyter`) no está instalada, o cuando el kernel no es Python.

<Note>
  La confirmación de Quick Pick es independiente de los hooks `PreToolUse`. Una entrada de lista de permitidos para `mcp__ide__executeCode` permite que Claude *proponga* ejecutar una celda; la Quick Pick dentro de VS Code es lo que permite que *realmente* se ejecute.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  Solucionar problemas comunes
</h2>

<h3 id="extension-won’t-install">
  La extensión no se instala
</h3>

* Asegúrese de tener una versión compatible de VS Code (1.94.0 o posterior)
* Verifique que VS Code tenga permiso para instalar extensiones
* Intente instalar directamente desde [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code)

<h3 id="spark-icon-not-visible">
  El icono Spark no es visible
</h3>

El icono Spark aparece en la **Barra de herramientas del editor** (esquina superior derecha del editor) cuando tiene un archivo abierto. Si no lo ve:

1. **Abra un archivo**: El icono requiere que un archivo esté abierto. Solo tener una carpeta abierta no es suficiente.
2. **Verifique la versión de VS Code**: Requiere 1.94.0 o superior (Ayuda → Acerca de)
3. **Reinicie VS Code**: Ejecute "Developer: Reload Window" desde la Paleta de comandos
4. **Desactive extensiones conflictivas**: Desactive temporalmente otras extensiones de IA (Cline, Continue, etc.)
5. **Verifique la confianza del espacio de trabajo**: La extensión no funciona en Modo restringido

Alternativamente, si ha establecido [`preferredLocation`](#extension-settings) en `sidebar`, o ha abierto Claude con **Claude Code: Open in Side Bar**, haga clic en "✻ Claude Code" en la **Barra de estado** (esquina inferior derecha). Esto funciona incluso sin un archivo abierto. También puede usar la **Paleta de comandos** (`Cmd+Shift+P` / `Ctrl+Shift+P`) y escribir "Claude Code".

<h3 id="cmd-esc-does-nothing-on-macos">
  Cmd+Esc no hace nada en macOS
</h3>

En macOS Tahoe y posterior, el atajo del sistema Game Overlay está vinculado a `Cmd+Esc` de forma predeterminada e intercepta la pulsación de tecla antes de que llegue a VS Code. Para liberar el atajo:

1. Abra Configuración del sistema
2. Vaya a Teclado, luego Atajos de teclado, luego Controladores de juegos
3. Desmarque la casilla Game Overlay

Alternativamente, revinculle la extensión a una tecla diferente: abra el editor de [Atajos de teclado](https://code.visualstudio.com/docs/configure/keybindings) de VS Code (`Cmd+K Cmd+S`), busque `Claude Code: Focus input`, y asigne un nuevo atajo.

<h3 id="claude-code-never-responds">
  Claude Code nunca responde
</h3>

Si Claude Code no responde a sus indicaciones:

1. **Verifique su conexión a Internet**: Asegúrese de tener una conexión a Internet estable
2. **Inicie una nueva conversación**: Intente iniciar una conversación nueva para ver si el problema persiste
3. **Pruebe la CLI**: Ejecute `claude` desde la terminal para ver si obtiene mensajes de error más detallados

Si los problemas persisten, [presente un problema en GitHub](https://github.com/anthropics/claude-code/issues) con detalles sobre el error.

<h2 id="uninstall-the-extension">
  Desinstalar la extensión
</h2>

Para desinstalar la extensión Claude Code:

1. Abra la vista Extensiones (`Cmd+Shift+X` en Mac o `Ctrl+Shift+X` en Windows/Linux)
2. Busque "Claude Code"
3. Haga clic en **Desinstalar**

Si ejecuta `claude` en una terminal integrada de VS Code, Claude Code reinstala la extensión automáticamente. Para mantenerla desinstalada, desactive **Auto-install IDE extension** en `/config`, o establezca [`autoInstallIdeExtension`](/docs/es/settings-reference#autoinstallideextension) en `false`. También puede establecer la variable de entorno [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/es/env-vars) en `1`.

Para eliminar también los datos de la extensión y restablecer toda la configuración, elimine el directorio de almacenamiento de la extensión para su plataforma.

En macOS:

```bash theme={null}
rm -rf ~/Library/"Application Support"/Code/User/globalStorage/anthropic.claude-code
```

En Linux:

```bash theme={null}
rm -rf ~/.config/Code/User/globalStorage/anthropic.claude-code
```

En Windows, en PowerShell:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:APPDATA\Code\User\globalStorage\anthropic.claude-code"
```

Para obtener ayuda adicional, consulte la [guía de solución de problemas](/docs/es/troubleshooting).

<h2 id="next-steps">
  Próximos pasos
</h2>

Ahora que tienes Claude Code configurado en VS Code:

* [Explora flujos de trabajo comunes](/docs/es/common-workflows) para aprovechar al máximo Claude Code
* [Configura MCP servers](/docs/es/mcp) para extender las capacidades de Claude con herramientas externas. Agrega y administra servidores con `/mcp` en el panel de chat.
* [Configura la configuración de Claude Code](/docs/es/settings) para personalizar comandos permitidos, hooks y más. Esta configuración se comparte entre la extensión y la CLI.
