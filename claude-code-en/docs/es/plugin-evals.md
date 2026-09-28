> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Prueba plugins con evals

> Escriba casos de eval para su plugin de Claude Code, ejecútelos con claude plugin eval, califique los resultados, compare con una línea base sin plugin y controle CI en la puntuación.

El comando shell `claude plugin eval` ejecuta su [plugin](/docs/es/plugins/overview) contra un conjunto de casos de prueba y califica los resultados. Cada caso es un prompt realista más uno o más calificadores. Un calificador es una verificación de aprobación/fallo sobre lo que Claude produjo, como una expresión regular sobre la respuesta, si se llamó a una herramienta particular, o una rúbrica que un segundo modelo juzga sobre la respuesta.

No tiene que escribir el conjunto a mano; `claude plugin eval init` le pregunta sobre su plugin, propone los casos y calificadores, los prueba, y escribe los archivos, y puede pedirle a Claude que haga lo mismo desde una sesión que ya tiene abierta.

Use evals para:

* Medir qué tan confiablemente su plugin dirige a Claude a producir el resultado correcto
* Detectar regresiones cuando cambia el plugin o se lanza un nuevo modelo
* Ver qué contribuye el plugin en comparación con ningún plugin

Esta página es para autores de plugins y skills que tienen un plugin funcional y desean probar su comportamiento, y para equipos que controlan cambios de plugins en CI. Su formato de caso es separado del archivo `evals/evals.json` que usa el [plugin skill-creator](/docs/es/skills#run-evals-with-skill-creator). Para crear un plugin, consulte [Crear un plugin](/docs/es/plugins/create); para verificar los archivos de un plugin en busca de errores de sintaxis y esquema en lugar de su comportamiento, use [`claude plugin validate`](/docs/es/plugins/cli-reference#plugin-validate).

<Note>
  Cada ejecución de eval y cada calificador de juez es una llamada de modelo real en su cuenta, contada contra el uso de su plan o su factura de API, así que verifique los [requisitos](#requirements) primero. Luego [cree su primer conjunto de eval](#create-your-first-eval-suite), o vaya a [Ejecutar evals en CI](#run-evals-in-ci) si ya tiene uno.
</Note>

<h2 id="requirements">
  Requisitos
</h2>

Para ejecutar evals de plugins necesita:

* Claude Code v2.1.269 o posterior. Ejecute `claude --version` para verificar y `claude update` para actualizar.
* Un directorio de plugin con un manifiesto `plugin.json` o `.claude-plugin/plugin.json`, o un [plugin de directorio de skills](/docs/es/plugins/loading#plugins-shared-through-a-repository).
* La misma autenticación y proveedor de modelo que usan sus sesiones normales de Claude Code. Las ejecuciones de eval, los calificadores puntuados por juez, y `claude plugin eval init` llaman al modelo con sus credenciales, por lo que cuentan contra sus límites de uso del plan o su factura de API. Cuando el comando reporta un costo, la cifra es una [estimación de precio de lista](/docs/es/costs) de esas llamadas.

<h2 id="how-an-eval-run-works">
  Cómo funciona una ejecución de eval
</h2>

Un conjunto de eval vive en un directorio llamado `evals/` dentro de su plugin, distribuido como muestra [Escribir y refinar casos](#write-and-refine-cases). Cada caso es su propio subdirectorio con un [prompt](#set-run-limits-and-tools-in-prompt-md) y uno o más [calificadores](#grade-the-result). El prompt es algo que una persona que usa su plugin podría escribir, como una solicitud que uno de sus skills debería manejar.

<h3 id="what-happens-in-a-run">
  Qué sucede en una ejecución
</h3>

Para cada ejecución de un caso, Claude Code inicia una sesión nueva, [aislada](#how-runs-are-isolated) [no interactiva](/docs/es/headless) con solo su plugin cargado, envía el prompt, y deja que Claude trabaje hasta que termine o alcance el límite de turnos o tiempo del caso. Cada calificador luego verifica la respuesta final, la transcripción, o un archivo que Claude creó, y aprueba o falla.

<h3 id="how-a-case-is-scored">
  Cómo se califica un caso
</h3>

Una ejecución de un agente no determinista le dice poco, así que cada caso se ejecuta tres veces por defecto. La puntuación de una ejecución es la fracción de sus calificadores que aprobaron, ponderada si establece pesos, y la puntuación del caso es el promedio en sus ejecuciones. Un caso aprueba cuando su puntuación cumple con el [`--threshold`](#command-options), `1.0` por defecto.

En llamadas de modelo, un conjunto hace aproximadamente casos × ejecuciones ejecuciones de agentes con el plugin y tantas más para la [línea base sin plugin](#the-no-plugin-baseline), más tres llamadas cortas de juez por calificador `llm` o `baseline` por ejecución.

<h3 id="the-no-plugin-baseline">
  La línea base sin plugin
</h3>

Una puntuación alta por sí sola no le dice que el plugin ayudó, porque Claude podría hacerlo igual sin él. Para separar los dos, las ejecuciones de cada caso se repiten sin plugin cargado por defecto, y obtiene dos puntuaciones, `WITH` y `W/OUT`. Su diferencia, `Δ`, es lo que el plugin contribuyó. Si un caso puntúa 1.0 tanto con como sin el plugin, el plugin no es lo que lo hizo pasar.

Los dos conjuntos de ejecuciones se llaman el brazo con y el brazo sin; [Comparar con una línea base sin plugin](#compare-against-a-no-plugin-baseline) cubre cómo se califican los calificadores en ambos brazos y cómo desactivar la línea base.

<h2 id="create-your-first-eval-suite">
  Cree su primer conjunto de eval
</h2>

Este tutorial escribe un caso para su propio plugin, lo ejecuta, y lee el resultado. Antes de comenzar, asegúrese de tener:

* Claude Code v2.1.269 o posterior y los otros [requisitos](#requirements)
* Una terminal abierta en el directorio raíz de su plugin, el que contiene `plugin.json` o `.claude-plugin/plugin.json`
* Un skill en el plugin que desea probar, y una solicitud que un usuario escribiría que debería activarlo

<Steps>
  <Step title="Crear los casos">
    Desde la raíz del plugin, ejecute:

    ```bash theme={null}
    claude plugin eval init
    ```

    Si Claude Code aún no confía en este directorio, primero pregunta `Trust this plugin directory?`; responda `y`.

    Luego se abre una sesión interactiva de Claude Code. Claude lee su plugin y le pregunta qué se vería bien, propone prompts que deberían y no deberían activar el plugin, diseña calificadores para cada uno, los prueba una vez para verificar que se comportan, y escribe un directorio de caso por prompt bajo `evals/`, cada uno nombrado según su prompt. Cuando Claude le dice que el conjunto está listo, salga de esa sesión con `/exit` o Ctrl+D para volver a su shell.

    Si ya tiene una sesión de Claude Code abierta en la raíz del plugin, puede pedirle a Claude que ejecute `claude plugin eval init`. Claude ejecuta el comando y luego le hace las mismas preguntas en esa conversación.

    Si prefiere escribir un caso usted mismo para ver exactamente qué contienen los archivos, siga [Escribir un caso a mano](#write-a-case-manually) y vuelva aquí para ejecutarlo.
  </Step>

  <Step title="Ejecutar el conjunto">
    De vuelta en su shell en la raíz del plugin, ejecute cada caso bajo `evals/`:

    ```bash theme={null}
    claude plugin eval .
    ```

    Ya confió en este directorio durante el paso 1, así que la ejecución comienza inmediatamente. Si escribió el caso a mano en su lugar, la ejecución primero pregunta `Trust this plugin directory? [y/N]`; responda `y`. [Lo que una ejecución puede acceder](#security) explica a qué está accediendo.

    Cada caso se ejecuta tres veces con su plugin y tres veces sin él, así que un caso es seis ejecuciones. Una línea de progreso se imprime cuando cada ejecución termina, con la puntuación de esa ejecución y el veredicto de cada calificador.
  </Step>

  <Step title="Leer el resumen">
    Cuando el conjunto termina ve una tabla de resumen, seguida de dónde fue el informe:

    ```text theme={null}
    CASE        WITH  W/OUT Δ      RUNS COST    NOTES
    first-case  1.00  0.33  +0.67  6    $0.41

    1 case(s) · mean Δ +0.67 · 74s · $0.41
    Report: /Users/you/my-plugin/evals/results/2026-09-10T17-02-11-482Z/report.html
    Published: https://claude.ai/... · keep local next time with --no-publish
    ```

    `WITH` es la puntuación del caso con su plugin cargado, `W/OUT` es la puntuación sin él, y un `Δ` positivo significa que el plugin aumentó la puntuación. `COST` es una estimación de precio de lista de las llamadas de modelo, y `NOTES` muestra la explicación del calificador que falla con mayor peso, o el error de la ejecución, del brazo con.
  </Step>

  <Step title="Abrir el informe e iterar">
    Abra la URL `Published:`, o la ruta `Report:` cuando no aparezca una línea `Published:`, para ver el veredicto de cada calificador y la explicación para cada ejecución, y para calificadores `llm` los votos del juez y el fragmento que juzgó. La línea `Published:` aparece solo cuando su cuenta puede [publicar informes](#html-report).

    El hallazgo más común al principio es un `Δ` cerca de cero con el calificador `tool_used: Skill` del caso fallando, lo que significa que Claude no está eligiendo su skill en fraseología natural. Ajuste la [`description`](/docs/es/skills#frontmatter-reference) del skill, ejecute `claude plugin eval .` nuevamente, y compare.

    Para iterar en un caso de manera económica, ejecute un solo brazo una vez. Una sola ejecución es ruidosa, así que confirme cualquier cambio en las tres ejecuciones predeterminadas antes de confiar en él. Con un brazo la tabla muestra columnas `SCORE` y `PASS%` en lugar de `WITH`, `W/OUT`, y `Δ`:

    ```bash theme={null}
    claude plugin eval . --case <case-name> --runs 1 --ablation none
    ```

    Reemplace `<case-name>` con uno de los nombres de directorio bajo `evals/`.
  </Step>
</Steps>

<h2 id="write-and-refine-cases">
  Escribir y refinar casos
</h2>

Los casos que `claude plugin eval init` escribe son archivos simples que puede abrir, cambiar, y agregar. Un caso es un directorio bajo el directorio de eval del plugin que contiene un `prompt.md`, un `case.yaml`, o ambos. Para agrupar casos, anídelos bajo un directorio que no sea en sí mismo un caso; cualquier cosa dentro de un directorio de caso, como `graders/` y archivos de fixture, pertenece a ese caso.

Este es el diseño que `claude plugin eval init` escribe y el que usar para nuevos conjuntos. La [referencia de conjunto de eval](#eval-suite-reference) tiene el árbol completo, incluyendo mocks y resultados:

```text theme={null}
my-plugin/
├── .claude-plugin/plugin.json
├── skills/...
└── evals/
    ├── first-case/
    │   ├── prompt.md          # frontmatter: case fields; body: the prompt
    │   ├── graders/
    │   │   ├── criteria.md    # frontmatter: type + options; body: rubric or pattern
    │   │   └── skill-fired.md
    │   └── case.yaml          # optional: only for context.* fields
    ├── ignores-unrelated-request/
    │   └── ...
    └── results/               # written by each run; add to .gitignore
```

<h3 id="write-a-case-manually">
  Escribir un caso a mano
</h3>

Hacer que Claude escriba los casos con `claude plugin eval init` es el camino recomendado. Para escribir uno usted mismo en su lugar, comience desde una plantilla en blanco. El siguiente comando escribe un caso llamado `first-case` con un `prompt.md` de marcador de posición y un calificador de marcador de posición, y no ejecuta nada:

```bash theme={null}
claude plugin eval init --bare first-case
```

```text theme={null}
evals/first-case/
├── prompt.md            # the prompt sent to Claude, plus run limits
└── graders/
    └── criteria.md      # one grader: how to score the result
```

En `prompt.md` escribe el mensaje que Claude recibe en cada ejecución, y establece los límites de la ejecución y las herramientas que el caso puede usar en su frontmatter. Abra `evals/first-case/prompt.md` y reemplace el cuerpo del marcador de posición con una solicitud que uno de sus skills debería manejar, fraseada de la manera que un usuario la escribiría en lugar de nombrar el skill. Este ejemplo es para un skill que redacta mensajes de commit; use su propia solicitud:

```markdown theme={null}
---
max_turns: 10
allowed_tools: [Read, Glob, Grep, Skill]
---

Write me a commit message for this change: I renamed getUser to fetchUser and updated the three call sites.
```

Cada ejecución comienza en un directorio de trabajo vacío, así que ponga lo que la tarea necesita en el prompt mismo, o [configure el espacio de trabajo](#add-setup-or-history-with-case-yaml) primero.

La [lista completa de campos de frontmatter](#prompt-md-fields) cubre el modelo, timeout, tags, y variables de entorno.

Cada archivo bajo `graders/` es una verificación aplicada después de la ejecución. Abra `evals/first-case/graders/criteria.md` y reemplace el marcador de posición con una rúbrica para el modelo de juez, escrita como condiciones PASS y FAIL concretas:

```markdown theme={null}
---
type: llm
---

PASS if <what a correct response contains>.
FAIL if <what a wrong or missing response looks like>.
```

Luego agregue un segundo calificador que verifique si su skill es lo que produjo la respuesta. Cree `evals/first-case/graders/skill-fired.md`, reemplazando `your-skill-name` con el nombre del directorio del skill bajo `skills/`, que es el nombre por el que Claude lo invoca:

```markdown theme={null}
---
type: tool_used
tool: Skill
input_match: '"skill"\s*:\s*"(?:[\w-]+:)?your-skill-name"'
---
```

Esto aprueba cuando Claude invocó ese skill al menos una vez durante la ejecución, incluyendo por su forma `plugin-name:skill-name` con espacio de nombres.

[Tipos de calificadores](#grader-types) enumera las otras verificaciones disponibles, como coincidir una expresión regular o confirmar que se creó un archivo.

Con ambos archivos guardados, ejecute el caso de la manera que el [inicio rápido](#create-your-first-eval-suite) hace, con `claude plugin eval .` desde la raíz del plugin.

<h3 id="set-run-limits-and-tools-in-prompt-md">
  Establecer límites de ejecución y herramientas en prompt.md
</h3>

Establezca `max_turns`, `timeout_seconds`, `model`, `tags`, y las `allowed_tools` que puede usar en el frontmatter de `prompt.md`; la referencia [frontmatter de prompt.md](#prompt-md-fields) enumera cada campo y su predeterminado.

Claude recibe el cuerpo exactamente como lo escribió. Las menciones `@path` en él no se expanden en archivos adjuntos, así que si Claude necesita leer un archivo, otorgue una herramienta para él en `allowed_tools`.

<h3 id="grade-the-result">
  Elegir y ponderar calificadores
</h3>

El frontmatter de un calificador establece su `type`, y opcionalmente un `weight` que lo hace contar para más de la puntuación de la ejecución y un [`arm`](#compare-against-a-no-plugin-baseline) que controla cómo se califica contra la línea base. De los seis tipos, `regex`, `tool_used`, `tool_order`, y `file_exists` se calculan a partir de la transcripción y archivos y no cuestan nada, mientras que `llm` y `baseline` llaman a un modelo de juez y se suman al costo de la ejecución.

No hay calificadores de código personalizado.

[Tipos de calificadores](#grader-types) enumera las opciones de cada tipo y la condición de aprobación, y [lo que un calificador puede ver](#what-a-grader-can-look-at) enumera los valores que `target` y `focus` aceptan.

El juez para calificadores `llm` y `baseline` es un modelo pequeño y rápido por defecto. Pase `--judge-model sonnet` o un ID de modelo completo para usar uno más fuerte para rúbricas matizadas.

<h4 id="choose-graders-that-give-a-stable-signal">
  Elegir calificadores que den una señal estable
</h4>

Un calificador `llm` pide a un modelo un veredicto, así que su respuesta puede diferir entre ejecuciones, y difiere más cuanto más largo sea el texto que tiene que leer. Estos hábitos mantienen las puntuaciones de un conjunto lo suficientemente estables para confiar:

* Para salida larga como un archivo generado, califíquelo con un calificador `regex` sobre el contenido del archivo, que verifica el archivo completo de la misma manera cada vez. Mantenga calificadores `llm` para salidas cortas, con rúbricas escritas como condiciones PASS y FAIL concretas.
* Dé a cada caso un calificador sobre el resultado, como el mensaje final o un archivo producido, y uno sobre los pasos que Claude tomó para producirlo, como `tool_used` o `tool_order`. Juntos le dicen tanto si la respuesta fue correcta como si su plugin la produjo.
* Si un calificador `tool_used: Skill` de un caso aprueba pero `Δ` es negativo, sospeche del juez antes que del plugin. Un modelo de juez pequeño puede marcar una respuesta correcta como incorrecta porque está formateada diferente de lo que la rúbrica describe. Re-ejecute con `--judge-model sonnet`, y ajuste la rúbrica para que el formato no decida el veredicto.
* Para verificar que una compilación o prueba pasó dentro de la ejecución, pida a Claude que la ejecute y escriba el resultado en un archivo, califique ese archivo, y afirme que el comando se ejecutó con un calificador `tool_used` cuyo `input_match` nombra el comando.

<h3 id="compare-against-a-no-plugin-baseline">
  Calificar contra la línea base sin plugin
</h3>

Cuando un plugin está bajo prueba, cada caso se ejecuta en dos brazos por defecto. El brazo con es sus ejecuciones con el plugin cargado, y el brazo sin es el mismo número de ejecuciones sin plugin en absoluto. El resumen e informe muestran ambas puntuaciones y `Δ`, la puntuación del brazo con menos la puntuación del brazo sin.

Pase `--ablation none` para ejecutar solo el brazo con, lo que reduce a la mitad el costo cuando no necesita la comparación, como mientras itera en calificadores.

En una ejecución de dos brazos, algunos calificadores se reportan con `scored: false`. Una verificación como "el skill fue invocado" nunca puede pasar sin el plugin, así que contarla empujaría el brazo sin hacia cero e inflaría `Δ`. Para mantener los dos brazos comparables, Claude Code excluye tales calificadores de la puntuación en ambos brazos y los reporta en el brazo con como indicadores de aprobación/fallo solo. Eso incluye:

* Cada calificador `tool_used` cuya `tool` es `Skill`
* Cada calificador `regex` con `target: mock_calls` y cada calificador `llm` con `focus: mock_calls`, cuando cada [servidor simulado](#mock-mcp-servers) en el caso es uno que su plugin declara
* Cualquier calificador que marque `arm: with-only`

Tres configuraciones cambian esa exclusión:

* **Cada calificador excluido**: si cada calificador en un caso está en el conjunto excluido, se califican normalmente en su lugar, ya que no habría nada más que calificar.
* **`arm: both`**: establezca `arm: both` en un calificador para calificarlo en ambos brazos independientemente, que es lo que desea para una verificación "no debe invocar el skill" con `min: 0` y `max: 0`.
* **`--ablation none`**: bajo `--ablation none` nada se excluye, así que el mismo conjunto puede producir una puntuación absoluta diferente en los dos modos.

<h3 id="use-a-different-eval-directory">
  Usar un directorio de eval diferente
</h3>

Si `evals/` ya está ocupado por otra herramienta, mantenga el conjunto en un directorio diferente. Puede registrar ese directorio en el `plugin.json` del plugin para que cada ejecución y cada colaborador lo use, o pasarlo en la línea de comandos para una sola ejecución:

* **En `plugin.json`**: agregue `"experimental": { "evals": "quality/evals" }`.
* **En la línea de comandos**: pase `--eval-dir quality/evals` tanto a `claude plugin eval` como a `claude plugin eval init`.

Si establece ambos, se usa el directorio de la bandera. Dé una ruta relativa de nombres de directorio simples como `qa` o `quality/evals`; una ruta absoluta o una que contenga `..` se rechaza: como un valor de bandera es un error, mientras que un valor de manifiesto inutilizable imprime una línea `Warning:` y la ejecución usa `evals/` en su lugar. Los casos, resultados, e salida `init` se mueven todos a ese directorio.

<h2 id="set-up-fixtures-and-mocks">
  Configurar fixtures y mocks
</h2>

Un caso puede necesitar más que un prompt: archivos o un repositorio git en el espacio de trabajo, una conversación anterior para continuar, o respuestas de los servidores MCP con los que su plugin se conecta. Cada uno de esos se configura junto al caso para que las ejecuciones permanezcan repetibles.

<h3 id="add-setup-or-history-with-case-yaml">
  Sembrar el espacio de trabajo o conversación
</h3>

Cada ejecución comienza en un espacio de trabajo vacío. Cuando un caso necesita más que el prompt, agregue un `case.yaml` junto a `prompt.md` con un bloque `context`:

* **Archivos de fixture o un repositorio git**: escriba un script Bash en el directorio del caso y nómbrelo en `context.scaffold_script`. El script se ejecuta como usted, fuera del sandbox del agente, y solo cuando pasa `--scaffold`, así que pase esa bandera solo para conjuntos que usted u su organización escribieron.
* **Una conversación anterior para continuar**: guarde la transcripción como un archivo `.jsonl` y nómbrelo en `context.history_file`, y el prompt del caso se convierte en el siguiente turno del usuario.
* **Directorios de fixture que Claude puede leer durante la ejecución**: enumérelos en `context.add_dirs`.

Un `case.yaml` también necesita `schema_version: "1.1"` y `name`; la referencia [campos de case.yaml](#case-yaml-fields) tiene la lista completa.

Este `case.yaml` siembra un espacio de trabajo desde un script y permite que Claude lea fixtures desde un directorio `resources/`:

```yaml theme={null}
schema_version: "1.1"
name: changelog-from-diff
tags: [smoke]
context:
  scaffold_script: fixture.sh
  add_dirs: [resources]
```

<h3 id="mock-mcp-servers">
  Mock MCP servers
</h3>

Puede evaluar un plugin cuyos skills llaman a herramientas MCP sin el servicio real detrás de ellas. Ponga un archivo Markdown por herramienta bajo `evals/mocks/<server>/<tool>.md` para todo el conjunto, o bajo un directorio `mocks/` propio de un caso para un caso, donde `<server>` es el nombre del servidor en la [configuración MCP](/docs/es/plugins/components#mcp-servers) de su plugin.

Una ejecución nunca inicia los servidores MCP reales de su plugin a menos que lo pida. Claude Code registra un sustituto bajo el nombre propio de cada servidor. Las herramientas con un archivo mock responden desde él y se permiten sin una concesión `--allow-tools`, y una herramienta sin archivo mock no está disponible para Claude. Un servidor sin mocks en absoluto aparece en la línea de progreso `mocked:` del caso como `plugin_<plugin>_<server>[not started: no mock]`.

El cuerpo del archivo es lo que la herramienta devuelve a Claude. Este mock se interpone por una herramienta `create_issue` en un servidor llamado `tracker`, verifica la entrada que Claude envía, y devuelve el título. Guárdelo como `evals/mocks/tracker/create_issue.md`:

```markdown theme={null}
---
expect:
  title: string
  priority: [low, medium, high]
---

Created issue #4821: {{input.title}}
```

El cuerpo y frontmatter de un archivo mock aceptan estas opciones:

* **Sustituciones**: inserte campos de la entrada de la llamada con `{{input.<field>}}`, y el contenido de un archivo de fixture junto al mock con `{{file:fixtures/{input.<field>}.json}}`.
* **`expect:`**: el bloque `expect:` protege la entrada. Si una llamada lo viola, la ejecución se detiene con puntuación 0 y registra por qué, así que un caso puede afirmar qué pidió su plugin al servidor.
* **`error: true`**: establezca `error: true` para devolver el cuerpo como un error de herramienta en su lugar.
* **`type: agent`**: establezca `type: agent` para que un modelo pequeño responda como el servidor desde instrucciones en el cuerpo.

La [referencia de archivo mock](#mock-files) enumera cada clave y los archivos `_server.md` y `_tools.json`.

Para calificar las llamadas mismas, apunte un calificador a `target: mock_calls`.

Para ejecutar contra los servidores MCP reales del plugin en su lugar, pase una de estas banderas. De cualquier manera esos procesos se ejecutan como usted, fuera del sandbox de la ejecución, y sus herramientas necesitan una concesión [`--allow-tools`](#grant-tools):

* **`--allow-real-servers`**: inicie el proceso real para cada servidor que no haya simulado, y continúe respondiendo herramientas simuladas desde sus archivos
* **`--mocks off`**: ignore `mocks/` completamente e inicie cada servidor que el plugin declara

<h4 id="replay-agent-mock-answers">
  Reproducir respuestas de mock de agente
</h4>

Un mock `type: agent` responde con una llamada al [`--judge-model`](#command-options), así que su salida varía entre ejecuciones y cambia si cambia el juez. Cuando una ejecución se completa sin un error o aborto, Claude Code guarda cada respuesta que un mock de agente dio bajo el directorio de resultados en `mock-recordings/`.

Abra `ADOPT.txt` allí para ver cada grabación y el directorio `.replay/<server>/` para copiarla, junto al mock que la produjo. Después de copiar una grabación allí, las ejecuciones posteriores responden la llamada idéntica desde ella sin llamada de modelo. Confirme `mocks/.replay/` junto con el resto de `mocks/` para que las ejecuciones de CI sean repetibles.

<h2 id="run-evals">
  Ejecutar evals
</h2>

Una vez que existe un conjunto, `claude plugin eval` lo ejecuta. Elige qué plugin y casos ejecutar con el argumento de destino, otorga cualquier herramienta que los casos necesiten más allá del conjunto de solo lectura con `--allow-tools`, y controla el conteo de ejecuciones, modelos, costo, y salida con las otras opciones.

<h3 id="choose-what-to-evaluate">
  Elegir qué evaluar
</h3>

La mayoría de las veces ejecuta `claude plugin eval .` desde la raíz del plugin, que ejecuta cada caso en el conjunto con el plugin en el que está parado cargado. Para ejecutar un archivo de caso único, o para evaluar un plugin que instaló en lugar de uno que está desarrollando, pase un destino diferente:

| Destino                                                     | Qué se ejecuta                                                                                                                                                                                              |
| :---------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Un directorio raíz de plugin, como `.`                      | Cada caso bajo su directorio de eval, con ese plugin cargado                                                                                                                                                |
| Un archivo `prompt.md` o `case.yaml` único                  | Ese caso, con su plugin envolvente cargado                                                                                                                                                                  |
| Un plugin instalado por nombre, `name` o `name@marketplace` | Los casos en el directorio de eval de la copia instalada, con la copia instalada cargada. Los resultados se escriben bajo `./evals/results/` en su directorio actual, o `./<dir>/results/` con `--eval-dir` |
| `name@skills-dir`                                           | Lo mismo, para un [plugin de directorio de skills](/docs/es/plugins/loading#plugins-shared-through-a-repository)                                                                                                 |
| Omitido                                                     | El directorio actual como una ruta                                                                                                                                                                          |

Agregue `--case <glob>` para filtrar por nombre de caso y `--tag <tag>` para mantener casos con cualquiera de los tags dados.

Ponga el destino antes de `--tag`, `--allow-tools`, y `--json`. Los primeros dos toman una lista y `--json` toma una ruta opcional, así que cada uno de ellos lee un destino que sigue como su propio valor.

<h3 id="grant-tools">
  Otorgar herramientas
</h3>

Las ejecuciones nunca se detienen para pedir permiso. Las herramientas integradas que necesitan una concesión que no otorgó, como `Bash`, `Write`, `Edit`, `WebFetch`, y `WebSearch`, se eliminan de la sesión, así que Claude no puede llamarlas en absoluto.

Una ejecución permite solo las herramientas de solo lectura que el caso enumera en `allowed_tools`, de `Read`, `Glob`, `Grep`, `NotebookRead`, `Skill`, `AskUserQuestion`, `Agent`, `TodoWrite`, y las herramientas de tarea `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, y `TaskStop`, más lo que otorgue con `--allow-tools`. Esa concesión se aplica a cada caso en la ejecución. Para permitir que los casos usen `Bash`, `Write`, `Edit`, `WebFetch`, o `WebSearch`, otórguelos usted mismo:

```bash theme={null}
claude plugin eval . --allow-tools Write Edit "Bash(npm test *)"
```

Cuando un caso pidió una herramienta que no otorgó, la salida de progreso la enumera como `not granted`. Las herramientas en un servidor MCP [simulado](#mock-mcp-servers) no necesitan concesión. Las herramientas en un servidor MCP de plugin real necesitan tanto el servidor iniciado, con `--allow-real-servers` o `--mocks off`, como una concesión por nombre, como `--allow-tools "mcp__plugin_my-plugin_github__*"`; las herramientas MCP de un plugin se nombran `mcp__plugin_<plugin>_<server>__<tool>`.

Cuando otorga `Bash` en cualquier forma, cada comando se ejecuta bajo el [sandbox a nivel de SO](/docs/es/sandboxing) de Claude Code. Las escrituras se limitan al espacio de trabajo de la ejecución, su directorio de inicio y la configuración de Claude Code son ilegibles, y el acceso a la red se limita a dominios que otorga con `--allow-tools "WebFetch(domain:example.com)"`. Si otorga Bash o PowerShell en una máquina sin backend de sandbox, Claude Code rechaza cada ejecución en lugar de ejecutarla sin confinar, y el caso muestra un error de ejecución y generalmente puntúa 0. Windows nativo no tiene backend, así que ejecute conjuntos que otorguen shell bajo WSL2; en Linux, instale `bubblewrap` y `socat` primero. Vea los [requisitos previos de sandboxing](/docs/es/sandboxing).

<h3 id="command-options">
  Opciones de comando
</h3>

Esta tabla cubre las opciones para conteo de ejecuciones, modelos, puntuación, costo, otorgamiento de herramientas, mocks, y salida. Ejecute `claude plugin eval --help` para la lista completa, que también incluye `--case`, `--tag`, `--eval-dir`, `--no-scaffold`, `--report`, y `--verbose`.

| Opción                     | Predeterminado                                                                                        | Efecto                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------- | :---------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `--runs <n>`               | `runs` de cada caso, si no 3                                                                          | Ejecuciones por caso por brazo                                                                                                                                                                                                                                                                                                                                            |
| `-j`, `--concurrency <n>`  | `1`                                                                                                   | Ejecute hasta este número de ejecuciones de agente a la vez, de 1 a 8. Comparten el límite de velocidad de su cuenta, así que esto acorta el tiempo de reloj de pared en lugar de aumentar el rendimiento más allá de ese límite. Los resultados mantienen el orden del caso                                                                                              |
| `--model <model>`          | `model` de cada caso, si no `ANTHROPIC_MODEL` si se establece, si no el predeterminado de Claude Code | Modelo para el agente bajo prueba. Fíjelo en CI para que un lanzamiento de modelo no se confunda con una regresión de plugin                                                                                                                                                                                                                                              |
| `--judge-model <model>`    | Un modelo pequeño y rápido                                                                            | Modelo para calificadores `llm` y `baseline`                                                                                                                                                                                                                                                                                                                              |
| `--ablation <mode>`        | `with-without` cuando un plugin se resuelve, si no `none`                                             | Si también ejecutar cada caso sin el plugin para medir qué agrega. `none` ejecuta un brazo; `with-without` agrega la línea base sin plugin                                                                                                                                                                                                                                |
| `--threshold <0..1>`       | `1.0`                                                                                                 | Un caso aprueba cuando su puntuación de brazo con es al menos esto. Cualquier caso por debajo hace que el comando salga 1                                                                                                                                                                                                                                                 |
| `--max-cost-usd <usd>`     | Sin techo                                                                                             | Un techo en la estimación de costo de precio de lista de la ejecución, no en el uso del plan. Se verifica antes de que cada ejecución comience. Una vez gastado, nada más comienza; las ejecuciones ya en vuelo terminan, así que el gasto puede pasar el techo por esas ejecuciones. Si alguna ejecución se deja sin iniciar, el comando sale 2 con resultados parciales |
| `--allow-tools <tools...>` | Ninguno                                                                                               | Otorgue herramientas más allá del conjunto de solo lectura. Vea [Otorgar herramientas](#grant-tools)                                                                                                                                                                                                                                                                      |
| `--scaffold`               | Apagado                                                                                               | Ejecute el [`scaffold_script`](#add-setup-or-history-with-case-yaml) de cada caso                                                                                                                                                                                                                                                                                         |
| `--trust-plugin`           | Apagado                                                                                               | Omita el primer aviso de confianza de ejecución para un plugin cuyo código y conjunto ejecutaría usted mismo. Páselo en CI para que el trabajo nunca sea rechazado por o dejado esperando en el aviso. Vea [Lo que una ejecución puede acceder](#security)                                                                                                                |
| `--mocks <mode>`           | `record`                                                                                              | `record` responde llamadas de herramientas MCP desde [mocks](#mock-mcp-servers), no inicia los servidores MCP reales del plugin, y guarda respuestas de mock de agente para reproducción. `off` ignora mocks e inicia los servidores MCP reales del plugin                                                                                                                |
| `--allow-real-servers`     | Apagado                                                                                               | Con `--mocks record`, también inicie los servidores MCP reales del plugin para servidores que no tengan mock                                                                                                                                                                                                                                                              |
| `--json [path]`            | Apagado                                                                                               | Imprima el [documento de resultado](#json-result) a stdout, o escriba a una ruta que termine en `.json`. La ejecución es silenciosa: sin líneas de progreso o tabla de resumen                                                                                                                                                                                            |
| `--output-dir <dir>`       | `<eval dir>/results/<timestamp>/`                                                                     | Dónde van `aggregate-result.json` y `report.html`                                                                                                                                                                                                                                                                                                                         |
| `--no-publish`             |                                                                                                       | Mantenga el informe HTML local. Vea [Informe HTML](#html-report)                                                                                                                                                                                                                                                                                                          |
| `--publish-report`         |                                                                                                       | Publique el informe incluso donde se mantendría local por defecto, como una ejecución que una sesión de Claude Code inició                                                                                                                                                                                                                                                |
| `--keep-temp`              | Apagado                                                                                               | Mantenga el directorio sandbox de cada ejecución e imprima su ruta, para depuración de lo que Claude produjo                                                                                                                                                                                                                                                              |

<h3 id="run-evals-in-ci">
  Ejecutar evals en CI
</h3>

En su trabajo de CI, ejecute el conjunto con `--json` para escribir el resultado para archivado, y falle la compilación en el código de salida. Pase `--trust-plugin` para que el trabajo nunca espere en el [primer aviso de confianza](#security), fije ambos modelos para que las puntuaciones sean comparables en el tiempo, mantenga el informe local, y establezca un techo de costo como límite superior:

```bash theme={null}
claude plugin eval . \
  --trust-plugin \
  --json results.json \
  --threshold 0.8 \
  --model claude-sonnet-5 \
  --judge-model claude-haiku-4-5 \
  --no-publish \
  --max-cost-usd 20
```

El código de salida del trabajo le dice qué sucedió:

| Código de salida | Significado                                                                                                                                                                                                                           |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 0                | Cada caso puntuó en o por encima de `--threshold` y cada archivo de caso cargó                                                                                                                                                        |
| 1                | Un caso puntuó por debajo del umbral, un archivo de caso falló al cargar, no se encontraron casos, una ejecución no pudo iniciarse, el directorio del plugin no es confiable y `--trust-plugin` no se pasó, o una opción fue inválida |
| 2                | Ejecución parcial: se alcanzó el techo `--max-cost-usd`, o su credencial fue rechazada antes o en la primera ejecución. `results.json` aún se escribe con `partial: true` y la razón                                                  |
| 130              | Interrumpido. Los resultados parciales se escriben                                                                                                                                                                                    |
| 143              | Terminado, como por un timeout de CI                                                                                                                                                                                                  |

Los problemas al escribir o publicar el informe HTML nunca cambian el código de salida.

Para ver por qué un caso puntuó bajo, ejecute localmente sin `--json` para que se impriman las líneas de progreso por ejecución y del calificador.

Un ejecutor de CI también necesita estos en su lugar:

* **Instalar y credenciales**: un ejecutor de CI necesita una instalación de Claude Code y [credenciales en el entorno](/docs/es/authentication) como `ANTHROPIC_API_KEY`.
* **Confianza**: sin `--trust-plugin`, un trabajo cuyo directorio de checkout Claude Code aún no confía necesita el [primer aviso de confianza](#trust-the-plugin-directory), y una ejecución que no puede preguntar es rechazada con salida 1.
* **`init` en CI**: `claude plugin eval init` necesita una terminal para hacerle sus preguntas; en CI, ejecute `claude plugin eval init --bare <name>` para obtener la plantilla en blanco.

Para mantener los costos predecibles, dé a cada cambio rápido conjuntos solo con calificadores que no llamen a un juez, use `--ablation none` donde no necesite `Δ`, y deje documentos `partial: true` y ejecuciones con `skippedPaidGraders` fuera de cualquier tendencia que grafique.

<h2 id="read-the-results">
  Leer los resultados
</h2>

Cada ejecución con al menos un caso escribe un directorio `results/<timestamp>/` dentro del directorio de eval, conteniendo `aggregate-result.json` y `report.html`. Para un destino de ruta que está bajo el plugin; para un plugin que nombró, está bajo su directorio actual, como muestra la [tabla de destino](#choose-what-to-evaluate). La tabla de resumen, el JSON, y el informe todos renderizan los mismos datos de resultado.

<h3 id="html-report">
  Informe HTML
</h3>

`report.html` es un archivo único y autónomo que no hace solicitudes externas, así que puede adjuntarlo a un trabajo de CI o abrirlo desde el disco. Este ejemplo es la parte superior de un informe para una ejecución de conjunto de tres casos con `--threshold 0.8`; el costo mostrado es una estimación de precio de lista y varía con el modelo y el número de casos:

<img src="https://mintcdn.com/claude-code/qq7LHDi_F0aeFHgk/images/plugin-eval-report.png?fit=max&auto=format&n=qq7LHDi_F0aeFHgk&q=85&s=106eb6e6a70a6565f891ea3a4564f87d" alt="Parte superior de un informe de eval: una línea de veredicto que dice &#x22;Plugin effect: +33.3 pts vs baseline, improved 2, flat 1, regressed 0 of 3 cases&#x22;, cinco mosaicos de resumen para puntuación del conjunto, delta de ablación, puntuación de línea base, casos que pasan el umbral, y ejecuciones perfectas, luego el primer caso con su delta, barra de puntuación, y una ejecución cuyos dos calificadores muestran aprobación" width="1360" height="1032" data-path="images/plugin-eval-report.png" />

Léalo de arriba hacia abajo:

* **La línea de veredicto y los mosaicos** responden si el plugin ayudó en todo el conjunto. La puntuación del conjunto es la media de las puntuaciones con plugin por caso, Ablation Δ es qué tan lejos está por encima o por debajo de la puntuación de línea base, y Cases cuenta cuántos cumplieron el umbral. Perfect runs es la proporción de ejecuciones con plugin donde cada calificador aprobó.
* **Cada tarjeta de caso** muestra el `Δ` del caso y la puntuación con plugin, con una marca en la barra en el umbral. Un caso cuyo `Δ` es negativo obtiene un borde izquierdo rojo, así que las regresiones se destacan cuando desplaza.
* **Dentro de un caso**, las ejecuciones con plugin vienen primero y las ejecuciones de línea base después. Cada ejecución enumera sus calificadores con un chip de aprobación o rechazo. Un calificador fallido ya está expandido con su explicación, y un calificador `llm` también muestra los votos del juez y la evidencia que se le mostró, que es donde descubre por qué una ejecución puntuó bajo. Los calificadores que no cuentan hacia la puntuación, como `tool_used: Skill`, llevan un badge de `plugin-fired indicator`.
* **Prompt y Graders**, debajo de las ejecuciones, muestran el prompt del caso y la rúbrica o patrón de cada calificador, para que alguien que lea el informe sin el conjunto pueda ver qué se preguntó y qué contó como bueno.

Si está conectado con una suscripción de claude.ai y los [artefactos](/docs/es/artifacts) están disponibles para su cuenta, Claude Code también publica el informe como un artefacto privado e imprime `Published: <url>`. Pase `--no-publish` para mantenerlo local. Si no aparece una línea `Published:`, como con autenticación de clave API, el archivo local es el informe.

Una ejecución que una sesión de Claude Code inició, como cuando pide a Claude que ejecute el conjunto para usted, también se mantiene local, y su línea `Report:` dice `kept local`. Agregue `--publish-report` a ese comando para publicarlo.

<h3 id="json-result">
  Resultado JSON
</h3>

`aggregate-result.json`, y salida `--json`, es un documento versionado con `schemaVersion: 1` para que scripts de CI analicen. Los nombres de campo están en camelCase y se agregan nuevos campos sin renombrar los existentes, así que escriba su script para ignorar campos que no reconozca.

Estos son los campos que un script de control generalmente lee. El documento también lleva la configuración del conjunto, cada definición de calificador, y resultados de calificador por ejecución con explicaciones y evidencia:

| Campo                                             | Significado                                                                                                                                                                                                       |
| :------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `partial`, `partialReason`                        | `true` con `cost_ceiling`, `interrupted`, o `auth_failed` cuando el conjunto no terminó. Deje resultados parciales fuera de gráficos de tendencia                                                                 |
| `aggregates.overallScore`                         | Puntuación de caso promedio en el conjunto                                                                                                                                                                        |
| `aggregates.casesPassed`, `aggregates.casesTotal` | Casos en o por encima de `--threshold`, y el total                                                                                                                                                                |
| `aggregates.meanDelta`                            | `Δ` promedio en casos, bajo el modo de dos brazos                                                                                                                                                                 |
| `cases[].name`                                    | Nombre del caso                                                                                                                                                                                                   |
| `cases[].aggregates.score`                        | Puntuación de ejecución de brazo con promedio para el caso                                                                                                                                                        |
| `cases[].aggregates.delta`                        | Puntuación de brazo con menos puntuación de brazo sin. Omitido cuando los brazos no son comparables                                                                                                               |
| `cases[].arms.with[].error`                       | `null`, o por qué una ejecución terminó anormalmente, como `timed out after 300s`. Una ejecución que comenzó pero terminó mal aún se califica en lo que produjo, así que un error no nulo no implica puntuación 0 |
| `cases[].arms.with[].aborted`                     | Presente cuando un [mock](#mock-mcp-servers) `expect:` o `abort_when` detuvo la ejecución, con `server`, `tool`, y `reason`. La ejecución puntúa 0 y `error` permanece `null`                                     |
| `cases[].arms.with[].skippedPaidGraders`          | `true` cuando el techo de costo omitió los calificadores de juez de esta ejecución, así que su puntuación no es comparable                                                                                        |
| `costUsd`, `durationSeconds`, `claudeVersion`     | Costo estimado a precio de lista incluyendo llamadas de juez, segundos de reloj de pared, y la versión de Claude Code que ejecutó el conjunto                                                                     |

<h2 id="security">
  Lo que una ejecución puede acceder
</h2>

`claude plugin eval` carga los skills, hooks y agentes del plugin de destino y ejecuta su conjunto de eval en su máquina, como usted. Apuntarlo a un plugin es la misma decisión de confianza que `claude --plugin-dir`, así que solo evalúe plugins en los que confía.

El aislamiento descrito en esta sección limita lo que el agente bajo prueba puede alcanzar; no es un límite contra el código del plugin mismo, y un conjunto que aprueba no dice nada sobre si el plugin es seguro.

<h3 id="trust-the-plugin-directory">
  Confiar en el directorio del plugin
</h3>

La primera vez que ejecuta `claude plugin eval` contra un directorio, Claude Code pregunta `Trust this plugin directory?` antes de cargar nada de él, a menos que ya aceptara el aviso de confianza allí en una sesión interactiva de `claude`. Dentro de un repositorio git, responder sí confía en todo el repositorio, para sesiones interactivas también. Cuando stdin o stdout no es una terminal, bajo `--json`, o cuando la variable de entorno `CI` se establece en un valor verdadero como `true`, la ejecución no puede preguntar y se rechaza con salida 1; pase `--trust-plugin` para afirmar la confianza usted mismo, solo para un plugin que ejecutaría en su propia máquina. Un destino que nombra en lugar de dar como ruta, significando un plugin instalado o un plugin de directorio de skills, omite el aviso.

Algunas partes del plugin y conjunto se ejecutan solo cuando pasa su bandera para esa ejecución:

* El [`scaffold_script`](#add-setup-or-history-with-case-yaml) de un caso con `--scaffold`
* [Herramientas más allá del conjunto de solo lectura](#grant-tools) con `--allow-tools`
* Los [servidores MCP reales](#mock-mcp-servers) del plugin con `--allow-real-servers` o `--mocks off`

Las `allowed_tools` de un caso y el frontmatter `allowed-tools` propio de un skill no pueden ampliar ninguno de ellos.

Cuando el plugin incluye hooks que no escribió, o inicia sus servidores MCP reales, trate sus puntuaciones como consultivas a menos que las ejecutara en un entorno aislado como un contenedor o ejecutor de CI, ya que los hooks y servidores se ejecutan fuera del sandbox del agente y podrían modificar los archivos que los calificadores leen.

<h3 id="how-runs-are-isolated">
  Cómo se aíslan las ejecuciones
</h3>

Cada ejecución obtiene un directorio de inicio temporal, directorio de trabajo, y configuración de Claude Code, y el agente bajo prueba se ejecuta allí como un proceso hijo `claude -p` con solo su plugin cargado. Tenga en cuenta estas consecuencias cuando escriba casos:

* **Nada personal o a nivel de proyecto carga.** Sus configuraciones de usuario, hooks, archivos `CLAUDE.md`, servidores MCP, otros plugins instalados, memoria, y skills están ausentes, y ningún `.claude/` o `.mcp.json` a nivel de proyecto por encima del sandbox se lee. La mayoría de su entorno de shell también se retiene; solo una [lista de permitidos](#prompt-md-fields) y variables `EVAL_*` llegan a la ejecución. Si el plugin necesita configuración, envíela en el plugin, créela en un `scaffold_script`, o pase variables `EVAL_*`.
* **La política administrada aún puede restringir una ejecución.** Las restricciones en [configuración administrada](/docs/es/managed-settings) que un administrador implementó en la máquina se aplican dentro de una ejecución, así que los resultados en una máquina administrada pueden diferir de una no administrada por esa política.
* **La herramienta Artifact está apagada.** Un skill que publica un [artefacto](/docs/es/artifacts) puede calificarse solo en lo que produce antes de ese paso.
* **Las definiciones del caso están ocultas del agente.** Una ejecución no puede leer el directorio de eval, así que Claude no puede ver el prompt del caso, sus calificadores, o casos hermanos.
* **Sin sandbox de red fuera de comandos de shell.** Los comandos de shell que otorga se ejecutan bajo las reglas de sandbox de la red. Una concesión `WebFetch(domain:…)` alcanza ese dominio directamente, y los hooks propios del plugin y cualquier servidor MCP real que inicie pueden alcanzar cualquier host.

<h2 id="eval-suite-reference">
  Referencia de conjunto de eval
</h2>

Todo lo que un conjunto de eval puede contener vive bajo el directorio de eval del plugin, `evals/` a menos que [configure otro](#use-a-different-eval-directory). Este árbol muestra cada archivo que `claude plugin eval` lee o escribe allí; solo `prompt.md` o `case.yaml` es requerido para que un caso exista:

```text theme={null}
evals/
├── <case>/                        # one directory per case; nest under a non-case directory to group
│   ├── prompt.md                  # frontmatter: case and run fields; body: the prompt
│   ├── case.yaml                  # optional: context.* fields, or the whole case in one file
│   ├── graders/
│   │   └── <name>.md              # one grader per file; frontmatter: type and options; body: rubric
│   ├── mocks/                     # optional: mocks for this case only, same layout as below
│   └── <fixtures, scripts, transcripts referenced by case.yaml>
├── mocks/                         # optional: suite-wide MCP mocks
│   ├── <server>/
│   │   ├── <tool>.md              # one mocked tool; body: the tool result
│   │   ├── _server.md             # optional: one agent that answers several tools
│   │   ├── _tools.json            # optional: saved tools/list response for real descriptions and schemas
│   │   └── fixtures/              # files inserted with {{file:fixtures/...}}
│   └── .replay/<server>/          # adopted agent-mock recordings, answered without a model call
└── results/<timestamp>/           # written by each run; add results/ to .gitignore
    ├── aggregate-result.json
    ├── report.html
    └── mock-recordings/           # agent-mock answers from clean runs, with ADOPT.txt
```

<h3 id="prompt-md-fields">
  Frontmatter de prompt.md
</h3>

El frontmatter de `prompt.md` acepta estos campos. Una clave desconocida es un error:

| Campo                  | Predeterminado                      | Propósito                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :--------------------- | :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `schema_version`       | `"1.1"`, establecido para usted     | Versión del formato de caso. Los casos escritos como `prompt.md` lo obtienen automáticamente, así que rara vez lo establece                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `name`                 | El nombre del directorio            | Nombre del caso. Los globs `--case` lo coinciden y el informe lo usa como clave                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `description`          |                                     | Para humanos. No se usa en tiempo de ejecución                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `tags`                 | `[]`                                | Etiquetas para filtrado `--tag`. Un caso se ejecuta si alguno de sus tags coincide                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `plugins`              | El plugin envolvente más cercano    | Directorios de plugin bajo prueba, relativos al directorio del caso. Establezca `plugins: ["../.."]` cuando la detección automática no encuentra su plugin; vea [el plugin no cargó](#the-baseline-arm-shows-no-plugin-or-delta-is-zero)                                                                                                                                                                                                                                                                                                                                                     |
| `runs`                 | `3`                                 | Ejecuciones por brazo, 1 a 50. `--runs` lo anula                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `expected_outcome`     |                                     | Para humanos. No se usa en tiempo de ejecución                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `model`                | El predeterminado de la sesión hijo | Modelo para el agente bajo prueba. `--model` lo anula                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `max_turns`            | `10`                                | Límite de turno, hasta 200. Alcanzarlo se registra como un error de ejecución y generalmente baja la puntuación, así que establézcalo generosamente                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `timeout_seconds`      | `300`                               | Límite de reloj de pared por ejecución, hasta 3600                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `allowed_tools`        | `[]`                                | Herramientas que el caso quiere, como `[Read, Glob, Grep, Skill]`. Las herramientas de solo lectura se otorgan cuando se enumeran aquí; para cualquier otra cosa, vea [Otorgar herramientas](#grant-tools)                                                                                                                                                                                                                                                                                                                                                                                   |
| `append_system_prompt` |                                     | Texto anexado al prompt del sistema de la sesión hijo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `env`                  | `{}`                                | Variables de entorno adicionales para la sesión hijo. Las claves deben coincidir con `EVAL_[A-Z0-9_]*`; cualquier otra clave falla la ejecución. La ejecución hereda solo una lista de permitidos de su shell: conceptos básicos como `PATH` y configuración regional, configuración de proxy y certificado, las variables que seleccionan y autentican su proveedor de modelo, la mayoría de `ANTHROPIC_*` y `CLAUDE_CODE_*` configuración, y `EVAL_*`. Para entregar al plugin cualquier otra cosa, como una configuración de cadena de herramientas, expórtela como una variable `EVAL_*` |

<h3 id="case-yaml-fields">
  Campos de case.yaml
</h3>

`case.yaml` es una alternativa o complemento a `prompt.md`: describe un caso en YAML y agrega los campos que apuntan a otros archivos. Requiere `schema_version: "1.1"` y `name`. Los campos `description`, `tags`, `plugins`, `runs`, y `expected_outcome` de `prompt.md` van en el nivel superior; `model`, `max_turns`, `timeout_seconds`, `allowed_tools`, `append_system_prompt`, y `env` van bajo `execution:`. Cuando ambos archivos existen, el frontmatter de `prompt.md` anula los campos coincidentes de `case.yaml`, el cuerpo de `prompt.md` es el prompt, y `graders/*.md` se agregan después de cualquier calificador enumerado en `case.yaml`.

Estos campos existen solo en `case.yaml`:

| Campo                     | Propósito                                                                                                                                                                                                                                                  |
| :------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `context.scaffold_script` | Un script Bash en el directorio del caso que se ejecuta en el espacio de trabajo vacío antes de que Claude comience, para crear archivos de fixture o un repositorio git. Se ejecuta solo cuando pasa [`--scaffold`](#add-setup-or-history-with-case-yaml) |
| `context.history_file`    | Una transcripción `.jsonl` en el directorio del caso para reanudar. El prompt del caso se convierte en el siguiente turno del usuario                                                                                                                      |
| `context.add_dirs`        | Directorios dentro del directorio del caso que Claude puede leer durante la ejecución, otorgados de solo lectura                                                                                                                                           |
| `execution.prompt`        | El prompt, cuando mantiene todo el caso en `case.yaml` y omite `prompt.md`                                                                                                                                                                                 |
| `graders`                 | Una lista de calificadores, cada uno con un `name` más las mismas claves que un archivo `graders/*.md` toma en frontmatter. Para calificadores `llm`, ponga la rúbrica en `criteria`                                                                       |

<h3 id="grader-frontmatter">
  Frontmatter de calificador
</h3>

Cada archivo de calificador bajo `graders/` toma estas claves en frontmatter, más las opciones para su tipo. El nombre del calificador es el nombre de archivo sin `.md`:

| Clave    | Predeterminado | Propósito                                                                                                                                                                                                                       |
| :------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`   | requerido      | Uno de los [tipos de calificador](#grader-types)                                                                                                                                                                                |
| `weight` | `1`            | Peso relativo en la puntuación de la ejecución. Cualquier número positivo                                                                                                                                                       |
| `arm`    | sin establecer | `with-only` excluye el calificador de la puntuación en una [ejecución de dos brazos](#compare-against-a-no-plugin-baseline); `both` fuerza un calificador que Claude Code de otro modo excluiría a ser puntuado en ambos brazos |

<h4 id="what-a-grader-can-look-at">
  Lo que un calificador puede ver
</h4>

Los calificadores `regex` toman un `target` y los calificadores `llm` toman un `focus`. Ambos aceptan los mismos valores:

| Valor                            | Lo que el calificador ve                                                                                                                                                                                                                                                                                                                         |
| :------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `last_message`                   | El texto de respuesta final de Claude. Este es el predeterminado                                                                                                                                                                                                                                                                                 |
| `trace`                          | La sesión como JSON, una línea por mensaje. Un calificador `regex` ve cada mensaje; un juez `llm` ve los primeros 12 y los últimos 12. Las comillas y saltos de línea dentro de ella están escapados en JSON, así que una expresión regular coincide con `\"` en lugar de `"`                                                                    |
| `files`                          | La lista de rutas que Claude creó durante la ejecución, una por línea. No sus contenidos, y no archivos que un scaffold creó o que Claude solo modificó                                                                                                                                                                                          |
| `{ source: file, path: <path> }` | El contenido de un archivo en el espacio de trabajo después de la ejecución. Use esto para calificar lo que el plugin produjo. Un archivo PNG, JPEG, GIF, o WebP se muestra a un juez `llm` como una imagen. Un juez `llm` rechaza otros archivos binarios como `.pptx` o PDF; renderícelos a una imagen o escríbalos como texto y califique eso |
| `mock_calls`                     | Cada llamada que Claude hizo a una [herramienta MCP simulada](#mock-mcp-servers), con su entrada y la respuesta del mock                                                                                                                                                                                                                         |

<h4 id="grader-types">
  Tipos de calificador
</h4>

Cada tipo de calificador a continuación enumera sus opciones y cuándo aprueba:

| Tipo          | Opciones                              | Aprueba cuando                                                                                                                                                                                                                                                                          |
| :------------ | :------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `regex`       | `pattern`, `flags`, `match`, `target` | La expresión regular de JavaScript `pattern` se encuentra en el destino. Establezca `match: not_contains` para requerir ausencia o `match: "count:N"` para requerir exactamente N coincidencias. Ponga insensibilidad a mayúsculas en `flags: i`; `(?i)` en línea no se admite          |
| `tool_used`   | `tool`, `input_match`, `min`, `max`   | El número de llamadas a `tool` cuya entrada codificada en JSON coincide con la expresión regular `input_match` opcional está entre `min`, predeterminado 1, y `max`, predeterminado ilimitado. Para afirmar que una herramienta nunca se llamó, establezca tanto `min: 0` como `max: 0` |
| `tool_order`  | `before`, `after`                     | Ambas herramientas fueron llamadas y la primera llamada coincidente `before` precede a la primera llamada coincidente `after`. Cada una es un nombre de herramienta o `{ tool, input_match }`                                                                                           |
| `file_exists` | `path`, `exists`                      | Un archivo que Claude creó coincide con el glob `path`, o ninguno lo hace con `exists: false`. Solo los archivos creados durante la ejecución cuentan                                                                                                                                   |
| `llm`         | `criteria`, `focus`                   | Un modelo de juez vota PASS en la rúbrica en al menos dos de tres votos. En el diseño `.md` el cuerpo del archivo es los criterios                                                                                                                                                      |
| `baseline`    | `baseline_file`, `criteria`           | Un juez encuentra que la ejecución satisface los criterios al menos tan bien como la transcripción de referencia en `baseline_file`, un `.jsonl` en el directorio del caso                                                                                                              |

<h3 id="mock-files">
  Archivos mock
</h3>

Un archivo `<tool>.md` bajo `mocks/<server>/` responde una herramienta. Su cuerpo es el resultado de la herramienta, con sustituciones `{{input.<field>}}` y `{{file:fixtures/<name>}}`. Su frontmatter acepta estas claves:

| Clave        | Predeterminado | Propósito                                                                                                                                                                                                                                                                                                           |
| :----------- | :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`       | `fixed`        | `fixed` devuelve el cuerpo como está escrito. `agent` trata el cuerpo como instrucciones para un modelo pequeño que juega el servidor para la ejecución y ve llamadas anteriores como historial                                                                                                                     |
| `expect`     | sin establecer | Un mapa de rutas de entrada punteadas a un nombre de tipo como `string`, `number`, `boolean`, `array`, u `object`, una `/regex/`, un literal, o una lista de literales permitidos. Una llamada que lo viola detiene la ejecución con puntuación 0 y se reporta como `aborted` con el servidor, herramienta, y razón |
| `error`      | `false`        | Solo `fixed`. Devuelva el cuerpo como un error de herramienta                                                                                                                                                                                                                                                       |
| `abort_when` | sin establecer | Solo `agent`. Prosa enumerando las únicas condiciones bajo las cuales el agente puede detener la ejecución                                                                                                                                                                                                          |

Dos archivos opcionales se sientan junto a los archivos de herramienta en el directorio de un servidor:

* **`_server.md`**: un único mock `type: agent` que responde varias herramientas, enumeradas en su clave frontmatter `tools:`. Un `<tool>.md` para la misma herramienta tiene precedencia. Ponga una guardia `expect:` en el `<tool>.md` individual, no aquí
* **`_tools.json`**: una respuesta `tools/list` guardada del servidor real, para que las herramientas simuladas lleven sus descripciones reales y esquemas de entrada en lugar de un marcador de posición permisivo

El directorio `mocks/` propio de un caso usa el mismo diseño y anula el archivo de mocks del conjunto archivo por archivo.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Estos son los problemas que los autores encuentran más a menudo, indexados en lo que ve.

<h3 id="plugin-eval-is-currently-in-early-access">
  "plugin eval is currently in early access"
</h3>

Su compilación es anterior a la disponibilidad general del comando. Ejecute `claude update`, luego ejecute el comando nuevamente en una sesión nueva.

<h3 id="plugin-eval-is-currently-unavailable">
  "plugin eval is currently unavailable"
</h3>

Anthropic ha desactivado el comando del lado del servidor. Nada en su máquina lo vuelve a activar; ejecute `claude update` e intente nuevamente en una sesión nueva más tarde.

<h3 id="is-not-a-trusted-plugin-directory-and-this-run-cannot-stop-to-ask-you-about-it">
  "is not a trusted plugin directory, and this run cannot stop to ask you about it"
</h3>

Esta es la primera ejecución contra un directorio que Claude Code aún no confía, y no puede preguntar porque stdin o stdout no es una terminal, pasó `--json`, o la variable de entorno `CI` está establecida en un valor verdadero como `true`. Ejecute `claude plugin eval <dir>` una vez en una terminal y responda el aviso, o pase `--trust-plugin` si confía en el código y conjunto del plugin. Vea [Lo que una ejecución puede acceder](#security).

<h3 id="no-eval-cases-found">
  "No eval cases found"
</h3>

No existe `<case>/prompt.md` o `<case>/case.yaml` bajo el directorio de eval en efecto, o sus filtros `--case` y `--tag` no coincidieron con ningún caso. Ejecute desde la raíz del plugin, o ejecute `claude plugin eval init` para crear un conjunto.

<h3 id="the-baseline-arm-shows-no-plugin-or-delta-is-zero">
  El brazo de línea base muestra sin plugin, o delta es cero
</h3>

Si el resumen no tiene columna `W/OUT`, o el caso falla con "ablation requested but no plugin resolved", no se encontró plugin para el caso. Agregue `plugins: ["../.."]` al caso, dando la ruta desde el directorio del caso al directorio del plugin.

Si el plugin cargó y `Δ` aún está cerca de cero con su calificador `tool_used: Skill` fallando, eso es generalmente un hallazgo real, significando que la `description` del skill no se activa en la fraseología del prompt. Ajuste la descripción y re-ejecute el mismo conjunto.

<h3 id="agent-type-’-’-not-found-for-one-of-your-plugin’s-agents">
  "Agent type '...' not found" for one of your plugin's agents
</h3>

Por defecto, cada caso se ejecuta tanto con su plugin como sin él, y las ejecuciones sin él son la [línea base sin plugin](#the-no-plugin-baseline). Cuando Claude distribuye uno de los agentes de su plugin en una ejecución de línea base, la llamada de herramienta Agent falla con `Agent type '<plugin>:<agent-name>' not found. Available agents: ...`. La lista nombra solo agentes que existen sin el plugin, como los [subagentes integrados](/docs/es/sub-agents#built-in-subagents).

El error es esperado, ya que `Δ` compara sus ejecuciones de plugin contra la línea base. En el resultado JSON, las ejecuciones de línea base están bajo `cases[].arms.without`.

En ejecuciones con su plugin cargado, un caso que enumera `Agent` en `allowed_tools` puede distribuir uno de los agentes de su plugin por su nombre con espacio de nombres, como `my-plugin:code-reviewer` para el agente `code-reviewer` en un plugin llamado `my-plugin`. Para omitir las ejecuciones de línea base, pase `--ablation none`.

<h3 id="everything-scores-zero-although-the-right-files-were-produced">
  Todo puntúa cero aunque se produjeron los archivos correctos
</h3>

Sus calificadores apuntan a `files`, la lista de rutas creadas, cuando quisieron decir el contenido del archivo. Use `{ source: file, path: <path> }` como el `target` o `focus`. Separadamente, `file_exists` cuenta solo archivos creados durante la ejecución, así que un archivo que el scaffold creó o que Claude solo editó es invisible para él; califique su contenido, o use `tool_used` en `Edit`.

<h3 id="a-regex-over-the-trace-doesn’t-match-text-i-can-see">
  Una expresión regular sobre la traza no coincide con texto que puedo ver
</h3>

* **Destino incorrecto**: el `target` predeterminado es `last_message`, no la traza.
* **Escape JSON**: cuando apunta a `trace`, es JSON por línea, así que las comillas aparecen como `\"`.
* **Sintaxis de expresión regular**: las expresiones regulares usan sintaxis de JavaScript, así que ponga `i` en `flags` en lugar de escribir `(?i)`.

<h3 id="tools-are-denied-mcp-tools-are-missing-or-bash-won’t-run">
  Las herramientas se deniegan, las herramientas MCP faltan, o Bash no se ejecutará
</h3>

Cualquier cosa más allá del conjunto de solo lectura necesita su concesión, como `--allow-tools Bash Write`. Sus servidores MCP personales nunca cargan en una ejecución. Los servidores propios del plugin no se inician a menos que [opte por](#mock-mcp-servers), y sus herramientas entonces también necesitan una concesión `--allow-tools "mcp__plugin_<plugin>_<server>__*"`; una herramienta simulada no necesita ninguna.

<h3 id="the-run-exits-1-but-the-results-look-fine">
  La ejecución sale 1 pero los resultados se ven bien
</h3>

El `--threshold` predeterminado es 1.0, así que el comando sale 1 cuando cualquier caso puntúa por debajo de perfecto. Establezca un umbral que coincida con la puntuación que requiere. La salida 1 también cubre un archivo de caso que falló al cargar, que se reporta en stderr por encima de la tabla.

<h3 id="json-output-path-must-end-in-json">
  "--json output path must end in .json"
</h3>

Puso el destino después de `--json`, así que se leyó como la ruta de salida. Ponga el destino primero, como en `claude plugin eval . --json`, o dé a `--json` una ruta `.json` explícita.

<h3 id="a-grader-shows-passed-false-under-a-run-that-scored-1-0">
  Un calificador muestra passed: false bajo una ejecución que puntuó 1.0
</h3>

Ese calificador se excluye de la puntuación por diseño en una ejecución de dos brazos, y su campo `scored` es `false`. Vea [Comparar con una línea base sin plugin](#compare-against-a-no-plugin-baseline).

<h3 id="runs-fail-with-a-usage-limit-or-rate-limit-error-partway-through">
  Las ejecuciones fallan con un error de límite de uso o límite de velocidad a mitad de camino
</h3>

Si su cuenta alcanza el límite de uso de su plan o un límite de velocidad de API mientras se ejecuta un conjunto, cada ejecución posterior termina con ese error, se califica en lo que produjo, y generalmente puntúa 0. El conjunto aún termina y no se marca `partial`, así que el resultado puede parecer una regresión. Verifique la columna `NOTES` o `cases[].arms.with[].error` en el JSON para el mensaje de límite antes de confiar en las puntuaciones, luego re-ejecute después de que el límite se reinicie, con `--runs 1` o un filtro `--case` si necesita mantenerse bajo él.

<h3 id="runs-time-out-or-hit-the-turn-cap">
  Las ejecuciones agotan el tiempo o alcanzan el límite de turno
</h3>

Los predeterminados son 10 turnos y 300 segundos. Aumente `max_turns` y `timeout_seconds` en el caso para tareas que necesiten más, y use `--max-cost-usd` como el techo de costo en lugar de límites ajustados por ejecución.

<h2 id="see-also">
  Ver también
</h2>

* [Crear un plugin](/docs/es/plugins/create): construya el plugin que está probando, y cárguelo con `--plugin-dir` durante el desarrollo
* [Referencia de comandos de plugins](/docs/es/plugins/cli-reference#plugin-eval): las entradas de comando `plugin eval` y `plugin eval init`. La clave [`experimental.evals`](/docs/es/plugins/manifest-reference#fields) del manifiesto está en la referencia del manifiesto
* [Skills](/docs/es/skills): cómo la descripción de un skill decide cuándo Claude lo invoca, que es lo que un caso que verifica si el skill se activa está midiendo
* [Sandboxing](/docs/es/sandboxing): el sandbox a nivel de SO que se aplica cuando otorga Bash a una ejecución
* [Publicar un plugin](/docs/es/plugins/publish): publique el plugin una vez que su conjunto apruebe
* [Medir el costo y uso del plugin](/docs/es/plugins/measure): lo que el plugin añade al contexto de cada sesión y si la gente aún lo usa
