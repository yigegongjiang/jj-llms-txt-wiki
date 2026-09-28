> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referencia de herramientas

> Referencia completa de las herramientas que Claude Code puede usar, incluidos los requisitos de permisos y el comportamiento por herramienta.

Claude Code tiene acceso a un conjunto de herramientas integradas que le ayudan a entender y modificar tu base de código. Los nombres de las herramientas son las cadenas exactas que utilizas en [reglas de permisos](/docs/es/permissions#tool-specific-permission-rules), [listas de herramientas de subagentes](/docs/es/sub-agents), y [coincidencias de hooks](/docs/es/hooks).

Para controlar qué herramientas puede usar Claude y cuándo solicita permiso primero, configura [reglas de permisos](/docs/es/permissions#tool-specific-permission-rules) en tu configuración, [hooks](/docs/es/hooks), o [lista de herramientas de un subagente](/docs/es/sub-agents#supported-frontmatter-fields). Consulta [Configurar herramientas con reglas de permisos y hooks](#configure-tools-with-permission-rules-and-hooks) para cada lugar que acepte un nombre de herramienta.

Para agregar herramientas personalizadas, conecta un [servidor MCP](/docs/es/mcp). Para extender Claude con flujos de trabajo basados en prompts reutilizables, escribe una [skill](/docs/es/skills), que se ejecuta a través de la herramienta `Skill` existente en lugar de agregar una nueva entrada de herramienta.

<Info>
  En los planes Pro, Max y Team, Claude Code inicia sesiones en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), donde un clasificador decide la mayoría de estos mensajes en lugar de ti. La columna `Permission required` muestra si la herramienta solicita permiso en [modo manual](/docs/es/permission-modes) para rutas dentro del directorio de trabajo. Las herramientas de acceso a archivos marcadas como No, incluidas `Read`, `Grep` y `Glob`, aún solicitan permiso para rutas fuera del [directorio de trabajo y directorios adicionales](/docs/es/permissions#working-directories). `Bash` está marcada como Sí pero ejecuta un conjunto integrado de [comandos de solo lectura](/docs/es/permissions#read-only-commands) sin solicitar permiso.
</Info>

| Herramienta            | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Permiso requerido |
| :--------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------- |
| `Agent`                | Genera un [subagente](/docs/es/sub-agents) con su propia ventana de contexto para manejar una tarea. Con [equipos de agentes](/docs/es/agent-teams) habilitados, una llamada que lleve un `name` puede lanzar un [compañero de equipo](/docs/es/agent-teams#how-claude-starts-agent-teams) en su lugar. Consulta [Comportamiento de la herramienta Agent](#agent-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | No                |
| `Artifact`             | Publica un archivo HTML o Markdown como un [artefacto](/docs/es/artifacts): una página privada e interactiva en claude.ai. Puedes compartirlo con un enlace público, o dentro de tu organización en los planes Team y Enterprise, donde el uso compartido público requiere que un Propietario lo [habilite](/docs/es/artifacts#control-public-sharing). Requiere un plan Pro, Max, Team o Enterprise y autenticación `/login`; consulta [Disponibilidad](/docs/es/artifacts#availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Sí                |
| `AskUserQuestion`      | Hace preguntas de opción múltiple para recopilar requisitos o aclarar ambigüedades. Las preguntas permanecen abiertas hasta que las respondas de forma predeterminada. Consulta [Comportamiento de la herramienta AskUserQuestion](#askuserquestion-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | No                |
| `Bash`                 | Ejecuta comandos de shell en tu entorno. Consulta [Comportamiento de la herramienta Bash](#bash-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Sí                |
| `CronCreate`           | Programa un prompt recurrente o de una sola vez dentro de la sesión actual. Las tareas tienen alcance de sesión y se restauran en `--resume` o `--continue` si no han expirado. Consulta [tareas programadas](/docs/es/scheduled-tasks)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | No                |
| `CronDelete`           | Cancela una tarea programada por ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | No                |
| `CronList`             | Lista todas las tareas programadas en la sesión                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | No                |
| `Edit`                 | Realiza ediciones dirigidas a archivos específicos. Consulta [Comportamiento de la herramienta Edit](#edit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Sí                |
| `EndConversation`      | Finaliza la sesión, en casos raros de entrada abusiva sostenida o cuando solicitas a Claude que demuestre la herramienta. Requiere Claude Code v2.1.213 o posterior. Consulta [Comportamiento de la herramienta EndConversation](#endconversation-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | No                |
| `EnterPlanMode`        | Cambia a Plan Mode para diseñar un enfoque antes de codificar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | No                |
| `EnterWorktree`        | Crea un [git worktree](/docs/es/worktrees) aislado y cambia a él. Pasa una `path` para cambiar a un worktree existente en lugar de crear uno nuevo. En la primera entrada, el destino puede ser un worktree del repositorio actual o, en un espacio de trabajo de múltiples repositorios, de un repositorio anidado dentro de él. Antes de v2.1.203, se rechazaba un worktree de un repositorio anidado. Una `path` fuera de `.claude/worktrees/` solicita tu aprobación antes de entrar, ya que mueve el directorio de trabajo de la sesión y el acceso de escritura a esa ubicación. La creación de nuevos worktrees y las rutas bajo `.claude/worktrees/` no solicitan permiso. Antes de v2.1.206, Claude entraba en rutas fuera de `.claude/worktrees/` sin solicitar permiso. Desde dentro de una sesión de worktree, o desde un subagente con un directorio de trabajo fijado como [`isolation: worktree`](/docs/es/sub-agents#supported-frontmatter-fields), solo está disponible la forma `path` y el destino debe estar bajo `.claude/worktrees/` del repositorio de la sesión | Sí                |
| `ExitPlanMode`         | Presenta un plan para aprobación y sale de Plan Mode                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Sí                |
| `ExitWorktree`         | Sale de una sesión de worktree y regresa al directorio original. No disponible para subagentes que ya se ejecutan en su propio directorio de trabajo, como con [`isolation: worktree`](/docs/es/sub-agents#supported-frontmatter-fields)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | No                |
| `Glob`                 | Encuentra archivos basándose en coincidencia de patrones. Ausente de forma predeterminada en macOS, Linux y WSL. Consulta [Comportamiento de la herramienta Glob](#glob-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | No                |
| `Grep`                 | Busca patrones en el contenido de archivos. Ausente de forma predeterminada en macOS, Linux y WSL. Consulta [Comportamiento de la herramienta Grep](#grep-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | No                |
| `ListAgents`           | Lista los agentes con los que Claude puede comunicarse con `SendMessage`: subagentes en la sesión, compañeros de [equipo de agentes](/docs/es/agent-teams), tus otras sesiones locales de Claude Code, y, mientras esta sesión esté conectada a [Remote Control](/docs/es/remote-control), tus sesiones de [Claude Code en la web](/docs/es/claude-code-on-the-web) y tus sesiones de Remote Control en otras máquinas. Respalda el comando `/list-agents`. Consulta [mensajería entre sesiones](/docs/es/cross-session-messaging). Requiere Claude Code v2.1.224 o posterior, y aparece solo en sesiones donde [la mensajería entre sesiones está habilitada](/docs/es/cross-session-messaging#availability). Las filas de compañeros de equipo y la primera línea que muestra el nombre de esta sesión requieren v2.1.239 o posterior                                                                                                                                                                                                                                                                | No                |
| `ListMcpResourcesTool` | Lista los recursos expuestos por los [servidores MCP](/docs/es/mcp) conectados                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | No                |
| `LSP`                  | Inteligencia de código a través de servidores de lenguaje: ir a definiciones, encontrar referencias, reportar errores de tipo y advertencias. Consulta [Comportamiento de la herramienta LSP](#lsp-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | No                |
| `Monitor`              | Ejecuta un comando en segundo plano y devuelve cada línea de salida a Claude, para que pueda reaccionar a entradas de registro, cambios de archivos o estado sondeado a mitad de la conversación. También puede abrir un WebSocket y tratar cada mensaje entrante como un evento. Consulta [Herramienta Monitor](#monitor-tool)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Sí                |
| `NotebookEdit`         | Modifica celdas de cuadernos Jupyter. Consulta [Comportamiento de la herramienta NotebookEdit](#notebookedit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Sí                |
| `PowerShell`           | Ejecuta comandos de PowerShell de forma nativa. Consulta [Herramienta PowerShell](#powershell-tool) para disponibilidad                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Sí                |
| `PushNotification`     | Envía una notificación de escritorio, y un push telefónico cuando [Remote Control](/docs/es/remote-control) está conectado, para que una tarea de larga duración o [tarea programada](/docs/es/scheduled-tasks) pueda alcanzarte cuando te alejes. La entrega de push se ejecuta a través de infraestructura alojada por Anthropic, que no es accesible desde Amazon Bedrock, Claude Platform en AWS, Agent Platform de Google Cloud, o Microsoft Foundry                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | No                |
| `Read`                 | Lee el contenido de archivos. Consulta [Comportamiento de la herramienta Read](#read-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | No                |
| `ReadMcpResourceTool`  | Lee un recurso MCP específico por URI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | No                |
| `RemoteTrigger`        | Crea, actualiza, ejecuta y lista [Routines](/docs/es/routines) en claude.ai. Respalda el comando `/schedule`. La [referencia de entrada de `RemoteTrigger`](/docs/es/agent-sdk/typescript#remotetrigger) documenta cada acción y las políticas de organización que eliminan la herramienta. Las Routines viven en claude.ai y requieren un plan Pro, Max, Team o Enterprise, por lo que esta herramienta no es accesible desde Amazon Bedrock, Claude Platform en AWS, Agent Platform de Google Cloud, o Microsoft Foundry                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | No                |
| `ReportFindings`       | Reporta hallazgos de revisión de código como una lista estructurada, con un archivo, resumen y escenario de fallo por hallazgo, para que Claude Code pueda renderizarlos en lugar de imprimirlos como texto. Claude lo llama cuando las instrucciones activas de revisión de código le indican que lo haga. Requiere Claude Code v2.1.196 o posterior. A partir de v2.1.199, un hallazgo también puede llevar un slug `category` opcional, como `correctness` o `test-coverage`, mostrado junto a la ubicación del archivo en la lista renderizada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | No                |
| `ScheduleWakeup`       | Reprograma la siguiente iteración de un [`/loop` autónomo](/docs/es/scheduled-tasks#let-claude-choose-the-interval). Claude lo llama al final de cada iteración para elegir cuándo se ejecuta la siguiente, entre uno y sesenta minutos; no lo llamas directamente. Para terminar el bucle en su lugar, Claude lo llama con `stop: true`, que cancela el wakeup pendiente. El campo `stop` requiere Claude Code v2.1.202 o posterior. El wakeup pendiente aparece en `session_crons` en [Entrada de Stop hook](/docs/es/hooks#stop-input)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | No                |
| `SendFeedback`         | Redacta un informe de retroalimentación sobre Claude Code, cubriendo un problema del producto o el comportamiento de Claude en la sesión, y lo pone en cola en tu máquina para que lo revises. Claude Code no envía nada hasta que elijas enviar el borrador. Consulta [Comportamiento de la herramienta SendFeedback](#sendfeedback-tool-behavior). Requiere Claude Code v2.1.238 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | No                |
| `SendMessage`          | Envía un mensaje a otro agente: un compañero de [equipo de agentes](/docs/es/agent-teams), un [subagente que reanuda](/docs/es/sub-agents#resume-subagents) por ID o nombre de agente, o una de tus otras sesiones de Claude Code, en esta máquina o más allá. La mensajería de otras sesiones requiere Claude Code v2.1.224 o posterior. [Mensajería entre sesiones](/docs/es/cross-session-messaging) cubre qué sesiones puede alcanzar Claude, [cómo se ve un mensaje cuando llega](/docs/es/cross-session-messaging#what-a-message-looks-like), y [cómo Claude recibe un aviso cuando otra sesión se queda inactiva](/docs/es/cross-session-messaging#get-a-notice-when-another-session-goes-idle). Claude puede incluir una entrada `summary` opcional, típicamente de 5-10 palabras, que Claude Code muestra como una vista previa de una línea. Cuando Claude la omite en un [mensaje de texto plano](/docs/es/cross-session-messaging#limitations), Claude Code usa la primera línea del mensaje como resumen. Claude Code trunca un resumen más largo que 200 caracteres con puntos suspensivos    | No                |
| `SendUserFile`         | Envía archivos de la sesión a ti con un título opcional, para que un informe generado, diagrama, captura de pantalla o artefacto construido llegue a tu dispositivo en lugar de solo ser mencionado en la transcripción. A partir de v2.1.196, la entrada `display` opcional controla la presentación: `render` abre el archivo en línea en el cliente, `attach` muestra solo una tarjeta de descarga, y cuando no está establecida el cliente decide por tipo de archivo. Disponible cuando un cliente [Remote Control](/docs/es/remote-control) está conectado o la sesión se ejecuta en un entorno en la nube administrado como [Claude Code en la web](/docs/es/claude-code-on-the-web). La entrega se ejecuta a través de infraestructura alojada por Anthropic, por lo que la herramienta no está disponible en Amazon Bedrock, Agent Platform de Google Cloud, o Microsoft Foundry                                                                                                                                                                                               | No                |
| `ShareOnboardingGuide` | Carga `ONBOARDING.md` y devuelve un enlace de uso compartido que los compañeros de equipo pueden abrir en Claude Code. Llamado desde `/team-onboarding` después de que se escribe la guía. Disponible para suscriptores de claude.ai en los planes Pro, Max, Team y Enterprise                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Sí                |
| `Skill`                | Ejecuta una [skill](/docs/es/skills#control-who-invokes-a-skill) dentro de la conversación principal                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Sí                |
| `SubagentHandback`     | Entrega el informe final de un subagente a cualquier conversación que reciba el resultado de ese subagente. Proporcionado solo en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), a subagentes que la herramienta Agent ejecuta localmente que no sean [bifurcaciones](/docs/es/sub-agents#fork-the-current-conversation), y disponible en la CLI de terminal, extensiones de IDE, sesiones en la nube y el Agent SDK; el clasificador revisa el informe antes de que se entregue. Requiere Claude Code v2.1.271 o posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | No                |
| `TaskCreate`           | Crea una nueva tarea en la lista de tareas. Proporcionado de forma predeterminada solo en los modelos enumerados en [Disponibilidad de la herramienta Task](#task-tool-availability), y en otros modelos cuando optas por participar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | No                |
| `TaskGet`              | Recupera detalles completos para una tarea específica. Proporcionado de forma predeterminada solo en los modelos enumerados en [Disponibilidad de la herramienta Task](#task-tool-availability), y en otros modelos cuando optas por participar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | No                |
| `TaskList`             | Lista todas las tareas con su estado actual. Proporcionado de forma predeterminada solo en los modelos enumerados en [Disponibilidad de la herramienta Task](#task-tool-availability), y en otros modelos cuando optas por participar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | No                |
| `TaskOutput`           | Recupera la salida de una tarea en segundo plano. Deprecado a favor de `Read` en la ruta del archivo de salida de la tarea. Cuando ninguna tarea coincide con el ID, el error lista los agentes en segundo plano en ejecución por ID y descripción. Antes de v2.1.203, el error solo nombraba el ID faltante                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | No                |
| `TaskStop`             | Detiene una tarea en segundo plano en ejecución por ID. También acepta un compañero de [equipo de agentes](/docs/es/agent-teams) o un agente en segundo plano nombrado por ID o nombre de agente. Antes de v2.1.198, solo aceptaba un ID de tarea en segundo plano. Cuando ninguna tarea coincide con el ID, el error lista los agentes en segundo plano en ejecución por ID y descripción, incluidos los agentes que otro agente generó. Antes de v2.1.203, el error listaba compañeros de equipo en ejecución y agentes nombrados pero no agentes en segundo plano que otro agente generó, por lo que no podían ser identificados o detenidos desde la conversación principal                                                                                                                                                                                                                                                                                                                                                                                                    | No                |
| `TaskUpdate`           | Actualiza el estado de la tarea, dependencias, detalles, o elimina tareas. Proporcionado de forma predeterminada solo en los modelos enumerados en [Disponibilidad de la herramienta Task](#task-tool-availability), y en otros modelos cuando optas por participar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | No                |
| `TodoWrite`            | Gestiona la lista de verificación de tareas de la sesión. Deshabilitado de forma predeterminada a favor de `TaskCreate`, `TaskGet`, `TaskList` y `TaskUpdate`. Establece `CLAUDE_CODE_ENABLE_TASKS=0` para habilitarlo nuevamente en [sesiones que tienen las herramientas de seguimiento de tareas](#task-tool-availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | No                |
| `ToolSearch`           | Busca y carga herramientas diferidas cuando [búsqueda de herramientas](/docs/es/mcp#scale-with-mcp-tool-search) está habilitada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | No                |
| `WaitForMcpServers`    | Espera a uno o más [servidores MCP](/docs/es/mcp) que aún se están conectando en segundo plano, para que una solicitud pueda usar sus herramientas sin reiniciar la sesión. Claude lo llama cuando un servidor necesario aún no está conectado. Solo aparece cuando [búsqueda de herramientas](/docs/es/mcp#scale-with-mcp-tool-search) está deshabilitada, ya que `ToolSearch` maneja la espera cuando está habilitada                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | No                |
| `WebFetch`             | Obtiene contenido de una URL especificada. Consulta [Comportamiento de la herramienta WebFetch](#webfetch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Sí                |
| `WebSearch`            | Realiza búsquedas web. Consulta [Comportamiento de la herramienta WebSearch](#websearch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Sí                |
| `Workflow`             | Ejecuta un [flujo de trabajo dinámico](/docs/es/workflows): un script que orquesta muchos subagentes en segundo plano y devuelve un resultado consolidado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Sí                |
| `Write`                | Crea o sobrescribe archivos. Consulta [Comportamiento de la herramienta Write](#write-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Sí                |

<h2 id="configure-tools-with-permission-rules-and-hooks">
  Configurar herramientas con reglas de permisos y hooks
</h2>

En la mayoría de los casos, Claude decide cuándo usar estas herramientas y no es necesario que las nombre usted mismo al interactuar con Claude. Usted hace referencia a los nombres de las herramientas directamente al definir permisos y otras configuraciones:

* en [`permissions.allow`](/docs/es/settings-reference#permissions-allow) y [`permissions.deny`](/docs/es/settings-reference#permissions-deny) en la configuración, y la interfaz `/permissions`
* en los indicadores CLI [`--allowedTools` y `--disallowedTools`](/docs/es/cli-reference)
* en las opciones [`allowedTools` y `disallowedTools`](/docs/es/agent-sdk/permissions#allow-and-deny-rules) del Agent SDK
* en el frontmatter [`allowed-tools`](/docs/es/skills#frontmatter-reference) de una skill
* en la condición [`if`](/docs/es/hooks-guide#filter-by-tool-name-and-arguments-with-the-if-field) de un hook

Todos estos aceptan el mismo formato de regla, `ToolName(specifier)`. El especificador depende de la herramienta, y varias herramientas comparten un formato:

| Formato de regla               | Se aplica a               | Detalles                                                                             |
| :----------------------------- | :------------------------ | :----------------------------------------------------------------------------------- |
| `Bash(npm run *)`              | Bash, Monitor             | [Coincidencia de patrón de comando](/docs/es/permissions#bash)                            |
| `PowerShell(Get-ChildItem *)`  | PowerShell                | [Coincidencia de patrón de comando](/docs/es/permissions#powershell)                      |
| `Read(~/secrets/**)`           | Read, Grep, Glob, LSP     | [Coincidencia de patrón de ruta](/docs/es/permissions#read-and-edit)                      |
| `Edit(/src/**)`                | Edit, Write, NotebookEdit | [Coincidencia de patrón de ruta](/docs/es/permissions#read-and-edit)                      |
| `Skill(deploy *)`              | Skill                     | [Coincidencia de nombre de skill](/docs/es/skills#restrict-claude%E2%80%99s-skill-access) |
| `Agent(Explore)`               | Agent                     | [Coincidencia de tipo de subagente](/docs/es/permissions#agent-subagents)                 |
| `WebFetch(domain:example.com)` | WebFetch                  | [Coincidencia de dominio](/docs/es/permissions#webfetch)                                  |
| `WebSearch`                    | WebSearch                 | Sin especificador; permitir o denegar la herramienta en su totalidad                 |

Las herramientas no listadas aquí, como `ExitPlanMode` o `ShareOnboardingGuide`, aceptan solo el nombre de la herramienta sin especificador.

Una regla de permiso `Edit(...)` también otorga acceso de lectura a la misma ruta, por lo que no necesita una regla `Read(...)` coincidente. Una regla de denegación `Read(...)` también bloquea las herramientas Edit y Write en la misma ruta, incluida la creación de un archivo nuevo allí, porque ambas herramientas cambian contenido que Claude debe poder leer nuevamente. La verificación de denegación `Read` requiere Claude Code v2.1.208 o posterior en ediciones, y v2.1.228 o posterior en escrituras.

Los campos `matcher` de hooks usan nombres de herramientas sin formato entre paréntesis, no el formato de regla entre paréntesis. Consulte [patrones de coincidencia](/docs/es/hooks#matcher-patterns) para las reglas de coincidencia. Para los nombres de campo que cada herramienta pasa a `tool_input` en hooks, consulte la [referencia de entrada PreToolUse](/docs/es/hooks#pretooluse-input).

<h2 id="agent-tool-behavior">
  Comportamiento de la herramienta Agent
</h2>

La herramienta Agent genera un subagente en una ventana de contexto separada. El subagente trabaja en su tarea de forma autónoma y luego devuelve su resultado a la conversación principal. El principal no ve las llamadas a herramientas intermedias ni las salidas del subagente, solo ese resultado final. Con [agent teams](/docs/es/agent-teams) habilitados, una llamada que lleve un `name` puede lanzar un [teammate](/docs/es/agent-teams#how-claude-starts-agent-teams) en su lugar, que informa a través de mensajes de equipo en lugar de devolver un resultado.

Para limitar cuántos turnos ejecuta un subagente, establezca `maxTurns` en la [definición del subagente](/docs/es/sub-agents#supported-frontmatter-fields). Cuando el subagente alcanza el límite, Claude Code marca el resultado devuelto como salida parcial, y Claude puede [reanudar el subagente](/docs/es/sub-agents#resume-subagents) para continuar.

La misma herramienta Agent también lanza [subagentes bifurcados](/docs/es/sub-agents#fork-the-current-conversation) dondequiera que [fork mode](/docs/es/sub-agents#turn-fork-mode-on-or-off) esté activado. Una bifurcación hereda la conversación principal completa en lugar de comenzar desde cero, se ejecuta en segundo plano aparte de los [casos que permanecen en primer plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background), y aún muestra solicitudes de permiso en su terminal. El resto de esta sección describe subagentes que no son bifurcaciones.

Las herramientas que un subagente que no es bifurcación puede usar dependen de los campos `tools` y `disallowedTools` en la [definición del subagente](/docs/es/sub-agents):

* **Ninguno de los campos establecido**: el subagente hereda todas las [herramientas disponibles para subagentes](/docs/es/sub-agents#available-tools).
* **Solo `tools`**: el subagente obtiene solo las herramientas listadas.
* **Solo `disallowedTools`**: el subagente obtiene todas las herramientas principales excepto las listadas.
* **Ambos establecidos**: `disallowedTools` tiene prioridad. Una herramienta listada en ambos se elimina.

En todos los casos, el conjunto resuelto se limita a las [herramientas disponibles para subagentes](/docs/es/sub-agents#available-tools): una herramienta que no está disponible para subagentes nunca se otorga, incluso cuando se lista en `tools`. Donde se cumplen las condiciones en la entrada de la tabla de herramientas de `SubagentHandback`, Claude Code también le da al subagente esa herramienta, incluso si la deja fuera de `tools` o la lista en `disallowedTools`.

Si cada entrada en la lista `tools` de un subagente no coincide con una herramienta utilizable, la herramienta Agent generalmente devuelve un error nombrando las entradas en lugar de lanzar el subagente; consulte [Agent would be spawned with zero tools](/docs/es/errors#agent-would-be-spawned-with-zero-tools) para el mensaje y cómo corregir cada entrada.

Lanzar el subagente no solicita permiso en sí mismo. Claude Code verifica las llamadas a herramientas propias del subagente contra sus reglas de permiso mientras se ejecuta.

Dónde ve los solicitudes de permiso de un subagente depende de si se ejecuta en primer plano o en segundo plano. Claude Code ejecuta subagentes en segundo plano de forma predeterminada, aparte de los [casos que se ejecutan en primer plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background).

* **Subagentes en primer plano** muestran los mismos solicitudes de permiso que vería en la conversación principal, en el momento en que ocurre cada llamada a herramienta.
* **Subagentes en segundo plano** muestran solicitudes de permiso en su sesión principal a partir de v2.1.186. El solicitud nombra qué subagente está pidiendo, y presionar Esc deniega esa llamada a herramienta sin detener el subagente. Antes de v2.1.186, los subagentes en segundo plano denegaban automáticamente cualquier llamada a herramienta que de otro modo solicitaría y continuaban sin esa herramienta.

Para [limitar lo que un subagente puede alcanzar](/docs/es/sub-agents#control-subagent-capabilities) en primer lugar, reduzca su campo `tools`, por ejemplo dejando Bash fuera de la lista, o establezca reglas de denegación en su configuración.

<h2 id="askuserquestion-tool-behavior">
  Comportamiento de la herramienta AskUserQuestion
</h2>

Claude utiliza `AskUserQuestion` para hacerle preguntas de opción múltiple cuando necesita una decisión o una aclaración. Responda seleccionando una opción, o escriba su propio texto a través de la fila `Other` o el campo de notas.

Cuando responde escribiendo su propio texto, Claude Code retransmite la respuesta con una redacción neutral para que Claude siga lo que escribió, incluida una solicitud para esperar o explicar primero.

<h3 id="question-auto-continue-timeout">
  Tiempo de espera de continuación automática de preguntas
</h3>

Las preguntas permanecen abiertas hasta que las responda. Si desea que una pregunta que deja sin responder se cierre eventualmente y permita que Claude continúe sin usted, establezca la configuración [`askUserQuestionTimeout`](/docs/es/settings-reference#askuserquestiontimeout) en `60s`, `5m`, o `10m`, ya sea en su `settings.json` de usuario o desde la fila **Question auto-continue timeout** en `/config`.

Después de que una pregunta permanezca tanto tiempo sin entrada, el diálogo se cierra automáticamente: envía cualquier opción que ya haya seleccionado y le dice a Claude que es posible que esté alejado de su teclado, por lo que Claude procede según su propio criterio y puede volver a preguntar más tarde. Verá una cuenta regresiva para los últimos 20 segundos. Presione cualquier tecla para reiniciar el temporizador; en terminales que informan el enfoque, cambiar a la ventana también lo reinicia.

El tiempo de espera se aplica solo a las preguntas de opción múltiple de `AskUserQuestion`; los avisos de permiso, incluida la aprobación del plan, nunca se resuelven automáticamente en inactividad.

<h2 id="bash-tool-behavior">
  Comportamiento de la herramienta Bash
</h2>

La herramienta Bash ejecuta cada comando en un proceso separado.

<h3 id="what-persists-between-commands">
  Qué persiste entre comandos
</h3>

* Cuando Claude ejecuta `cd` en la sesión principal, el nuevo directorio de trabajo se mantiene en comandos Bash posteriores siempre que permanezca dentro del directorio del proyecto o un [directorio de trabajo adicional](/docs/es/permissions#working-directories) que agregó con `--add-dir`, `/add-dir`, o `additionalDirectories` en la configuración. Esto incluye comandos que Claude ejecuta en respuesta a sus mensajes posteriores.
  * Las sesiones de subagentes nunca mantienen cambios de directorio de trabajo.
  * Si `cd` sale de esos directorios, Claude Code se reinicia al directorio del proyecto y añade `Shell cwd was reset to <dir>` al resultado de la herramienta.
  * Para desactivar este mantenimiento de modo que cada comando Bash comience en el directorio del proyecto, establezca `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR=1`.
* Las variables de entorno no persisten. Un `export` en un comando no estará disponible en el siguiente.
* Los alias y funciones de shell definidos en su archivo de inicio de shell están disponibles. Al iniciar la sesión, Claude Code obtiene `~/.zshrc`, `~/.bashrc`, o `~/.profile` según su shell, captura los alias, funciones y opciones de shell resultantes, y los aplica a cada comando Bash.

Active su virtualenv o entorno conda antes de lanzar Claude Code. Para hacer que las variables de entorno persistan entre comandos Bash, establezca [`CLAUDE_ENV_FILE`](/docs/es/env-vars) en un script de shell antes de lanzar Claude Code, o use un [hook SessionStart](/docs/es/hooks#persist-environment-variables) para poblarlo dinámicamente.

<h3 id="timeout-and-output-limits">
  Límites de tiempo de espera y salida
</h3>

Cada comando se ejecuta bajo un tiempo de espera, y Claude lo gestiona: cuando necesita más tiempo que el predeterminado para un comando, pasa el parámetro `timeout` con esa llamada — usted nunca establece un tiempo de espera por comando. Dos [variables de entorno](/docs/es/env-vars) limitan lo que Claude obtiene:

* `BASH_DEFAULT_TIMEOUT_MS` — el predeterminado cuando Claude no pasa tiempo de espera; dos minutos de forma predeterminada
* `BASH_MAX_TIMEOUT_MS` — con el predeterminado, establece el límite máximo que limita lo que Claude solicita: el límite máximo efectivo es el mayor de los dos, diez minutos de forma predeterminada

<h4 id="output-limits">
  Límites de salida
</h4>

Claude Code transmite la salida de un comando a un archivo de trabajo mientras se ejecuta el comando; un comando cuya salida supera 5 GB se detiene. Cuando el comando finaliza, Claude Code lee la salida nuevamente desde ese archivo, hasta la ventana de lectura descrita a continuación. La cantidad de salida que llega a Claude en línea depende de si Claude Code trata el resultado como un fallo:

| Resultado | Lo que Claude obtiene                                                                                                                                                                                                                                                                                     |
| :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Válido    | En línea hasta aproximadamente 30.000 caracteres de forma predeterminada; más allá de eso, la ruta de un archivo guardado en el directorio de sesión y truncado más allá de 64 MiB, más una vista previa de hasta los primeros 2.000 caracteres, y Claude lee o busca el archivo cuando necesita el resto |
| Fallo     | En línea hasta aproximadamente 10.000 caracteres; más allá de eso, un extracto de cabeza y cola de ese tamaño cortado de la ventana de lectura, sin ruta de archivo                                                                                                                                       |

Un comando que sale con código 1 cuenta como un resultado válido para la herramienta Bash solo cuando Claude Code reconoce el código de salida 1 como un resultado benigno para ese comando: `grep`, `rg`, `egrep`, `fgrep`, `find`, `diff`, `test`, y `[`, más `git diff` y `git grep`. Todos los demás comandos que salen con código 1 cuentan como un fallo, incluso cuando el código de salida 1 es un resultado informativo benigno: sin coincidencias para `pgrep` y `jq -e`, archivos que difieren para `cmp`.

[`BASH_MAX_OUTPUT_LENGTH`](/docs/es/env-vars) establece cuántos caracteres de salida Claude Code lee desde el archivo de trabajo hacia el resultado de un comando: 30.000 de forma predeterminada, hasta un límite máximo fijo de 150.000. Aumente esto cuando sus comandos se desborden rutinariamente de esa ventana, como una compilación detallada o un registro de suite de pruebas completo. Aumentarlo amplía la ventana de lectura, que también es la ventana desde la cual se corta el extracto de un comando fallido. No aumenta los límites en línea: un resultado válido sobre el límite en línea llega como una ruta de archivo más vista previa independientemente de esta variable.

Para cambiar cuánta salida válida recibe Claude en línea, establezca la configuración [`bashOutputMaxChars`](/docs/es/settings-reference#bashoutputmaxchars) en su lugar, hasta 128.000 caracteres. Dimensiona el límite en línea y la ventana de lectura juntos, y Claude Code ignora `BASH_MAX_OUTPUT_LENGTH`. Requiere Claude Code v2.1.261 o posterior.

<h3 id="background-commands">
  Comandos en segundo plano
</h3>

Para procesos de larga duración como servidores de desarrollo o compilaciones de vigilancia, Claude puede establecer `run_in_background: true` para iniciar el comando como una tarea en segundo plano y continuar trabajando mientras se ejecuta. Liste y detenga tareas en segundo plano con `/tasks`. Después de detener una allí, o desde un cliente conectado como la aplicación de escritorio, Claude continúa en lugar de esperar. Si un subagente inició el comando, es ese subagente el que continúa.

Un comando que un [subagente en primer plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background) inició se detiene cuando ese subagente da su respuesta final. Un comando que la conversación principal o un subagente en segundo plano inició sigue ejecutándose después de una respuesta final. En modo no interactivo con la bandera `-p`, [los comandos en segundo plano terminan poco después del resultado final de la ejecución](/docs/es/headless#background-tasks-at-exit).

Cuando un comando alcanza su tiempo de espera sin terminar, Claude Code lo mueve al segundo plano en lugar de detenerlo, a menos que el comando comience con `sleep`. Claude continúa trabajando mientras el comando sigue ejecutándose. Claude Code aplica las mismas reglas de vida útil a un comando movido que a cualquier otro comando en segundo plano, por lo que aún detiene el comando de un subagente en primer plano en la respuesta final de ese subagente. Establecer [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS=1`](/docs/es/env-vars#variables) desactiva el segundo plano automático junto con el resto de la funcionalidad de tareas en segundo plano.

El resultado de un comando movido al segundo plano indica lo que sucedió:

* Cuando el tiempo de espera activa el movimiento, el resultado lo reporta explícitamente: `Command did not complete within its 120s timeout and was moved to the background`, con los segundos coincidiendo con el tiempo de espera que se aplicó, seguido del ID de tarea y la ruta del archivo en el que se escribe la salida.
* Un `cd`, `pushd`, `popd`, o `chdir` dentro de un comando que se mueve al segundo plano nunca se mantiene: el resultado indica `Session cwd remains <dir>; directory changes made by the backgrounded command do not apply to subsequent commands.`, por lo que Claude no actúa sobre un cambio de directorio que no sucedió.

<h3 id="memory-limit-on-linux-and-wsl">
  Límite de memoria en Linux y WSL
</h3>

En Linux y WSL, establezca [`CLAUDE_CODE_TOOL_MEMORY_LIMIT`](/docs/es/env-vars#variables) en un tamaño como `4G` para limitar la memoria que los comandos de herramientas Bash, PowerShell y [Monitor](#monitor-tool) pueden usar, de modo que una compilación descontrolada no consuma la memoria que el resto de la sesión necesita. Requiere Claude Code v2.1.233 o posterior. Antes de v2.1.246, los comandos de herramientas Monitor se ejecutaban fuera del límite.

* Escriba el tamaño como un número de bytes o con un sufijo `K`, `M`, `G`, o `T`. Establezca `0`, `off`, `false`, `no`, o `none` para desactivar el límite. Claude Code ignora cualquier otro valor que no pueda leer como un tamaño, como `4e9`.
* Claude Code cuenta todos los comandos Bash, PowerShell y Monitor de una sesión contra el límite único, no cada comando por su cuenta.
* Claude Code aplica el límite con un cgroup de memoria. Cuando no puede configurar el cgroup, los comandos se ejecutan sin límite, y el registro de depuración de `claude --debug` indica por qué.
* Después de que el primer proceso que Claude Code inicia ha activado el límite, o lo ha desactivado debido a un valor desactivado o una configuración de cgroup fallida, Claude Code mantiene ese resultado hasta que reinicie. Para aplicar un valor cambiado o eliminado, o una configuración corregida, lance `claude` nuevamente.
* Cuando los comandos no pueden mantenerse bajo el límite, el kernel detiene un comando, y nada en su resultado nombra el límite.

Claude Code también puede contar otros tipos de procesos que inicia contra el mismo límite. Establezca [`CLAUDE_CODE_TOOL_MEMORY_CGROUP_EXCLUDE`](/docs/es/env-vars#variables) en una lista separada por comas de los tipos a eximir del límite; Claude Code aplica el límite a cada tipo que no esté en su lista. Establézcalo en `none` para limitar cada tipo, o en `all-new` para limitar solo comandos de herramientas Bash, PowerShell y Monitor. Requiere Claude Code v2.1.246 o posterior. Los tipos que puede nombrar:

* `mcp`: [servidores MCP](/docs/es/mcp) locales
* `lsp`: [servidores de lenguaje](#lsp-tool-behavior)
* `hooks`: comandos de [hook](/docs/es/hooks)
* `plugin`: comandos que ejecutan [plugins](/docs/es/plugins/overview)
* `helper`: comandos auxiliares propios de Claude Code, como `git`
* `agent`: procesos secundarios de Claude Code, como [compañeros agentes](/docs/es/agent-teams)

Sea lo que sea que liste, estas reglas se aplican:

* **Nombres desconocidos**: Claude Code ignora nombres que no reconoce
* **Bash, PowerShell y Monitor**: Claude Code mantiene comandos de herramientas Bash, PowerShell y Monitor bajo el límite sea lo que sea que liste
* **Variable no establecida**: Claude Code toma el conjunto de otros tipos limitados de la configuración que Anthropic entrega desde el servidor, y ese conjunto puede cambiar con el tiempo, por lo que establezca la variable cuando necesite un conjunto que no cambie
* **Hooks de control de permisos**: incluso con cada tipo limitado, Claude Code excluye del límite un hook que puede bloquear o cambiar el resultado de una acción, y cualquier servidor MCP que tal hook llame, por lo que el kernel matando un hook de control de permisos no puede permitir la acción que estaba bloqueando

<h2 id="edit-tool-behavior">
  Comportamiento de la herramienta Edit
</h2>

La herramienta Edit realiza reemplazo exacto de cadenas. Toma un `old_string` y un `new_string` y reemplaza el primero con el segundo. No utiliza expresiones regulares ni coincidencia aproximada.

Deben pasar tres comprobaciones para que se aplique una edición. Antes de cualquiera de ellas, se rechaza una ruta coincidente con una [regla de denegación `Read`](/docs/es/permissions#tool-specific-permission-rules), incluida la creación de un archivo nuevo allí. El rechazo requiere Claude Code v2.1.208 o posterior.

* **Read-before-edit**: Claude lee el archivo en la conversación actual antes de editarlo, y una lectura interrumpida con un aviso [`PARTIAL view`](#read-tool-behavior) no cuenta. Claude Opus 4.6, Claude Haiku 4.5 y modelos más antiguos siempre requieren la lectura. Los modelos más nuevos pueden editar un archivo no leído cuando leerlo no requeriría un aviso de permiso y la herramienta Read está disponible.
* **Match**: `old_string` debe aparecer en el archivo exactamente como está escrito. Una sola diferencia de carácter de espacio en blanco o indentación es suficiente para no coincidir.
* **Uniqueness**: `old_string` debe aparecer exactamente una vez. Cuando aparece más de una vez, Claude proporciona una cadena más larga con suficiente contexto circundante para identificar una ocurrencia, o establece `replace_all: true` para reemplazarlas todas.

Un archivo que cambió en el disco después de que Claude lo leyó por última vez aún puede editarse cuando `old_string` coincide exactamente con el contenido actual de forma inequívoca y Claude Code puede leer el archivo sin solicitar permiso. La coincidencia con el contenido actual del archivo mantiene esto seguro, y el resultado indica que el archivo contiene otros cambios para que Claude lo vuelva a leer antes de ediciones que dependan del contenido circundante. En cualquier otro caso, como un `old_string` obsoleto o uno que coincida más de una vez sin `replace_all`, Claude lee el archivo nuevamente antes de editar. El manejo relajado de archivos no leídos y modificados requiere Claude Code v2.1.208 o posterior; antes de eso, Claude Code rechazaba cualquier edición a un archivo que no hubiera leído en la conversación o que hubiera cambiado en el disco después de la lectura.

Ver un archivo con Bash también satisface el requisito de read-before-edit cuando el comando es `cat`, `nl`, `bat`, `batcat`, `head`, `tail`, `sed -n 'X,Yp'`, `grep`, `egrep`, `fgrep`, o `rg` en un único archivo sin tuberías o redirecciones. La salida canalizada y otros comandos Bash no cuentan hacia la comprobación de read-before-edit.

Ver un archivo con Bash afecta solo la elegibilidad de edición, no los permisos. Consulte [Reglas de permisos Read y Edit](/docs/es/permissions#read-and-edit) para saber qué comandos Bash cubren sus reglas de denegación `Read` y `Edit`.

<h2 id="endconversation-tool-behavior">
  Comportamiento de la herramienta EndConversation
</h2>

La herramienta EndConversation finaliza la sesión actual. Claude la utiliza solo en dos situaciones:

* como último recurso contra entrada abusiva sostenida, después de intentos fallidos de redirigir la conversación y después de una advertencia clara en un mensaje anterior
* cuando usted solicita explícitamente ver la herramienta demostrada y confirma que desea finalizar la sesión

La frustración general, las palabras malsonantes o una tarea que va mal no califican, ni tampoco las solicitudes de contenido dañino, que Claude rechaza en lugar de finalizar la sesión. Claude Code sigue el mismo enfoque que claude.ai, que puede [finalizar un subconjunto raro de chats](https://www.anthropic.com/research/end-subset-conversations).

Después de que Claude finaliza una sesión interactiva, la sesión se bloquea. Los nuevos mensajes y la mayoría de los comandos devuelven `Claude ended this conversation. Start a new session (or /clear) to continue.`, y solo `/clear`, `/resume`, `/help`, `/exit` y `/feedback` siguen ejecutándose. Claude Code registra el final en la transcripción de la sesión, por lo que reanudar una sesión finalizada restaura el bloqueo; el historial de la sesión no se elimina.

Reanudar una sesión finalizada en [modo no interactivo](/docs/es/headless) con la bandera `-p` genera un error y sale con código 1, por lo que un script no lee la ejecución finalizada como un éxito.

La herramienta nunca solicita permiso, y los [hooks PreToolUse](/docs/es/hooks#pretooluse) no se ejecutan para ella. Mientras que cualquier otra herramienta permanezca, tampoco puede bloquearla: las [reglas de denegación y solicitud](/docs/es/permissions#tool-specific-permission-rules) que nombran `EndConversation` no tienen efecto, y ni `--disallowedTools` ni una lista `--tools` pueden eliminarla. La exención es deliberada: la herramienta no hace nada excepto finalizar la conversación, nunca lee ni modifica archivos o datos, y una salvaguarda de este tipo se mantiene solo si la sesión a la que se aplica no puede desactivarla. Cuando sus reglas de denegación eliminan todas las demás herramientas y también coinciden con `EndConversation`, como lo hace `"*"`, Claude Code también la elimina en lugar de dejarla como la única herramienta, a menos que una regla de permiso nombre `EndConversation` explícitamente. Una lista de denegación que elimina todas las demás herramientas sin coincidir con `EndConversation` la deja en su lugar.

Los [subagentes](/docs/es/sub-agents) nunca obtienen la herramienta. Las tareas en segundo plano que comparten la lista de herramientas de la conversación principal la ven, pero llamarla allí no finaliza nada.

La herramienta aparece solo cuando se cumplen todas las siguientes condiciones:

* **Versión**: Claude Code v2.1.213 o posterior.
* **Modelo**: el modelo de la sesión es Claude Opus 4.8, Claude Sonnet 5, Claude Fable 5, o una versión posterior de una de esas familias.
* **Superficie**: una sesión de terminal interactiva, incluida una sesión `claude` en el terminal integrado de un IDE, que es cómo lo ejecuta el [plugin de JetBrains](/docs/es/jetbrains). Otras superficies no incluyen la herramienta, tales como:
  * ejecuciones no interactivas `-p`
  * sesiones a través de los paquetes TypeScript y Python del [Agent SDK](/docs/es/agent-sdk/overview)
  * el panel de la [extensión de VS Code](/docs/es/vs-code), que incluye su propio CLI
  * [GitHub Actions](/docs/es/github-actions)
  * [Claude Code en la web](/docs/es/claude-code-on-the-web)
* **Modo de inicio**: no una sesión [`--bare`](/docs/es/headless#start-faster-with-bare-mode). El modo bare carga solo herramientas de shell y archivo, por lo que la herramienta nunca se registra allí.
* **Proveedor**: no disponible en [Amazon Bedrock](/docs/es/amazon-bedrock), [Claude Platform en AWS](/docs/es/claude-platform-on-aws), [Agent Platform de Google Cloud](/docs/es/google-vertex-ai), o [Microsoft Foundry](/docs/es/microsoft-foundry), o en sesiones que iniciaron sesión a través de una [puerta de enlace en la nube](/docs/es/claude-apps-gateway).

<h2 id="glob-tool-behavior">
  Comportamiento de la herramienta Glob
</h2>

La herramienta Glob encuentra archivos por patrón de nombre. En Windows, es parte del conjunto de herramientas predeterminado. En macOS, Linux y WSL, Claude Code deja Glob y [Grep](#grep-tool-behavior) fuera del conjunto de herramientas predeterminado, y Claude busca con `find` y `grep` a través de la herramienta Bash en su lugar. En el shell de Claude esos dos comandos ejecutan versiones integradas de `bfs` y `ugrep`, y las búsquedas llegan a sus hooks y reglas de permisos como llamadas `Bash`.

En macOS, Linux y WSL, obtiene las herramientas Glob y Grep de vuelta en estos casos:

* Nombra `Glob` o `Grep` en [`--tools` o `--allowedTools`](/docs/es/cli-reference#cli-flags) cuando inicia la sesión, o en las opciones equivalentes de [Agent SDK](/docs/es/agent-sdk/overview). Con `--tools` obtiene los que enumera, y nombrar cualquiera de las herramientas en `--allowedTools` restaura ambas. Una regla de permiso en un archivo de configuración no tiene este efecto.
* Una regla de permisos [deny](/docs/es/permissions#match-all-uses-of-a-tool), la bandera `--disallowedTools`, o [`--restricted`](/docs/es/cli-reference#cli-flags) elimina `Bash` de la sesión.
* Un [subagente](/docs/es/sub-agents#available-tools) enumera `Glob` o `Grep` en su campo `tools` y deja fuera `Bash`. Las herramientas enumeradas vuelven solo para ese subagente, o para toda la sesión cuando se ejecuta como el agente de sesión principal a través de [`--agent`](/docs/es/sub-agents#invoke-subagents-explicitly) o la configuración `agent`.

Glob admite sintaxis glob estándar incluyendo `**` para coincidencia de directorio recursivo:

* `**/*.js` coincide con todos los archivos `.js` a cualquier profundidad
* `src/**/*.ts` coincide con todos los archivos `.ts` bajo `src/`
* `*.{json,yaml}` coincide con archivos `.json` y `.yaml` en el directorio actual

Los resultados se ordenan por tiempo de modificación y se limitan a 100 archivos. Si se alcanza el límite, Claude ve una bandera de truncamiento en el resultado y puede estrechar el patrón.

Glob no respeta `.gitignore` por defecto, por lo que encuentra archivos ignorados por git junto con los rastreados. Esto difiere de [Grep](#grep-tool-behavior), que omite archivos ignorados por git. Para hacer que Glob respete `.gitignore`, establezca `CLAUDE_CODE_GLOB_NO_IGNORE=false` antes de lanzar Claude Code.

Claude Code decide el permiso para una llamada Glob antes de verificar si el directorio de búsqueda existe. Aún ejecuta la verificación de permiso de lectura para una `path` faltante fuera de los [directorios de trabajo](/docs/es/permissions#working-directories), por lo que una solicitud de permiso para una ruta no significa que la ruta exista.

Un valor de `pattern` o `path` que contenga un byte nulo devuelve un error pidiendo a Claude que lo elimine.&#x20;

<h2 id="grep-tool-behavior">
  Comportamiento de la herramienta Grep
</h2>

La herramienta Grep busca patrones en el contenido de archivos. Donde [Glob](#glob-tool-behavior) encuentra archivos por nombre, Grep encuentra líneas dentro de ellos. En macOS, Linux y WSL, Grep está ausente por defecto bajo las mismas condiciones que Glob. Consulte [Comportamiento de la herramienta Glob](#glob-tool-behavior) para saber cuándo ambas herramientas están disponibles.

Grep se basa en [ripgrep](https://github.com/BurntSushi/ripgrep) y utiliza la sintaxis regex de ripgrep, no grep de POSIX. Los patrones que incluyen metacaracteres regex necesitan escape. Por ejemplo, encontrar `interface{}` en código Go requiere el patrón `interface\{\}`.

Un patrón, glob o tipo de archivo que ripgrep rechaza devuelve un error que incluye el diagnóstico de ripgrep, para que Claude pueda corregir la entrada y buscar de nuevo. Antes de v2.1.208, Claude Code reportaba una entrada rechazada como `No files found` en lugar de un error, incluso cuando el texto buscado existía en los archivos de destino.

Tres modos de salida controlan lo que regresa:

* `files_with_matches`: solo rutas de archivo, sin contenido de línea. Este es el predeterminado.
* `content`: líneas coincidentes con número de archivo y línea. Cuando el parámetro `offset` de la herramienta apunta más allá de la última coincidencia para un patrón que tiene coincidencias, Grep devuelve `No entries at this offset`, por lo que Claude amplía o reinicia el offset en lugar de concluir que el patrón no coincide.
* `count`: recuento de coincidencias por archivo, seguido de un total en todos los archivos coincidentes. El total cubre cada coincidencia incluso cuando los parámetros `head_limit` u `offset` de la herramienta truncan las entradas por archivo listadas. Antes de v2.1.208, el total solo sumaba las entradas listadas.

Claude puede limitar resultados por archivo con el parámetro `glob`, como `**/*.tsx`, o por lenguaje con el parámetro `type`, como `py` o `rust`. Por defecto, los patrones coinciden dentro de una sola línea. Claude puede establecer `multiline: true` para coincidir entre límites de línea.

Grep respeta `.gitignore`, por lo que los archivos ignorados por git se omiten. Para buscar un archivo ignorado por git, Claude pasa su ruta directamente.

Claude Code decide el permiso para una llamada Grep antes de verificar si la `path` de búsqueda existe. Aún ejecuta la verificación de permiso de lectura para una `path` faltante fuera de los [directorios de trabajo](/docs/es/permissions#working-directories), por lo que un aviso de permiso para una ruta no significa que la ruta exista.

<h2 id="lsp-tool-behavior">
  Comportamiento de la herramienta LSP
</h2>

La herramienta LSP proporciona a Claude inteligencia de código desde un servidor de lenguaje en ejecución. Después de cada edición de archivo, informa automáticamente de errores de tipo y advertencias para que Claude pueda corregir problemas sin un paso de compilación separado. Claude también puede llamarla directamente para navegar por el código:

* Saltar a la definición de un símbolo
* Encontrar todas las referencias a un símbolo
* Obtener información de tipo en una posición
* Listar símbolos en un archivo
* Buscar un símbolo por nombre en todo el espacio de trabajo
* Encontrar implementaciones de una interfaz
* Rastrear jerarquías de llamadas

Claude Code mantiene la herramienta inactiva hasta que instale un [plugin de inteligencia de código](/docs/es/plugins/code-intelligence) para su lenguaje. En [sesiones en la nube](/docs/es/claude-code-on-the-web), Claude Code no inicia servidores de lenguaje de plugins, por lo que la herramienta LSP permanece inactiva allí. Claude Code toma la configuración del servidor de lenguaje del plugin, y usted instala el binario del servidor por su cuenta.

Claude Code devuelve un resultado de error para cada llamada LSP en un archivo cuyo servidor de lenguaje no puede iniciar.

<h2 id="monitor-tool">
  Herramienta Monitor
</h2>

La herramienta Monitor permite que Claude observe algo en segundo plano y reaccione cuando cambia, sin pausar la conversación. Pida a Claude que:

* Siga un archivo de registro y marque errores a medida que aparecen
* Sondee una PR o trabajo de CI y reporte cuando su estado cambia
* Observe un directorio para cambios de archivos
* Rastrear la salida de cualquier script de larga duración que señale
* Conectarse a una fuente de WebSocket e informar cada mensaje a medida que llega

Para la mayoría de las observaciones, Claude escribe un pequeño script, lo ejecuta en segundo plano, y recibe cada línea de salida a medida que llega. Para un servidor que ya envía eventos, Claude puede abrir una [fuente de WebSocket](#websocket-source) en lugar de ejecutar un script.

Continúa trabajando en la misma sesión y Claude interviene cuando llega un evento.

Cada observación que Claude inicia tiene un plazo: 5 minutos por defecto, como máximo 30 minutos, y como máximo 10 minutos en una ejecución [no interactiva](/docs/es/headless) dada una única solicitud con `-p`.

En el plazo, la observación termina. Claude recibe un aviso, por lo que puede reiniciar la observación si aún es necesaria.

Detenga un monitor pidiendo a Claude que lo cancele o terminando la sesión. Cuando detiene una [subagente](/docs/es/sub-agents) que inició monitores, por ejemplo desde `/tasks`, esos monitores se detienen con ella.

Cuando Monitor ejecuta un comando, utiliza las mismas [reglas de permisos que Bash](/docs/es/permissions#tool-specific-permission-rules), por lo que los patrones `allow` y `deny` que tiene establecidos para Bash se aplican aquí también. Mientras [el modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) está activo, Claude Code reserva reglas de permiso que nombran `Monitor` en sí mismo, junto con las otras [reglas de permiso amplias que descarta](/docs/es/permission-modes#how-the-classifier-evaluates-actions), por lo que el clasificador revisa los comandos de Monitor de la misma manera que revisa los comandos de Bash.

La [fuente de WebSocket](#websocket-source) tiene su propio mensaje de aprobación, que el clasificador también decide en modo automático.

La herramienta no está disponible en Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry. Tampoco está disponible cuando `DISABLE_TELEMETRY` o `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` está establecido.

Los plugins pueden declarar monitores que se inician automáticamente cuando el plugin está activo, en lugar de pedirle a Claude que los inicie. Consulte [monitores de plugins](/docs/es/plugins/components#monitors).

<h3 id="websocket-source">
  Fuente de WebSocket
</h3>

<Note>
  La fuente de WebSocket requiere Claude Code v2.1.195 o posterior.
</Note>

Cuando un servidor ya envía eventos a través de un WebSocket, Claude puede conectarse directamente en lugar de escribir un script de sondeo. Cada tipo de actividad de socket se convierte en un evento o termina la observación:

* **Mensajes de texto**: cada uno se convierte en un evento, incluso cuando el mensaje abarca múltiples líneas.
* **Mensajes binarios**: no se transmiten. Claude recibe una línea de marcador de posición como `[binary frame, 512 bytes]` en su lugar.
* **Mensajes mayores de 1 MiB**: la observación termina, por lo que suscríbase a una fuente filtrada donde exista una.
* **Cierre de socket**: la observación termina y Claude recibe el código de cierre.

Una observación de WebSocket toma una entrada `ws` en lugar de `command`, y una única llamada de Monitor no puede combinar los dos. La entrada `ws` tiene dos campos:

| Campo       | Requerido | Descripción                                                                                                                                                                   |
| :---------- | :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`       | Sí        | El punto final al que conectarse. Debe ser una URL `ws://` o `wss://` sin credenciales incrustadas o espacios en blanco, utilizando solo caracteres ASCII                     |
| `protocols` | No        | Nombres de subprotocolo de WebSocket para ofrecer durante el apretón de manos. Cada entrada debe ser un token de subprotocolo válido, y la lista no puede contener duplicados |

El plazo `timeout_ms` se aplica también a una observación de WebSocket: la observación termina en el plazo, y `TaskStop` la cancela temprano.

Abrir un WebSocket solicita aprobación; en [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) el clasificador decide en su lugar. El mensaje no ofrece una opción para omitir futuros mensajes para el mismo host.

Claude Code rechaza las URL que apuntan a una dirección privada, link-local o de metadatos en la nube, incluidos los nombres de host que se resuelven en una. También rechaza los hosts en `sandbox.network.deniedDomains`, y cuando [`allowManagedDomainsOnly`](/docs/es/settings-reference#sandbox-network-allowmanageddomainsonly) está establecido en la configuración administrada, cualquier host fuera de la lista de permitidos administrada.

<h2 id="notebookedit-tool-behavior">
  Comportamiento de la herramienta NotebookEdit
</h2>

NotebookEdit modifica un cuaderno Jupyter una celda a la vez, dirigiéndose a celdas por su `cell_id`. No realiza reemplazo de cadenas en todo el cuaderno de la manera que [Edit](#edit-tool-behavior) lo hace en archivos simples.

Tres modos de edición controlan lo que sucede con la celda objetivo:

* `replace`: sobrescribe la fuente de la celda. Este es el predeterminado.
* `insert`: agrega una nueva celda después de la objetivo. Sin `cell_id`, la nueva celda va al inicio del cuaderno. Requiere `cell_type` establecido a `code` o `markdown`.
* `delete`: elimina la celda objetivo.

Las reglas de permisos utilizan el formato de ruta `Edit(...)`. Una regla como `Edit(notebooks/**)` cubre llamadas de NotebookEdit en archivos en ese directorio.

<h2 id="powershell-tool">
  Herramienta PowerShell
</h2>

La herramienta PowerShell permite que Claude ejecute comandos de PowerShell de forma nativa. En Windows, esto significa que los comandos se ejecutan en PowerShell en lugar de enrutarse a través de Git Bash. La disponibilidad de la herramienta depende de su plataforma:

* **Windows sin Git Bash**: la herramienta se habilita automáticamente.
* **Windows con Git Bash instalado**: la herramienta está habilitada de forma predeterminada para cuentas de claude.ai y Console; establezca `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` para habilitarla en sesiones de Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry, o `0` para desactivarla.
* **Linux, macOS y WSL**: la herramienta es opcional.

Sus hooks [PreToolUse](/docs/es/hooks#powershell) reciben la cadena de comando de la herramienta en `tool_input.command`, con los mismos campos que la herramienta Bash.

Coincida con `Bash|PowerShell` en hooks que inspeccionen comandos de shell; la [sección de entrada del hook PowerShell](/docs/es/hooks#powershell) explica por qué no es suficiente coincidir solo con `Bash`.

<h3 id="enable-the-powershell-tool">
  Habilitar la herramienta PowerShell
</h3>

Establezca `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` en su entorno o en `settings.json`:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_USE_POWERSHELL_TOOL": "1"
  }
}
```

En Windows, establezca la variable en `0` para desactivar la herramienta. En Linux, macOS y WSL, la herramienta requiere PowerShell 7 o posterior: instale `pwsh` y asegúrese de que esté en su `PATH`.

En Windows, Claude Code detecta automáticamente `pwsh.exe` para PowerShell 7+ con una alternativa a `powershell.exe` para PowerShell 5.1. Cuando la herramienta está habilitada, Claude trata PowerShell como el shell principal. La herramienta Bash sigue estando disponible para scripts POSIX cuando Git Bash está instalado.

Claude Code inicia PowerShell con `-ExecutionPolicy Bypass` solo en el ámbito del proceso, por lo que los scripts `.ps1` y las importaciones de módulos funcionan en instalaciones predeterminadas de Windows sin cambiar la política de la máquina. El bypass de ámbito de proceso no anula la Política de grupo `MachinePolicy` o `UserPolicy`, por lo que las políticas empresariales siguen siendo aplicables. Para respetar la política de ejecución efectiva de la máquina en su lugar, establezca `CLAUDE_CODE_POWERSHELL_RESPECT_EXECUTION_POLICY=1`.

<h3 id="shell-selection-in-settings-hooks-and-skills">
  Selección de shell en configuración, hooks y skills
</h3>

Tres configuraciones adicionales controlan dónde se usa PowerShell:

* `"defaultShell": "powershell"` en [`settings.json`](/docs/es/settings-reference#all-settings): enruta comandos interactivos `!` a través de PowerShell. Requiere que la herramienta PowerShell esté habilitada.
* `"shell": "powershell"` en [hooks de comando](/docs/es/hooks#command-hook-fields) individuales: ejecuta ese hook en PowerShell. Los hooks inician PowerShell directamente, por lo que esto funciona independientemente de `CLAUDE_CODE_USE_POWERSHELL_TOOL`.
* `shell: powershell` en [frontmatter de skill](/docs/es/skills#frontmatter-reference): ejecuta bloques `` !`command` `` en PowerShell. Requiere que la herramienta PowerShell esté habilitada.

El mismo comportamiento de reinicio del directorio de trabajo de la sesión principal descrito en la sección de la herramienta Bash se aplica a los comandos de PowerShell, incluida la variable de entorno `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR`.

A partir de v2.1.196, el código de salida 1 de `grep`, `rg`, `egrep`, `fgrep`, `findstr` y `git grep` significa que no hay coincidencias. El código de salida 1 de `git diff` significa que existen diferencias. Ninguno de estos resultados se reporta a Claude como un error de comando. Para `robocopy`, los códigos de salida de 0 a 7 son resultados informativos, como archivos copiados o archivos adicionales detectados. Los códigos de salida de 8 o superior cuentan como errores.

<h3 id="windows-encoding-and-exit-codes">
  Codificación de Windows y códigos de salida
</h3>

En Windows, los siguientes comportamientos de codificación y código de salida de PowerShell requieren Claude Code v2.1.214 o posterior:

* La redirección con `>` y `>>` escribe archivos UTF-8 en PowerShell 5.1
* Claude Code codifica el texto canalizando a la entrada estándar de un comando nativo como UTF-8
* Claude Code captura la salida de error sin secuencias de escape ANSI
* Un comando cuyo proceso secundario espera en la entrada estándar recibe fin de archivo en lugar de colgarse
* El código de salida 1 de `where.exe` significa que no hay coincidencia, y de `fc.exe` y `diff.exe` significa que los archivos difieren, por lo que cuando el comando produce salida, Claude Code trata ese código de salida como una respuesta negativa válida en lugar de un error de comando. Claude Code aún reporta una forma silenciada, como `where.exe /Q` o una redirección a `$null`, como un error en el código de salida 1

Antes de v2.1.214, `>` en PowerShell 5.1 escribía archivos UTF-16LE, la entrada canalizadora no ASCII llegaba como `?`, y los scripts de Python podrían fallar con un `UnicodeEncodeError` al imprimir caracteres no ASCII.

<h3 id="preview-limitations">
  Limitaciones de vista previa
</h3>

La herramienta PowerShell tiene las siguientes limitaciones conocidas durante la vista previa:

* Los perfiles de PowerShell no se cargan
* En Windows, el sandboxing no es compatible

<h2 id="read-tool-behavior">
  Comportamiento de la herramienta Read
</h2>

La herramienta Read toma una ruta de archivo y devuelve el contenido con números de línea. Claude recibe instrucciones para pasar siempre rutas absolutas.

De forma predeterminada, Read devuelve el archivo desde el inicio. Cuando una lectura de archivo completo excede el límite de tokens, Read devuelve la primera página con un aviso de `PARTIAL view` que le indica a Claude cuánto del archivo recibió y cómo leer más con `offset` y `limit`. Una lectura que pasa un `offset` o `limit` explícito y aún así excede el límite de tokens devuelve un error.

Una lectura con un `limit` explícito se detiene tan pronto como las líneas seleccionadas excedan lo que el límite de tokens podría caber y devuelve un error sin cargar el resto del rango. El error le indica a Claude que use un `limit` más pequeño, o que busque contenido específico con [Grep](#grep-tool-behavior) en su lugar cuando una sola línea sea tan grande. Antes de v2.1.208, Claude Code cargaba todo el rango en memoria antes de rechazarlo, por lo que leer un archivo con una sola línea extremadamente larga podría agotar la memoria.

Leer un archivo vacío devuelve un aviso de que el archivo existe pero su contenido está vacío, y un `offset` más allá de la última línea devuelve un aviso que proporciona el recuento de líneas del archivo. Antes de v2.1.208, leer un archivo vacío devolvía el aviso de fin de archivo en su lugar.

Read maneja varios tipos de archivo más allá del texto sin formato:

* **Imágenes**: PNG, JPG y otros formatos de imagen se devuelven como contenido visual que Claude puede ver, no como bytes sin procesar. Claude Code redimensiona y recomprime imágenes grandes para que se ajusten a los límites de tamaño de imagen del modelo antes de enviarlas, por lo que Claude puede ver una versión reducida de una captura de pantalla grande. A partir de v2.1.196, una imagen que sigue siendo más grande que 500KB después de ese redimensionamiento se recodifica como JPEG con calidad reducida con sus dimensiones de píxeles sin cambios. Si Claude pierde detalles a nivel de píxel fino en una imagen grande, pídale que primero recorte la región de interés, por ejemplo con ImageMagick a través de Bash.
* **PDFs**: Claude lee archivos `.pdf` cortos completos. Para PDFs más largos que 10 páginas, lee en rangos con un parámetro `pages`, como `"1-5"`, hasta 20 páginas a la vez.
* **Cuadernos Jupyter**: Los archivos `.ipynb` devuelven todas las celdas con sus salidas, incluido código, markdown y visualizaciones. Claude Code se niega a leer un archivo de cuaderno de más de 100 MB; el error le indica a Claude cómo leer una porción del cuaderno en su lugar, como un segmento de celdas, con un comando de shell.

Read solo lee archivos, no directorios. Claude enumera el contenido del directorio con un comando de shell como `ls`.

<h2 id="sendfeedback-tool-behavior">
  Comportamiento de la herramienta SendFeedback
</h2>

Los comentarios redactados por Claude son un informe de comentarios sobre Claude Code que Claude escribe para usted. Requiere Claude Code v2.1.238 o posterior. Claude Code guarda cada borrador en su máquina bajo `~/.claude/feedback/drafts/`, y nada llega a Anthropic hasta que lo envíe. Claude redacta uno con la herramienta SendFeedback cuando:

* Una herramienta o comando sigue fallando
* No puede ayudarle con algo que le pidió
* Usted señala un error que cometió, o lo nota
* Le pide que presente comentarios

<h3 id="what-you-see-when-claude-drafts">
  Lo que ve cuando Claude redacta
</h3>

Después de que Claude pone en cola un borrador, ve una tarjeta encima de su prompt con el título del borrador. Presione `1` para revisar el borrador, presione `2` dos veces para enviarlo tal como está escrito, o presione `0` para descartarlo. Un borrador descartado permanece en su cola. Después de descartar una tarjeta, Claude Code pregunta si desea desactivar los comentarios redactados por Claude. Deja de preguntar una vez que ha rechazado dos veces.

De forma predeterminada, ve como máximo tres tarjetas en una sesión; Anthropic puede ajustar ese límite desde el servidor sin una versión. Después del límite, y siempre que establezca [`feedbackDrafts`](/docs/es/settings-reference#feedbackdrafts) en `quiet`, ve solo un recuento de borradores en cola en el pie de página del prompt.

<h3 id="review-and-edit-a-draft">
  Revisar y editar un borrador
</h3>

Ejecute `/feedback` sin argumentos para abrir su cola. Enumera todos los borradores en cola de todas sus sesiones, incluidos los borradores cuyas tarjetas descartó o nunca vio. Seleccione un borrador para abrirlo para revisión, donde puede:

* Editar el título, área y detalles
* Establecer **Send transcript** en `yes` o `no`. Cuando la transcripción de la sesión donde Claude puso en cola el borrador aún está disponible, comienza en `yes`, que envía esa conversación a Anthropic; `no` envía solo el informe
* Enviar el borrador, descartarlo o dejarlo en la cola para más tarde

Para escribir un informe usted mismo, presione `w` para el diálogo de comentarios estándar. `/feedback` con texto después, y `/bug`, abren ese diálogo directamente.

<h3 id="send-a-draft">
  Enviar un borrador
</h3>

Cuando envía un borrador, Claude Code lo envía de la misma manera que un informe `/feedback`, con la misma [retención](/docs/es/data-usage#feedback-using-the-%2Ffeedback-command), y elimina el borrador de su máquina. Cuando envía desde la tarjeta, muestra `✓ Sent`; cuando envía desde la cola, se cierra con un ID de recepción.

El informe lleva:

* Su título, área y detalles
* Información del entorno, como su versión de Claude Code, sistema operativo y modelo
* Los ID de solicitudes de API recientes
* La transcripción de la conversación, cuando dejó **Send transcript** en `yes` en la pantalla de revisión. Enviar desde la tarjeta nunca incluye la transcripción

Claude Code mantiene su directorio de trabajo en el borrador local para poder encontrar la transcripción, y no envía el directorio.

En [organizaciones con retención de datos cero](/docs/es/zero-data-retention#features-disabled-under-zdr), Claude Code deja la herramienta fuera, como lo hace para `/feedback`. Si una sesión en tal organización aún ofrece la herramienta, los borradores permanecen en su máquina, y el envío falla con `Feedback collection is not available for organizations with custom data retention policies.`

<h3 id="discard-or-keep-a-draft">
  Descartar o mantener un borrador
</h3>

Cuando descarta un borrador, Claude Code lo elimina de su máquina. Un borrador que deja en la cola expira después de 30 días, o después de [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays) cuando es más corto. La cola contiene 10 borradores en todas sus sesiones, y cuando Claude pone en cola un undécimo, Claude Code elimina el más antiguo. Cuando ejecuta `/exit` con borradores de la sesión aún en la cola, Claude Code pregunta si desea revisarlos o descartarlos antes de salir.

<h3 id="turn-claude-drafted-feedback-off">
  Desactivar los comentarios redactados por Claude
</h3>

Establezca **Claude-drafted feedback** en `off` en `/config`, que escribe la configuración [`feedbackDrafts`](/docs/es/settings-reference#feedbackdrafts), o establezca [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/es/env-vars) para una sesión. Con cualquiera de estos, Claude no puede poner en cola borradores. Para mantener la redacción sin tarjetas, establezca `feedbackDrafts` en `quiet` en su lugar. Los administradores pueden establecer `feedbackDrafts` en [configuración administrada](/docs/es/managed-settings), que tiene prioridad sobre su propia configuración.

<h3 id="sessions-without-claude-drafted-feedback">
  Sesiones sin comentarios redactados por Claude
</h3>

Claude Code incluye la herramienta en sesiones de terminal interactivas en su propia máquina que usan la API de Claude en lugar de un proveedor de nube. La deja fuera de:

* Ejecuciones no interactivas `-p` y sesiones de [Agent SDK](/docs/es/agent-sdk/overview), que no tienen pantalla para revisar la cola
* Sesiones en la nube como [Claude Code en la web](/docs/es/claude-code-on-the-web), que no pueden escribir en la cola en su máquina
* Sesiones en [Amazon Bedrock](/docs/es/amazon-bedrock), [Claude Platform en AWS](/docs/es/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/es/google-vertex-ai), o [Microsoft Foundry](/docs/es/microsoft-foundry)
* Sesiones donde establece [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/es/env-vars) o [`DISABLE_FEEDBACK_COMMAND=1`](/docs/es/env-vars), establece `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` en cualquier valor no vacío, o desactivó la [obtención de indicadores de características](/docs/es/env-vars#features-that-need-feature-flag-fetching)
* Organizaciones que han desactivado comentarios de productos, y [organizaciones con retención de datos cero](/docs/es/zero-data-retention#features-disabled-under-zdr)

<h2 id="task-tool-availability">
  Disponibilidad de la herramienta Task
</h2>

Las herramientas de seguimiento de tareas, `TaskCreate`, `TaskGet`, `TaskUpdate`, `TaskList` y `TodoWrite`, están disponibles de forma predeterminada solo en modelos Claude 3.x, Opus 4 a través de 4.7, Sonnet 4 a través de 4.6 y Haiku 4.5. Siempre que las herramientas estén disponibles, obtiene las cuatro herramientas Task, o `TodoWrite` en su lugar cuando establece [`CLAUDE_CODE_ENABLE_TASKS=0`](/docs/es/env-vars).

En todos los demás modelos, Claude Code omite las herramientas a menos que opte por usarlas. Lo mismo se aplica a un ID de modelo que Claude Code no reconoce, como un nombre de modelo personalizado servido a través de una [puerta de enlace LLM](/docs/es/llm-gateway). En modelos más nuevos, Claude realiza un seguimiento del trabajo de varios pasos sin una lista de verificación escrita, y las definiciones y recordatorios de las herramientas ocupan contexto. Sin las herramientas, Claude no agrega nada a la [lista de tareas](/docs/es/interactive-mode#task-list) mientras trabaja.

Si desea utilizar estas herramientas en un modelo que no las tiene de forma predeterminada, haga una de las siguientes cosas:

* Exporte [`CLAUDE_CODE_ENABLE_TODO_TOOLS=1`](/docs/es/env-vars) antes de iniciar Claude Code, por ejemplo `CLAUDE_CODE_ENABLE_TODO_TOOLS=1 claude`. Claude Code entonces proporciona las mismas herramientas en cada modelo y cada proveedor
* Nombre una de las herramientas en [`--allowedTools`](/docs/es/cli-reference#cli-flags), por ejemplo `claude --allowedTools TaskCreate`
* Liste las herramientas en [`--tools`](/docs/es/cli-reference#cli-flags), que restringe las herramientas integradas de la sesión a las que nombra. Incluya las herramientas que desea junto con las otras herramientas integradas que utiliza
* En el Agent SDK, las opciones [`allowedTools` y `tools`](/docs/es/agent-sdk/todo-tracking#model-availability) funcionan de la misma manera que las dos banderas

En [sesiones en segundo plano](/docs/es/agent-view) y en [sesiones en la nube](/docs/es/claude-code-on-the-web), Claude Code proporciona las mismas herramientas en cada modelo, listado o no.

Claude Code proporciona a una subagente las herramientas solo cuando su sesión las tiene, incluso cuando la subagente ejecuta un modelo diferente. Un compañero de [equipo de agentes](/docs/es/agent-teams) en proceso sigue su sesión de la misma manera, mientras que un compañero en su propio [panel dividido](/docs/es/agent-teams#choose-a-display-mode) se ejecuta como un proceso Claude Code separado, por lo que su propio modelo decide. Sin las herramientas Task, un agente se coordina con su equipo a través de mensajes en lugar de la [lista de tareas compartida](/docs/es/agent-teams#assign-and-claim-tasks).

El conjunto predeterminado descrito aquí se aplica en Claude Code v2.1.268 y posteriores.

<h2 id="webfetch-tool-behavior">
  Comportamiento de la herramienta WebFetch
</h2>

WebFetch toma una URL y un mensaje que describe qué extraer. Obtiene la página, convierte la respuesta a Markdown cuando el servidor devuelve HTML, y ejecuta el mensaje contra el contenido utilizando un modelo pequeño y rápido. Para la mayoría de las búsquedas, Claude recibe la respuesta de ese modelo, no la página sin procesar. El paso de conversión no es configurable.

Esto hace que WebFetch sea lossy por diseño. El mensaje de extracción determina qué llega a Claude, por lo que un resultado que dice que una página no menciona algo puede significar solo que el mensaje no lo preguntó. Pida a Claude que obtenga nuevamente con un mensaje más específico, o use `curl` a través de Bash para la página sin procesar.

Algunos comportamientos dan forma a la respuesta que Claude recibe:

* WebFetch rechaza `localhost` y cualquier otro nombre de host sin un punto, como un nombre de intranet simple, antes de hacer una solicitud. El [error que devuelve](/docs/es/errors#webfetch-cannot-fetch-localhost) le dice a Claude que alcance servidores locales con `curl` a través de Bash en su lugar.
* Las URLs HTTP se actualizan automáticamente a HTTPS.
* Las páginas grandes se truncan a un límite de caracteres fijo antes del procesamiento.
* WebFetch almacena en caché cada respuesta durante 15 minutos de forma predeterminada, por lo que las búsquedas repetidas de la misma URL se devuelven rápidamente. En Claude Code v2.1.233 o posterior, establezca [`CLAUDE_CODE_WEBFETCH_CACHE_TTL_MS`](/docs/es/env-vars#variables) para cambiar cuánto tiempo WebFetch mantiene cada respuesta.
* Una página que no haya terminado de descargar dentro de cinco minutos, incluidos los redireccionamientos que WebFetch sigue, falla con un error de límite de tiempo. En Claude Code v2.1.268 o posterior, establezca [`CLAUDE_CODE_WEBFETCH_DEADLINE_MS`](/docs/es/env-vars#variables) para cambiar el límite, o a `0` para eliminarlo.
* Cuando una URL se redirige a un host diferente, WebFetch devuelve un resultado de texto que nombra la URL original y el destino de redirección en lugar de seguirlo. Claude luego obtiene la nueva URL con una segunda llamada a WebFetch.
* Cuando el paso de extracción golpea una API sobrecargada, Claude Code lo reintenta con retroceso; una búsqueda que aún falla devuelve un resultado de error. Antes de v2.1.212, el texto de error de la API podría llegar a Claude como si fuera el contenido de la página extraída.

En los modos Manual y `acceptEdits` [de permisos](/docs/es/permission-modes), WebFetch solicita antes de obtener, excepto para dominios que sus [reglas de permisos](/docs/es/permissions#manage-permissions) ya permiten o deniegan y un conjunto integrado de dominios de documentación preaprobados que se obtienen sin solicitud. Sea cual sea lo que sus reglas permitan, una búsqueda también pasa primero la [verificación de seguridad del dominio WebFetch](/docs/es/data-usage#webfetch-domain-safety-check); esa sección cubre qué envía la verificación y la configuración que la omite. El mensaje ofrece tres opciones:

* **Sí**: aprueba solo esta búsqueda. La siguiente llamada a WebFetch solicita nuevamente, incluso para el mismo dominio.
* **Sí, y no preguntar nuevamente para `<domain>`**: aprueba la búsqueda y guarda una regla de permiso `WebFetch(domain:...)` para ese dominio en `.claude/settings.local.json` para ese repositorio. Consulte [cómo persisten las aprobaciones guardadas](/docs/es/permissions#permission-system). Cuando su organización establece [`allowManagedPermissionRulesOnly`](/docs/es/permissions#managed-only-settings), Claude Code oculta esta opción.
* **No, y dígale a Claude qué hacer diferente**: rechaza la búsqueda.

Para permitir un dominio por adelantado sin solicitud, agregue una regla de permiso como `WebFetch(domain:example.com)`; `WebFetch(domain:*)` permite cada dominio. Los modos de permisos `auto` y `bypassPermissions` [de permisos](/docs/es/permissions#permission-modes) omiten la solicitud, excepto para un dominio que coincida con una regla `ask` explícita.

Una regla `WebFetch(domain:...)` explícita en `deny`, `ask` o `allow` tiene precedencia sobre el conjunto preaprobado, por lo que puede bloquear un dominio preaprobado o requerir una solicitud para él.

WebFetch establece un encabezado `User-Agent` que comienza con `Claude-User`, y un encabezado `Accept` que prefiere Markdown sobre HTML para que los servidores que admiten negociación de contenido puedan devolver Markdown directamente.

Los comandos en sandbox no heredan el conjunto integrado de dominios de documentación preaprobados de WebFetch. Para permitir que un comando en sandbox alcance un dominio sin solicitud, agregue el dominio a [`allowedDomains`](/docs/es/settings-reference#sandbox-network-alloweddomains) o permítalo con una regla `WebFetch(domain:...)`, que el [sandbox también respeta](/docs/es/sandboxing#network-isolation). WebFetch nunca lee la lista de permisos del sandbox a cambio, por lo que agregar un dominio a una lista de permisos de red de sandbox u organización no impide que WebFetch solicite por él.

<h2 id="websearch-tool-behavior">
  Comportamiento de la herramienta WebSearch
</h2>

WebSearch ejecuta una consulta contra el backend de [búsqueda web](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool) de Anthropic y devuelve títulos de resultados y URLs. No obtiene las páginas de resultados. Para leer una página que Claude encuentra en los resultados de búsqueda, continúa con [WebFetch](#webfetch-tool-behavior).

La herramienta puede emitir hasta ocho búsquedas de backend por llamada, refinando la búsqueda internamente antes de devolver resultados. Claude puede limitar los resultados con `allowed_domains` para incluir solo ciertos hosts, o `blocked_domains` para excluirlos. Las dos listas no se pueden combinar en una sola llamada.

Cuando la solicitud de búsqueda afecta a una API sobrecargada, Claude Code la reintenta con retroceso; una llamada que aún falla devuelve un resultado de error. Antes de v2.1.212, el texto de error de la API podría llegar a Claude como si fueran resultados de búsqueda.

Las reglas de permisos de WebSearch no requieren especificador. Una entrada `WebSearch` simple en `allow` o `deny` es la única forma.

El backend de búsqueda no es configurable. Para buscar con un proveedor diferente, agregue un [servidor MCP](/docs/es/mcp) que exponga una herramienta de búsqueda.

<Note>
  WebSearch está disponible en la API de Claude y en [Claude Platform en AWS](/docs/es/claude-platform-on-aws). En Microsoft Foundry requiere un [despliegue alojado en Anthropic](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options): los despliegues alojados en Azure no admiten herramientas del lado del servidor, por lo que la llamada de WebSearch falla. En la plataforma de agentes de Google Cloud funciona con Claude 4 y modelos posteriores, incluidos Opus, Sonnet y Haiku. Amazon Bedrock no expone la herramienta de búsqueda web del lado del servidor.
</Note>

<h3 id="session-search-limit">
  Límite de búsqueda de sesión
</h3>

Una sesión puede realizar como máximo 200 llamadas de WebSearch, contadas en toda la conversación principal y cada [subagente](/docs/es/sub-agents) que genera, por lo que las búsquedas realizadas por expansiones de investigación paralela cuentan contra el mismo límite. El límite requiere Claude Code v2.1.212 o posterior. Cuando Claude alcanza el límite, las llamadas posteriores devuelven un aviso que le dice a Claude que continúe con la información que ya ha recopilado, en lugar de un error que invitaría a un reintento. Usted no ve el aviso: una llamada limitada aparece en la conversación como una búsqueda que no hizo nada, y si Claude necesita más búsquedas, el aviso le dice que le pida que aumente el límite.

Establezca la variable de entorno [`CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`](/docs/es/env-vars) para cambiar el límite; acepta un número entero positivo, por lo que el límite se puede aumentar pero no desactivar. Ejecutar [`/clear`](/docs/es/commands#all-commands) reinicia el contador. Si el trabajo que aún puede generar [subagentes](/docs/es/sub-agents) sobrevive al borrado, como un flujo de trabajo en ejecución, el contador se mantiene en su lugar.

<h2 id="write-tool-behavior">
  Comportamiento de la herramienta Write
</h2>

La herramienta Write crea un archivo nuevo o sobrescribe uno existente con el contenido completo proporcionado. No añade ni fusiona.

Si Claude debe leer un archivo existente en la conversación actual antes de sobrescribirlo depende del modelo y del archivo:

* Claude Opus 4.6, Claude Haiku 4.5 y modelos más antiguos siempre requieren la lectura, por lo que una operación Write en un archivo existente no leído falla con un error.
* Los modelos más nuevos pueden sobrescribir un archivo que nunca leyeron en esta sesión bajo las mismas condiciones que [read-before-edit](#edit-tool-behavior): leerlo no necesitaría un aviso de permiso y la herramienta Read está disponible.
* Los notebooks de Jupyter y los archivos que Claude ha leído solo parcialmente con un aviso [`PARTIAL view`](#read-tool-behavior) requieren la lectura en todos los modelos.

Esta restricción no se aplica a archivos nuevos. Antes de v2.1.228, todos los modelos requerían la lectura antes de sobrescribir un archivo existente.

Ver el archivo con Bash también satisface este requisito bajo las mismas reglas descritas en [Comportamiento de la herramienta Edit](#edit-tool-behavior).

Para cambios parciales en un archivo existente, Claude utiliza Edit en lugar de Write.

<h2 id="check-which-tools-are-available">
  Verificar qué herramientas están disponibles
</h2>

Su conjunto exacto de herramientas depende de su proveedor, plataforma y configuración. Para verificar qué está cargado en una sesión en ejecución, pregúntele a Claude directamente:

```text theme={null}
¿Qué herramientas tienes disponibles?
```

Claude proporciona un resumen conversacional. Para nombres exactos de herramientas MCP, ejecute `/mcp`.

<Note>
  La [herramienta advisor](/docs/es/advisor) es una [herramienta de servidor](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool) que ejecuta la API, en lugar de una herramienta que implementa Claude Code. No tiene un nombre que pueda referenciar en reglas de permisos o coincidencias de hooks.
</Note>

<h2 id="see-also">
  Véase también
</h2>

* [Servidores MCP](/docs/es/mcp): agregue herramientas personalizadas conectando servidores externos
* [Permisos](/docs/es/permissions): sistema de permisos, sintaxis de reglas y patrones específicos de herramientas
* [Subagents](/docs/es/sub-agents): configure el acceso a herramientas para subagents
* [Hooks](/docs/es/hooks-guide): ejecute comandos personalizados antes o después de la ejecución de herramientas
