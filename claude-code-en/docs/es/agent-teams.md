> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orquestar equipos de sesiones de Claude Code

> Coordine múltiples instancias de Claude Code trabajando juntas como un equipo, con tareas compartidas, mensajería entre agentes y gestión centralizada.

<Warning>
  Los equipos de agentes son experimentales y están deshabilitados por defecto. Habilítelos agregando `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` a su [settings.json](/docs/es/settings) o entorno. Sin esa variable, ningún equipo se configura al inicio de la sesión, no se escriben directorios de equipo, y Claude no genera ni propone compañeros de equipo. Los equipos de agentes tienen [limitaciones conocidas](#limitations) alrededor de la reanudación de sesiones, coordinación de tareas y comportamiento de apagado.
</Warning>

Los equipos de agentes le permiten coordinar múltiples instancias de Claude Code trabajando juntas. Una sesión actúa como el líder del equipo, coordinando el trabajo, asignando tareas y sintetizando resultados. Los compañeros de equipo trabajan de forma independiente, cada uno en su propia ventana de contexto, y se comunican directamente entre sí.

A diferencia de los [subagents](/docs/es/sub-agents), que se ejecutan dentro de una única sesión y solo pueden reportar al agente principal, también puede interactuar directamente con compañeros de equipo individuales sin pasar por el líder.

<Note>
  Esta página describe equipos de agentes a partir de v2.1.178. Con `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` configurado, generar un compañero de equipo ya no necesita un paso de configuración, y la limpieza ocurre automáticamente cuando la sesión sale. Antes de v2.1.178, usted le pedía a Claude que creara y nombrara un equipo primero, y Claude usaba las herramientas `TeamCreate` y `TeamDelete` para configurarlo y eliminarlo. Ambas herramientas ya no existen. La entrada `team_name` en la herramienta Agent se acepta pero se ignora, y el campo `team_name` en `TaskCreated`, `TaskCompleted`, y `TeammateIdle` [cargas útiles de hooks](/docs/es/hooks#taskcreated) lleva el nombre derivado de la sesión y está deprecado.
</Note>

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

Tanto los equipos de agentes como los [subagents](/docs/es/sub-agents) le permiten paralelizar el trabajo, pero operan de manera diferente. Elija según si sus trabajadores necesitan comunicarse entre sí:

<Frame caption="Los subagents solo reportan resultados al agente principal y nunca se hablan entre sí. En los equipos de agentes, los compañeros de equipo comparten una lista de tareas, reclaman trabajo y se comunican directamente entre sí.">
  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-light.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=2f8db9b4f3705dd3ab931fbe2d96e42a" className="dark:hidden" alt="Diagrama comparando arquitecturas de subagent y equipo de agentes. Los subagents son generados por el agente principal, hacen trabajo y reportan resultados. Los equipos de agentes se coordinan a través de una lista de tareas compartida, con compañeros de equipo comunicándose directamente entre sí." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-light.png" />

  <img src="https://mintcdn.com/claude-code/nsvRFSDNfpSU5nT7/images/subagents-vs-agent-teams-dark.png?fit=max&auto=format&n=nsvRFSDNfpSU5nT7&q=85&s=d573a037540f2ada6a9ae7d8285b46fd" className="hidden dark:block" alt="Diagrama comparando arquitecturas de subagent y equipo de agentes. Los subagents son generados por el agente principal, hacen trabajo y reportan resultados. Los equipos de agentes se coordinan a través de una lista de tareas compartida, con compañeros de equipo comunicándose directamente entre sí." width="4245" height="1615" data-path="images/subagents-vs-agent-teams-dark.png" />
</Frame>

|                     | Subagents                                                       | Equipos de agentes                                               |
| :------------------ | :-------------------------------------------------------------- | :--------------------------------------------------------------- |
| **Contexto**        | Ventana de contexto propia; los resultados regresan al llamador | Ventana de contexto propia; completamente independiente          |
| **Comunicación**    | Reportan resultados solo al agente principal                    | Los compañeros de equipo se envían mensajes directamente         |
| **Coordinación**    | El agente principal gestiona todo el trabajo                    | Lista de tareas compartida con auto-coordinación                 |
| **Mejor para**      | Tareas enfocadas donde solo importa el resultado                | Trabajo complejo que requiere discusión y colaboración           |
| **Costo de tokens** | Menor: resultados resumidos de vuelta al contexto principal     | Mayor: cada compañero de equipo es una instancia Claude separada |

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

<h2 id="start-your-first-agent-team">
  Inicie su primer equipo de agentes
</h2>

Después de habilitar los equipos de agentes, describa la tarea y los compañeros de equipo que desea en lenguaje natural. Claude los genera y coordina el trabajo según su indicación.

Este ejemplo funciona bien porque los tres roles son independientes y pueden explorar el problema sin esperar el uno al otro:

```text theme={null}
Estoy diseñando una herramienta CLI que ayuda a los desarrolladores a rastrear
comentarios TODO en su base de código. Genere tres compañeros de equipo para explorar
esto desde diferentes ángulos: uno en UX, uno en arquitectura técnica, uno jugando
al abogado del diablo.
```

A partir de ahí, Claude completa una [lista de tareas compartida](/docs/es/interactive-mode#task-list), genera compañeros de equipo para cada perspectiva, los hace explorar el problema, y sintetiza hallazgos cuando termina.

El panel del agente del líder enumera los compañeros de equipo debajo de la entrada del indicador. Desde el panel:

* **Flechas arriba y abajo**: seleccionar un compañero de equipo
* **Intro**: abrir la transcripción del compañero de equipo seleccionado y enviarle un mensaje directamente
* **Escape**: interrumpir el turno actual del compañero de equipo seleccionado

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

El valor predeterminado es `"in-process"`. Antes de v2.1.179, el valor predeterminado era `"auto"`, por lo que las sesiones actualizadas que anteriormente abrían paneles divididos ahora permanecen en una terminal a menos que establezca el modo explícitamente. Configure `"auto"` para habilitar paneles divididos cuando ya esté ejecutándose dentro de una sesión tmux o su terminal sea iTerm2, retrocediendo a en proceso de lo contrario. La configuración `"tmux"` habilita el modo de panel dividido y detecta automáticamente si usar tmux o iTerm2 según su terminal.

A partir de v2.1.186, configure `"iterm2"` para usar explícitamente paneles divididos nativos de iTerm2. Este modo requiere la [CLI `it2`](https://github.com/mkusaka/it2) y muestra un error con el comando de instalación si falta `it2`. El indicador de configuración que ofrece instalar `it2` o cambiar a tmux aparece bajo `"auto"` o `"tmux"` cuando su terminal es iTerm2 y tmux está disponible como alternativa.

Para anular el valor predeterminado, configure [`teammateMode`](/docs/es/settings#available-settings) en `~/.claude/settings.json`:

```json theme={null}
{
  "teammateMode": "auto"
}
```

Para establecer el modo para una única sesión, páselo como una bandera:

```bash theme={null}
claude --teammate-mode auto
```

El modo de panel dividido requiere [tmux](https://github.com/tmux/tmux/wiki) o iTerm2 con la [CLI `it2`](https://github.com/mkusaka/it2). Para instalar manualmente:

* **tmux**: instale a través del gestor de paquetes de su sistema. Vea la [wiki de tmux](https://github.com/tmux/tmux/wiki/Installing) para instrucciones específicas de la plataforma.
* **iTerm2**: instale la [CLI `it2`](https://github.com/mkusaka/it2), luego habilite la API de Python en **iTerm2 → Settings → General → Magic → Enable Python API**.

<h3 id="specify-teammates-and-models">
  Especificar compañeros de equipo y modelos
</h3>

Claude decide el número de compañeros de equipo a generar según su tarea, o puede especificar exactamente lo que desea:

```text theme={null}
Spawn 4 teammates to refactor these modules in parallel. Use Sonnet for
each teammate.
```

Los compañeros de equipo no heredan la selección `/model` del líder de forma predeterminada. Para cambiar el modelo utilizado cuando el indicador no especifica uno, configure **Modelo de compañero de equipo predeterminado** en `/config`. Seleccione **Predeterminado (modelo del líder)** para que los compañeros de equipo sigan el modelo actual del líder.

Los compañeros de equipo heredan el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) del líder. En modo de panel dividido esto se aplica desde v2.1.186; las versiones anteriores no pasaban el esfuerzo de sesión del líder a los compañeros de equipo de panel dividido.

<h3 id="require-plan-approval-for-teammates">
  Requerir aprobación de plan para compañeros de equipo
</h3>

Para tareas complejas o riesgosas, puede requerir que los compañeros de equipo planifiquen antes de implementar. El compañero de equipo trabaja en modo de plan de solo lectura hasta que el líder apruebe su enfoque:

```text theme={null}
Spawn an architect teammate to refactor the authentication module.
Require plan approval before they make any changes.
```

Cuando un compañero de equipo termina de planificar, envía una solicitud de aprobación de plan al líder. El líder revisa el plan y lo aprueba o lo rechaza con retroalimentación. Si se rechaza, el compañero de equipo permanece en modo de plan, revisa según la retroalimentación y reenvía. Una vez aprobado, el compañero de equipo sale del modo de plan y comienza la implementación.

El líder toma decisiones de aprobación de forma autónoma. Para influir en el juicio del líder, proporcione criterios en su indicación, como "solo aprueba planes que incluyan cobertura de pruebas" o "rechaza planes que modifiquen el esquema de la base de datos".

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

El líder puede asignar tareas explícitamente, o los compañeros de equipo pueden auto-reclamar:

* **El líder asigna**: dígale al líder qué tarea dar a qué compañero de equipo
* **Auto-reclamar**: después de terminar una tarea, un compañero de equipo recoge la siguiente tarea sin asignar y sin bloquear por su cuenta

El reclamo de tareas usa bloqueo de archivos para prevenir condiciones de carrera cuando múltiples compañeros de equipo intentan reclamar la misma tarea simultáneamente.

<h3 id="shut-down-teammates">
  Apagar compañeros de equipo
</h3>

Para terminar gracefully la sesión de un compañero de equipo, refiriéndose a él por nombre. Por ejemplo, con un compañero de equipo llamado investigador:

```text theme={null}
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

Un equipo de agentes se forma cuando se genera el primer compañero de equipo, con la sesión principal actuando como el líder. Hay dos formas en que se generan los compañeros de equipo:

* **Usted solicita compañeros de equipo**: dé a Claude una tarea que se beneficie del trabajo paralelo y solicite explícitamente compañeros de equipo. Claude los genera según sus instrucciones.
* **Claude propone compañeros de equipo**: si Claude determina que su tarea se beneficiaría del trabajo paralelo, puede sugerir generar compañeros de equipo. Usted confirma antes de que proceda.

En ambos casos, usted mantiene el control. Claude no generará compañeros de equipo sin su aprobación.

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

Vea [Elegir un modo de visualización](#choose-a-display-mode) para opciones de configuración de visualización. Los mensajes de los compañeros de equipo llegan al líder automáticamente.

El buzón de cada agente es un archivo JSON en `~/.claude/teams/{team-name}/inboxes/{agent-name}.json`. Claude Code valida cada entrada cuando lee un archivo de buzón. Las entradas que no coinciden con el formato de mensaje se reportan como errores y se eliminan del archivo; los mensajes válidos aún se entregan. Antes de v2.1.207, una única entrada de buzón malformada causaba un error repetido cada segundo y bloqueaba la entrega para ese buzón hasta que eliminara manualmente el archivo.

El sistema gestiona las dependencias de tareas automáticamente. Cuando un compañero de equipo completa una tarea de la que otras tareas dependen, las tareas bloqueadas se desbloquean sin intervención manual.

Los equipos y tareas se almacenan localmente bajo un nombre derivado de la sesión. El nombre es `session-` seguido de los primeros ocho caracteres del ID de sesión:

* **Configuración del equipo**: `~/.claude/teams/{team-name}/config.json`
* **Lista de tareas**: `~/.claude/tasks/{team-name}/`

Claude Code genera ambos automáticamente al inicio de la sesión y los actualiza a medida que los compañeros de equipo se unen, se quedan inactivos o se van. El directorio de configuración del equipo se elimina cuando la sesión termina. El directorio de lista de tareas persiste localmente y nunca se carga, por lo que las sesiones reanudadas mantienen sus tareas. La retención se rige por el mismo [`cleanupPeriodDays`](/docs/es/settings#available-settings) que ya controla para transcripciones de sesión.

La configuración del equipo contiene estado de tiempo de ejecución como IDs de sesión e IDs de panel tmux, así que no la edite manualmente ni la pre-autorice: sus cambios se sobrescriben en la siguiente actualización de estado.

Para definir roles de compañeros de equipo reutilizables, use [definiciones de subagents](#use-subagent-definitions-for-teammates) en su lugar.

La configuración del equipo contiene un array `members` con el nombre de cada compañero de equipo, ID de agente y tipo de agente. Los compañeros de equipo pueden leer este archivo para descubrir otros miembros del equipo.

No hay equivalente a nivel de proyecto de la configuración del equipo. Un archivo como `.claude/teams/teams.json` en su directorio de proyecto no se reconoce como configuración; Claude lo trata como un archivo ordinario.

<h3 id="use-subagent-definitions-for-teammates">
  Usar definiciones de subagents para compañeros de equipo
</h3>

Al generar un compañero de equipo, puede hacer referencia a un tipo de [subagent](/docs/es/sub-agents) de cualquier [alcance de subagent](/docs/es/sub-agents#choose-the-subagent-scope): proyecto, usuario, plugin o definido por CLI. Esto le permite definir un rol una vez, como un revisor de seguridad o ejecutor de pruebas, y reutilizarlo tanto como un subagent delegado como un compañero de equipo de equipo de agentes.

Para usar una definición de subagent, mencione por nombre cuando le pida a Claude que genere el compañero de equipo:

```text theme={null}
Genera un compañero de equipo usando el tipo de agente security-reviewer para auditar el módulo de autenticación.
```

El compañero de equipo honra los campos `tools` y `model` de esa definición, y el cuerpo de la definición se añade al prompt del sistema del compañero de equipo como instrucciones adicionales en lugar de reemplazarlo. Las herramientas de coordinación de equipos como `SendMessage` y las herramientas de gestión de tareas siempre están disponibles para un compañero de equipo incluso cuando `tools` restringe otras herramientas.

<Note>
  Los campos `skills` y `mcpServers` en la portada de una definición de subagent no se aplican cuando esa definición se ejecuta como un compañero de equipo. Los compañeros de equipo cargan skills y MCP servers desde su configuración de proyecto y usuario, igual que una sesión regular.
</Note>

<h3 id="permissions">
  Permisos
</h3>

Los compañeros de equipo comienzan con la configuración de permisos del líder. Si el líder se ejecuta con `--dangerously-skip-permissions`, todos los compañeros de equipo también lo hacen. Después de generar, puede cambiar los modos de compañeros de equipo individuales, pero no puede establecer modos por compañero de equipo en el momento de la generación.

Cuando un agente envía un mensaje a otro a través de `SendMessage`, se le dice al agente receptor que provino de otra sesión de Claude, no de usted. Un compañero de equipo no puede aprobar una solicitud de permiso o proporcionar consentimiento en su nombre, y un compañero de equipo al que se le negó una acción no puede retransmitirla a otro compañero de equipo para eludir la verificación. En [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), el clasificador trata una afirmación de aprobación retransmitida desde otro agente como entrada no confiable en lugar de confirmación de usted.

Las solicitudes de permiso de compañeros de equipo aparecen en la sesión del líder, así que apruébelas allí usted mismo. [Aprobación de plan](#require-plan-approval-for-teammates) es la excepción diseñada: la sesión del líder otorga aprobaciones de plan de compañeros de equipo sin una solicitud separada para usted.

<h3 id="context-and-communication">
  Contexto y comunicación
</h3>

Cada compañero de equipo tiene su propia ventana de contexto. Cuando se genera, un compañero de equipo carga el mismo contexto de proyecto que una sesión regular: CLAUDE.md, MCP servers y skills. También recibe la indicación de generación del líder. El historial de conversación del líder no se transfiere.

**Cómo los compañeros de equipo comparten información:**

* **Entrega automática de mensajes**: cuando los compañeros de equipo envían mensajes, se entregan automáticamente a los destinatarios. El líder no necesita sondear actualizaciones.
* **Notificaciones de inactividad**: cuando un compañero de equipo termina y se detiene, notifica automáticamente al líder. A partir de v2.1.198, un compañero de equipo cuyo turno termina en un error de API notifica al líder que falló e incluye el texto del error, en lugar de parecer que termina normalmente.
* **Lista de tareas compartida**: todos los agentes pueden ver el estado de la tarea y reclamar trabajo disponible.
* **Mensajería de compañeros de equipo**: enviar un mensaje a un compañero de equipo específico por nombre. Para llegar a todos, envíe un mensaje por destinatario.

El líder asigna a cada compañero de equipo un nombre cuando lo genera, y cualquier compañero de equipo puede enviar un mensaje a otro por ese nombre. Para obtener nombres predecibles que pueda referenciar en indicaciones posteriores, dígale al líder cómo llamar a cada compañero de equipo en su instrucción de generación.

<h3 id="token-usage">
  Uso de tokens
</h3>

Los equipos de agentes usan significativamente más tokens que una única sesión. Cada compañero de equipo tiene su propia ventana de contexto, y el uso de tokens escala con el número de compañeros de equipo activos. Para investigación, revisión y trabajo de nuevas características, los tokens adicionales generalmente valen la pena. Para tareas rutinarias, una única sesión es más rentable. Vea [costos de tokens de equipos de agentes](/docs/es/costs#agent-team-token-costs) para orientación de uso.

<h2 id="use-case-examples">
  Ejemplos de casos de uso
</h2>

Estos ejemplos muestran cómo los equipos de agentes manejan tareas donde la exploración paralela agrega valor.

<h3 id="run-a-parallel-code-review">
  Ejecutar una revisión de código paralela
</h3>

Un único revisor tiende a gravitar hacia un tipo de problema a la vez. Dividir criterios de revisión en dominios independientes significa que la seguridad, el rendimiento y la cobertura de pruebas reciben atención exhaustiva simultáneamente. La indicación asigna a cada compañero de equipo una lente distinta para que no se superpongan:

```text theme={null}
Crea un equipo de agentes para revisar la PR #142. Genera tres revisores:
- Uno enfocado en implicaciones de seguridad
- Uno verificando impacto de rendimiento
- Uno validando cobertura de pruebas
Que cada uno revise e informe hallazgos.
```

Cada revisor trabaja desde la misma PR pero aplica un filtro diferente. El líder sintetiza hallazgos en los tres después de que terminen.

<h3 id="investigate-with-competing-hypotheses">
  Investigar con hipótesis competidoras
</h3>

Cuando la causa raíz es poco clara, un único agente tiende a encontrar una explicación plausible y dejar de buscar. La indicación lucha contra esto haciendo que los compañeros de equipo sean explícitamente adversarios: el trabajo de cada uno no es solo investigar su propia teoría sino desafiar las de los demás.

```text theme={null}
Los usuarios reportan que la aplicación se cierra después de un mensaje en lugar de
mantenerse conectada. Genera 5 compañeros de equipo de agentes para investigar
diferentes hipótesis. Haz que se hablen entre sí para intentar refutar las teorías
de los demás, como un debate científico. Actualiza el documento de hallazgos con
cualquier consenso que emerja.
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

```text theme={null}
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

Comience con 3-5 compañeros de equipo para la mayoría de flujos de trabajo. Esto equilibra el trabajo paralelo con coordinación manejable. Los ejemplos en esta guía usan 3-5 compañeros de equipo porque ese rango funciona bien en diferentes tipos de tareas.

Tener 5-6 [tareas](/docs/es/agent-teams#architecture) por compañero de equipo mantiene a todos productivos sin cambio de contexto excesivo. Si tiene 15 tareas independientes, 3 compañeros de equipo es un buen punto de partida.

Escale solo cuando el trabajo genuinamente se beneficie de tener compañeros de equipo trabajando simultáneamente. Tres compañeros de equipo enfocados a menudo superan a cinco dispersos.

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

```text theme={null}
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

<h3 id="too-many-permission-prompts">
  Demasiados avisos de permisos
</h3>

Las solicitudes de permisos de compañeros de equipo suben al líder, lo que puede crear fricción. Pre-apruebe operaciones comunes en su [configuración de permisos](/docs/es/permissions) antes de generar compañeros de equipo para reducir interrupciones.

<h3 id="teammates-stopping-on-errors">
  Los compañeros de equipo se detienen en errores
</h3>

Los compañeros de equipo pueden detenerse después de encontrar errores en lugar de recuperarse. Verifique su salida seleccionando el compañero de equipo en el panel del agente y presionando Intro en modo en proceso, o haciendo clic en el panel en modo dividido, luego:

* Deles instrucciones adicionales directamente
* Genere un compañero de equipo de reemplazo para continuar el trabajo

A partir de v2.1.198, un mensaje del líder u otro compañero de equipo despierta a un compañero de equipo en proceso que está esperando reintentar una solicitud de API fallida, por lo que lo reintenta inmediatamente en lugar de esperar el retraso de reintento completo.

<h3 id="lead-shuts-down-before-work-is-done">
  El líder se apaga antes de que el trabajo esté hecho
</h3>

El líder puede decidir que el equipo está terminado antes de que todas las tareas estén realmente completas. Si esto sucede, dígale que continúe. También puede decirle al líder que espere a que los compañeros de equipo terminen antes de proceder si comienza a hacer trabajo en lugar de delegar.

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
* **Sin subagentes de fondo de compañeros de equipo en proceso**: los propios subagentes de un compañero de equipo en proceso se ejecutan en primer plano. Solicitar uno de fondo, ya sea con `run_in_background` o una definición de subagente que establezca `background: true`, devuelve un error, porque el trabajo de fondo de un compañero de equipo no puede sobrevivir al proceso del líder. Los subagentes lanzados desde la conversación principal siguen el [valor predeterminado de fondo](/docs/es/sub-agents#run-subagents-in-foreground-or-background).
* **El líder es fijo**: la sesión principal es el líder de por vida. No puede promover un compañero de equipo a líder ni transferir liderazgo.
* **Permisos establecidos en la generación**: todos los compañeros de equipo comienzan con el modo de permiso del líder. Puede cambiar modos de compañeros de equipo individuales después de generar, pero no puede establecer modos por compañero de equipo en el momento de la generación.
* **Los paneles divididos requieren tmux o iTerm2**: el modo en proceso predeterminado funciona en cualquier terminal. El modo de panel dividido no es compatible con la terminal integrada de VS Code, Windows Terminal o Ghostty.

<Tip>
  **`CLAUDE.md` funciona normalmente**: los compañeros de equipo leen archivos `CLAUDE.md` de su directorio de trabajo. Use esto para proporcionar orientación específica del proyecto a todos los compañeros de equipo.
</Tip>

<h2 id="next-steps">
  Próximos pasos
</h2>

Explore enfoques relacionados para trabajo paralelo y delegación:

* **Delegación ligera**: [subagents](/docs/es/sub-agents) generan agentes auxiliares para investigación o verificación dentro de su sesión, mejor para tareas que no necesitan coordinación entre agentes
* **Sesiones paralelas manuales**: [Git worktrees](/docs/es/worktrees) le permiten ejecutar múltiples sesiones de Claude Code usted mismo sin coordinación de equipo automatizada
* **Comparar enfoques**: vea la comparación [subagent vs agent team](/docs/es/features-overview#compare-similar-features) para un desglose lado a lado
