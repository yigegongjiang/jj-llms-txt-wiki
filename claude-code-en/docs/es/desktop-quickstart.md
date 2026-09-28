> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Comenzar con la aplicación de escritorio

> Instale Claude Code en el escritorio e inicie su primera sesión de codificación

La aplicación de escritorio le proporciona Claude Code con una interfaz gráfica diseñada para ejecutar múltiples sesiones lado a lado: una barra lateral para gestionar trabajo paralelo, un diseño de arrastrar y soltar con terminal integrada y editor de archivos, revisión visual de diferencias, vista previa de aplicaciones en vivo, monitoreo de PR de GitHub con fusión automática, y tareas programadas. No se requiere terminal.

<CardGroup cols={3}>
  <Card title="Descargar para macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Compilación universal para Intel y Apple Silicon
  </Card>

  <Card title="Descargar para Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Para procesadores x64
  </Card>

  <Card title="Obtener Claude para Linux (beta)" icon="linux" href="/docs/es/desktop-linux">
    apt o .deb para Ubuntu y Debian
  </Card>
</CardGroup>

Para Windows ARM64, descargue el [instalador ARM64](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). En Linux, instale con apt; consulte [Claude Desktop en Linux](/docs/es/desktop-linux).

<Note>
  Claude Code requiere una [suscripción Pro, Max, Team o Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing).
</Note>

Esta página lo guía a través de la instalación de la aplicación e iniciando su primera sesión. Si ya está configurado, consulte [Usar Claude Code Desktop](/docs/es/desktop) para la referencia completa.

La aplicación de escritorio tiene tres pestañas:

* **Chat**: Conversación general sin acceso a archivos, similar a claude.ai.
* **Cowork**: Un agente autónomo de fondo que trabaja en tareas en una máquina virtual en sandbox con su propio entorno, ejecutándose de forma independiente mientras realiza otro trabajo. Las sesiones de Cowork en el dispositivo ejecutan la VM en su computadora; las sesiones de Cowork remotas se ejecutan en una VM administrada por Anthropic en su lugar.
* **Code**: Un asistente de codificación interactivo con acceso directo a sus archivos locales. Dependiendo del modo de permisos, usted aprueba cada cambio conforme Claude lo propone o revisa los cambios después de que Claude los realiza.

