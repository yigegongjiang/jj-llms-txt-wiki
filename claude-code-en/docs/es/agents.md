> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ejecutar agentes en paralelo

> Compare las formas en que Claude Code puede realizar múltiples tareas simultáneamente: subagentes, vista de agentes, equipos de agentes y flujos de trabajo dinámicos.

[Subagentes](/docs/es/sub-agents), [vista de agentes](/docs/es/agent-view), [equipos de agentes](/docs/es/agent-teams) y [flujos de trabajo dinámicos](/docs/es/workflows) cada uno paraleliza el trabajo de una manera diferente. El correcto depende de si desea permanecer en cada conversación usted mismo, delegar tareas y volver a verificar más tarde, o si desea que Claude coordine un grupo de trabajadores para usted.

| Enfoque                                      | Lo que le proporciona                                                                                                                                                  | Úselo cuando                                                                                                                                                                                                                                                       |
| :------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Subagentes](/docs/es/sub-agents)                 | Trabajadores delegados dentro de una sesión que realizan una tarea secundaria en su propio contexto y devuelven un resumen                                             | Una tarea secundaria inundaría su conversación principal con resultados de búsqueda, registros o contenidos de archivos que no volverá a consultar                                                                                                                 |
| [Vista de agentes](/docs/es/agent-view)           | Una pantalla para enviar y monitorear sesiones que se ejecutan en segundo plano, abierta con `claude agents`. Vista previa de investigación                            | Tiene varias tareas independientes y desea delegarlas, verificar el estado de un vistazo e intervenir solo cuando una lo necesite                                                                                                                                  |
| [Equipos de agentes](/docs/es/agent-teams)        | Múltiples sesiones coordinadas con una lista de tareas compartida y mensajería entre agentes, administradas por un líder. Experimental y deshabilitado por defecto     | Desea que Claude divida un proyecto en partes, las asigne y mantenga a los trabajadores sincronizados                                                                                                                                                              |
| [Flujos de trabajo dinámicos](/docs/es/workflows) | Un script que ejecuta muchos subagentes y verifica sus resultados, para un trabajo demasiado grande para coordinar en un solo turno o que necesita más de un solo paso | Una tarea crece más allá de un puñado de subagentes, o desea que los hallazgos se verifiquen entre sí: una auditoría en toda la base de código, una migración de 500 archivos, investigación verificada de forma cruzada, o un plan elaborado desde varios ángulos |

En cada enfoque, los trabajadores son sesiones de Claude. Para involucrar una herramienta diferente, expóngala a Claude como un [servidor MCP](/docs/es/mcp).

Dos herramientas más apoyan este trabajo sin ser una forma de ejecutar agentes en sí mismos:

* [Worktrees](/docs/es/worktrees) le dan a cada sesión un checkout de git separado, por lo que las sesiones paralelas nunca editan los mismos archivos. Úselos para sesiones que ejecuta usted mismo. La vista de agentes mueve automáticamente cada sesión enviada a su propio worktree, y los subagentes que genera pueden obtener uno también.
* [`/batch`](/docs/es/commands) es una [skill](/docs/es/skills) que hace que Claude divida un cambio grande en 5 a 30 subagentes aislados en worktree que cada uno abre una solicitud de extracción. Es un uso empaquetado de subagentes y worktrees, no un estilo de coordinación separado.

Algunas otras características ejecutan Claude sin que usted dirija cada paso, pero resuelven un problema diferente al de dividir el trabajo entre agentes:

