> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orquestar equipos de sesiones de Claude Code

> Coordine múltiples instancias de Claude Code trabajando juntas como un equipo, con tareas compartidas, mensajería entre agentes y gestión centralizada.

<Warning>
  Los equipos de agentes son experimentales y están deshabilitados por defecto. Habilítelos configurando `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` en su [settings.json](/docs/es/settings) o entorno. Sin esa variable, ningún equipo se configura al inicio de la sesión, no se escriben directorios de equipo, y Claude no genera ni propone compañeros de equipo. Los equipos de agentes tienen [limitaciones conocidas](#limitations) alrededor de la reanudación de sesiones, coordinación de tareas y comportamiento de apagado.
</Warning>

Los equipos de agentes le permiten coordinar múltiples instancias de Claude Code trabajando juntas. Una sesión actúa como el líder del equipo, coordinando el trabajo, asignando tareas y sintetizando resultados. Los compañeros de equipo trabajan de forma independiente, cada uno en su propia ventana de contexto, y se comunican directamente entre sí. También puede hablar con cualquier compañero de equipo directamente sin pasar por el líder.

Antes de configurar un equipo, verifique si una opción más ligera hace el trabajo. Los [subagentes](/docs/es/sub-agents) funcionan dentro de una única sesión, y con [mensajería entre sesiones](/docs/es/cross-session-messaging) Claude puede pasar hallazgos entre las sesiones que usted ejecuta por sí mismo.

<h2 id="when-to-use-agent-teams">
  Cuándo usar equipos de agentes
</h2>

Los equipos de agentes son más efectivos para tareas donde la exploración paralela agrega valor real. Vea [ejemplos de casos de uso](#use-case-examples) para escenarios completos. Los casos de uso más sólidos son:

* **Investigación y revisión**: múltiples compañeros de equipo pueden investigar diferentes aspectos de un problema simultáneamente, luego compartir y desafiar los hallazgos de los demás
* **Nuevos módulos o características**: los compañeros de equipo pueden poseer cada uno una pieza separada sin pisarse mutuamente
* **Depuración con hipótesis competidoras**: los compañeros de equipo prueban diferentes teorías en paralelo y convergen en la respuesta más rápidamente
* **Coordinación entre capas**: cambios que abarcan frontend, backend y pruebas, cada uno propiedad de un compañero de equipo diferente

Los equipos de agentes agregan sobrecarga de coordinación y usan significativamente más tokens que una única sesión. Funcionan mejor cuando los compañeros de equipo pueden operar de forma independiente. Para tareas secuenciales, ediciones del mismo archivo o trabajo con muchas dependencias, una única sesión o [subagents](/docs/es/sub-agents) son más efectivos.

<h3 id="compare-with-subagents">
  Comparar con subagents
</h3>

Tanto los equipos de agentes como los [subagents](/docs/es/sub-agents) le permiten paralelizar el trabajo, pero operan de manera diferente. Para sesiones separadas que pasen mensajes entre sí sin un equipo, vea [mensajería entre sesiones](/docs/es/cross-session-messaging).

<Frame caption="Los subagents reportan resultados al agente principal. En los equipos de agentes, los compañeros de equipo comparten una lista de tareas, reclaman trabajo y se comunican directamente entre sí.">
  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-light.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=2f8db9b4f3705dd3ab931fbe2d96e42a" className="dark:hidden" alt="Diagrama comparando arquitecturas de subagent y equipo de agentes. Los subagents son generados por el agente principal, hacen trabajo y reportan resultados. Los equipos de agentes se coordinan a través de una lista de tareas compartida, con compañeros de equipo comunicándose directamente entre sí." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-light.png" />

  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-dark.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=d573a037540f2ada6a9ae7d8285b46fd" className="hidden dark:block" alt="Diagrama comparando arquitecturas de subagent y equipo de agentes. Los subagents son generados por el agente principal, hacen trabajo y reportan resultados. Los equipos de agentes se coordinan a través de una lista de tareas compartida, con compañeros de equipo comunicándose directamente entre sí." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-dark.png" />
</Frame>

|                     | Subagents                                                                                                                                                               | Equipos de agentes                                                                                                                                                     |
| :------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Contexto**        | Ventana de contexto propia; los resultados regresan al llamador                                                                                                         | Ventana de contexto propia; completamente independiente                                                                                                                |
| **Comunicación**    | Devuelven un resultado al llamador. Los subagents que Claude nombró cuando los generó también pueden [enviarse mensajes entre sí](/docs/es/sub-agents#what-loads-at-startup) | Los compañeros de equipo se envían mensajes directamente                                                                                                               |
| **Coordinación**    | El agente principal gestiona todo el trabajo                                                                                                                            | Auto-coordinación a través de mensajes, más una lista de tareas compartida para [agentes que tienen las herramientas Task](/docs/es/tools-reference#task-tool-availability) |
| **Mejor para**      | Tareas enfocadas donde solo importa el resultado                                                                                                                        | Trabajo complejo que requiere discusión y colaboración                                                                                                                 |
| **Costo de tokens** | Menor: resultados resumidos de vuelta al contexto principal                                                                                                             | Mayor: cada compañero de equipo es una instancia Claude separada                                                                                                       |

Use subagents cuando necesite trabajadores rápidos y enfocados que reporten. Use equipos de agentes cuando los compañeros de equipo necesiten compartir hallazgos, desafiarse mutuamente y coordinarse por su cuenta.

<h2 id="enable-agent-teams">
  Habilitar equipos de agentes
</h2>

Los equipos de agentes están deshabilitados por defecto. Habilítelos configurando la variable de entorno `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` a `1`, ya sea en su entorno de shell o a través de [settings.json](/docs/es/settings):

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1"
  }
}
```

Habilitar equipos de agentes también cambia la delegación ordinaria. Claude puede [nombrar un subagente](/docs/es/sub-agents#subagent-names) por su cuenta, y mientras los equipos de agentes estén habilitados, un subagente que Claude nombra se inicia como un compañero de equipo, por lo que los equipos pueden formarse incluso cuando usted no solicitó uno. Para más información, consulte [Cómo Claude inicia equipos de agentes](#how-claude-starts-agent-teams); para desactivar el comportamiento, consulte [Claude genera compañeros de equipo en lugar de subagentes](#claude-spawns-teammates-instead-of-subagents).

Generar compañeros de equipo también requiere una sesión interactiva. En [modo no interactivo](/docs/es/headless) con la bandera `-p`, incluidas las sesiones de Agent SDK, Claude no genera compañeros de equipo, y un subagente que Claude nombra se ejecuta como un [subagente](/docs/es/sub-agents) ordinario incluso con equipos de agentes habilitados.

<h2 id="start-your-first-agent-team">
  Inicie su primer equipo de agentes
</h2>

Después de habilitar los equipos de agentes, describa la tarea y los compañeros de equipo que desea en lenguaje natural. Claude los genera y coordina el trabajo según su indicación.

Este ejemplo funciona bien porque los tres roles son independientes y pueden explorar el problema sin esperar el uno al otro:

```text wrap theme={null}
Estoy diseñando una herramienta CLI que ayuda a los desarrolladores a rastrear
comentarios TODO en su base de código. Genere tres compañeros de equipo para explorar
esto desde diferentes ángulos: uno en UX, uno en arquitectura técnica, uno jugando
al abogado del diablo.
```

A partir de ahí, Claude completa una [lista de tareas compartida](/docs/es/interactive-mode#task-list) en una [sesión que tiene las herramientas Task](/docs/es/tools-reference#task-tool-availability), genera compañeros de equipo para cada perspectiva, los hace explorar el problema, y sintetiza hallazgos cuando termina.

Claude a veces puede usar [subagentes](/docs/es/sub-agents) en lugar de crear un equipo. Los subagentes aparecen en el mismo panel de agentes que los compañeros de equipo, por lo que el panel solo no confirma que se formó un equipo. Si Claude generó subagentes en su lugar, pregunte de nuevo y solicite explícitamente un equipo de agentes.

El panel del agente del líder enumera los compañeros de equipo debajo de la entrada del indicador. Desde el panel:

* **Flechas arriba y abajo**: seleccionar un compañero de equipo
* **Intro**: abrir la transcripción del compañero de equipo seleccionado y enviarle un mensaje directamente
* **Escape**: borrar la selección. Mientras está viendo la transcripción de un compañero de equipo, Escape interrumpe el turno actual de ese compañero de equipo

A partir de v2.1.199, la fila de un compañero de equipo inactivo permanece en el panel mientras cualquier compañero de equipo o subagente siga trabajando, por lo que puede seleccionarlo para revisar su transcripción o enviarle más trabajo. Una vez que cada agente en el panel está inactivo, las filas inactivas se ocultan después de 30 segundos y reaparecen en el siguiente turno del compañero de equipo; el compañero de equipo sigue ejecutándose y es direccionable mientras está oculto. En v2.1.181 a v2.1.198, una fila inactiva se ocultaba 30 segundos después de que su propio turno terminaba, incluso mientras otros compañeros de equipo seguían trabajando; las filas inactivas no se ocultan en versiones anteriores a v2.1.181.

Cuando más de tres compañeros de equipo están inactivos a la vez, las filas más allá de las primeras tres se contraen en una sola fila que cuenta los compañeros de equipo contraídos, como `2 idle agents` cuando cinco están inactivos. Selecciónela y presione Intro para expandir las filas contraídas, o presione Esc para contraerlas de nuevo. Los compañeros de equipo que trabajan, los compañeros de equipo que fallaron, y el compañero de equipo que está viendo siempre mantienen sus propias filas.

Si desea que cada compañero de equipo esté en su propio panel dividido, vea [Elegir un modo de visualización](#choose-a-display-mode).

<h2 id="control-your-agent-team">
  Controle su equipo de agentes
</h2>

Dígale al líder lo que desea en lenguaje natural. Maneja la coordinación del equipo, asignación de tareas y delegación según sus instrucciones.

<h3 id="choose-a-display-mode">
  Elegir un modo de visualización
</h3>

Los equipos de agentes admiten dos modos de visualización:

* **En proceso**: todos los compañeros de equipo se ejecutan dentro de su terminal principal. Use las teclas de flecha arriba y abajo en el panel del agente para seleccionar un compañero de equipo, luego presione Intro para verlo y escriba para enviarle un mensaje directamente. Funciona en cualquier terminal, sin configuración adicional requerida.
* **Paneles divididos**: cada compañero de equipo obtiene su propio panel. Puede ver la salida de todos a la vez y hacer clic en un panel para interactuar directamente. Requiere tmux o iTerm2.

<Note>
  `tmux` tiene limitaciones conocidas en ciertos sistemas operativos y tradicionalmente funciona mejor en macOS. Usar `tmux -CC` en iTerm2 es el punto de entrada sugerido en `tmux`.
</Note>

El valor predeterminado es `"in-process"`. Configure `"auto"` para habilitar paneles divididos cuando ya esté ejecutándose dentro de una sesión tmux, o cuando su terminal sea iTerm2 con la CLI `it2` instalada, retrocediendo a en proceso de lo contrario. La configuración `"tmux"` habilita el modo de panel dividido y detecta automáticamente si usar tmux o iTerm2 según su terminal.

Configure `"iterm2"` para usar explícitamente paneles divididos nativos de iTerm2. Este modo requiere la [CLI `it2`](https://github.com/mkusaka/it2) y muestra un error con el comando de instalación si falta `it2`. El indicador de configuración que ofrece instalar `it2` o cambiar a tmux aparece bajo `"auto"` o `"tmux"` cuando su terminal es iTerm2 y tmux está disponible como alternativa.

Para anular el valor predeterminado, configure [`teammateMode`](/docs/es/settings-reference#teammatemode) en `~/.claude/settings.json`:

```json theme={null}
{
  "teammateMode": "auto"
}
```

Para establecer el modo para una única sesión, páselo como una bandera:

```bash theme={null}
claude --teammate-mode auto
```

El indicador `--teammate-mode` es experimental y no aparece en `claude --help`.

El modo de panel dividido requiere [tmux](https://github.com/tmux/tmux/wiki) o iTerm2 con la [CLI `it2`](https://github.com/mkusaka/it2). Para instalar manualmente:

* **tmux**: instale a través del gestor de paquetes de su sistema. Vea la [wiki de tmux](https://github.com/tmux/tmux/wiki/Installing) para instrucciones específicas de la plataforma.
* **iTerm2**: instale la [CLI `it2`](https://github.com/mkusaka/it2), luego habilite la API de Python en **iTerm2 → Settings → General → Magic → Enable Python API**.

<h3 id="specify-teammates-and-models">
  Especificar compañeros de equipo y modelos
</h3>

Claude decide el número de compañeros de equipo a generar según su tarea, o puede especificar exactamente lo que desea:

```text wrap theme={null}
Spawn 4 teammates to refactor these modules in parallel. Use Sonnet for
each teammate.
```

Claude Code elige el modelo de cada compañero de equipo del primero de estos que se aplique:

1. El modelo que su indicación de generación nombra para ese compañero de equipo.
2. Para un compañero de equipo generado a partir de una [definición de subagente](#use-subagent-definitions-for-teammates), el `model` de la definición, donde `inherit` selecciona el modelo del líder.
3. [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/es/model-config#environment-variables), cuando está configurado en algo distinto de `inherit`.
4. El modelo actual del líder.

Si configura [`CLAUDE_CODE_SUBAGENT_MODEL_FORCE=1`](/docs/es/sub-agents#run-every-subagent-on-one-model), las dos primeras fuentes no se aplican. Claude Code elige el modelo de cada compañero de equipo de `CLAUDE_CODE_SUBAGENT_MODEL` cuando está configurado en algo distinto de `inherit`, y del modelo actual del líder de lo contrario. Requiere Claude Code v2.1.257 o posterior.

Antes de v2.1.251, `CLAUDE_CODE_SUBAGENT_MODEL` venía primero en este orden.

<Note>
  `teammateDefaultModel` fue eliminado en v2.1.234; Claude Code ignora un valor restante. Nombre el modelo en su indicación en su lugar.
</Note>

Claude Code verifica el modelo que selecciona para un compañero de equipo contra la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización. Cuando la lista de permitidos bloquea un valor, Claude Code sustituye otro modelo:

* **Alias de familia como `opus`**: En la API de Anthropic y Claude Platform en AWS, Claude Code ejecuta el compañero de equipo en la versión más nueva de esa familia que la lista de permitidos permite. En proveedores con IDs de modelo específicos del proveedor, donde la [sustitución no opera](/docs/es/model-config#restrict-model-selection), un alias bloqueado retrocede como cualquier otro valor bloqueado según la siguiente viñeta
* **Cualquier otro valor bloqueado, incluido un alias de familia en proveedores donde la sustitución no opera, o uno cuya familia no tiene versión permitida**: Claude Code ejecuta el compañero de equipo en el modelo del líder en su lugar. Si configura `CLAUDE_CODE_SUBAGENT_MODEL`, Claude Code intenta ese modelo primero, bajo estas mismas reglas

Los compañeros de equipo heredan el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) del líder. En modo de panel dividido esto se aplica desde v2.1.186; las versiones anteriores no pasaban el esfuerzo de sesión del líder a los compañeros de equipo de panel dividido.

<h3 id="have-teammates-plan-before-implementing">
  Que los compañeros de equipo planifiquen antes de implementar
</h3>

Para tareas complejas o riesgosas, puede hacer que los compañeros de equipo planifiquen antes de implementar. Un compañero de equipo que Claude genera mientras el líder está en [modo de plan](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) funciona en modo de plan de solo lectura hasta que su plan esté listo. Cambie el líder al modo de plan primero, luego pida el compañero de equipo:

```text wrap theme={null}
Spawn an architect teammate to refactor the authentication module.
```

Cuando un compañero de equipo termina de planificar, envía una solicitud de aprobación de plan al líder. Claude Code aprueba el plan en la sesión del líder tan pronto como llega la solicitud, sin que el líder lo revise. Los edits y comandos del compañero de equipo aún pasan por los indicadores de permiso descritos en [Permisos](#permissions). Una vez aprobado, el compañero de equipo sale del modo de plan y comienza la implementación.

<h3 id="talk-to-teammates-directly">
  Hable directamente con los compañeros de equipo
</h3>

Cada compañero de equipo es una sesión completa e independiente de Claude Code. Puede enviar un mensaje a cualquier compañero de equipo directamente para dar instrucciones adicionales, hacer preguntas de seguimiento o redirigir su enfoque.

* **Modo en proceso**: use las teclas de flecha arriba y abajo en el panel del agente para seleccionar un compañero de equipo, luego presione Intro para ver su sesión y escriba para enviarle un mensaje. Presione `x` en un compañero de equipo seleccionado para detenerlo. Presione Ctrl+T para alternar la lista de tareas.
* **Modo de panel dividido**: haga clic en el panel de un compañero de equipo para interactuar directamente con su sesión. Cada compañero de equipo tiene una vista completa de su propio terminal.

Mientras está viendo un compañero de equipo en proceso, el texto sin formato y las [skills](/docs/es/skills) van a ese compañero de equipo, pero los comandos integrados aún se ejecutan en la sesión del líder.

El modelo y el modo rápido de un compañero de equipo se fijan cuando se genera, por lo que `/model` y `/fast` solo cambian la configuración del líder. A partir de v2.1.199, escribir cualquiera de estos comandos mientras se ve un compañero de equipo muestra un aviso de que el cambio se aplica al líder; las versiones anteriores lo aplicaban al líder sin indicación. `/effort` aún se aplica a los turnos posteriores del compañero de equipo visto, porque los compañeros de equipo siguen el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) del líder.

<h3 id="assign-and-claim-tasks">
  Asignar y reclamar tareas
</h3>

La lista de tareas compartida coordina el trabajo en todo el equipo. El líder crea tareas y los compañeros de equipo las trabajan. Las tareas tienen tres estados: pendiente, en progreso y completada. Las tareas también pueden depender de otras tareas: una tarea pendiente con dependencias sin resolver no puede ser reclamada hasta que esas dependencias se completen.

Los agentes [sin las herramientas Task](/docs/es/tools-reference#task-tool-availability) coordinan a través de mensajes en su lugar de la lista de tareas compartida.

El líder puede asignar tareas explícitamente, o los compañeros de equipo pueden auto-reclamar:

* **El líder asigna**: dígale al líder qué tarea dar a qué compañero de equipo
* **Auto-reclamar**: después de terminar una tarea, un compañero de equipo recoge la siguiente tarea sin asignar y sin bloquear por su cuenta

El reclamo de tareas usa bloqueo de archivos para prevenir condiciones de carrera cuando múltiples compañeros de equipo intentan reclamar la misma tarea simultáneamente.

<h3 id="shut-down-teammates">
  Apagar compañeros de equipo
</h3>

Para terminar gracefully la sesión de un compañero de equipo, refiriéndose a él por nombre. Por ejemplo, con un compañero de equipo llamado investigador:

```text wrap theme={null}
Ask the researcher teammate to shut down
```

El líder envía una solicitud de apagado. El compañero de equipo puede aprobar, saliendo gracefully, o rechazar con una explicación.

Los directorios compartidos del equipo se limpian automáticamente cuando finaliza la sesión, por lo que no hay un paso de limpieza separado. Vea [Arquitectura](#architecture) para saber qué directorios se eliminan y cuáles persisten para sesiones reanudadas.

<h3 id="enforce-quality-gates-with-hooks">
  Aplicar puertas de calidad con hooks
</h3>

Use [hooks](/docs/es/hooks) para aplicar reglas cuando los compañeros de equipo terminen el trabajo o las tareas se creen o completen:

* [`TeammateIdle`](/docs/es/hooks#teammateidle): se ejecuta cuando un compañero de equipo está a punto de quedarse inactivo. Salga con código 2 para enviar retroalimentación y mantener al compañero de equipo trabajando.
* [`TaskCreated`](/docs/es/hooks#taskcreated): se ejecuta cuando una tarea está siendo creada. Salga con código 2 para prevenir la creación y enviar retroalimentación.
* [`TaskCompleted`](/docs/es/hooks#taskcompleted): se ejecuta cuando una tarea está siendo marcada como completada. Salga con código 2 para prevenir la finalización y enviar retroalimentación.

<h2 id="how-agent-teams-work">
  Cómo funcionan los equipos de agentes
</h2>

Esta sección cubre la arquitectura y la mecánica detrás de los equipos de agentes. Si desea comenzar a usarlos, vea [Controle su equipo de agentes](#control-your-agent-team) arriba.

<h3 id="how-claude-starts-agent-teams">
  Cómo Claude inicia equipos de agentes
</h3>

Para iniciar un equipo, solicite compañeros de equipo a Claude. Claude lanza un compañero de equipo cuando llama a la [herramienta Agent](/docs/es/tools-reference) con un [`name`](/docs/es/sub-agents#subagent-names) mientras los equipos de agentes están habilitados, a menos que la llamada sea un [fork](/docs/es/sub-agents#fork-the-current-conversation) o pase `isolation` en la llamada misma. Claude Code no le pide que confirme el lanzamiento.

Claude también nombra subagentes ordinarios por su cuenta para poder enviarles mensajes más tarde. Esas llamadas siguen la misma regla, por lo que los equipos pueden formarse incluso cuando no solicitó uno. Si prefiere subagentes en su lugar, [desactive los equipos de agentes](#claude-spawns-teammates-instead-of-subagents).

<h3 id="architecture">
  Arquitectura
</h3>

Un equipo de agentes consiste en:

| Componente               | Rol                                                                                        |
| :----------------------- | :----------------------------------------------------------------------------------------- |
| **Líder del equipo**     | La sesión principal de Claude Code que genera compañeros de equipo y coordina el trabajo   |
| **Compañeros de equipo** | Instancias separadas de Claude Code que cada una trabaja en tareas asignadas               |
| **Lista de tareas**      | Lista compartida de elementos de trabajo que los compañeros de equipo reclaman y completan |
| **Buzón**                | Sistema de mensajería para comunicación entre agentes                                      |

El buzón de cada agente es un archivo JSON en `~/.claude/teams/{team-name}/inboxes/{agent-name}.json`. Claude Code valida cada entrada cuando lee un archivo de buzón. Las entradas que no coinciden con el formato de mensaje se reportan como errores y se eliminan del archivo; los mensajes válidos aún se entregan. Antes de v2.1.207, una única entrada de buzón malformada causaba un error repetido cada segundo y bloqueaba la entrega para ese buzón hasta que eliminara manualmente el archivo.

Claude Code reporta un mensaje como enviado solo cuando la escritura en el archivo de buzón del destinatario tiene éxito, ya sea que el mensaje sea texto plano o un mensaje de protocolo estructurado como una aprobación de plan o una solicitud de cierre. Cuando la escritura falla, por ejemplo porque el disco está lleno o el directorio de buzón no es escribible, el agente remitente recibe un error y nada se envía. Vea [No se pudo escribir en la bandeja de entrada de un compañero de equipo](/docs/es/errors#failed-to-write-to-a-teammate-inbox) para los mensajes de error y pasos de recuperación.

Claude Code gestiona las dependencias de tareas automáticamente: cuando un compañero de equipo completa una tarea de la que otras tareas dependen, desbloquea las tareas dependientes sin ninguna acción de su parte.

Los equipos y tareas se almacenan localmente bajo un nombre derivado de la sesión. El nombre es `session-` seguido de los primeros ocho caracteres del ID de sesión:

* **Configuración del equipo**: `~/.claude/teams/{team-name}/config.json`
* **Lista de tareas**: `~/.claude/tasks/{team-name}/`

Claude Code genera ambos automáticamente al inicio de la sesión y los actualiza a medida que los compañeros de equipo se unen, se quedan inactivos o se van. El directorio de configuración del equipo se elimina cuando la sesión termina. El directorio de lista de tareas persiste localmente y nunca se carga, por lo que las sesiones reanudadas mantienen sus tareas. La retención se rige por el mismo [`cleanupPeriodDays`](/docs/es/settings-reference#cleanupperioddays) que ya controla para transcripciones de sesión, siguiendo las [reglas de barrido de retención](/docs/es/claude-directory#cleaned-up-automatically).

La configuración del equipo contiene estado de tiempo de ejecución como IDs de sesión e IDs de panel tmux, así que no la edite manualmente ni la pre-autorice: sus cambios se sobrescriben en la siguiente actualización de estado.

Para definir roles de compañeros de equipo reutilizables, use [definiciones de subagents](#use-subagent-definitions-for-teammates) en su lugar.

La configuración del equipo contiene un array `members` con el nombre de cada miembro y el ID del agente. La entrada del líder siempre lleva el tipo de agente `team-lead`. La entrada de un compañero de equipo lleva cualquier tipo de agente que el líder nombró al generarlo, ya sea un [tipo integrado](/docs/es/sub-agents#built-in-subagents) o una [definición de subagente](#use-subagent-definitions-for-teammates), y omite el campo cuando el líder no nombró ninguno. Los compañeros de equipo pueden leer este archivo para descubrir otros miembros del equipo.

No hay equivalente a nivel de proyecto de la configuración del equipo. Un archivo como `.claude/teams/teams.json` en su directorio de proyecto no se reconoce como configuración; Claude lo trata como un archivo ordinario.

<h3 id="use-subagent-definitions-for-teammates">
  Usar definiciones de subagents para compañeros de equipo
</h3>

Cuando genera un compañero de equipo en cualquier modo de visualización, puede hacer referencia a un tipo de [subagente](/docs/es/sub-agents) del proyecto, usuario o alcance de [subagente](/docs/es/sub-agents#choose-the-subagent-scope) administrado. Esto le permite definir un rol una vez, como un revisor de seguridad o ejecutor de pruebas, y reutilizarlo tanto como un subagente delegado como un compañero de equipo de equipo de agentes.

Para usar una definición de subagente, mencione por nombre cuando le pida a Claude que genere el compañero de equipo:

```text wrap theme={null}
Genere un compañero de equipo usando el tipo de agente security-reviewer para auditar el módulo de autenticación.
```

Claude Code lee la definición de subagente que nombró y aplica estas partes a él. Donde una parte depende del [modo de visualización](#choose-a-display-mode) del compañero de equipo, la entrada lo indica:

* **`tools`**: Claude Code limita al compañero de equipo a las herramientas en la lista `tools` de la definición. Para un compañero de equipo en proceso, Claude Code añade `SendMessage` a esa lista, y en una [sesión que tiene las herramientas Task](/docs/es/tools-reference#task-tool-availability) también añade `TaskCreate`, `TaskGet`, `TaskList` y `TaskUpdate`.
* **`model`**: Claude Code usa el `model` de la definición en cualquier modo de visualización cuando su indicación de generación no nombra uno. Vea [cómo Claude Code elige el modelo de un compañero de equipo](#specify-teammates-and-models).
* **Body**: para un compañero de equipo en proceso, Claude Code añade el cuerpo de la definición a su prompt del sistema predeterminado como instrucciones adicionales. Para un compañero de equipo de panel dividido, Claude Code usa el cuerpo en lugar de su prompt del sistema predeterminado.
* **`skills`**: Claude Code no aplica el `skills` de la definición a un compañero de equipo en ningún modo de visualización. El compañero de equipo carga skills desde su configuración de proyecto y usuario.
* **`mcpServers`**: para un compañero de equipo de panel dividido, Claude Code aplica el `mcpServers` de la definición bajo las [reglas para ese campo](/docs/es/sub-agents#scope-mcp-servers-to-a-subagent), que cubren una sesión iniciada con `--agent` también. Un compañero de equipo en proceso ignora el campo y carga servidores MCP desde su configuración de proyecto y usuario.

Cuando Claude envía un mensaje a un compañero de equipo en proceso que ya no se está ejecutando, Claude Code lo trae de vuelta en la misma sesión, restaura cualquier conversación guardada para él, y le da el mensaje como su siguiente indicación. Después de reanudar una sesión, los compañeros de equipo no se traen de vuelta de esta manera, según [la limitación de reanudación](#limitations).

Para un compañero de equipo que trae de vuelta, Claude Code vuelve a aplicar una definición que provino del directorio `.claude/agents/` de un proyecto o un directorio `--add-dir` solo si ha [confiado en la carpeta en la que se encuentra el archivo del agente](/docs/es/permissions#what-runs-before-you-trust-a-folder). Confiar en una carpeta padre no cuenta. Hasta entonces, el compañero de equipo vuelve con ninguna de las herramientas o instrucciones de la definición, manteniendo solo las herramientas que Claude Code añade a cada compañero de equipo en proceso. Vea [la definición del agente del compañero de equipo no fue restaurada](/docs/es/errors#teammate-agent-definition-not-restored) para el texto de notificación.

<h3 id="permissions">
  Permisos
</h3>

Los compañeros de equipo comienzan con el modo de permiso del líder, excepto el modo [`dontAsk`](/docs/es/permission-modes#allow-only-pre-approved-tools-with-dontask-mode), que no heredan. Si el líder se ejecuta con `--dangerously-skip-permissions`, todos los compañeros de equipo también lo hacen. Después de generar, puede cambiar el modo de permiso de un compañero de equipo individual, pero no puede establecer modos de permiso por compañero de equipo en el momento de la generación.

Las solicitudes de permiso de compañeros de equipo aparecen en la sesión del líder, así que apruébelas allí usted mismo. [Aprobación de plan](#have-teammates-plan-before-implementing) es la excepción diseñada: la sesión del líder otorga aprobaciones de plan de compañeros de equipo sin una solicitud separada para usted.

<h4 id="messages-between-agents">
  Mensajes entre agentes
</h4>

Cuando un agente envía un mensaje a otro a través de `SendMessage`, Claude Code le dice al agente receptor que el mensaje provino de otra sesión de Claude, no de usted. Un compañero de equipo no puede aprobar una solicitud de permiso o proporcionar consentimiento en su nombre, y un compañero de equipo al que se le negó una acción no puede retransmitirla a otro compañero de equipo para eludir la verificación. Las mismas reglas se aplican a un mensaje que llega desde [una de sus otras sesiones de Claude Code](/docs/es/cross-session-messaging#how-a-session-treats-an-incoming-message), fuera del equipo completamente.

En [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), el clasificador aplica dos verificaciones a los mensajes entre agentes:

* Trata una afirmación de aprobación retransmitida desde otro agente como entrada no confiable en lugar de confirmación de usted.
* Revisa cada mensaje antes de que Claude Code lo entregue, ya sea un mensaje plano o un mensaje de protocolo estructurado como una solicitud de cierre o una respuesta de aprobación de plan. Un mensaje que bloquea nunca llega al destinatario.

<h3 id="context-and-communication">
  Contexto y comunicación
</h3>

Cada compañero de equipo tiene su propia ventana de contexto. Cuando se genera, un compañero de equipo carga el mismo contexto de proyecto que una sesión regular: CLAUDE.md, MCP servers y skills. También recibe la indicación de generación del líder. El historial de conversación del líder no se transfiere.

**Cómo los compañeros de equipo comparten información:**

* **Entrega automática de mensajes**: cuando los compañeros de equipo envían mensajes, se entregan automáticamente a los destinatarios. El líder no necesita sondear actualizaciones.
* **Notificaciones de inactividad**: cuando un compañero de equipo termina y se detiene, notifica automáticamente al líder e incluye su respuesta final en la notificación. Un compañero de equipo cuyo turno termina en un error de API notifica al líder que falló e incluye el texto del error.
* **Lista de tareas compartida**: [agentes que tienen las herramientas Task](/docs/es/tools-reference#task-tool-availability) pueden ver el estado de la tarea y reclamar trabajo disponible.
* **Mensajería de compañeros de equipo**: enviar un mensaje a un compañero de equipo específico por nombre. Para llegar a todos, envíe un mensaje por destinatario.

El líder asigna a cada compañero de equipo un nombre cuando lo genera, y cualquier compañero de equipo puede enviar un mensaje a otro por ese nombre. Para obtener nombres predecibles que pueda referenciar en indicaciones posteriores, dígale al líder cómo llamar a cada compañero de equipo en su instrucción de generación.

<h3 id="token-usage">
  Uso de tokens
</h3>

Los equipos de agentes usan significativamente más tokens que una única sesión. Cada compañero de equipo tiene su propia ventana de contexto, y el uso de tokens escala con el número de compañeros de equipo activos. Para investigación, revisión y trabajo de nuevas características, los tokens adicionales generalmente valen la pena. Para tareas rutinarias, una única sesión es más rentable. Vea [costos de tokens de equipos de agentes](/docs/es/costs#agent-team-token-costs) para orientación de uso.

El cache de prompt de un compañero de equipo en proceso cae fuera del bucket TTL de cache de la conversación principal]\(/es/prompt-caching#which-ttl-each-request-gets), por lo que su cache se mantiene durante cinco minutos por defecto, incluso en una suscripción de Claude. Para mantenerlo durante una hora, establezca [`subagentPromptCacheTtl`](/docs/es/settings-reference#subagentpromptcachettl) en `1h`. La API factura escrituras de cache de 1 hora a una tasa más alta.

<h2 id="use-case-examples">
  Ejemplos de casos de uso
</h2>

Estos ejemplos muestran cómo los equipos de agentes manejan tareas donde la exploración paralela agrega valor.

<h3 id="run-a-parallel-code-review">
  Ejecutar una revisión de código paralela
</h3>

Un único revisor tiende a gravitar hacia un tipo de problema a la vez. Dividir criterios de revisión en dominios independientes significa que la seguridad, el rendimiento y la cobertura de pruebas reciben atención exhaustiva simultáneamente. La indicación asigna a cada compañero de equipo una lente distinta para que no se superpongan:

```text wrap theme={null}
Spawn three teammates to review PR #142:
- One focused on security implications
- One checking performance impact
- One validating test coverage
Have them each review and report findings.
```

Cada revisor trabaja desde la misma PR pero aplica un filtro diferente. El líder sintetiza hallazgos en los tres después de que terminen.

<h3 id="investigate-with-competing-hypotheses">
  Investigar con hipótesis competidoras
</h3>

Cuando la causa raíz es poco clara, un único agente tiende a encontrar una explicación plausible y dejar de buscar. La indicación lucha contra esto haciendo que los compañeros de equipo sean explícitamente adversarios: el trabajo de cada uno no es solo investigar su propia teoría sino desafiar las de los demás.

```text wrap theme={null}
Users report the app exits after one message instead of staying connected.
Spawn 5 agent teammates to investigate different hypotheses. Have them talk to
each other to try to disprove each other's theories, like a scientific
debate. Update the findings doc with whatever consensus emerges.
```

La estructura de debate es el mecanismo clave aquí. La investigación secuencial sufre de anclaje: una vez que se explora una teoría, la investigación posterior está sesgada hacia ella.

Con múltiples investigadores independientes intentando activamente refutar mutuamente, la teoría que sobrevive es mucho más probable que sea la causa raíz real.

<h2 id="best-practices">
  Mejores prácticas
</h2>

<h3 id="give-teammates-enough-context">
  Dé a los compañeros de equipo suficiente contexto
</h3>

Los compañeros de equipo cargan contexto de proyecto automáticamente, incluyendo CLAUDE.md, MCP servers y skills, pero no heredan el historial de conversación del líder. Vea [Contexto y comunicación](#context-and-communication) para detalles. Incluya detalles específicos de la tarea en la indicación de generación:

```text wrap theme={null}
Spawn a security reviewer teammate with the prompt: "Review the authentication module
at src/auth/ for security vulnerabilities. Focus on token handling, session
management, and input validation. The app uses JWT tokens stored in
httpOnly cookies. Report any issues with severity ratings."
```

<h3 id="choose-an-appropriate-team-size">
  Elegir un tamaño de equipo apropiado
</h3>

No hay límite duro en el número de compañeros de equipo, pero se aplican restricciones prácticas:

* **Los costos de tokens escalan linealmente**: cada compañero de equipo tiene su propia ventana de contexto y consume tokens independientemente. Vea [costos de tokens de equipos de agentes](/docs/es/costs#agent-team-token-costs) para detalles.
* **La sobrecarga de coordinación aumenta**: más compañeros de equipo significa más comunicación, coordinación de tareas y potencial para conflictos
* **Rendimientos decrecientes**: más allá de cierto punto, compañeros de equipo adicionales no aceleran el trabajo proporcionalmente

Comience con 3-5 compañeros de equipo para la mayoría de flujos de trabajo. Esto equilibra el trabajo paralelo con coordinación manejable. Si tiene 15 tareas independientes, 3 compañeros de equipo es un buen punto de partida.

Escale solo cuando el trabajo se beneficie de tener compañeros de equipo trabajando simultáneamente. Tres compañeros de equipo enfocados a menudo superan a cinco dispersos.

<h3 id="size-tasks-appropriately">
  Dimensionar tareas apropiadamente
</h3>

* **Demasiado pequeñas**: la sobrecarga de coordinación excede el beneficio
* **Demasiado grandes**: los compañeros de equipo trabajan demasiado tiempo sin check-ins, aumentando el riesgo de esfuerzo desperdiciado
* **Justo bien**: unidades auto-contenidas que producen un entregable claro, como una función, un archivo de prueba o una revisión

<Tip>
  El líder divide el trabajo en tareas y las asigna a los compañeros de equipo automáticamente. Si no está creando suficientes tareas, pídele que divida el trabajo en piezas más pequeñas. Tener 5-6 tareas por compañero de equipo mantiene a todos productivos y permite al líder reasignar trabajo si alguien se queda atrapado.
</Tip>

<h3 id="wait-for-teammates-to-finish">
  Espere a que los compañeros de equipo terminen
</h3>

A veces el líder comienza a implementar tareas por sí mismo en lugar de esperar a los compañeros de equipo. Si nota esto:

```text wrap theme={null}
Wait for your teammates to complete their tasks before proceeding
```

<h3 id="start-with-research-and-review">
  Comience con investigación y revisión
</h3>

Si es nuevo en equipos de agentes, comience con tareas que tengan límites claros y no requieran escribir código: revisar una PR, investigar una biblioteca o investigar un error. Estas tareas muestran el valor de la exploración paralela sin los desafíos de coordinación que vienen con la implementación paralela.

<h3 id="avoid-file-conflicts">
  Evitar conflictos de archivos
</h3>

Dos compañeros de equipo editando el mismo archivo lleva a sobrescrituras. Divida el trabajo para que cada compañero de equipo posea un conjunto diferente de archivos.

<h3 id="monitor-and-steer">
  Monitorear y dirigir
</h3>

Verifique el progreso de los compañeros de equipo, redirija enfoques que no estén funcionando y sintetice hallazgos a medida que lleguen. Dejar que un equipo se ejecute desatendido durante demasiado tiempo aumenta el riesgo de esfuerzo desperdiciado.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="teammates-not-appearing">
  Los compañeros de equipo no aparecen
</h3>

Si los compañeros de equipo no aparecen después de que le pida a Claude que cree un equipo:

* En modo en proceso, los compañeros de equipo aparecen en el panel del agente debajo de la entrada del mensaje. Use las teclas de flecha arriba y abajo para seleccionar uno, luego presione Intro para verlo.
* Una fila de compañero de equipo que desapareció después de estar inactiva ha sido ocultada, no detenida. Las filas inactivas se ocultan 30 segundos después de que todo el panel se queda inactivo y reaparecen en el siguiente turno del compañero de equipo. Cuando más de tres compañeros de equipo están inactivos, sus filas excedentes se contraen en una única fila `N idle agents` que Intro expande. Envíe un mensaje al compañero de equipo por nombre para traer de vuelta una fila oculta.
* Verifique que la tarea que le dio a Claude fue lo suficientemente compleja para justificar un equipo. Claude decide si generar compañeros de equipo según la tarea.
* Si solicitó explícitamente paneles divididos, asegúrese de que tmux esté instalado y disponible en su PATH:
  ```bash theme={null}
  which tmux
  ```
* Para iTerm2, verifique que la CLI `it2` esté instalada y la API de Python esté habilitada en las preferencias de iTerm2.

<h3 id="claude-spawns-teammates-instead-of-subagents">
  Claude genera compañeros de equipo en lugar de subagentes
</h3>

Mientras los equipos de agentes estén habilitados, un subagente que Claude nombra en la sesión del líder se inicia como un compañero de equipo. Claude [puede nombrar subagentes por su cuenta](#how-claude-starts-agent-teams), por lo que esto puede suceder durante la delegación que nunca enmarcó como trabajo en equipo.

Para hacer que los subagentes nombrados se inicien como subagentes nuevamente, desactive los equipos de agentes configurando `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` a `0`:

```json settings.json theme={null}
{
  "env": {
    "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "0"
  }
}
```

No necesita iniciar una nueva sesión: Claude Code reaplica los valores `env` del archivo de configuración a la sesión en ejecución cuando guarda, y relee la variable cada vez que Claude genera un subagente, por lo que el siguiente subagente que Claude nombra se inicia como un subagente.

Configurar la variable a `0` en su `settings.json` de usuario anula una exportación de shell. Otras fuentes de configuración aún pueden habilitar equipos de agentes:

* **Archivos de configuración de mayor precedencia**: la configuración del proyecto, la configuración local y una carga `--settings` se aplican después de la configuración del usuario, por lo que una entrada `env` que establece la variable a `1` en cualquiera de ellas gana. Consulte [Precedencia de configuración](/docs/es/settings#settings-precedence).
* **Configuración administrada**: la [configuración administrada](/docs/es/server-managed-settings) se aplica después de todas las otras fuentes. Si su organización habilita equipos de agentes allí, pida a su administrador que cambie el valor administrado.

Después del cambio, Claude aún puede nombrar subagentes, y el nombre sigue funcionando como una [dirección `SendMessage`](/docs/es/sub-agents#resume-subagents). Claude recibe el resultado de cada subagente cuando se completa.

<h3 id="too-many-permission-prompts">
  Demasiados avisos de permisos
</h3>

Las solicitudes de permisos de compañeros de equipo suben al líder, lo que puede crear fricción. Pre-apruebe operaciones comunes en su [configuración de permisos](/docs/es/permissions) antes de generar compañeros de equipo para reducir interrupciones.

<h3 id="agents-stopping-early">
  Los agentes se detienen antes de tiempo
</h3>

Los compañeros de equipo pueden detenerse después de encontrar errores en lugar de recuperarse. Verifique su salida seleccionando el compañero de equipo en el panel del agente y presionando Intro en modo en proceso, o haciendo clic en el panel en modo dividido, luego:

* Deles instrucciones adicionales directamente
* Genere un compañero de equipo de reemplazo para continuar el trabajo

Un mensaje del líder u otro compañero de equipo despierta a un compañero de equipo en proceso que está esperando reintentar una solicitud de API fallida, por lo que lo reintenta inmediatamente en lugar de esperar el retraso de reintento completo.

El líder también puede detenerse antes de tiempo, decidiendo que el equipo está terminado antes de que todas las tareas estén realmente completas. Si eso sucede, dígale que continúe.

<h3 id="orphaned-tmux-sessions">
  Sesiones tmux huérfanas
</h3>

Si una sesión tmux persiste después de que el equipo termina, puede no haber sido completamente limpiada. Enumere sesiones y mate la creada por el equipo:

```bash theme={null}
tmux ls
tmux kill-session -t <session-name>
```

<h2 id="limitations">
  Limitaciones
</h2>

Los equipos de agentes son experimentales. Las limitaciones actuales a tener en cuenta:

* **Sin reanudación de sesión con compañeros de equipo en proceso**: `/resume` y `/rewind` no restauran compañeros de equipo en proceso. Después de reanudar una sesión, el líder puede intentar enviar mensajes a compañeros de equipo que ya no existen. Si esto sucede, dígale al líder que genere nuevos compañeros de equipo.
* **El estado de la tarea puede retrasarse**: los compañeros de equipo a veces no marcan las tareas como completadas, lo que bloquea tareas dependientes. Si una tarea parece atrapada, verifique si el trabajo está realmente hecho y actualice el estado de la tarea manualmente o dígale al líder que empuje al compañero de equipo.
* **El apagado puede ser lento**: los compañeros de equipo terminan su solicitud actual o llamada de herramienta antes de apagarse, lo que puede tomar tiempo.
* **Un equipo por sesión**: una sesión tiene exactamente un equipo, limitado a esa sesión. No puede crear equipos nombrados adicionales ni compartir un equipo entre sesiones.
* **Sin equipos anidados**: los compañeros de equipo no pueden generar sus propios compañeros de equipo. Solo el líder puede gestionar el equipo.
* **Sin subagentes de fondo de compañeros de equipo en proceso**: los propios subagentes de un compañero de equipo en proceso se ejecutan en primer plano, porque el trabajo de fondo de un compañero de equipo no puede sobrevivir al proceso del líder. Claude Code devuelve un error cuando un compañero de equipo genera un subagente cuya definición establece `background: true`. Una solicitud `run_in_background: true` de un compañero de equipo también falla, ya sea con un error o ejecutándose silenciosamente en primer plano, como se describe en [cómo Claude Code elige primer plano o fondo](/docs/es/sub-agents#run-subagents-in-foreground-or-background). Los subagentes lanzados desde la conversación principal siguen el [valor predeterminado de fondo](/docs/es/sub-agents#run-subagents-in-foreground-or-background).
* **El líder es fijo**: la sesión principal es el líder de por vida. No puede promover un compañero de equipo a líder ni transferir liderazgo.
* **Permisos establecidos en la generación**: los compañeros de equipo comienzan con el modo de permiso descrito en [Permisos](#permissions). Puede cambiar el modo de permiso de un compañero de equipo individual después de generarlo, pero no puede establecer modos de permiso por compañero de equipo en el momento de la generación.
* **Los paneles divididos requieren tmux o iTerm2**: el modo en proceso predeterminado funciona en cualquier terminal. El modo de panel dividido no es compatible con la terminal integrada de VS Code, Windows Terminal o Ghostty.

<h2 id="next-steps">
  Próximos pasos
</h2>

Explore enfoques relacionados para trabajo paralelo y delegación:

* **Delegación ligera**: [subagents](/docs/es/sub-agents) generan agentes auxiliares para investigación o verificación dentro de su sesión, mejor para tareas que no necesitan coordinación entre agentes
* **Mensajería entre sus propias sesiones**: [mensajería entre sesiones](/docs/es/cross-session-messaging) permite que Claude transmita hallazgos entre las sesiones que usted ejecuta
* **Sesiones paralelas manuales**: [Git worktrees](/docs/es/worktrees) le permiten ejecutar múltiples sesiones de Claude Code usted mismo sin coordinación de equipo automatizada
