> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orquestar subagentes a escala con flujos de trabajo dinámicos

> Los dynamic workflows orquestan muchos subagentes a partir de un script que Claude escribe y que puede volver a ejecutar. Úselos para auditorías de base de código, migraciones grandes e investigación con verificación cruzada.

<Note>
  Los dynamic workflows requieren Claude Code v2.1.154 o posterior y están disponibles en todos los planes pagos, con acceso a la API de Anthropic, y en Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry. En Pro, actívelos desde la fila Dynamic workflows en `/config`.
</Note>

Un dynamic workflow es un script de JavaScript que orquesta [subagentes](/docs/es/sub-agents) a escala. Claude escribe el script para la tarea que describe, y un runtime lo ejecuta en segundo plano mientras su sesión permanece receptiva.

Recurra a un workflow cuando una tarea necesita más agentes de los que una conversación puede coordinar, o cuando desea que la orquestación esté codificada como un script que pueda leer y volver a ejecutar. Los ejemplos incluyen un barrido de errores en toda la base de código, una migración de 500 archivos, una pregunta de investigación que necesita que las fuentes se verifiquen mutuamente, y un plan difícil que vale la pena redactar desde varios ángulos independientes antes de comprometerse con uno.

<h2 id="when-to-use-a-workflow">
  Cuándo usar un workflow
</h2>

[Subagentes](/docs/es/sub-agents), [skills](/docs/es/skills), [equipos de agentes](/docs/es/agent-teams) y workflows pueden ejecutar una tarea de varios pasos. La diferencia es quién tiene el plan:

|                                            | Subagentes                         | Skills                         | Equipos de agentes                                | Workflows                                  |
| :----------------------------------------- | :--------------------------------- | :----------------------------- | :------------------------------------------------ | :----------------------------------------- |
| Qué es                                     | Un Claude trabajador que genera    | Instrucciones que Claude sigue | Un agente líder supervisando sesiones entre pares | Un script que ejecuta el runtime           |
| Quién decide qué se ejecuta a continuación | Claude, turno a turno              | Claude, siguiendo el prompt    | El agente líder, turno a turno                    | El script                                  |
| Dónde viven los resultados intermedios     | Ventana de contexto de Claude      | Ventana de contexto de Claude  | Una lista de tareas compartida                    | Variables del script                       |
| Qué es repetible                           | La definición del trabajador       | Las instrucciones              | La definición del equipo                          | La orquestación en sí                      |
| Escala                                     | Algunas tareas delegadas por turno | Igual que los subagentes       | Un puñado de pares de larga duración              | Docenas a cientos de agentes por ejecución |
| Interrupción                               | Reinicia el turno                  | Reinicia el turno              | Los compañeros de equipo siguen ejecutándose      | Reanudable en la misma sesión              |

Un workflow mueve el plan al código. Con subagentes, skills y equipos de agentes, Claude es el orquestador: decide turno a turno qué generar o asignar a continuación, y cada resultado llega a una ventana de contexto. Un script de workflow mantiene el bucle, la ramificación y los resultados intermedios en sí mismo, por lo que el contexto de Claude solo contiene la respuesta final.

Mover el plan al código también permite que un workflow aplique un patrón de calidad repetible, no solo ejecutar más agentes: puede tener agentes independientes que revisen adversarialmente los hallazgos de los demás antes de que se informen, o redacten un plan desde varios ángulos y los sopesen entre sí, para obtener un resultado más confiable que una sola pasada.

<h2 id="run-a-bundled-workflow">
  Ejecutar un workflow incluido
</h2>