* Un [comando bash en segundo plano](/docs/es/interactive-mode#background-bash-commands) ejecuta un comando de shell sin bloquear la conversación. No genera un agente.
* Un [subagente bifurcado](/docs/es/sub-agents#fork-the-current-conversation) es un subagente que hereda su contexto de conversación completo en lugar de comenzar de nuevo. Es una forma de generar un subagente, no una superficie separada.
* Una [rutina](/docs/es/routines) ejecuta una sesión según un cronograma en la nube de Anthropic, no en paralelo en su máquina.

<Note>
  Ejecutar varias sesiones o subagentes a la vez multiplica el uso de tokens. Consulte [Costos](/docs/es/costs) para obtener detalles de uso y límites de velocidad.
</Note>

<h2 id="choose-an-approach">
  Elija un enfoque
</h2>

El enfoque correcto depende de quién coordina el trabajo, si los trabajadores necesitan comunicarse y si editan los mismos archivos:

* **¿Quién coordina el trabajo?**
  * Claude delega y recopila resultados dentro de una conversación: [subagentes](/docs/es/sub-agents)
  * Usted entrega tareas independientes y verifica más tarde: [vista de agentes](/docs/es/agent-view)
  * Claude planifica, asigna y supervisa un grupo de trabajadores: [equipos de agentes](/docs/es/agent-teams), experimental y deshabilitado por defecto
  * Un script mantiene el plan en lugar del juicio turno a turno de Claude: [flujos de trabajo dinámicos](/docs/es/workflows). Vea [cómo los flujos de trabajo se comparan con los subagentes y las skills](/docs/es/workflows#when-to-use-a-workflow)
* **¿Necesitan los trabajadores hablar entre sí?** Los subagentes reportan resultados nuevamente a la conversación que los generó, y las sesiones de vista de agentes reportan solo a usted. Los compañeros de equipo en un equipo de agentes comparten una lista de tareas y se envían mensajes directamente entre sí.
* **¿Tocan las tareas los mismos archivos?** Aísle el trabajo con [worktrees](/docs/es/worktrees). Los subagentes y las sesiones que ejecuta usted mismo pueden usar cada uno un worktree separado. Los equipos de agentes no aíslan a los compañeros de equipo en worktrees, así que [particione el trabajo](/docs/es/agent-teams#avoid-file-conflicts) para que cada compañero de equipo sea propietario de un conjunto diferente de archivos.

<h2 id="check-on-running-work">
  Verifique el trabajo en ejecución
</h2>

El comando para verificar el trabajo en ejecución depende de qué enfoque utilizó:

* Para sesiones en segundo plano, `claude agents` abre [vista de agentes](/docs/es/agent-view): una pantalla que muestra cada sesión, su estado y cuáles necesitan su entrada.
* Para subagentes en la sesión actual, los subagentes en segundo plano nombrados aparecen en la escritura de mención @- con su estado. A partir de v2.1.198, `/agents` ya no abre un panel; imprime un aviso que apunta a las ubicaciones de archivos de subagentes. Para [crear y editar subagentes personalizados](/docs/es/sub-agents#configure-subagents), pregunte a Claude o edite los archivos directamente. A pesar del nombre similar, `/agents` es separado de `claude agents`.
* Para cualquier cosa que se ejecute en segundo plano de la sesión actual, `/tasks` enumera cada elemento y le permite verificar, adjuntar o detener. La lista también incluye subagentes que han terminado.
* Para flujos de trabajo dinámicos, `/workflows` enumera ejecuciones en ejecución y completadas, la fase en la que se encuentra cada una y cuántos agentes han terminado.

Para una vista de escritorio de todas sus sesiones, consulte [sesiones paralelas en la aplicación de escritorio](/docs/es/desktop#work-in-parallel-with-sessions).

<h2 id="learn-more">
  Obtenga más información
</h2>

Cada guía a continuación cubre la configuración y configuración para un enfoque:

* [Crear subagentes personalizados](/docs/es/sub-agents): defina especialistas reutilizables y controle qué herramientas pueden usar.
* [Administrar agentes con vista de agentes](/docs/es/agent-view): envíe sesiones, observe su estado y adjunte cuando una lo necesite.
* [Orquestar equipos de agentes](/docs/es/agent-teams): configure un líder y compañeros de equipo, asigne tareas y revise su trabajo.
* [Orquestar flujos de trabajo dinámicos](/docs/es/workflows): ejecute un flujo de trabajo agrupado o haga que Claude escriba uno que ejecute muchos subagentes y verifique sus hallazgos entre sí.
* [Ejecutar sesiones paralelas con worktrees](/docs/es/worktrees): inicie Claude en un checkout aislado, controle qué se copia y limpie después.