Chat y Cowork se tratan en el [Centro de Ayuda de Claude](https://support.claude.com/); la instalación e implementación de la aplicación de escritorio se tratan en los [artículos de soporte de Claude Desktop](https://support.claude.com/en/collections/16163169-claude-desktop). Esta página se enfoca en la pestaña **Code**.

<h2 id="install">
  Instalar
</h2>

<Steps>
  <Step title="Instalar e iniciar sesión">
    En macOS y Windows, descargue el instalador desde los enlaces anteriores y ejecútelo. En Linux, siga los pasos de instalación en [Claude Desktop en Linux](/docs/es/desktop-linux). Inicie Claude desde su carpeta Aplicaciones en macOS, el menú Inicio en Windows, o su lanzador de aplicaciones en Linux, luego inicie sesión con su cuenta de Anthropic.
  </Step>

  <Step title="Abrir la pestaña Code">
    Haga clic en la pestaña **Code** en el centro superior. Si hacer clic en Code le solicita actualizar, debe [suscribirse a un plan de pago](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_upgrade) primero. Si le solicita iniciar sesión en línea, complete el inicio de sesión y reinicie la aplicación. Si ve un error 403, consulte [solución de problemas de autenticación](/docs/es/desktop#403-or-authentication-errors-in-the-code-tab).
  </Step>
</Steps>

La aplicación de escritorio incluye Claude Code. No necesita instalar Node.js o la CLI por separado. Para usar `claude` desde la terminal, instale la CLI por separado. Consulte [Comenzar con la CLI](/docs/es/quickstart).

<h2 id="start-your-first-session">
  Inicie su primera sesión
</h2>

Con la pestaña Code abierta, elija un proyecto y dé a Claude algo que hacer.

<Steps>
  <Step title="Elija un entorno y carpeta">
    Seleccione **Local** para ejecutar Claude en su máquina utilizando sus archivos directamente. Haga clic en **Select folder** y elija su directorio de proyecto.

    <Tip>
      Comience con un proyecto pequeño que conozca bien. Es la forma más rápida de ver qué puede hacer Claude Code.
    </Tip>

    También puede seleccionar:

    * **Cloud**: Ejecute sesiones en la nube que continúen incluso si cierra la aplicación. Consulte [Usar Claude Code en la nube](/docs/es/claude-code-on-the-web) para saber cómo funcionan las sesiones en la nube.
    * **SSH**: Conéctese a una máquina remota a través de SSH, como sus propios servidores, máquinas virtuales en la nube o contenedores de desarrollo. Desktop instala Claude Code en la máquina remota automáticamente la primera vez que se conecta.
    * **WSL** (Windows): Ejecute la sesión dentro de una [distribución WSL 2](/docs/es/desktop-wsl); Claude Code, las herramientas y git se ejecutan en el lado de Linux con rutas nativas.
  </Step>

  <Step title="Elija un modelo">
    Seleccione un modelo en el menú desplegable junto al botón de envío. Consulte [models](/docs/es/model-config#available-models) para una comparación de los modelos disponibles. Puede cambiar el modelo más tarde desde el mismo menú desplegable.
  </Step>

  <Step title="Dígale a Claude qué hacer">
    Escriba lo que desea que Claude haga:

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    Una [session](/docs/es/desktop#work-in-parallel-with-sessions) es una conversación con Claude sobre su código. Cada sesión realiza un seguimiento de su propio contexto y cambios.
  </Step>

  <Step title="Revise y acepte los cambios">
    Lo que sucede a continuación depende del [permission mode](/docs/es/desktop#choose-a-permission-mode) que se muestra en el selector junto al botón de envío:

    * **Auto or Accept edits**: Claude aplica sus cambios de archivo, y aparece un indicador como `+12 -1` para que pueda revisarlos en la vista de diferencias
    * **Manual**: Claude propone cada cambio y espera su aprobación antes de aplicarlo. Sus archivos no se modifican hasta que acepte, y si rechaza un cambio, Claude le pregunta cómo le gustaría proceder en su lugar

    En modo Manual, verá:

    1. Una [diff view](/docs/es/desktop#review-changes-with-diff-view) que muestra exactamente qué cambiará en cada archivo
    2. Botones Aceptar/Rechazar para aprobar o rechazar cada cambio
    3. Actualizaciones en tiempo real mientras Claude trabaja en su solicitud
  </Step>
</Steps>

<h2 id="now-what">
  ¿Y ahora qué?
</h2>

Ha realizado su primera edición. Para obtener la referencia completa sobre todo lo que puede hacer Claude Code Desktop, consulte [Usar Claude Code Desktop](/docs/es/desktop). Aquí hay algunas cosas para probar a continuación.

**Interrumpir y redirigir.** Puede redirigir a Claude en cualquier momento. Haga clic en el botón de parada para interrumpir inmediatamente, o escriba una corrección y presione **Intro** para enviarla sin detener la acción en ejecución. De cualquier forma, no tiene que esperar a que termine o empezar de nuevo.

**Proporcione más contexto a Claude.** Escriba `@filename` en el cuadro de solicitud para extraer un archivo específico a la conversación, adjunte imágenes y PDF usando el botón de adjuntos, o arrastre y suelte archivos directamente en la solicitud. Cuanto más contexto tenga Claude, mejores serán los resultados. Consulte [Agregar archivos y contexto](/docs/es/desktop#add-files-and-context-to-prompts).

**Use skills para tareas repetibles.** Escriba `/` o haga clic en **+** → **Slash commands** para examinar [comandos integrados](/docs/es/commands), [skills personalizados](/docs/es/skills) y skills de plugins. Los skills son solicitudes reutilizables que puede invocar siempre que las necesite, como listas de verificación de revisión de código o pasos de implementación.

**Revise los cambios antes de confirmar.** Después de que Claude edite archivos, aparece un indicador `+12 -1`. Haga clic en él para abrir la [vista de diferencias](/docs/es/desktop#review-changes-with-diff-view), revise las modificaciones archivo por archivo y comente en líneas específicas. Claude lee sus comentarios y revisa. Haga clic en **Review code** para que Claude evalúe los diffs en sí mismo y deje sugerencias en línea.

**Ajuste cuánto control tiene.** Su [modo de permisos](/docs/es/desktop#choose-a-permission-mode) establece cuánto puede hacer Claude sin pedir aprobación:

* **Auto**: un clasificador revisa las acciones en segundo plano y bloquea las arriesgadas en lugar de preguntarle.
* **Manual**: Claude pregunta antes de editar archivos o ejecutar comandos.
* **Accept edits**: Claude acepta automáticamente ediciones de archivos para una iteración más rápida.
* **Plan**: Claude propone un enfoque sin editar ningún archivo, lo cual es útil antes de una refactorización grande.

**Agregue plugins para más capacidades.** Haga clic en el botón **+** junto al cuadro de solicitud y seleccione **Plugins** para examinar e instalar [plugins](/docs/es/desktop#install-plugins) que agreguen skills, agentes, servidores MCP y más.

**Organice su espacio de trabajo.** Arrastre los paneles de chat, diferencias, terminal, archivo y navegador a cualquier diseño que desee. Abra la terminal con **Ctrl+\`** para ejecutar comandos junto con su sesión, o haga clic en una ruta de archivo para abrirla en el panel de archivos. Consulte [Organizar su espacio de trabajo](/docs/es/desktop#arrange-your-workspace).

**Obtenga una vista previa de su aplicación.** Cuando ejecute su servidor de desarrollo en el escritorio, su aplicación se abre en el panel del navegador, que también puede [abrir sitios externos](/docs/es/desktop#browse-external-sites). Claude puede ver la aplicación en ejecución, probar puntos finales, inspeccionar registros e iterar sobre lo que ve. Consulte [Obtenga una vista previa de su aplicación](/docs/es/desktop#preview-your-app).

**Realice un seguimiento de su solicitud de extracción.** Después de abrir un PR, Claude Code monitorea los resultados de las verificaciones de CI y puede corregir automáticamente fallos o fusionar el PR una vez que todas las verificaciones pasen. Consulte [Monitorear el estado de la solicitud de extracción](/docs/es/desktop#monitor-pull-request-status).

**Ponga a Claude en un horario.** Configure [tareas programadas](/docs/es/desktop-scheduled-tasks) para ejecutar Claude automáticamente de forma recurrente: una revisión de código diaria cada mañana, una auditoría de dependencias semanal, o un resumen que extraiga de sus herramientas conectadas.

**Escale cuando esté listo.** Abra [sesiones paralelas](/docs/es/desktop#work-in-parallel-with-sessions) desde la barra lateral para trabajar en múltiples tareas a la vez, cada una en su propio Git worktree, y abra el [panel de tareas](/docs/es/desktop#watch-background-tasks) para ver los subagentes y comandos en segundo plano que una sesión tiene en ejecución. Abra un [chat lateral](/docs/es/desktop#ask-a-side-question-without-derailing-the-session) para hacer una pregunta sin descarrilar el hilo principal. Envíe [trabajo de larga duración a la nube](/docs/es/desktop#run-long-running-tasks-in-the-cloud) para que continúe incluso si cierra la aplicación, o [continúe una sesión en la web o en su IDE](/docs/es/desktop#continue-in-another-surface) si una tarea toma más tiempo del esperado. [Conecte herramientas externas](/docs/es/desktop#extend-claude-code) como GitHub, Slack y Linear para reunir su flujo de trabajo.

<h2 id="what’s-next">
  Qué sigue
</h2>

* [Usar Claude Code Desktop](/docs/es/desktop): modos de permisos, sesiones paralelas, vista de diferencias, conectores y configuración empresarial
* [¿Viene de la CLI?](/docs/es/desktop#coming-from-the-cli): ejecute Desktop y la CLI en el mismo proyecto, y compare características, equivalentes de banderas y lo que no está disponible en Desktop
* [Solución de problemas](/docs/es/desktop#troubleshooting): soluciones para errores comunes y problemas de configuración
* [Mejores prácticas](/docs/es/best-practices): consejos para escribir indicaciones efectivas y aprovechar al máximo Claude Code
* [Flujos de trabajo comunes](/docs/es/common-workflows): tutoriales para depuración, refactorización, pruebas y más