La forma más rápida de ver un workflow en acción es ejecutar `/deep-research`, el [workflow integrado](#bundled-workflows) que Claude Code incluye para investigar una pregunta en muchas fuentes. Verá agentes trabajar a través de un conjunto de fases en segundo plano mientras su sesión permanece libre, y obtendrá un informe al final en lugar de una transcripción turno a turno.

<Steps>
  <Step title="Ejecutar el workflow">
    Ejecute `/deep-research` con una pregunta que desee investigar. Distribuye búsquedas web en varios ángulos, obtiene y verifica cruzadamente las fuentes que encuentra, y sintetiza un informe citado.

    ```text theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Permitir workflows">
    Claude Code pregunta si permitir el workflow. Seleccione **Yes** para continuar. El prompt exacto depende de su modo de permiso. Consulte [Aprobar el plan antes de que se ejecute](#approve-the-plan-before-it-runs) para las opciones por modo.
  </Step>

  <Step title="Ver progreso">
    La ejecución comienza en segundo plano. Ejecute `/workflows`, use las teclas de flecha para seleccionar la ejecución y presione Enter para abrir su vista de progreso:

    ```text theme={null}
    /workflows
    ```

    La vista muestra cada fase con su recuento de agentes, total de tokens y tiempo transcurrido. Profundice en cualquier fase para ver sus agentes y qué encontró cada uno. Consulte [Ver la ejecución](#watch-the-run) para el conjunto completo de controles.

    También puede ver desde el panel de tareas debajo del cuadro de entrada: aparece un resumen de progreso de una línea mientras se ejecuta. Presione la flecha hacia abajo para enfocarlo, luego Enter para expandir.
  </Step>

  <Step title="Leer el informe">
    Cuando se completa la ejecución, el informe llega a su sesión. Cita las fuentes de las que proviene cada afirmación, con afirmaciones que no sobrevivieron la verificación cruzada ya filtradas.

    A partir de v2.1.196, cuando los agentes verificadores no pueden verificar una afirmación, como después de un límite de velocidad o error de API, el informe enumera esa afirmación como no verificada en lugar de contarla como refutada.
  </Step>
</Steps>

Para ejecutar un workflow para su propia tarea, [haga que Claude escriba uno](#have-claude-write-a-workflow), y una vez que una ejecución haga lo que deseaba puede [guardarlo](#save-the-workflow-for-reuse) como un comando propio.

<h3 id="bundled-workflows">
  Workflows incluidos
</h3>

Claude Code incluye `/deep-research` como un workflow integrado:

| Comando                     | Qué hace                                                                                                                                                                                                                                                                                                                                                  |
| :-------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/deep-research <question>` | Distribuye búsquedas web en una pregunta en varios ángulos, obtiene y verifica cruzadamente las fuentes que encuentra, vota en cada afirmación y devuelve un informe citado con afirmaciones que no sobrevivieron la verificación cruzada filtradas. Requiere que la [herramienta WebSearch](/docs/es/tools-reference#websearch-tool-behavior) esté disponible |

Los [workflows que guarda](#save-the-workflow-for-reuse) usted mismo se convierten en comandos de la misma manera y aparecen en el autocompletado `/` junto con los incluidos.

<h3 id="watch-the-run">
  Ver la ejecución
</h3>

Los workflows se ejecutan en segundo plano, por lo que la sesión permanece receptiva mientras los agentes trabajan. Ejecute `/workflows` en cualquier momento para enumerar los workflows en ejecución y completados, luego seleccione uno para abrir su vista de progreso.

```text theme={null}
/workflows
```

La vista de progreso muestra cada fase con sus recuentos de agentes, totales de tokens y tiempo transcurrido. El pie de página enumera la clave para cada acción:

| Clave         | Acción                                                                                                                      |
| :------------ | :-------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`     | Seleccionar una fase o agente                                                                                               |
| `Enter` o `→` | Profundizar en la fase seleccionada, luego en un agente para leer su prompt, llamadas de herramientas recientes y resultado |
| `Esc` o `←`   | Retroceder un nivel. En v2.1.203 a v2.1.205, `←` no retrocedía de una fase o agente; use `Esc` en esas versiones            |
| `j` / `k`     | Desplazarse dentro del detalle del agente cuando se desborda                                                                |
| `f`           | Filtrar la lista de agentes en la fase seleccionada por estado. Presione de nuevo para ciclar                               |
| `p`           | Pausar o reanudar la ejecución                                                                                              |
| `x`           | Detener el agente seleccionado, o detener todo el workflow cuando el enfoque está en la ejecución                           |
| `r`           | Reiniciar el agente en ejecución seleccionado                                                                               |
| `s`           | [Guardar](#save-the-workflow-for-reuse) el script de la ejecución como un comando                                           |

<h2 id="have-claude-write-a-workflow">
  Hacer que Claude escriba un workflow
</h2>

Puede hacer que Claude escriba un workflow para su tarea de dos formas:

* [Pedir un workflow en su prompt](#ask-for-a-workflow-in-your-prompt) en su prompt, ya sea con sus propias palabras o incluyendo la palabra clave `ultracode`, y Claude escribe uno para la tarea.
* [Dejar que Claude decida con ultracode](#let-claude-decide-with-ultracode): establezca `/effort ultracode` y Claude planifica un workflow para cada tarea sustancial en la sesión.

También puede ejecutar un comando de workflow que ya existe: un [workflow incluido](#bundled-workflows) como `/deep-research`, o uno que ha [guardado](#save-the-workflow-for-reuse).

<h3 id="ask-for-a-workflow-in-your-prompt">
  Pedir un workflow en su prompt
</h3>

Para ejecutar una sola tarea como un workflow sin cambiar el nivel de esfuerzo de la sesión, incluya la palabra clave `ultracode` en su prompt. Pedir con sus propias palabras, por ejemplo "usar un workflow" o "ejecutar un workflow", también funciona: Claude trata una solicitud directa como el mismo opt-in. Antes de v2.1.160 la palabra clave literal era `workflow`; las solicitudes en lenguaje natural funcionan en ambas versiones.

```text theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code resalta la palabra clave en su entrada y Claude escribe un script de workflow para la tarea en lugar de trabajar a través de ella turno a turno. Si no tenía la intención de iniciar un workflow, presione `Option+W` en macOS o `Alt+W` en Windows y Linux para descartar el resaltado para este prompt, o presione retroceso mientras el cursor está justo después de la palabra clave resaltada. Para evitar que la palabra clave se active en absoluto, desactive Ultracode keyword trigger en `/config`.

Si ya tiene un orquestador construido de otra manera, como una carpeta de prompts de subagentes o una skill que distribuye trabajo, puede señalar a Claude hacia él y pedir un workflow que haga lo mismo.

<h3 id="let-claude-decide-with-ultracode">
  Dejar que Claude decida con ultracode
</h3>

Ultracode es una configuración de Claude Code que combina `xhigh` [esfuerzo de razonamiento](/docs/es/model-config#adjust-effort-level) con orquestación automática de workflows. Con él activado, Claude planifica un workflow para cada tarea sustancial en lugar de esperar a que lo pida.

```text theme={null}
/effort ultracode
```

Para iniciar una sesión con ultracode ya activado, lance con `claude --effort ultracode`. Requiere Claude Code v2.1.203 o posterior.

Con ultracode activado, Claude decide cuándo una tarea justifica un workflow. Una sola solicitud puede convertirse en varios workflows seguidos: uno para entender el código, uno para hacer el cambio y uno para verificarlo. Esto se aplica a cada tarea en la sesión, por lo que cada solicitud usa más tokens y toma más tiempo que en niveles de esfuerzo más bajos.

Ultracode dura la sesión actual y se reinicia cuando comienza una nueva. Vuelva con `/effort high` cuando regrese al trabajo rutinario. Está disponible en modelos que admiten `xhigh` [esfuerzo](/docs/es/model-config#adjust-effort-level); en otros modelos el menú `/effort` no lo ofrece.

<h3 id="approve-the-plan-before-it-runs">
  Aprobar el plan antes de que se ejecute
</h3>

En la CLI, el prompt por ejecución muestra las fases planeadas y estas opciones:

* **Yes, run it**: inicia la ejecución
* **Yes, and don't ask again for `<name>` in `<path>`**: inicia y omite este prompt para este workflow en este proyecto de ahora en adelante
* **View raw script**: lee el script antes de decidir
* **No**: cancelar

`Ctrl+G` abre el script en su editor. `Tab` le permite ajustar el prompt antes de que comience la ejecución.

Si ve este prompt depende de su [modo de permiso](/docs/es/permission-modes):

| Modo de permiso                         | Cuándo se le solicita                                                                                                                                                                                                     |
| :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Predeterminado, aceptar ediciones       | Cada ejecución, a menos que haya seleccionado **Yes, and don't ask again** para ese workflow en este proyecto                                                                                                             |
| Auto                                    | Solo en el primer lanzamiento. Cualquier **Yes** registra el consentimiento en su configuración de usuario, y los lanzamientos posteriores comienzan sin solicitar. Se omite completamente cuando ultracode está activado |
| Omitir permisos, `claude -p`, Agent SDK | Nunca. La ejecución comienza inmediatamente                                                                                                                                                                               |

En la aplicación de escritorio, una tarjeta de aprobación muestra el nombre del workflow, la lista de fases y una advertencia de uso de tokens, con acciones **Once**, **Always** y **Deny**. La vista de progreso aparece en el panel lateral Background tasks.

Su modo de permiso controla solo el prompt de lanzamiento anterior. Los subagentes que genera el workflow siempre se ejecutan en modo `acceptEdits` y heredan su [lista de permitidos de herramientas](/docs/es/settings#permission-settings), independientemente del modo de su sesión. Las ediciones de archivos se aprueban automáticamente.

Los comandos de shell, las búsquedas web y las herramientas MCP que no están en su lista de permitidos aún pueden solicitarle durante la ejecución. Para evitar esto en una ejecución larga, agregue los comandos que necesitan los agentes a su lista de permitidos antes de comenzar.

En `claude -p` y el Agent SDK no hay nadie a quien solicitar, por lo que las llamadas de herramientas siguen sus reglas de permiso configuradas sin confirmación interactiva.

<h3 id="save-the-workflow-for-reuse">
  Guardar el workflow para reutilización
</h3>

Cuando Claude escribe un workflow para una tarea que repetirá, puede guardar el script de esa ejecución como un comando. Un proceso como una revisión que ejecuta en cada rama luego ejecuta la misma orquestación cada vez.

Ejecute `/workflows`, seleccione la ejecución que desea mantener y presione `s`. En el diálogo de guardado, Tab alterna entre las dos ubicaciones de guardado:

* `.claude/workflows/` en su proyecto: compartido con todos los que clonan el repositorio
* `~/.claude/workflows/` en su directorio de inicio: disponible en cada proyecto, visible solo para usted. Si establece [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars), esta ubicación es el directorio `workflows/` bajo esa ruta.

El diálogo de guardado muestra la ruta resuelta para la ubicación personal. Antes de v2.1.208, mostraba `~/.claude/workflows/` incluso cuando `CLAUDE_CONFIG_DIR` estaba establecido; el archivo aún se guardaba en el directorio configurado.

Presione Enter para guardar. El workflow se ejecuta como `/<name>` en futuras sesiones desde cualquier ubicación.

En un monorepo con varios directorios `.claude/`, puede mantener workflows junto al paquete al que se aplican. A partir de v2.1.178, guardar en la ubicación del proyecto escribe en el directorio `.claude/workflows/` más cercano que ya existe entre su directorio de trabajo y la raíz del repositorio, o en la raíz del repositorio si aún no existe ninguno. Los workflows del proyecto también se cargan desde cada `.claude/workflows/` a lo largo de esa ruta, y cuando más de uno define el mismo nombre Claude Code ejecuta el más cercano al directorio de trabajo.

Si un workflow de proyecto y un workflow personal comparten un nombre, se ejecuta el del proyecto.

<h3 id="pass-input-to-a-saved-workflow">
  Pasar entrada a un workflow guardado
</h3>

Un workflow guardado puede aceptar entrada a través del parámetro `args`. El script lo lee como una variable global llamada `args`. Úselo para proporcionar una pregunta de investigación, una lista de rutas de destino o un objeto de configuración en el momento de la invocación en lugar de editar el script para cada ejecución.

El siguiente prompt ejecuta un workflow guardado con una lista de números de problemas:

```text theme={null}
> Run /triage-issues on issues 1024, 1025, and 1030
```

Claude pasa la lista como datos estructurados, por lo que el script puede llamar a métodos de matriz y objeto en `args` directamente sin analizarlo primero. Si se omite `args`, la variable global es `undefined` dentro del script.

<h2 id="example-workflow-prompts">
  Ejemplos de prompts de workflow
</h2>

Un workflow se ajusta mejor cuando la tarea es más grande de lo que un agente puede mantener en contexto, o cuando el mismo paso necesita ejecutarse en muchos elementos. Los prompts a continuación muestran formas comunes. Cada uno pide a Claude que escriba y ejecute un workflow para esa tarea; usted no escribe el script usted mismo.

<h3 id="audit-many-files-for-the-same-issue">
  Auditar muchos archivos para el mismo problema
</h3>

Distribuya un agente por archivo, luego recopile y verifique los hallazgos.

```text theme={null}
> use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Seguir arreglando hasta que pase una verificación
</h3>

Ejecute un verificador, arregle lo que falló y repita hasta que pase o deje de hacer progreso.

```text theme={null}
> use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Migrar muchos archivos en paralelo
</h3>

Descubra los archivos a migrar, transforme cada uno en una copia aislada para que las ediciones no entren en conflicto, y verifique cada resultado.

```text theme={null}
> use a workflow to migrate every component under src/components/ from styled-components to Tailwind, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Revisar cada archivo modificado y escribir un resumen
</h3>

Ejecute un revisor por archivo, luego entregue todos los hallazgos a un agente que los clasifique y deduplique.

```text theme={null}
> use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Investigar un tema en muchas fuentes
</h3>

Distribuya lectores en registros de cambios, problemas y documentos, luego sintetice. El workflow `/deep-research` incluido hace esto; también puede describir una versión más estrecha.

```text theme={null}
> use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Encontrar problemas hasta que la lista deje de crecer
</h3>

Siga buscando en rondas y deténgase cuando nuevas rondas no encuentren nada nuevo.

```text theme={null}
> use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
```

<h3 id="what-the-saved-script-looks-like">
  Cómo se ve el script guardado
</h3>

Cuando [guarda un workflow](#save-the-workflow-for-reuse), el archivo en `.claude/workflows/` contiene un bloque `meta` seguido de un cuerpo de script que orquesta subagentes. Generalmente no necesita editarlo, pero aquí está la forma de uno pequeño para que pueda reconocer lo que Claude generó:

```javascript theme={null}
export const meta = {
  name: 'audit-routes',
  description: 'Audit every route handler for missing auth checks',
}

const found = await agent('List every .ts file under src/routes/.', {
  schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
})

const audits = await pipeline(found.files, file =>
  agent(`Audit ${file} for missing authentication checks.`, { label: file }),
)

return audits.filter(Boolean)
```

El cuerpo es JavaScript simple con `await` de nivel superior. `agent()` genera un subagente y `pipeline()` ejecuta uno por elemento en una lista. Si desea editar un script a mano, pida a Claude que le guíe a través del cambio, o consulte la entrada de herramienta Workflow en la [referencia del Agent SDK](/docs/es/agent-sdk/typescript) para el conjunto completo de opciones.

<h2 id="how-a-workflow-runs">
  Cómo se ejecuta un workflow
</h2>

El runtime del workflow ejecuta el script en un entorno aislado, separado de su conversación. Los resultados intermedios permanecen en variables de script en lugar de llegar al contexto de Claude.

Cada ejecución escribe su script en un archivo bajo el directorio de su sesión en `~/.claude/projects/`. Claude recibe la ruta cuando comienza la ejecución, por lo que puede solicitarla. Puede abrir ese archivo para leer la orquestación que Claude escribió, compararlo con el script de una ejecución anterior, o editarlo y pedir a Claude que reinicie desde la versión editada.

El runtime rastrea el resultado de cada agente a medida que avanza la ejecución, lo que es lo que hace que una ejecución sea [reanudable](#resume-after-a-pause) dentro de la misma sesión.

<h3 id="behavior-and-limits">
  Comportamiento y límites
</h3>

El runtime aplica las siguientes restricciones:

| Restricción                                                                   | Por qué                                                                                                                                         |
| :---------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| Sin entrada de usuario a mitad de ejecución                                   | Solo los prompts de permiso del agente pueden pausar una ejecución. Para la aprobación entre etapas, ejecute cada etapa como su propio workflow |
| Sin acceso directo al sistema de archivos o shell desde el workflow en sí     | Los agentes leen, escriben y ejecutan comandos. El script coordina los agentes                                                                  |
| Hasta 16 agentes concurrentes, menos en máquinas con núcleos de CPU limitados | Limita el uso de recursos locales                                                                                                               |
| 1.000 agentes totales por ejecución                                           | Previene bucles descontrolados                                                                                                                  |

<h2 id="manage-runs">
  Gestionar ejecuciones
</h2>

Una vez que comienza una ejecución, la gestiona desde la vista `/workflows`, o expandiendo su línea de progreso en el panel de tareas debajo del cuadro de entrada.

<h3 id="resume-after-a-pause">
  Reanudar después de una pausa
</h3>

Si detiene una ejecución, puede reanudarla: los agentes que ya se completaron devuelven sus resultados en caché, y el resto se ejecuta en vivo. Un agente que aún se estaba ejecutando cuando detuvo no se guarda e inicia de nuevo al reanudar, por lo que un workflow que distribuye el trabajo entre muchos agentes pequeños preserva más progreso que un agente largo. Reanude una ejecución pausada desde `/workflows` seleccionándola y presionando `p`, o pida a Claude que relance el workflow con el mismo script.

La reanudación funciona dentro de la misma sesión de Claude Code. Si sale de Claude Code mientras se ejecuta un workflow, la siguiente sesión inicia el workflow de nuevo.

<h3 id="cost">
  Costo
</h3>

Un workflow genera muchos agentes, por lo que una sola ejecución puede usar significativamente más tokens que trabajar a través de la misma tarea en conversación. Las ejecuciones cuentan hacia el uso de su plan y los límites de velocidad como cualquier otra sesión.

Para evaluar el gasto antes de comprometerse con una tarea grande, ejecute el workflow en un segmento pequeño primero: un directorio en lugar de todo el repositorio, o una pregunta estrecha en lugar de una amplia. La vista `/workflows` muestra el uso de tokens de cada agente a medida que avanza la ejecución, y puede detener la ejecución allí en cualquier momento sin perder el trabajo completado. Los límites de agente del runtime [limitan cuántos agentes puede generar una sola ejecución](#behavior-and-limits), lo que limita el costo de un script descontrolado. Para mantener cada ejecución más pequeña de forma predeterminada, [establezca una directriz de tamaño](#set-a-size-guideline) en `/config`.

Claude Code también marca una ejecución que crece inusualmente grande. Cuando un workflow programa más de 25 agentes, o su total de tokens proyectado supera 1,5 millones, su línea de progreso en el panel de tareas debajo del cuadro de entrada muestra una advertencia de `Large workflow`. La advertencia lo dirige a [`/workflows`](#watch-the-run), donde puede detener la ejecución. Requiere Claude Code v2.1.203 o posterior.

La advertencia es informativa: no pausa ni limita la ejecución. Dos configuraciones cambian cuando la ve:

* Si [establece una directriz de tamaño](#set-a-size-guideline), el recuento de agentes de la directriz reemplaza el umbral de 25 agentes.
* Las sesiones con [ultracode](#let-claude-decide-with-ultracode) activado no muestran la advertencia, porque activar ultracode ya lo incluye en ejecuciones grandes.

Cada agente en un workflow usa el modelo de su sesión a menos que el script dirija una etapa a uno diferente o la variable de entorno [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/es/model-config#environment-variables) esté configurada, que anula ambos. Para controlar el costo del modelo:

* Verifique `/model` antes de una ejecución grande si generalmente cambia a un modelo más pequeño para trabajo rutinario
* Pida a Claude que use un modelo más pequeño para etapas que no necesitan el más fuerte cuando describe la tarea

<h3 id="set-a-size-guideline">
  Establecer una directriz de tamaño
</h3>

La configuración de tamaño de workflow dinámico en `/config` mantiene los workflows que Claude escribe en una escala más pequeña de forma predeterminada. Claude Code envía la configuración a Claude como consejo, por lo que un prompt que requiere una escala diferente aún la anula. Requiere Claude Code v2.1.202 o posterior.

Cada valor establece el recuento de agentes que Claude tiene como objetivo en los scripts que escribe.

| Valor          | Orientación enviada a Claude                    |
| :------------- | :---------------------------------------------- |
| `unrestricted` | Sin directriz. Este es el valor predeterminado. |
| `small`        | Apunte a menos de 5 agentes.                    |
| `medium`       | Apunte a menos de 15 agentes.                   |
| `large`        | Apunte a menos de 50 agentes.                   |

Los cambios surten efecto en el siguiente prompt. Los [límites de agente del runtime](#behavior-and-limits) aún se aplican independientemente de la configuración.

<h3 id="turn-workflows-off">
  Desactivar workflows
</h3>

Los workflows están disponibles en la CLI, la aplicación de escritorio, las extensiones del IDE, [modo no interactivo](/docs/es/headless) con `claude -p` y el [Agent SDK](/docs/es/agent-sdk/overview). La misma configuración de desactivación se aplica en cada superficie.

Para desactivar workflows para usted:

* Desactive Dynamic workflows en `/config`. Persiste entre sesiones.
* Establezca `"disableWorkflows": true` en `~/.claude/settings.json`. Persiste entre sesiones.
* Establezca `CLAUDE_CODE_DISABLE_WORKFLOWS=1`. Se lee al inicio, por lo que se aplica dondequiera que lo establezca.

Para desactivar workflows para toda su organización, establezca `"disableWorkflows": true` en [configuración administrada](/docs/es/server-managed-settings), o use el botón de alternancia en la página [configuración de administrador de Claude Code](https://claude.ai/admin-settings/claude-code).

Cuando los workflows están desactivados, los comandos de workflow incluidos no están disponibles, la palabra clave `ultracode` ya no activa una ejecución, y `ultracode` se elimina del menú `/effort`.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Ejecutar agentes en paralelo](/docs/es/agents): comparar subagentes, vista de agente, equipos de agentes y workflows
* [Crear subagentes personalizados](/docs/es/sub-agents): la primitiva de trabajador que orquestan los workflows
* [Gestionar costos](/docs/es/costs): cómo las ejecuciones de múltiples agentes cuentan hacia los límites de uso
