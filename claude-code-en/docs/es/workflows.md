> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orquestar subagentes a escala con flujos de trabajo dinámicos

> Los dynamic workflows orquestan muchos subagentes a partir de un script que Claude escribe y que puede volver a ejecutar. Úselos para auditorías de base de código, migraciones grandes e investigación con verificación cruzada.

<Note>
  Los dynamic workflows están disponibles en todos los planes pagos, con acceso a la API de Anthropic, y en Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry. En Pro, actívelos desde la fila Dynamic workflows en `/config`.
</Note>

Un dynamic workflow es un script de JavaScript que orquesta muchos [subagentes](/docs/es/sub-agents) a la vez. Claude escribe el script para la tarea que describe, y un runtime lo ejecuta en segundo plano mientras su sesión permanece receptiva.

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
  Ejecutar un flujo de trabajo agrupado
</h2>

La forma más rápida de ver un flujo de trabajo en acción es ejecutar `/deep-research`, el [flujo de trabajo integrado](#bundled-workflows) que Claude Code incluye para investigar una pregunta en muchas fuentes. Verá que los agentes trabajan a través de un conjunto de fases en segundo plano mientras su sesión permanece libre, y obtendrá un informe al final en lugar de una transcripción turno por turno.

<Steps>
  <Step title="Ejecutar el flujo de trabajo">
    Ejecute `/deep-research` con una pregunta que desee investigar. Distribuye búsquedas web en varios ángulos, obtiene y verifica cruzadamente las fuentes que encuentra, y sintetiza un informe citado.

    ```text wrap theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Permitir flujos de trabajo">
    Claude Code pregunta si permitir el flujo de trabajo. Seleccione **Sí** para continuar. El mensaje exacto depende de su modo de permisos. Consulte [Aprobar el plan antes de que se ejecute](#approve-the-plan-before-it-runs) para las opciones por modo.
  </Step>

  <Step title="Ver el progreso">
    La ejecución comienza en segundo plano. Ejecute `/workflows`, use las teclas de flecha para seleccionar la ejecución y presione Intro para abrir su vista de progreso:

    ```text wrap theme={null}
    /workflows
    ```

    La vista muestra cada fase con su recuento de agentes, total de tokens y tiempo transcurrido. Profundice en cualquier fase para ver sus agentes y lo que cada uno encontró. Consulte [Ver la ejecución](#watch-the-run) para el conjunto completo de controles.

    También puede ver desde el panel de tareas debajo del cuadro de entrada: aparece un resumen de progreso de una línea mientras se ejecuta. Presione la flecha hacia abajo para enfocarlo y luego Intro para expandirlo.
  </Step>

  <Step title="Leer el informe">
    Cuando se completa la ejecución, el informe llega a su sesión. Cita las fuentes de las que provino cada afirmación, con afirmaciones que no sobrevivieron a la verificación cruzada ya filtradas.

    Cuando los agentes verificadores no pueden verificar una afirmación, como después de un límite de velocidad o error de API, el informe enumera esa afirmación como no verificada en lugar de contarla como refutada.
  </Step>
</Steps>

Para ejecutar un flujo de trabajo para su propia tarea, [haga que Claude escriba uno](#have-claude-write-a-workflow), y una vez que una ejecución haga lo que deseaba, puede [guardarlo](#save-the-workflow-for-reuse) como un comando propio.

<h3 id="bundled-workflows">
  Flujos de trabajo agrupados
</h3>

Claude Code incluye `/deep-research` como un flujo de trabajo integrado:

| Comando                     | Lo que hace                                                                                                                                                                                                                                                                                                                                                    |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/deep-research <question>` | Distribuye búsquedas web en una pregunta en varios ángulos, obtiene y verifica cruzadamente las fuentes que encuentra, vota sobre cada afirmación y devuelve un informe citado con afirmaciones que no sobrevivieron a la verificación cruzada filtradas. Requiere que la [herramienta WebSearch](/docs/es/tools-reference#websearch-tool-behavior) esté disponible |

`/deep-research` se ejecuta solo cuando lo invoca.

[Los flujos de trabajo que guarda](#save-the-workflow-for-reuse) usted mismo se convierten en comandos de la misma manera y aparecen en la autocompletación `/` junto con los agrupados.

<h3 id="watch-the-run">
  Ver la ejecución
</h3>

Los flujos de trabajo se ejecutan en segundo plano, por lo que la sesión permanece receptiva mientras los agentes trabajan. Ejecute `/workflows` en cualquier momento para enumerar los flujos de trabajo en ejecución y completados, luego seleccione uno para abrir su vista de progreso.

La vista de progreso muestra cada fase con sus recuentos de agentes, totales de tokens y tiempo transcurrido. El pie de página enumera la tecla para cada acción:

| Tecla         | Acción                                                                                                           |
| :------------ | :--------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`     | Seleccionar una fase o agente                                                                                    |
| `Enter` o `→` | Profundizar en la fase seleccionada y luego en el detalle de un agente. En el detalle, `Enter` expande o contrae |
| `Esc` o `←`   | Retroceder un nivel. En v2.1.203 a v2.1.205, `←` no retrocedió de una fase o agente; use `Esc` en esas versiones |
| `j` / `k`     | Desplazarse dentro del detalle del agente cuando se desborda                                                     |
| `f`           | Filtrar la lista de agentes en la fase seleccionada por estado. Presione nuevamente para ciclar                  |
| `p`           | Pausar o reanudar la ejecución                                                                                   |
| `x`           | Detener el agente seleccionado o detener todo el flujo de trabajo cuando el enfoque está en la ejecución         |
| `r`           | Reiniciar el agente en ejecución seleccionado                                                                    |
| `s`           | [Guardar](#save-the-workflow-for-reuse) el script de la ejecución como un comando                                |

El detalle del agente enumera el mensaje del agente, sus llamadas de herramientas recientes y su resultado. Cada llamada muestra su estado, como aún en ejecución o fallida. Cuando el agente mantiene su propia lista de tareas, el detalle la muestra también, con el estado de cada tarea.

Presione `Enter` para expandir el detalle. El mensaje y el resultado se muestran entonces en su totalidad, y cada llamada listada muestra su entrada y el inicio de su resultado.

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

Para ejecutar una sola tarea como un workflow sin cambiar el nivel de esfuerzo de la sesión, incluya la palabra clave `ultracode` en su prompt. Pedir con sus propias palabras, por ejemplo "usar un workflow" o "ejecutar un workflow", también funciona: Claude trata una solicitud directa como el mismo opt-in.

```text wrap theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code resalta la palabra clave en su entrada y Claude escribe un script de workflow para la tarea en lugar de trabajar a través de ella turno a turno. La palabra clave solo elige cómo Claude estructura el trabajo: las llamadas de herramientas de los agentes reciben las mismas comprobaciones de permiso y [sandboxing](/docs/es/sandboxing) que cualquier otra llamada de herramienta en la sesión.

Si la ejecución hace lo que deseaba, puede [guardarla como un comando](#save-the-workflow-for-reuse) después. Si ya tiene un orquestador construido de otra manera, como una carpeta de prompts de subagentes o una skill que distribuye trabajo, puede señalar a Claude hacia él y pedir un workflow que haga lo mismo.

<h4 id="dismiss-or-turn-off-the-keyword">
  Descartar o desactivar la palabra clave
</h4>

Si no tenía la intención de iniciar un workflow, presione `Option+W` en macOS o `Alt+W` en Windows y Linux para descartar el resaltado para este prompt, o presione retroceso mientras el cursor está justo después de la palabra clave resaltada. Para evitar que la palabra clave se active en absoluto, desactive Ultracode keyword trigger en `/config`.

<h4 id="where-the-keyword-works">
  Dónde funciona la palabra clave
</h4>

La palabra clave es un opt-in solo en un prompt que escribe usted mismo: en el prompt interactivo, en un panel de extensión IDE, en un cliente [Remote Control](/docs/es/remote-control), o en una aplicación Agent SDK que marca la [`origin`](/docs/es/agent-sdk/typescript#sdkmessageorigin) de su entrada de teclado como `{ kind: "human" }`. No inicia un workflow cuando llega a la sesión de otra manera:

* un prompt pasado con `-p`
* un prompt que una aplicación Agent SDK envía sin marcarlo como entrada humana
* un prompt de tarea programada
* una carga útil de webhook o comentario de solicitud de extracción retransmitido a la conversación

<Note>
  Antes de v2.1.210, la palabra clave iniciaba un workflow desde cualquiera de estas rutas también, incluyendo una carga útil de webhook o comentario de solicitud de extracción retransmitido a la conversación.
</Note>

<h3 id="let-claude-decide-with-ultracode">
  Dejar que Claude decida con ultracode
</h3>

Ultracode es una configuración de Claude Code que combina `xhigh` [esfuerzo de razonamiento](/docs/es/model-config#adjust-effort-level) con orquestación automática de workflows. Con él activado, Claude planifica un workflow para cada tarea sustancial en lugar de esperar a que lo pida.

```text wrap theme={null}
/effort ultracode
```

Para iniciar una sesión con ultracode ya activado, lance con `claude --effort ultracode`. Requiere Claude Code v2.1.203 o posterior.

Para activarlo mientras elige un modelo, mueva el control deslizante de esfuerzo del selector `/model` a `ultracode` con las teclas de flecha. [Ajustar nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) enumera las rutas que activan ultracode.

Con ultracode activado, Claude decide cuándo una tarea justifica un workflow. Una sola solicitud puede convertirse en varios workflows seguidos: uno para entender el código, uno para hacer el cambio y uno para verificarlo. Esto se aplica a cada tarea en la sesión, por lo que cada solicitud usa más tokens y toma más tiempo que en niveles de esfuerzo más bajos.

`/effort ultracode` dura la sesión actual; para que cada sesión comience con él, establezca la configuración [`ultracode`](/docs/es/settings-reference#ultracode). Vuelva con `/effort high` cuando regrese al trabajo rutinario. El menú `/effort` lo ofrece solo [cuando ultracode está disponible](/docs/es/model-config#when-ultracode-is-available).

<h3 id="approve-the-plan-before-it-runs">
  Aprobar el plan antes de que se ejecute
</h3>

En la CLI, el prompt por ejecución muestra las fases planeadas y estas opciones:

* **Yes, run it**: inicia la ejecución
* **Yes, and don't ask again for `<name>` in `<path>`**: inicia y omite este prompt para este workflow en este proyecto de ahora en adelante. Claude Code ofrece esta opción cuando ejecuta un workflow incluido, guardado o de plugin por nombre, no para un script que Claude escribió para la tarea actual.
* **View raw script**: lee el script antes de decidir
* **No**: cancelar

`Ctrl+G` abre el script en su editor. `Tab` le permite ajustar el prompt antes de que comience la ejecución.

Si ve este prompt depende de su [modo de permiso](/docs/es/permission-modes):

| Modo de permiso           | Cuándo se le solicita                                                                                                                                                                                                     |
| :------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Auto                      | Solo en el primer lanzamiento. Cualquier **Yes** registra el consentimiento en su configuración de usuario, y los lanzamientos posteriores comienzan sin solicitar. Se omite completamente cuando ultracode está activado |
| Manual, aceptar ediciones | Cada ejecución, a menos que haya seleccionado **Yes, and don't ask again** para ese workflow en este proyecto                                                                                                             |
| Omitir permisos           | Claude Code no le solicita. La ejecución comienza inmediatamente                                                                                                                                                          |
| `claude -p`, Agent SDK    | Claude Code no le solicita                                                                                                                                                                                                |

En `claude -p` y el Agent SDK, Claude Code nunca muestra este prompt. Ejecuta la llamada de herramienta Workflow a través de la misma [evaluación de permiso](/docs/es/agent-sdk/permissions#how-permissions-are-evaluated) que el resto de la sesión, por lo que las reglas de denegación, las reglas de solicitud y el modo `dontAsk` se aplican al lanzamiento como se aplican a cada llamada de herramienta. Para permitir que el workflow comience en estas ejecuciones, use uno de estos:

* **Regla de permiso**: `Workflow` en sus reglas de permitidos aprueba cada workflow, y `Workflow(<name>)` aprueba un workflow guardado por nombre.
* **Modo de permiso automático**: el [clasificador](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) revisa la llamada y puede aprobarla.
* **Modo de permisos de omisión**: Claude Code aprueba la llamada.
* **Un hook `PreToolUse`**: un [hook](/docs/es/hooks#pretooluse) que devuelve `allow` para la llamada la aprueba.
* **Su host**: un [`--permission-prompt-tool`](/docs/es/cli-reference#cli-flags) la aprueba, o, con el Agent SDK, una devolución de llamada [`canUseTool`](/docs/es/agent-sdk/permissions) o un hook [`PermissionRequest`](/docs/es/hooks#permissionrequest) la aprueba.

En la aplicación de escritorio, una tarjeta de aprobación muestra el nombre del workflow, la lista de fases y una advertencia de uso de tokens, con acciones **Once**, **Always** y **Deny**. La vista de progreso aparece en el panel lateral Background tasks.

Los subagentes que genera el workflow usan sus [reglas de permiso](/docs/es/settings-reference#permission-settings), y Claude Code elige su modo de permiso por las reglas bajo [qué modo de permiso ejecuta un subagente](/docs/es/sub-agents#permission-modes). Para evitar solicitudes en una ejecución larga, agregue las herramientas que necesitan los agentes a sus reglas de permitidos antes de comenzar.

<h3 id="save-the-workflow-for-reuse">
  Guardar el workflow para reutilización
</h3>

Cuando Claude escribe un workflow para una tarea que repetirá, puede guardar el script de esa ejecución como un comando. Un proceso como una revisión que ejecuta en cada rama luego ejecuta la misma orquestación cada vez.

Ejecute `/workflows`, seleccione la ejecución que desea mantener y presione `s`. En el diálogo de guardado, Tab alterna entre las dos ubicaciones de guardado:

* `.claude/workflows/` en su proyecto: compartido con todos los que clonan el repositorio
* `~/.claude/workflows/` en su directorio de inicio: disponible en cada proyecto, visible solo para usted. Si establece [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars), esta ubicación es el directorio `workflows/` bajo esa ruta.

El diálogo de guardado muestra la ruta resuelta para la ubicación personal.

Presione Enter para guardar. El workflow se ejecuta como `/<name>` en futuras sesiones desde cualquier ubicación.

Claude Code comprueba la ubicación de guardado para buscar enlaces simbólicos antes de escribir y muestra un error en lugar de escribir a través de uno. Lo que comprueba depende de dónde guarde:

* Ubicación del proyecto: Claude Code rechaza si `.claude`, `.claude/workflows` o el archivo de destino es un enlace simbólico.
* Ubicación personal: Claude Code rechaza solo si el archivo de destino en sí es un enlace simbólico, por lo que un directorio `~/.claude` administrado por una herramienta de dotfiles aún funciona.

Antes de v2.1.216, Claude Code seguía el enlace, lo que podría colocar el archivo fuera de la ubicación que eligió.

En un monorepo con varios directorios `.claude/`, puede mantener workflows junto al paquete al que se aplican. Guardar en la ubicación del proyecto escribe en el directorio `.claude/workflows/` más cercano que ya existe entre su directorio de trabajo y la raíz del repositorio, o en la raíz del repositorio si aún no existe ninguno. Los workflows del proyecto también se cargan desde cada `.claude/workflows/` a lo largo de esa ruta, y cuando más de uno define el mismo nombre Claude Code ejecuta el más cercano al directorio de trabajo.

Si un workflow de proyecto y un workflow personal comparten un nombre, se ejecuta el del proyecto.

<h3 id="distribute-a-workflow-in-a-plugin">
  Distribuir un workflow en un plugin
</h3>

Para compartir un workflow entre equipos o repositorios, inclúyalo en un [plugin](/docs/es/plugins/overview). Coloque el script en un directorio `workflows/` en la raíz del plugin, o apunte a una ubicación diferente con el [`workflows` campo de manifiesto](/docs/es/plugins/manifest-reference#fields).

Los workflows de plugin se espacian por el nombre del plugin. Un plugin llamado `acme-tools` que contiene un script cuyo `meta.name` es `release-audit` se ejecuta como `/acme-tools:release-audit`.

<h3 id="pass-input-to-a-saved-workflow">
  Pasar entrada a un workflow guardado
</h3>

Un workflow guardado puede aceptar entrada a través del parámetro `args`. El script lo lee como una variable global llamada `args`. Úselo para proporcionar una pregunta de investigación, una lista de rutas de destino o un objeto de configuración en el momento de la invocación en lugar de editar el script para cada ejecución.

El siguiente prompt ejecuta un workflow guardado con una lista de números de problemas:

```text wrap theme={null}
Run /triage-issues on issues 1024, 1025, and 1030
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

```text wrap theme={null}
use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Seguir arreglando hasta que pase una verificación
</h3>

Ejecute un verificador, arregle lo que falló y repita hasta que pase o deje de hacer progreso.

```text wrap theme={null}
use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Migrar muchos archivos en paralelo
</h3>

Descubra los archivos a migrar, transforme cada uno en una copia aislada para que las ediciones no entren en conflicto, y verifique cada resultado.

```text wrap theme={null}
use a workflow to migrate every component under src/components/ from JavaScript to TypeScript, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Revisar cada archivo modificado y escribir un resumen
</h3>

Ejecute un revisor por archivo, luego entregue todos los hallazgos a un agente que los clasifique y deduplique.

```text wrap theme={null}
use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Investigar un tema en muchas fuentes
</h3>

Distribuya lectores en registros de cambios, problemas y documentos, luego sintetice. El workflow `/deep-research` incluido hace esto; también puede describir una versión más estrecha.

```text wrap theme={null}
use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Encontrar problemas hasta que la lista deje de crecer
</h3>

Siga buscando en rondas y deténgase cuando nuevas rondas no encuentren nada nuevo.

```text wrap theme={null}
use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
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

El cuerpo es JavaScript simple con `await` de nivel superior. `agent()` genera un subagente, `pipeline()` ejecuta uno por elemento en una lista, y `parallel()` ejecuta un conjunto de tareas de agente al mismo tiempo y espera a que todas se completen.

Una llamada a `agent()` se resuelve a `null` si la detiene a mitad de la ejecución o si alcanza un error de API irrecuperable. `pipeline()` mantiene cada `null` en la matriz de resultados, por lo que el ejemplo termina con `.filter(Boolean)` para descartar esas entradas.

En [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode), el prompt que su script pasa a `agent()` no cuenta como una solicitud suya cuando el clasificador revisa las acciones de ese subagente, porque Claude Code lo marca como texto que el script calculó.

Si pasa un `schema` en una llamada a `agent()`, ese subagente devuelve JSON que coincide con la forma en lugar de prosa. Claude Code verifica el schema antes de iniciar el subagente: cuando puede probar que el schema se contradice a sí mismo, la llamada falla con un error que nombra la contradicción, y el subagente nunca comienza. Una contradicción que puede probar es una clave `required` que `additionalProperties: false` descarta.

Si la salida del subagente aún falla la validación después de cinco intentos, la llamada falla con un error que incluye el último fallo de validación. Para cambiar el número de intentos, establezca [`MAX_STRUCTURED_OUTPUT_RETRIES`](/docs/es/env-vars).

<h3 id="edit-a-saved-script">
  Editar un script guardado
</h3>

Para cambiar un [workflow que guardó](#save-the-workflow-for-reuse), edite su archivo `.js` o pida a Claude que haga el cambio. Antes de editar o preguntar, ejecute la [skill incluida](/docs/es/skills#bundled-skills) `/workflow-authoring` para cargar la referencia de escritura de scripts con la que Claude trabaja. La skill requiere Claude Code v2.1.248 o posterior.

Para ejecutar la versión editada en la sesión actual, ejecute [`/reload-skills`](/docs/es/commands#all-commands) para releer los directorios de workflow, luego ejecute `/<name>` nuevamente.

Claude Code aplica estas reglas a cada parte del archivo cuando carga y ejecuta el script:

* **Bloque `meta`**: mantenga `export const meta` como la primera declaración, y manténgalo como un objeto literal simple con un `name` y una `description`. Si contiene algo que no sea valores literales, como una variable, una llamada a función o una propagación, Claude Code elimina `/<name>` del autocompletado `/`.
* **Cuerpo**: además de `agent()`, `pipeline()` y `parallel()`, puede llamar a `phase()` para agrupar los agentes que siguen bajo un título en la vista de progreso, llamar a `log()` para mostrar un mensaje encima de las fases, y leer el global [`args`](#pass-input-to-a-saved-workflow). Si el cuerpo tiene un error de sintaxis, Claude Code lo reporta cuando ejecuta el workflow.
* **`phases`**: si las enumera en `meta`, dé a cada entrada exactamente el título que pasa a `phase()`. Un título de `phase()` sin entrada obtiene su propio grupo de progreso.
* **Marcas de tiempo y aleatoriedad**: Claude Code hace que `Date.now()`, `Math.random()` y un `new Date()` sin argumentos lancen una excepción dentro del script, para que una [ejecución relanzada](#resume-after-a-pause) repita las mismas llamadas a `agent()`. Pase una marca de tiempo a través de `args` en su lugar.

También puede editar [el script de una única ejecución](#how-a-workflow-runs) en lugar de la copia guardada. [Reanudar después de una pausa](#resume-after-a-pause) cubre qué agentes se ejecutan nuevamente cuando relanza un script editado. Para las entradas de la herramienta Workflow, consulte su entrada en la [referencia del Agent SDK](/docs/es/agent-sdk/typescript#workflow).

<h2 id="how-a-workflow-runs">
  Cómo se ejecuta un workflow
</h2>

El runtime del workflow ejecuta el script en un entorno aislado, separado de su conversación. Los resultados intermedios permanecen en variables de script en lugar de llegar al contexto de Claude.

Cada ejecución escribe su script en un archivo bajo el directorio de su sesión en `~/.claude/projects/`. Claude recibe la ruta cuando comienza la ejecución, por lo que puede solicitarla. Puede abrir ese archivo para leer la orquestación que Claude escribió, compararlo con el script de una ejecución anterior, o editarlo y pedir a Claude que reinicie desde la versión editada.

Claude solo puede iniciar un workflow desde un archivo de script que la sesión ya tenga permitido leer. Para ejecutar un script guardado fuera de su directorio de trabajo, primero agregue su directorio con [`/add-dir`](/docs/es/permissions#working-directories) o una [regla de permiso Read](/docs/es/permissions#read-and-edit).

El runtime rastrea el resultado de cada agente a medida que avanza la ejecución, lo que es lo que hace que una ejecución sea [reanudable](#resume-after-a-pause) dentro de la misma sesión.

<h3 id="prompt-caching-in-a-fan-out">
  Prompt caching en un fan-out
</h3>

Los agentes en la misma ejecución pueden leer el [prompt cache](/docs/es/prompt-caching#subagents-and-the-cache) de los demás. Dos agentes que se ejecutan con el mismo modelo, nivel de esfuerzo, tipo de agente, herramientas, esquema de salida y directorio de trabajo construyen el mismo prefijo de herramientas y prompt del sistema, por lo que un agente que comienza después de que la respuesta de un hermano coincidente ha comenzado lee el cache de ese hermano en su primera solicitud.

Las solicitudes de un agente de workflow caen fuera del [bucket de TTL de cache](/docs/es/prompt-caching#which-ttl-each-request-gets) de la conversación principal, por lo que su cache se mantiene durante cinco minutos por defecto, incluso en una suscripción de Claude. Para mantenerlo durante una hora, establezca [`subagentPromptCacheTtl`](/docs/es/settings-reference#subagentpromptcachettl) en `1h`. La API factura las escrituras de cache de una hora a una tasa más alta.

Cuando un fan-out inicia varios agentes coincidentes a la vez, Claude Code mantiene todos excepto el primero hasta que comienza la respuesta del primer agente, luego libera los agentes retenidos juntos para que sus primeras solicitudes lean el prefijo compartido en lugar de que cada uno lo procese sin cache. Claude Code limita la retención a [`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS`](/docs/es/env-vars) milisegundos, `5000` por defecto. Establézcalo en `0` para desactivar la retención.

<h3 id="behavior-and-limits">
  Comportamiento y límites
</h3>

El runtime aplica las siguientes restricciones:

| Restricción                                                                                                                                                                                                                                                                                                                            | Por qué                                                                                                                                                                                                                    |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sin entrada de usuario a mitad de ejecución                                                                                                                                                                                                                                                                                            | Una ejecución se pausa por sí sola solo para prompts de permiso del agente y una [espera de límite de uso](#when-a-run-hits-your-usage-limit). Para la aprobación entre etapas, ejecute cada etapa como su propio workflow |
| Sin acceso directo al sistema de archivos o shell desde el workflow en sí                                                                                                                                                                                                                                                              | Los agentes leen, escriben y ejecutan comandos. El script coordina los agentes                                                                                                                                             |
| Sin carga de módulos: un script que contiene `import()` falla antes de que comience la ejecución                                                                                                                                                                                                                                       | El cuerpo del script es JavaScript simple. Coloque el trabajo que necesita una biblioteca en la tarea de un agente                                                                                                         |
| Hasta 16 agentes concurrentes por defecto, menos cuando Claude Code tiene menos CPUs disponibles, incluso dentro de un contenedor con CPU limitada. Para cambiar el límite, establezca [`CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS`](/docs/es/env-vars#variables) en un valor de 1 a 256, lo que requiere Claude Code v2.1.269 o posterior | Limita el uso de recursos locales                                                                                                                                                                                          |
| En un fan-out, los agentes que comparten el prefijo de prompt-cache del primer agente se inician hasta 5 segundos después por defecto                                                                                                                                                                                                  | Todos excepto el primero leen el [prefijo que el primer agente almacenó en cache](#prompt-caching-in-a-fan-out) en lugar de que cada uno lo procese sin cache                                                              |
| Hasta 4.096 elementos en una única llamada `parallel()` o `pipeline()`: el runtime rechaza una lista más larga con un error                                                                                                                                                                                                            | Un límite silencioso dejaría caer parte de la carga de trabajo sin informar al script                                                                                                                                      |
| 1.000 agentes totales por ejecución                                                                                                                                                                                                                                                                                                    | Previene bucles descontrolados                                                                                                                                                                                             |

<h2 id="manage-runs">
  Gestionar ejecuciones
</h2>

Una vez que comienza una ejecución, la gestiona desde la vista `/workflows`, o expandiendo su línea de progreso en el panel de tareas debajo del cuadro de entrada.

Cuando detiene una ejecución, permanece en el panel de tareas mientras cualquiera de los procesos de sus agentes siga ejecutándose. Si la detiene de nuevo, Claude Code reenvía señales a esos procesos.

<h3 id="resume-after-a-pause">
  Reanudar después de una pausa
</h3>

Reanude una ejecución pausada desde `/workflows` seleccionándola y presionando `p`. Para una ejecución que detuvo, pida a Claude que relance el workflow con el mismo script. Si los agentes de la ejecución detenida aún no han salido, Claude Code rechaza el relanzamiento hasta que lo hagan, por lo que una segunda copia de esos agentes no puede ejecutarse junto a ellos.

Claude Code reproduce la ejecución en el orden en que los agentes comenzaron, y cada agente devuelve su resultado guardado o se ejecuta de nuevo:

* **Completado**: devuelve su resultado guardado. El primer agente cuyo prompt difiere de la ejecución anterior, porque editó el script o un agente anterior devolvió algo diferente, se ejecuta de nuevo, y también lo hace cada agente después de él, incluso los que se completaron.
* **Aún en ejecución cuando detuvo**: comienza de nuevo. Detener toda la ejecución no cuenta ningún agente como fallido.
* **Fallido**: se ejecuta de nuevo, y también lo hace cada agente que comenzó después de él, incluso los que se completaron. Detener un solo agente, seleccionándolo en [`/workflows`](#watch-the-run) y presionando `x`, cuenta como fallido.

Ese último caso significa que una falla en el medio de un fan-out reejecutará trabajo que ya se completó. Si un script inicia A, B, C y D en ese orden y B falla, relanzar devuelve A del caché y ejecuta B, C y D de nuevo.

Puede reanudar una ejecución dentro de la misma sesión de Claude Code. Lo que sucede con un workflow en ejecución cuando sale de la sesión depende de cómo salga:

* Si [coloca la sesión en segundo plano](/docs/es/agent-view#what-carries-over-when-you-background), Claude Code reproduce la ejecución de la misma manera en la sesión en segundo plano y la continúa.
* Si sale de Claude Code mientras se ejecuta un workflow y [la vista de agente está activada](/docs/es/agent-view#from-inside-a-session), el diálogo de salida ofrece `Move to background and exit`, que lleva la ejecución de la misma manera. Si elige `Exit and stop tasks` en su lugar, o la opción no se ofrece, la ejecución se detiene con la sesión. Claude Code mantiene los resultados guardados de la ejecución bajo el directorio de esa sesión en `~/.claude/projects/`, por lo que una sesión que reanuda con `claude --resume` puede reproducirlos cuando pide a Claude que relance el workflow. En una sesión que inicia nueva, Claude no tiene ninguna ejecución anterior que reproducir e inicia el workflow de nuevo como una nueva ejecución.

En una [sesión en la nube](/docs/es/claude-code-on-the-web), Claude Code también guarda los resultados de la ejecución con el historial de conversación de la sesión, que sobrevive cuando se reclama la VM de la sesión. Cuando [reabre tal sesión](/docs/es/claude-code-on-the-web#environment-expired) y pide a Claude que relance el workflow, los agentes completados aún devuelven sus resultados guardados.

En sesiones locales y en la nube por igual, cuando Claude relanza una ejecución anterior y Claude Code no puede encontrar los resultados guardados de esa ejecución en absoluto, el relanzamiento falla con un error `nothing to resume` en lugar de iniciar la ejecución de nuevo por su cuenta. Pida a Claude que inicie el workflow de nuevo como una nueva ejecución.

<h3 id="when-a-run-hits-your-usage-limit">
  Cuando una ejecución alcanza su límite de uso
</h3>

Cuando un agente alcanza su límite de uso de [claude.ai](/docs/es/interactive-mode#wait-for-a-usage-limit-to-reset), la ejecución se pausa en lugar de fallar ese agente: los agentes que alcanzan el límite esperan el reinicio, y no comienzan nuevos agentes. Poco después de que se reinicia el límite, los agentes en espera se ejecutan de nuevo y la ejecución continúa por su cuenta. Requiere Claude Code v2.1.271 o posterior; en versiones anteriores, los agentes afectados fallan.

Mientras la ejecución espera, su línea de progreso en el panel de tareas y el encabezado [`/workflows`](#watch-the-run) muestran cuándo se reinicia el límite.

La ejecución se pausa solo cuando se cumplen todas estas condiciones; cuando una no se cumple, el agente afectado falla en su lugar:

* La sesión es interactiva e inició sesión con una suscripción de claude.ai. Una ejecución no se pausa en [modo no interactivo](/docs/es/headless) con `claude -p` o el [Agent SDK](/docs/es/agent-sdk/overview), en una [sesión en segundo plano](/docs/es/agent-view), o en una sesión de compañero de [Remote Control](/docs/es/remote-control) o [equipo de agentes](/docs/es/agent-teams).
* [`autoContinueAtUsageLimit`](/docs/es/settings-reference#autocontinueatusagelimit) está activado, la misma configuración que permite que la sesión misma [espere a que se reinicie un límite de uso](/docs/es/interactive-mode#wait-for-a-usage-limit-to-reset). Si lo desactiva durante una espera, la espera termina y los agentes en espera fallan.
* El límite se reinicia dentro de 24 horas. Un límite semanal puede reiniciarse más adelante.
* La ejecución aún no ha esperado dos veces. Cuando alcanza el límite por tercera vez, el agente falla.

<h3 id="cost">
  Costo
</h3>

Un workflow genera muchos agentes, por lo que una sola ejecución puede usar significativamente más tokens que trabajar a través de la misma tarea en conversación. Las ejecuciones cuentan hacia el uso de su plan y los límites de velocidad.

Para evaluar el gasto antes de comprometerse con una tarea grande, ejecute el workflow en un segmento pequeño primero: un directorio en lugar de todo el repositorio, o una pregunta estrecha en lugar de una amplia. La vista `/workflows` muestra el uso de tokens de cada agente a medida que avanza la ejecución, y puede detener la ejecución allí en cualquier momento, generalmente sin perder el trabajo completado. [Reanudar después de una pausa](#resume-after-a-pause) cubre lo que una ejecución detenida conserva. Los límites de agente del runtime [limitan cuántos agentes puede generar una sola ejecución](#behavior-and-limits), lo que limita el costo de un script descontrolado. Para mantener las ejecuciones con menos agentes, elija la directriz de tamaño `small` [](#set-a-size-guideline).

Claude Code también marca una ejecución que crece inusualmente grande. Cuando un workflow programa más de 25 agentes, o su total de tokens proyectado supera 1,5 millones, su línea de progreso en el panel de tareas debajo del cuadro de entrada muestra una advertencia de `Large workflow`. La advertencia lo dirige a [`/workflows`](#watch-the-run), donde puede detener la ejecución.

La advertencia es informativa: no pausa ni limita la ejecución. Dos configuraciones cambian cuando la ve:

* Si elige una [directriz de tamaño](#set-a-size-guideline) usted mismo, su recuento de agentes reemplaza el umbral de 25 agentes. La directriz predeterminada integrada deja el umbral en 25.
* Las sesiones con [ultracode](#let-claude-decide-with-ultracode) activado no muestran la advertencia, porque activar ultracode ya lo incluye en ejecuciones grandes.

Claude Code elige el modelo de cada agente de workflow en el mismo [orden que usa para subagentes](/docs/es/sub-agents#choose-a-model). Un modelo que el script nombra para una etapa cuenta como el modelo por invocación en ese orden. Cuando nada más asigna uno, el agente se ejecuta en el modelo de su sesión.

Para controlar el costo del modelo:

* Verifique `/model` antes de una ejecución grande si generalmente cambia a un modelo más pequeño para trabajo rutinario
* Pida a Claude que use un modelo más pequeño para etapas que no necesitan el más fuerte cuando describe la tarea

Cuando la lista de permitidos [`availableModels`](/docs/es/model-config#restrict-model-selection) de su organización bloquea un modelo que el script solicita para un agente, ese agente se ejecuta en un modelo sustituido en su lugar, siguiendo las mismas [reglas de sustitución que los subagentes](/docs/es/sub-agents#choose-a-model). La vista de progreso de la ejecución en [`/workflows`](#watch-the-run) muestra una advertencia que nombra tanto los modelos solicitados como los sustituidos.

<h3 id="set-a-size-guideline">
  Establecer una directriz de tamaño
</h3>

Una directriz de tamaño le dice a Claude cuántos agentes debe apuntar cuando escribe un workflow dinámico. Claude Code envía la directriz a Claude como consejo, no como un límite, por lo que un prompt que requiere una escala diferente aún la anula. Requiere Claude Code v2.1.202 o posterior.

Cada valor se asigna a un recuento de agentes:

| Valor          | Recuento de agentes que Claude tiene como objetivo      |
| :------------- | :------------------------------------------------------ |
| `unrestricted` | Sin directriz: Claude dimensiona el workflow a la tarea |
| `small`        | Menos de 5 agentes                                      |
| `medium`       | Menos de 10 agentes                                     |
| `large`        | Menos de 50 agentes                                     |

El valor predeterminado es `medium`, o `small` cuando inicia sesión en un plan Pro con Claude Code v2.1.271 o posterior. Hasta que elija un valor, la fila `/config` marca el valor como predeterminado, y la línea `Running in background` del workflow nombra el tamaño en vigor. Requiere Claude Code v2.1.219 o posterior; las versiones anteriores tienen como valor predeterminado `unrestricted`.

Para cambiar la directriz, elija un valor para la configuración Dynamic workflow size en `/config`, o ejecute `/config workflowSizeGuideline=small`. En v2.1.219 y posterior, también puede establecer la clave [`workflowSizeGuideline`](/docs/es/settings-reference#workflowsizeguideline) en cualquier archivo de configuración; ese valor tiene prioridad sobre `/config`, y Claude Code oculta la fila `/config` mientras un archivo de configuración proporciona uno.

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

Cuando los workflows están desactivados, los comandos de workflow incluidos y la skill `/workflow-authoring` no están disponibles, la palabra clave `ultracode` ya no activa una ejecución, y `ultracode` se elimina del menú `/effort`.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Ejecutar agentes en paralelo](/docs/es/agents): comparar subagentes, vista de agente, equipos de agentes y workflows
* [Crear subagentes personalizados](/docs/es/sub-agents): la primitiva de trabajador que orquestan los workflows
* [Gestionar costos](/docs/es/costs): cómo las ejecuciones de múltiples agentes cuentan hacia los límites de uso
