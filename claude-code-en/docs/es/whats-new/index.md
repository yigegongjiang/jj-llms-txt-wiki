> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Novedades

> Un resumen semanal de las características notables de Claude Code, con fragmentos de código, demostraciones y contexto sobre por qué importan.

El resumen semanal para desarrolladores destaca las características más propensas a cambiar la forma en que trabaja. Cada entrada incluye código ejecutable, una breve demostración y un enlace a la documentación completa. Para cada corrección de errores y mejora menor, consulte el [registro de cambios](/docs/en/changelog).

<Update label="Week 37" description="September 7–11, 2026" tags={["v2.1.263–v2.1.269"]}>
  **`claude plugin eval`**: ejecute su plugin contra un conjunto de casos de prueba, califique los resultados y compare con una línea de base sin plugin. `claude plugin eval init` redacta los casos y calificadores para usted.

  También esta semana: extraiga cualquier **panel de Claude Code Desktop** en su propia ventana y acóplelo nuevamente más tarde; la configuración **`maxEffortLevel`** limita el nivel de esfuerzo en cada proveedor; y una página que **WebFetch** no ha terminado de descargar dentro de cinco minutos falla en lugar de colgarse.

  [Lea el resumen de la Week 37 →](/docs/es/whats-new/2026-w37)
</Update>

<Update label="Week 36" description="August 31 – September 4, 2026" tags={["v2.1.251–v2.1.261"]}>
  **Claude Fable 5.1**: disponible en Claude Code con una ventana de contexto de 1M de tokens.

  También esta semana: en planes Pro y Max, **el uso de computadora en la aplicación Desktop** funciona en segundo plano en macOS mientras continúa trabajando; en renderizado a pantalla completa, **`/diff`** abre un panel en vivo junto a la conversación que se actualiza mientras Claude edita; y **`/skill-doctor`** muestra qué cuesta cada una de sus skills en contexto y con qué frecuencia se usa.

  [Lea el resumen de la Week 36 →](/docs/es/whats-new/2026-w36)
</Update>

<Update label="Week 35" description="August 24–28, 2026" tags={["v2.1.240–v2.1.250"]}>
  **Reanudar sesiones de terminal en la aplicación Desktop**: escriba `/resume` en el cuadro de solicitud de Claude Code Desktop para retomar cualquier sesión que haya iniciado desde la CLI, con la conversación completa e intacta.

  También esta semana: **comentarios redactados por Claude** hace que Claude redacte un informe de comentarios cuando algo sale mal en una sesión, que usted revisa y envía desde `/feedback`; **`--restricted`** inicia una sesión sin las herramientas de ejecución de comandos ni su configuración de usuario y proyecto, para arneses de evaluación en máquinas compartidas; y la configuración **`modelPicker`** controla qué modelos lista el selector `/model`.

  [Lea el resumen de la Week 35 →](/docs/es/whats-new/2026-w35)
</Update>

<Update label="Week 34" description="August 17–21, 2026" tags={["v2.1.234–v2.1.239"]}>
  **`/design`**: una vista previa de investigación que trae el flujo de trabajo de artboard de Claude Design a la CLI y Claude Code Desktop, construido sobre artifacts, para que Claude redacte artboards editables para su interfaz de usuario e implemente el que elija.

  También esta semana: el **Concise output style** integrado hace que Claude comience con el resultado y omita el preámbulo; cualquier máquina que ejecute `claude remote-control` aparece como una **device card** en su teléfono para que pueda iniciar una sesión en ella desde la pestaña Code; y **`ANTHROPIC_DEFAULT_MODEL`** establece el modelo en el que comienzan las nuevas sesiones.

  [Lea el resumen de la Week 34 →](/docs/es/whats-new/2026-w34)
</Update>

<Update label="Week 33" description="August 10–14, 2026" tags={["v2.1.225–v2.1.233"]}>
  **Auto-continue después de un límite de uso en Desktop**: cuando alcanza su límite de sesión en Claude Code Desktop, marque **Auto-continue when limits reset** en la tarjeta de límite y la aplicación reintenta el turno interrumpido una vez que se restablece el límite.

  También esta semana: **fork mode** está activado de forma predeterminada en sesiones interactivas, para que Claude pueda entregar una tarea secundaria a un subagente que hereda la conversación completa; las URL de solicitud de fusión de **GitLab** funcionan con `--worktree` y la vista `claude agents`, y los mercados clonan URL de `gitlab.com` desnudas; y escribir **`@`** en el mensaje menciona otra sesión de Claude por nombre.

  [Lea el resumen de la Week 33 →](/docs/es/whats-new/2026-w33)
</Update>

