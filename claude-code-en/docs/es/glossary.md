> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Glosario

> Definiciones de terminología de Claude Code. Aprenda qué significan agentic loop, compaction, CLAUDE.md, hooks, subagents, MCP y otros conceptos centrales.

Este glosario define la terminología de Claude Code. Cada entrada enlaza a la página donde el concepto se cubre en profundidad. Para conceptos a nivel de modelo como tokens, temperature y RAG, consulte el [glosario de plataforma](https://platform.claude.com/docs/es/about-claude/glossary).

<h2 id="a">
  A
</h2>

<h3 id="agent-teams">
  Agent teams
</h3>

Múltiples sesiones independientes de Claude Code coordinadas por un líder de equipo, con una lista de tareas compartida y mensajería de igual a igual. A diferencia de [subagents](#subagent), que se ejecutan dentro de una única sesión e informan solo al padre, los compañeros de equipo tienen cada uno su propia ventana de contexto y puede interactuar directamente con cualquiera de ellos. Agent teams es experimental y debe habilitarse configurando `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`.

Más información: [Run agent teams](/docs/es/agent-teams)

<h3 id="agentic-coding">
  Agentic coding
</h3>

Un flujo de trabajo donde la IA puede leer archivos, ejecutar comandos y realizar cambios de forma autónoma mientras usted observa, redirige o se aleja, a diferencia de los asistentes basados en chat que solo responden con texto que debe aplicar usted mismo. Claude Code es agentic porque tiene [tools](#tool) que le permiten actuar, no solo aconsejar.

Más información: [How Claude Code works](/docs/es/how-claude-code-works)

<h3 id="agentic-harness">
  Agentic harness
</h3>

Las herramientas, gestión de contexto y entorno de ejecución que convierten un modelo de lenguaje en un agente de codificación capaz. Claude Code es el harness; Claude es el modelo dentro de él. El harness proporciona acceso a archivos, ejecución de shell, control de permisos, carga de memoria y el bucle que encadena acciones juntas.

Más información: [How Claude Code works](/docs/es/how-claude-code-works)

<h3 id="agentic-loop">
  Agentic loop
</h3>

El ciclo que Claude recorre para cada tarea: recopilar contexto, tomar acción, verificar resultados y repetir hasta terminar. Cada uso de herramienta devuelve información que informa el siguiente paso. Puede interrumpir el bucle en cualquier momento para redirigir. La mayoría de los puntos de extensión, incluidos [hooks](#hook), [skills](#skill) y [MCP](#mcp-model-context-protocol), se conectan a fases específicas de este bucle.

Más información: [How Claude Code works](/docs/es/how-claude-code-works#the-agentic-loop)

<h3 id="artifact">
  Artifact
</h3>

Una página web interactiva en vivo que Claude Code publica desde su sesión a una URL privada en claude.ai, para que pueda ver la salida visualmente o compartirla en lugar de leer texto de terminal. La página se actualiza en su lugar cuando la sesión se republica. Los artefactos que crea desde Claude Code aparecen en la misma galería que los artefactos creados en conversaciones de claude.ai. El uso compartido depende de su plan: en Pro y Max, un enlace público que cualquiera puede abrir; en Team y Enterprise, uso compartido dentro de su organización, más enlaces públicos una vez que un propietario los habilita.

Más información: [Share session output as artifacts](/docs/es/artifacts)

<h3 id="auto-memory">
  Auto memory
</h3>

Notas que Claude escribe para sí mismo basadas en sus correcciones y preferencias, almacenadas por repositorio git bajo `~/.claude/projects/`. Todos los worktrees del mismo repositorio comparten un directorio de auto memory. Las primeras 200 líneas o 25 KB del índice `MEMORY.md` se cargan al inicio de cada sesión. Auto memory es la contraparte escrita por Claude de [CLAUDE.md](#claude-md), que usted escribe.

Más información: [Auto memory](/docs/es/memory#auto-memory)

<h3 id="auto-mode">
  Auto mode
</h3>

Un [permission mode](#permission-mode) donde un modelo clasificador separado revisa acciones en segundo plano, por lo que la mayoría se ejecutan sin solicitudes de aprobación; las reglas de solicitud explícita aún generan solicitudes. El clasificador bloquea la escalada de alcance, la infraestructura no confiable y la [prompt injection](#prompt-injection). Nunca ve resultados de herramientas, por lo que las instrucciones inyectadas no pueden influir en sus decisiones.

Más información: [Eliminate prompts with auto mode](/docs/es/permission-modes#eliminate-prompts-with-auto-mode)

<h2 id="b">
  B
</h2>

<h3 id="bare-mode">
  Bare mode
</h3>

Una bandera de inicio, `--bare`, que omite el descubrimiento automático de hooks, skills, plugins, servidores MCP, auto memory y CLAUDE.md. Solo las banderas que pasa explícitamente tienen efecto. Se recomienda para CI y llamadas con script donde necesita un comportamiento idéntico en todas las máquinas independientemente de la configuración local.

Más información: [Iniciar más rápido con bare mode](/docs/es/headless#start-faster-with-bare-mode)

<h3 id="bundled-skills">
  Bundled skills
</h3>

Playbooks basados en prompts incluidos con Claude Code, como `/batch`, `/code-review`, `/debug` y `/loop`. A diferencia de los comandos integrados, que ejecutan lógica fija, bundled skills le da a Claude un prompt detallado y le permite orquestar el trabajo, por lo que pueden generar agentes, leer archivos y adaptarse a su base de código.

Más información: [Bundled skills](/docs/es/skills#bundled-skills)

<h2 id="c">
  C
</h2>

<h3 id="channel">
  Channel
</h3>

Un [MCP server](#mcp-model-context-protocol) que envía eventos a su sesión en ejecución para que Claude pueda reaccionar a cosas que suceden mientras está lejos de la terminal. Los canales pueden ser bidireccionales: Claude lee un evento entrante y responde a través del mismo canal. Telegram, Discord e iMessage se incluyen en la vista previa de investigación.

Más información: [Channels](/docs/es/channels)

<h3 id="checkpoint">
  Checkpoint
</h3>

Un punto de restauración creado en cada prompt que envía. Claude Code captura instantáneas de archivos antes de cada edición para que un checkpoint pueda revertirlos. Presione `Esc` dos veces o ejecute `/rewind` para restaurar código, conversación o ambos a un punto anterior, o para resumir parte de la conversación desde un mensaje seleccionado. Los checkpoints se guardan con la conversación, por lo que una sesión reanudada aún puede `/rewind` a ellos. Son separados de git y no rastrean cambios realizados a través de la herramienta Bash.

Más información: [Checkpointing](/docs/es/checkpointing)

<h3 id="claude-directory">
  `.claude` directory
</h3>

El directorio donde Claude Code lee la configuración con alcance de proyecto: configuración, hooks, skills, subagents, reglas y auto memory. Un proyecto tiene `.claude/` en su raíz; sus valores predeterminados a nivel de usuario están en `~/.claude/`.

Más información: [The `.claude` directory](/docs/es/claude-directory)

<h3 id="claude-md">
  CLAUDE.md
</h3>

Un archivo markdown de instrucciones persistentes que usted escribe para Claude, cargado al inicio de cada sesión como un mensaje de usuario después del prompt del sistema. Coloque convenciones de proyecto, notas de arquitectura y reglas "siempre haga X" aquí. El CLAUDE.md de raíz de proyecto sobrevive a [compaction](#compaction) y se relee fresco desde el disco después.

Puede colocar CLAUDE.md en alcance de proyecto en `./CLAUDE.md` o `./.claude/CLAUDE.md`, en alcance de usuario en `~/.claude/CLAUDE.md`, o como [managed policy](#managed-settings) para su organización. Todos los archivos descubiertos se concatenan en contexto en lugar de anularse entre sí, ordenados de alcance más amplio a más específico.

Más información: [CLAUDE.md files](/docs/es/memory#claude-md-files)

<h3 id="command">
  Command
</h3>

Una instrucción reutilizable que invoca escribiendo `/name` en el prompt. Los comandos integrados como `/clear`, `/model` y `/compact` controlan la sesión. Puede definir sus propios comandos como archivos en `.claude/commands/`, o instalarlos desde un [plugin](#plugin). [Skills](#skill) es la forma recomendada de empaquetar comandos de varios pasos.

Más información: [Commands](/docs/es/commands) · [Skills](/docs/es/skills)

<h3 id="compaction">
  Compaction
</h3>

Resumen automático de su conversación cuando la [context window](#context-window) se acerca a su límite. Las salidas de herramientas más antiguas se borran primero, luego se resume la conversación. El CLAUDE.md de raíz de proyecto y auto memory sobreviven a compaction y se recargan desde el disco; las instrucciones dadas solo en conversación pueden perderse. Ejecute `/compact` para activar manualmente, opcionalmente con un enfoque como `/compact focus on the API changes`.

Más información: [What survives compaction](/docs/es/context-window#what-survives-compaction) · [When context fills up](/docs/es/how-claude-code-works#when-context-fills-up)

<h3 id="context-window">
  Context window
</h3>

La memoria de trabajo para una sesión, que contiene historial de conversación, contenidos de archivos, salidas de comandos, CLAUDE.md, auto memory, skills cargadas e instrucciones del sistema. A medida que trabaja, el contexto se llena hasta que [compaction](#compaction) lo resume. Ejecute `/context` para ver qué está usando espacio. Para el concepto de modelo subyacente, consulte el [glosario de plataforma](https://platform.claude.com/docs/es/about-claude/glossary#context-window).

Más información: [Explore the context window](/docs/es/context-window)

<h2 id="d">
  D
</h2>

<h3 id="dispatch">
  Dispatch
</h3>

Un enrutador de tareas iniciado por teléfono que genera una sesión de Claude Code en la aplicación Desktop cuando envía una tarea de codificación desde la aplicación móvil de Claude. Su prompt se enruta a la herramienta correcta automáticamente. Disponible en planes Pro y Max.

Más información: [Sessions from Dispatch](/docs/es/desktop#sessions-from-dispatch)

<h2 id="e">
  E
</h2>

<h3 id="effort-level">
  Effort level
</h3>

Una configuración que controla cuánto del presupuesto de pensamiento de razonamiento adaptativo usa Claude en cada turno. Mayor esfuerzo significa más tokens de pensamiento y razonamiento más profundo; menor esfuerzo es más rápido y económico. El esfuerzo es compatible con Fable 5, en Opus 4.6 y posterior, y en Sonnet 4.6 y posterior.

Más información: [Adjust effort level](/docs/es/model-config#adjust-effort-level)

<h3 id="extended-thinking">
  Extended thinking
</h3>

Razonamiento paso a paso visible que el modelo realiza antes de responder. Puede ajustarlo con el [effort level](#effort-level), o limitar tokens de pensamiento con `MAX_THINKING_TOKENS` en modelos con un presupuesto de pensamiento fijo. El pensamiento aparece en texto gris cursiva en la terminal.

Más información: [Use extended thinking](/docs/es/model-config#extended-thinking)

<h2 id="h">
  H
</h2>

<h3 id="hook">
  Hook
</h3>

Un manejador definido por el usuario que se ejecuta automáticamente en un punto específico del ciclo de vida de Claude Code, como antes de que se ejecute una herramienta, después de una edición de archivo o al inicio de la sesión. Los manejadores pueden ser un comando de shell, punto final HTTP, herramienta MCP, prompt LLM o subagent. Los hooks son deterministas: se activan en puntos de ciclo de vida fijos en lugar de a discreción del modelo.

Una configuración de hook tiene tres niveles:

* **Hook event**: el punto del ciclo de vida
* **Matcher**: filtra qué eventos lo activan
* **Hook handler**: qué se ejecuta

Más información: [Get started with hooks](/docs/es/hooks-guide) · [Hooks reference](/docs/es/hooks)

<h2 id="m">
  M
</h2>

<h3 id="managed-settings">
  Managed settings
</h3>

Configuración aplicada en toda la organización por IT o DevOps, entregada desde los servidores de Anthropic a través de la consola de administración o implementada en dispositivos en una ruta a nivel de SO fuera de `~/.claude`. Los usuarios no pueden anular la configuración administrada desde ámbitos de menor precedencia. La entrega administrada por servidor se aplica en [configuraciones elegibles](/docs/es/server-managed-settings#platform-availability); consulte [Consideraciones de seguridad](/docs/es/server-managed-settings#security-considerations). Use esto para políticas de seguridad, requisitos de cumplimiento o herramientas estandarizadas en toda una flota.

Más información: [Server-managed settings](/docs/es/server-managed-settings) · [Settings files](/docs/es/settings#settings-files)

<h3 id="mcp-model-context-protocol">
  MCP (Model Context Protocol)
</h3>

Un estándar abierto para conectar herramientas de IA a fuentes de datos externas y servicios. Los servidores MCP le dan a Claude nuevas herramientas para Slack, Jira, bases de datos, navegadores y cientos de otras integraciones. Conecta servidores a través de `/mcp` o agregándolos a `.mcp.json`. Para el protocolo en sí, consulte el [glosario de plataforma](https://platform.claude.com/docs/es/about-claude/glossary#mcp-model-context-protocol).

Más información: [Model Context Protocol](/docs/es/mcp)

<h3 id="mcp-tool-search">
  MCP Tool Search
</h3>

Un mecanismo de ahorro de contexto que difiere los esquemas de herramientas MCP hasta que sea necesario. Solo los nombres de herramientas se cargan al inicio; Claude obtiene el esquema completo bajo demanda cuando decide usar una herramienta específica. Esto evita que los servidores MCP inactivos consuman mucho contexto.

Más información: [Scale with MCP Tool Search](/docs/es/mcp#scale-with-mcp-tool-search)

<h2 id="n">
  N
</h2>

<h3 id="non-interactive-mode">
  Non-interactive mode
</h3>

Un modo que ejecuta un único prompt y sale sin una sesión conversacional, invocado con `-p` o `--print`. Se usa para CI, scripts y piping. La ejecución se sigue guardando como una sesión reanudable a menos que pase `--no-session-persistence`. El [Agent SDK](/docs/es/agent-sdk/overview) es el equivalente de Python y TypeScript. Anteriormente llamado headless mode.

Más información: [Ejecutar Claude Code mediante programación](/docs/es/headless)

<h2 id="o">
  O
</h2>

<h3 id="output-style">
  Output style
</h3>

Una configuración que modifica el prompt del sistema de Claude para cambiar el comportamiento de respuesta, tono o formato. Los estilos de salida desactivan las partes específicas de ingeniería de software del prompt del sistema predeterminado, a diferencia de [CLAUDE.md](#claude-md) que se entrega como un mensaje de usuario después del prompt del sistema. Los estilos integrados incluyen Default, Proactive, Explanatory y Learning.

Más información: [Output styles](/docs/es/output-styles)

<h2 id="p">
  P
</h2>

<h3 id="permission-mode">
  Permission mode
</h3>

El comportamiento de aprobación de línea base para la sesión. Cicle con `Shift+Tab` en la CLI o use el selector de modo en VS Code, Desktop y claude.ai. Los modos disponibles son `default`, `acceptEdits`, `plan`, `auto`, `dontAsk` y `bypassPermissions`.

El modo `default` se etiqueta como Manual en la CLI y en las extensiones de VS Code y JetBrains, y Claude Code acepta `manual` como un alias para el valor.

Más información: [Elegir un permission mode](/docs/es/permission-modes)

<h3 id="permission-rule">
  Permission rule
</h3>

Una entrada de configuración que permite, pregunta o deniega una invocación de herramienta basada en el nombre de la herramienta y el patrón de argumento. Las reglas se evalúan deny→ask→allow, la primera coincidencia gana. Las reglas de permiso son controles de grano fino superpuestos en el [permission mode](#permission-mode) más amplio.

Más información: [Configurar permisos](/docs/es/permissions)

<h3 id="plan-mode">
  Plan mode
</h3>

Un [permission mode](#permission-mode) donde Claude investiga y propone cambios sin editar sus archivos fuente. Puede leer, buscar y ejecutar comandos de exploración, luego presenta un plan para aprobación antes de tocar nada. Ingrese al plan mode con `/plan` o presionando `Shift+Tab`.

Más información: [Analizar antes de editar con plan mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode)

<h3 id="plugin">
  Plugin
</h3>

Un paquete de skills, hooks, subagents y servidores MCP empaquetados como una unidad instalable única. Las skills de plugin se espacian de nombres como `plugin-name:skill-name` para que múltiples plugins coexistan. Distribuya plugins en equipos a través de un [marketplace](/docs/es/plugin-marketplaces).

Más información: [Plugins](/docs/es/plugins)

<h3 id="project-trust">
  Project trust
</h3>

Un diálogo que acepta un directorio antes de que Claude Code cargue su configuración. La aceptación se guarda por directorio de proyecto, excepto su directorio de inicio, donde la confianza se mantiene solo para la sesión actual y el mensaje reaparece en cada lanzamiento. Trust gates la instalación automática de plugins de marketplace y la ejecución de hooks definidos por proyecto. Confiar en un directorio significa que sus archivos `.claude/settings.json`, `.mcp.json` y otros archivos de configuración tienen efecto.

Más información: [El directorio `.claude`](/docs/es/claude-directory)

<h3 id="prompt-injection">
  Prompt injection
</h3>

Instrucciones hostiles incrustadas en un archivo, página web o resultado de herramienta que intentan redirigir a Claude hacia acciones que nunca pidió. Las defensas de Claude Code incluyen el sistema de permisos, detección de inyección de comandos y verificación de confianza. [Auto mode](#auto-mode) agrega una sonda del lado del servidor que escanea resultados de herramientas en busca de contenido sospechoso y un clasificador que nunca ve resultados de herramientas, por lo que el texto inyectado no puede influir en sus decisiones de aprobación.

Más información: [Protegerse contra prompt injection](/docs/es/security#protect-against-prompt-injection)

<h2 id="r">
  R
</h2>

<h3 id="remote-control">
  Remote Control
</h3>

Una forma de continuar una sesión local de Claude Code desde su teléfono o navegador a través de claude.ai. Su código se ejecuta y los archivos permanecen en su máquina; la interfaz es remota. Diferente de Claude Code en la web, que se ejecuta en un sandbox en la nube.

Más información: [Remote Control](/docs/es/remote-control)

<h3 id="rules">
  Rules
</h3>

Archivos de instrucciones modulares en `.claude/rules/` que se cargan junto con CLAUDE.md. Una regla puede tener alcance de ruta con frontmatter YAML `paths:` para que solo se cargue cuando Claude lee un archivo coincidente, manteniendo el contexto delgado hasta que sea relevante.

Más información: [Organize rules with `.claude/rules/`](/docs/es/memory#organize-rules-with-claude/rules/)

<h2 id="s">
  S
</h2>

<h3 id="sandboxing">
  Sandboxing
</h3>

Aislamiento de sistema de archivos y red a nivel de SO para la herramienta Bash. Los comandos se ejecutan dentro de un límite que define de antemano, para que Claude pueda trabajar libremente dentro de él sin solicitudes de aprobación por comando. Sandboxing es una capa separada de [permission rules](#permission-rule).

Más información: [Sandboxing](/docs/es/sandboxing)

<h3 id="session">
  Session
</h3>

Una conversación vinculada a su directorio actual, con su propia [context window](#context-window) independiente. Las sesiones pueden reanudarse con `claude -c`, bifurcarse con `--fork-session` para preservar el historial bajo un nuevo ID de sesión, o ejecutarse en paralelo en terminales. Ejecutar `/clear` inicia una nueva sesión; la anterior permanece almacenada y está disponible a través de `/resume`. La transcripción de cada sesión se almacena bajo `~/.claude/projects/`.

Más información: [Work with sessions](/docs/es/how-claude-code-works#work-with-sessions)

<h3 id="settings-layers">
  Settings layers
</h3>

La jerarquía desde la que Claude Code lee la configuración, en orden de precedencia de mayor a menor: [managed policy](#managed-settings), argumentos de línea de comandos, configuración local en `.claude/settings.local.json`, configuración de proyecto en `.claude/settings.json`, luego configuración de usuario en `~/.claude/settings.json`. Los arrays se fusionan en todas las capas; los escalares en una capa superior anulan los inferiores.

Más información: [Settings files](/docs/es/settings#settings-files)

<h3 id="skill">
  Skill
</h3>

Un archivo `SKILL.md` que contiene instrucciones, conocimiento o un flujo de trabajo que Claude agrega a su kit de herramientas. Claude carga una skill automáticamente cuando es relevante, o la invoca directamente con `/skill-name`. Las skills siguen el estándar abierto Agent Skills; Claude Code lo extiende con control de invocación y ejecución de subagent.

Las skills son el sucesor recomendado de comandos personalizados. Un archivo en `.claude/commands/deploy.md` y uno en `.claude/skills/deploy/SKILL.md` ambos crean `/deploy` y funcionan de la misma manera; los archivos de comando existentes continúan funcionando.

Más información: [Extend Claude with skills](/docs/es/skills)

<h3 id="subagent">
  Subagent
</h3>

Un asistente de IA especializado que se ejecuta en su propia ventana de contexto con un prompt del sistema personalizado, acceso a herramientas específicas y permisos independientes. Trabaja en una tarea delegada y devuelve un resumen a la conversación principal. Use subagents para mantener grandes exploraciones fuera de su contexto principal o para ejecutar investigación en paralelo. Diferente de [agent teams](#agent-teams), donde cada agente es una sesión completamente independiente con la que puede hablar directamente.

Los subagents integrados incluyen Explore, Plan y propósito general.

Más información: [Create custom subagents](/docs/es/sub-agents)

<h3 id="surface">
  Surface
</h3>

Cualquier lugar donde acceda a Claude Code: la CLI, VS Code, JetBrains, Desktop o claude.ai. Todas las superficies comparten el mismo motor, por lo que su CLAUDE.md, configuración y skills funcionan de la misma manera en todas ellas. Slack y la extensión de Chrome son integraciones que se conectan a una superficie en lugar de ser superficies en sí mismas.

Más información: [Platforms and integrations](/docs/es/platforms)

<h2 id="t">
  T
</h2>

<h3 id="teleport">
  Teleport
</h3>

Un comando, `/teleport`, que extrae una sesión de Claude Code en la nube a su terminal local. Claude obtiene la rama, carga el historial de conversación y reanuda desde el último estado de la sesión web. La dirección inversa es `--cloud`, que envía una tarea local para ejecutarse en la web.

Más información: [De web a terminal](/docs/es/claude-code-on-the-web#from-web-to-terminal)

<h3 id="tool">
  Tool
</h3>

Una acción que Claude puede tomar: leer un archivo, editar código, ejecutar un comando de shell, buscar en la web, generar un subagent. Las herramientas son lo que hace que Claude Code sea agentic. Sin ellas, Claude solo puede responder con texto. Cada uso de herramienta devuelve un resultado que informa la siguiente decisión de Claude en el [agentic loop](#agentic-loop).

Más información: [Herramientas disponibles para Claude](/docs/es/tools-reference)

<h3 id="turn">
  Turn
</h3>

Una respuesta completa de Claude dentro de una [sesión](#session). Un turn comienza cuando usted envía un mensaje y termina cuando Claude termina de responder, con cualquier número de llamadas de [herramienta](#tool) en el medio. Los [stop hooks](#hook) se activan al final de cada turn. Una sesión consta de muchos turns, y el [agentic loop](#agentic-loop) describe lo que sucede dentro de uno.

Más información: [Cómo funciona Claude Code](/docs/es/how-claude-code-works#the-agentic-loop)

<h2 id="v">
  V
</h2>

<h3 id="verification-loop">
  Verification loop
</h3>

Cómo una sesión sabe que el trabajo está realmente hecho en lugar de solo ser plausible. Usted le da a Claude una verificación que puede ejecutar, como un conjunto de pruebas, una compilación o una comparación de capturas de pantalla, y Claude itera hasta que la verificación pase en lugar de detenerse después de un intento. Un verification loop es el requisito previo para [`/goal`](/docs/es/goal), ejecuciones desatendidas y [dynamic workflows](/docs/es/workflows): sin uno, la única cosa que decide que el agente ha terminado es el agente mismo.

Más información: [Give Claude a way to verify its work](/docs/es/best-practices#give-claude-a-way-to-verify-its-work)

<h2 id="w">
  W
</h2>

<h3 id="worktree-isolation">
  Worktree isolation
</h3>

Un modo de aislamiento que ejecuta Claude en un worktree git separado bajo `.claude/worktrees/`, habilitado con la bandera `-w` o `isolation: worktree` en la configuración de subagent. Los cambios permanecen en una rama separada en un directorio separado, por lo que los agentes paralelos no sobrescriben los archivos de los demás.

Más información: [Ejecutar sesiones paralelas con git worktrees](/docs/es/worktrees)

***

<h2 id="deprecated-and-renamed-terms">
  Términos deprecados y renombrados
</h2>

Estos términos aparecen en documentos más antiguos, publicaciones de blog y contenido de la comunidad. Use el nombre actual cuando busque en este sitio.

| Término antiguo | Ahora llamado                                 | Notas                                          |
| --------------- | --------------------------------------------- | ---------------------------------------------- |
| Headless mode   | [Non-interactive mode](#non-interactive-mode) | Misma bandera `-p`, mismo comportamiento       |
| Custom commands | [Skills](#skill)                              | Los archivos `.claude/commands/` aún funcionan |
| Slash commands  | Commands                                      | "Slash" se eliminó de la copia del producto    |
