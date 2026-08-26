> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Novedades

> Un resumen semanal de las características notables de Claude Code, con fragmentos de código, demostraciones y contexto sobre por qué importan.

El resumen semanal para desarrolladores destaca las características más propensas a cambiar la forma en que trabaja. Cada entrada incluye código ejecutable, una breve demostración y un enlace a la documentación completa. Para cada corrección de errores y mejora menor, consulte el [registro de cambios](/docs/es/changelog).

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