<Update label="Week 32" description="August 3–7, 2026" tags={["v2.1.220–v2.1.224"]}>
  **Mensajería entre sesiones**: en macOS y Linux, sus sesiones de Claude Code ahora pueden enviarse mensajes entre sí, para que Claude transmita un hallazgo o una decisión de una sesión a otra en lugar de que usted lo vuelva a explicar.

  También esta semana: **entornos autohospedados** ejecutan sesiones en la nube de Claude Code en infraestructura que opera su organización, en beta pública en planes Team y Enterprise; **auto mode** se convierte en el modo de permiso predeterminado para nuevas sesiones en planes Pro, Max y Team a partir del 14 de agosto; y la **extensión de VS Code** obtiene Focus view.

  [Lea el resumen de la Week 32 →](/docs/es/whats-new/2026-w32)
</Update>

<Update label="Week 30" description="July 20–24, 2026" tags={["v2.1.214–v2.1.219"]}>
  **Claude Opus 5**: el nuevo modelo Opus predeterminado en Claude Code, con una ventana de contexto de 1M de tokens y modo rápido a \$10/\$50 por MTok.

  También esta semana: **Claude Code Desktop** abre un panel iOS Simulator en beta pública para que Claude pueda ejecutar su aplicación y navegar por ella mientras usted observa; el **plugin de seguridad de Claude** ejecuta un escaneo de vulnerabilidades multiagente de su base de código y convierte los hallazgos que elige en parches que aplica usted mismo; y **`/code-review`** se ejecuta como un subagente de fondo.

  [Lea el resumen de la Week 30 →](/docs/es/whats-new/2026-w30)
</Update>

<Update label="Week 29" description="July 13–17, 2026" tags={["v2.1.207–v2.1.212"]}>
  **Los artifacts llaman a sus conectores MCP**: un artifact publicado puede extraer datos en vivo y tomar acciones a través de los conectores MCP propios de cada visualizador cuando abre la página, y esta semana también agrega enlaces de uso compartido público, roles de editor en Team y Enterprise, y artifacts creados a partir de sesiones de Claude Tag.

  También esta semana: **screen reader mode** reemplaza la interfaz de terminal visual con texto plano y lineal para lectores de pantalla como VoiceOver y NVDA; **`/fork`** copia su conversación en una nueva sesión de fondo mientras continúa trabajando; y **auto mode** ya no necesita una variable de opción en Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry.

  [Lea el resumen de la Week 29 →](/docs/es/whats-new/2026-w29)
</Update>

<Update label="Week 28" description="July 6–10, 2026" tags={["v2.1.202–v2.1.206"]}>
  **Navegador integrado en la aplicación en Desktop**: Claude Code en desktop obtiene un navegador integrado, para que Claude pueda abrir documentos, diseños o cualquier otro sitio e interactuar con páginas de la misma manera que lo hace con sus vistas previas del servidor de desarrollo local.

  También esta semana: **`/doctor`** es una verificación completa de configuración que diagnostica problemas y puede solucionarlos, con `/checkup` como su alias; **el modo automático** bloquea la manipulación de transcripciones y solicita confirmación antes de `rm -rf` en variables sin resolver; y **las filas de vista de agentes** muestran una palabra de estado coloreada y un titular escrito por clasificador.

  [Lea el resumen de la Week 28 →](/docs/es/whats-new/2026-w28)
</Update>

<Update label="Week 27" description="June 29 – July 3, 2026" tags={["v2.1.195–v2.1.201"]}>
  **Claude Sonnet 5**: el nuevo modelo predeterminado para asientos de suscripción Pro, Team Standard y Enterprise, con codificación de nivel superior y uso de herramientas al precio de Sonnet, una ventana de contexto nativa de 1M de tokens y pensamiento adaptativo activado de forma predeterminada.

  También esta semana: **Claude en Chrome** está disponible de forma general en todos los planes directos de Anthropic; **los subagentes se ejecutan en segundo plano de forma predeterminada** para que Claude siga trabajando mientras se ejecutan; **Claude Desktop en Linux** llega en beta en Ubuntu y Debian; y **`/radio`** sintoniza la radio lo-fi de Claude FM.

  [Lea el resumen de la Week 27 →](/docs/es/whats-new/2026-w27)
</Update>

<Update label="Week 26" description="June 22–26, 2026" tags={["v2.1.185–v2.1.193"]}>
  **`claude mcp login`**: autentique un servidor MCP configurado desde su shell en lugar del menú interactivo `/mcp`, y borre sus credenciales almacenadas más tarde con `claude mcp logout`.

  También esta semana: **el modo shell responde a la salida de comandos** (`! npm test` obtiene una explicación sin un segundo mensaje); **`/rewind`** puede reanudar una conversación desde antes de que se ejecutara `/clear`; y **los subagentes de fondo** ahora muestran solicitudes de permiso en la sesión principal en lugar de denegarlas automáticamente.

  [Lea el resumen de la Week 26 →](/docs/es/whats-new/2026-w26)
</Update>

<Update label="Week 25" description="June 15–19, 2026" tags={["v2.1.178–v2.1.183"]}>
  **Artifacts**: convierta la salida de una sesión en una página en vivo y compartible en claude.ai que se actualiza en su lugar mientras la sesión funciona, ahora en beta en planes Team y Enterprise.

  También esta semana: **las reglas de negación y solicitud coinciden con parámetros de herramientas** con `Tool(param:value)`, por ejemplo `Agent(model:opus)`; **`/config key=value`** establece cualquier configuración desde el mensaje, en modo `-p`, y desde Remote Control; y **el modo automático bloquea comandos git destructivos** cuando no pidió descartar trabajo local.

  [Lea el resumen de la Week 25 →](/docs/es/whats-new/2026-w25)
</Update>

<Update label="Week 24" description="June 8–12, 2026" tags={["v2.1.166–v2.1.176"]}>
  **`/cd`**: mueva la sesión actual a un nuevo directorio de trabajo a mitad de la conversación sin reconstruir la caché de solicitud.

  También esta semana: **los sub-agentes pueden generar sus propios sub-agentes** (las cadenas de fondo están limitadas a cinco niveles de profundidad); **`--safe-mode`** inicia Claude Code con todas las personalizaciones deshabilitadas para solucionar problemas; y **`fallbackModel`** configura hasta tres modelos de respaldo que se intentan en orden.

  [Lea el resumen de la Week 24 →](/docs/es/whats-new/2026-w24)
</Update>

<Update label="Week 23" description="June 1–5, 2026" tags={["v2.1.158–v2.1.165"]}>
  **Modo automático en Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry**: el modo automático ahora está disponible en proveedores de terceros para Opus 4.7 y Opus 4.8, reemplazando solicitudes de permiso con comprobaciones de seguridad en segundo plano.

  También esta semana: **ediciones automáticas más seguras** solicitan confirmación antes de escribir archivos que pueden ejecutar código en modo `acceptEdits`; **`/plugin list`** imprime sus plugins instalados en línea; y **requisitos de versión** permiten que las implementaciones administradas requieran un rango de versión de Claude Code aprobado.

  [Lea el resumen de la Week 23 →](/docs/es/whats-new/2026-w23)
</Update>

<Update label="Week 22" description="May 25–29, 2026" tags={["v2.1.150–v2.1.157"]}>
  **Claude Opus 4.8**: el nuevo modelo predeterminado para Max, Team Premium, Enterprise de pago por uso, y cuentas de API de Anthropic, con alto esfuerzo de forma predeterminada y `/effort xhigh` para las tareas más difíciles.

  También esta semana: **flujos de trabajo dinámicos** orquestan docenas a cientos de subagentes desde un script que Claude escribe; el **plugin de orientación de seguridad** revisa los cambios de Claude en busca de vulnerabilidades mientras trabaja; y **modo rápido** se ejecuta en Opus 4.8 a \$10/\$50 por MTok.

  [Lea el resumen de la Week 22 →](/docs/es/whats-new/2026-w22)
</Update>

<Update label="Week 21" description="May 18–22, 2026" tags={["v2.1.143–v2.1.149"]}>
  **Modo automático en el plan Pro**: el modo automático ahora se ejecuta en cuentas Pro y admite Sonnet 4.6 junto con Opus, reemplazando solicitudes de permiso con comprobaciones de seguridad en segundo plano.

  También esta semana: **`/usage`** desglosa qué impulsa sus límites de plan por skill, subagente, plugin y servidor MCP; el nuevo comando **`/code-review`** reporta errores de corrección; y **sesiones en segundo plano** aparecen en `/resume` y permanecen activas cuando se fijan.

  [Lea el resumen de la Week 21 →](/docs/es/whats-new/2026-w21)
</Update>

<Update label="Week 20" description="May 11–15, 2026" tags={["v2.1.139–v2.1.142"]}>
  **Vista de agentes**: `claude agents` abre una pantalla para cada sesión de Claude Code, mostrando qué se está ejecutando, qué está bloqueado esperándolo, y qué está hecho.

  También esta semana: **`/goal`** mantiene a Claude trabajando entre turnos hasta que se cumple una condición de finalización; **modo rápido** ahora se ejecuta en Opus 4.7 de forma predeterminada; y el **menú Rewind** puede comprimir contexto anterior con "Summarize up to here".

  [Lea el resumen de la Week 20 →](/docs/es/whats-new/2026-w20)
</Update>

<Update label="Week 19" description="May 4–8, 2026" tags={["v2.1.128–v2.1.136"]}>
  **Los plugins se cargan desde archivos `.zip` y URLs**: `--plugin-dir` ahora acepta archivos `.zip`, y `--plugin-url` obtiene un archivo de plugin para la sesión actual.

  También esta semana: **`worktree.baseRef`** elige si los nuevos worktrees se ramifican desde el remoto predeterminado o desde `HEAD` local; **reglas de negación dura en modo automático** bloquean acciones incondicionalmente independientemente de excepciones de permiso; y **los hooks ven el nivel de esfuerzo activo** a través de `effort.level` y `$CLAUDE_EFFORT`.

  [Lea el resumen de la Week 19 →](/docs/es/whats-new/2026-w19)
</Update>

<Update label="Week 18" description="April 27 – May 1, 2026" tags={["v2.1.120–v2.1.126"]}>
  **Windows sin Git Bash**: Git para Windows ya no es necesario, y Claude Code usa PowerShell como herramienta de shell cuando Bash no está disponible.

  También esta semana: **`claude ultrareview`** trae revisión de código en la nube a CI y scripts; **`claude project purge`** limpia el estado local de un proyecto; y pegar una **URL de PR en `/resume`** encuentra la sesión que la creó.

  [Lea el resumen de la Week 18 →](/docs/es/whats-new/2026-w18)
</Update>

<Update label="Week 17" description="April 20–24, 2026" tags={["v2.1.114–v2.1.119"]}>
  **`/ultrareview`** se abre como una vista previa de investigación pública: una flota de agentes cazadores de errores se ejecuta en la nube y los hallazgos llegan automáticamente a su CLI o Desktop.

  También esta semana: **session recap** le muestra qué sucedió mientras una terminal no estaba enfocada; **custom themes** le permite crear y enviar paletas de colores desde `/theme` o un plugin; y **Claude Code en la web** recibe un rediseño con una nueva barra lateral de sesiones y diseño de arrastrar y soltar.

  [Lea el resumen de la Week 17 →](/docs/es/whats-new/2026-w17)
</Update>

<Update label="Week 16" description="April 13–17, 2026" tags={["v2.1.105–v2.1.113"]}>
  **Claude Opus 4.7** llega como el nuevo predeterminado en Max y Team Premium, con un nuevo nivel de esfuerzo `xhigh` que es la configuración recomendada para la mayoría del trabajo de codificación y un control deslizante interactivo `/effort` para ajustarlo.

  También esta semana: **Routines** en Claude Code en la web disparan agentes en la nube con plantillas desde una programación, evento de GitHub o llamada API; **notificaciones push móviles** le avisan a su teléfono cuando una tarea larga finaliza o Claude lo necesita; `/usage` muestra qué está impulsando sus límites; y la CLI se traslada a binarios nativos.

  [Lea el resumen de la Week 16 →](/docs/es/whats-new/2026-w16)
</Update>

<Update label="Week 15" description="April 6–10, 2026" tags={["v2.1.92–v2.1.101"]}>
  **Ultraplan** entra en vista previa temprana: redacte un plan en la nube desde su CLI, revíselo y comente en un editor web, luego ejecútelo de forma remota o extráigalo localmente. La primera ejecución ahora crea automáticamente un entorno en la nube para usted.

  También esta semana: la herramienta **Monitor** transmite eventos de fondo a la conversación para que Claude pueda monitorear registros y reaccionar en vivo, `/loop` se autoajusta cuando omite el intervalo, `/team-onboarding` empaqueta su configuración en una guía reproducible, y `/autofix-pr` activa la corrección automática de PR desde su terminal.

  [Lea el resumen de la Week 15 →](/docs/es/whats-new/2026-w15)
</Update>

<Update label="Week 14" description="March 30 – April 3, 2026" tags={["v2.1.86–v2.1.91"]}>
  **Computer use** llega a la CLI en vista previa de investigación: Claude puede abrir aplicaciones nativas, hacer clic en la interfaz de usuario y verificar cambios desde su terminal. Lo mejor para cerrar el ciclo en cosas que solo una GUI puede verificar.

  También esta semana: lecciones interactivas `/powerup`, renderizado de pantalla alternativa sin parpadeos, una anulación de tamaño de resultado MCP por herramienta de hasta 500K, y ejecutables de plugin en la `PATH` de la herramienta Bash.

  [Lea el resumen de la Week 14 →](/docs/es/whats-new/2026-w14)
</Update>

<Update label="Week 13" description="March 23–27, 2026" tags={["v2.1.83–v2.1.85"]}>
  **Auto mode** llega en vista previa de investigación: un clasificador maneja sus solicitudes de permiso para que las acciones seguras se ejecuten sin interrupción y las arriesgadas se bloqueen. El término medio entre aprobar todo y `--dangerously-skip-permissions`.

  También esta semana: uso de computadora en la aplicación Desktop, corrección automática de PR en Web, búsqueda de transcripción con `/`, una herramienta PowerShell nativa para Windows, y hooks `if` condicionales.

  [Lea el resumen de la Week 13 →](/docs/es/whats-new/2026-w13)
</Update>
