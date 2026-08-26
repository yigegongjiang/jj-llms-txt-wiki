> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review

> Configure revisiones automatizadas de PR que detecten errores lógicos, vulnerabilidades de seguridad y regresiones mediante análisis multiagente de su base de código completa

<Note>
  Code Review está en vista previa de investigación, disponible para suscripciones de [Team y Enterprise](https://claude.ai/admin-settings/claude-code). No está disponible para organizaciones con [Zero Data Retention](/docs/es/zero-data-retention) habilitado.
</Note>

Code Review analiza sus solicitudes de extracción de GitHub y publica hallazgos como comentarios en línea en las líneas de código donde encontró problemas. Una flota de agentes especializados examina los cambios de código en el contexto de su base de código completa, buscando errores lógicos, vulnerabilidades de seguridad, casos límite rotos y regresiones sutiles.

Los hallazgos se etiquetan por severidad y no aprueban ni bloquean su PR, por lo que los flujos de trabajo de revisión existentes permanecen intactos. Puede ajustar lo que Claude marca agregando un archivo `CLAUDE.md` o `REVIEW.md` a su repositorio.

Para ejecutar Claude en su propia infraestructura de CI en lugar de este servicio administrado, consulte [GitHub Actions](/docs/es/github-actions) o [GitLab CI/CD](/docs/es/gitlab-ci-cd). Para repositorios en una instancia de GitHub autohospedada, consulte [GitHub Enterprise Server](/docs/es/github-enterprise-server).

Esta página cubre:

* [Cómo funcionan las revisiones](#how-reviews-work)
* [Configuración](#set-up-code-review)
* [Disparar revisiones manualmente](#manually-trigger-reviews) con `@claude review` y `@claude review once`
* [Personalizar revisiones](#customize-reviews) con `CLAUDE.md` y `REVIEW.md`
* [Precios](#pricing)
* [Solución de problemas](#troubleshooting) ejecuciones fallidas y comentarios faltantes
* [Revisar un diff localmente](#review-a-diff-locally) con el comando `/code-review`

<Note>
  Para revisar un diff localmente en su terminal sin instalar la aplicación de GitHub, ejecute el comando `/code-review` en cualquier sesión de Claude Code. Consulte [Revisar un diff localmente](#review-a-diff-locally).
</Note>

<h2 id="how-reviews-work">
  Cómo funcionan las revisiones
</h2>

Una vez que un administrador [habilita Code Review](#set-up-code-review) para su organización, las revisiones se activan cuando se abre un PR, en cada push, o cuando se solicita manualmente, según el comportamiento configurado del repositorio. Comentar `@claude review` [inicia revisiones en un PR](#manually-trigger-reviews) en cualquier modo.

Cuando se ejecuta una revisión, múltiples agentes analizan el diff y el código circundante en paralelo en la infraestructura de Anthropic. Cada agente busca una clase diferente de problema, luego un paso de verificación verifica los candidatos contra el comportamiento real del código para filtrar falsos positivos. Los resultados se desduplican, se clasifican por severidad y se publican como comentarios en línea en las líneas específicas donde se encontraron problemas, con un resumen en el cuerpo de la revisión. Si no se encuentran problemas, Code Review actualiza la ejecución de verificación de GitHub para mostrar que no se detectaron problemas. Claude también puede publicar un breve comentario de confirmación en el PR.

Las revisiones se escalan en costo con el tamaño y la complejidad del PR, completándose en un promedio de 20 minutos. Los administradores pueden monitorear la actividad de revisión y el gasto a través del [panel de análisis](#view-usage).

<h3 id="severity-levels">
  Niveles de severidad
</h3>

Cada hallazgo se etiqueta con un nivel de severidad:

| Marcador | Severidad    | Significado                                                                  |
| :------- | :----------- | :--------------------------------------------------------------------------- |
| 🔴       | Importante   | Un error que debe corregirse antes de fusionar                               |
| 🟡       | Nit          | Un problema menor, vale la pena corregir pero no bloqueante                  |
| 🟣       | Preexistente | Un error que existe en la base de código pero no fue introducido por este PR |

Los hallazgos incluyen una sección de razonamiento extendido contraíble que puede expandir para entender por qué Claude marcó el problema y cómo verificó el problema.

<h3 id="rate-and-reply-to-findings">
  Calificar y responder a hallazgos
</h3>

Cada comentario de revisión de Claude llega con 👍 y 👎 ya adjuntos para que ambos botones aparezcan en la interfaz de usuario de GitHub para calificación de un clic. Haga clic en 👍 si el hallazgo fue útil o 👎 si fue incorrecto o ruidoso. Anthropic recopila conteos de reacciones después de que se fusiona el PR y los utiliza para ajustar el revisor. Las reacciones no activan una re-revisión ni cambian nada en el PR.

Responder a un comentario en línea no solicita a Claude que responda o actualice el PR. Para actuar sobre un hallazgo, corrija el código y haga push. Si el PR está suscrito a revisiones activadas por push, la siguiente ejecución resuelve el hilo cuando se corrige el problema. Para solicitar una revisión nueva sin hacer push, comente `@claude review once` como un [comentario de PR de nivel superior](#manually-trigger-reviews).

<h3 id="check-run-output">
  Salida de ejecución de verificación
</h3>

Más allá de los comentarios de revisión en línea, cada revisión completa la ejecución de verificación **Claude Code Review** que aparece junto a sus verificaciones de CI. Expanda su enlace **Details** para ver un resumen de cada hallazgo en un solo lugar, ordenado por severidad:

| Severidad     | Archivo:Línea             | Problema                                                                                                |
| ------------- | ------------------------- | ------------------------------------------------------------------------------------------------------- |
| 🔴 Importante | `src/auth/session.ts:142` | La actualización de token corre una carrera con el cierre de sesión, dejando sesiones obsoletas activas |
| 🟡 Nit        | `src/auth/session.ts:88`  | `parseExpiry` devuelve silenciosamente 0 en entrada malformada                                          |

Cada hallazgo también aparece como una anotación en la pestaña **Files changed**, marcado directamente en las líneas de diff relevantes. Los hallazgos importantes se representan con un marcador rojo, los nits con una advertencia amarilla y los errores preexistentes con un aviso gris. Las anotaciones y la tabla de severidad se escriben en la ejecución de verificación independientemente de los comentarios de revisión en línea, por lo que permanecen disponibles incluso si GitHub rechaza un comentario en línea en una línea que se movió.

La ejecución de verificación siempre se completa con una conclusión neutral para que nunca bloquee la fusión a través de reglas de protección de rama. Si desea bloquear fusiones en hallazgos de Code Review, lea el desglose de severidad de la salida de ejecución de verificación en su propio CI. La última línea del texto de Details es un comentario legible por máquina que su flujo de trabajo puede analizar con `gh` y jq:

```bash theme={null}
gh api repos/OWNER/REPO/check-runs/CHECK_RUN_ID \
  --jq '.output.text | split("bughunter-severity: ")[1] | split(" -->")[0] | fromjson'
```

Esto devuelve un objeto JSON con conteos por severidad, por ejemplo `{"normal": 2, "nit": 1, "pre_existing": 0}`. La clave `normal` contiene el conteo de hallazgos Importantes; un valor distinto de cero significa que Claude encontró al menos un error que vale la pena corregir antes de fusionar.

<h3 id="what-code-review-checks">
  Qué verifica Code Review
</h3>

Por defecto, Code Review se enfoca en la corrección: errores que romperían la producción, no preferencias de formato o cobertura de pruebas faltante. Puede expandir lo que verifica [agregando archivos de orientación](#customize-reviews) a su repositorio.

<h2 id="set-up-code-review">
  Configurar Code Review
</h2>

Un propietario habilita Code Review una vez para la organización y selecciona qué repositorios incluir.

<Steps>
  <Step title="Abrir configuración de administrador de Claude Code">
    Vaya a [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) y encuentre la sección Code Review. Necesita el rol de propietario o propietario principal en su organización de Claude y permiso para instalar GitHub Apps en su organización de GitHub.
  </Step>

  <Step title="Iniciar configuración">
    Haga clic en **Setup**. Esto inicia el flujo de instalación de GitHub App.
  </Step>

  <Step title="Instalar la Claude GitHub App">
    Siga las indicaciones para instalar la Claude GitHub App en su organización de GitHub. La aplicación solicita estos permisos de repositorio:

    * **Contents**: lectura y escritura
    * **Issues**: lectura y escritura
    * **Pull requests**: lectura y escritura

    Code Review utiliza acceso de lectura a contenidos y acceso de escritura a solicitudes de extracción. El conjunto de permisos más amplio también admite [GitHub Actions](/docs/es/github-actions) si lo habilita más adelante.
  </Step>

  <Step title="Seleccionar repositorios">
    Elija qué repositorios habilitar para Code Review. Si no ve un repositorio, asegúrese de haber dado a la Claude GitHub App acceso a él durante la instalación. Puede agregar más repositorios más adelante.
  </Step>

  <Step title="Establecer disparadores de revisión por repositorio">
    Después de que se complete la configuración, la sección Code Review muestra sus repositorios en una tabla. Para cada repositorio, use el menú desplegable **Review Behavior** para elegir cuándo se ejecutan las revisiones:

    * **Once after PR creation**: la revisión se ejecuta una vez cuando se abre un PR o se marca como listo para revisión
    * **After every push**: la revisión se ejecuta en cada push a la rama del PR, detectando nuevos problemas a medida que el PR evoluciona y resolviendo automáticamente los hilos cuando corrige problemas marcados
    * **Manual**: las revisiones comienzan solo cuando alguien [comenta `@claude review` o `@claude review once` en un PR](#manually-trigger-reviews); `@claude review` también suscribe el PR a revisiones en push posteriores

    Revisar en cada push ejecuta la mayoría de revisiones y cuesta más. El modo manual es útil para repositorios de alto tráfico donde desea optar por revisión en PR específicos, o para comenzar a revisar sus PR solo cuando estén listos.
  </Step>
</Steps>

La tabla de repositorios también muestra el costo promedio por revisión para cada repositorio basado en la actividad reciente. Use el menú de acciones de fila para activar o desactivar Code Review por repositorio, o para eliminar un repositorio por completo.

Para verificar la configuración, abra un PR de prueba. Si eligió un disparador automático, aparece una ejecución de verificación llamada **Claude Code Review** dentro de unos minutos. Si eligió Manual, comente `@claude review` en el PR para iniciar la primera revisión. Si no aparece ninguna ejecución de verificación, confirme que el repositorio esté listado en su configuración de administrador y que la Claude GitHub App tenga acceso a él.

<h2 id="manually-trigger-reviews">
  Disparar revisiones manualmente
</h2>

Dos comandos de comentario inician una revisión bajo demanda. Ambos funcionan independientemente del disparador configurado del repositorio, por lo que puede usarlos para optar por PR específicos en revisión en modo Manual o para obtener una re-revisión inmediata en otros modos.

| Comando               | Lo que hace                                                                      |
| :-------------------- | :------------------------------------------------------------------------------- |
| `@claude review`      | Inicia una revisión y suscribe el PR a revisiones activadas por push en adelante |
| `@claude review once` | Inicia una única revisión sin suscribir el PR a push futuros                     |

Use `@claude review once` cuando desee comentarios sobre el estado actual de un PR pero no desee que cada push posterior incurra en una revisión. Esto es útil para PR de larga duración con push frecuentes, o cuando desea una segunda opinión única sin cambiar el comportamiento de revisión del PR.

Para que cualquiera de los comandos active una revisión:

* Publíquelo como un comentario de PR de nivel superior, no un comentario en línea en una línea de diff
* Ponga el comando al inicio del comentario, con `once` en la misma línea si está usando la forma de un solo disparo
* Debe tener acceso de propietario, miembro o colaborador al repositorio
* El PR debe estar abierto

A diferencia de los disparadores automáticos, los disparadores manuales se ejecutan en PR de borrador, ya que una solicitud explícita señala que desea la revisión ahora independientemente del estado de borrador.

Si una revisión ya se está ejecutando en ese PR, la solicitud se pone en cola hasta que se complete la revisión en progreso. Puede monitorear el progreso a través de la ejecución de verificación en el PR.

<h2 id="customize-reviews">
  Personalizar revisiones
</h2>

Code Review lee dos archivos de su repositorio para guiar lo que marca. Difieren en cuán fuertemente influyen en la revisión:

* **`CLAUDE.md`**: instrucciones de proyecto compartidas que Claude Code utiliza para todas las tareas, no solo revisiones. Code Review lo lee como contexto de proyecto e marca las violaciones recién introducidas como nits.
* **`REVIEW.md`**: instrucciones solo de revisión, inyectadas directamente en cada agente en la canalización de revisión como prioridad más alta. Úselo para cambiar lo que se marca, con qué severidad y cómo se reportan los hallazgos.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review lee sus archivos `CLAUDE.md` del repositorio y trata las violaciones recién introducidas como hallazgos de [nivel nit](#severity-levels). Esto funciona bidireccionalamente: si su PR cambia el código de una manera que hace que una declaración `CLAUDE.md` esté desactualizada, Claude marca que los documentos necesitan actualización también.

Claude lee archivos `CLAUDE.md` en cada nivel de su jerarquía de directorios, por lo que las reglas en el `CLAUDE.md` de un subdirectorio se aplican solo a archivos bajo esa ruta. Consulte la [documentación de memoria](/docs/es/memory) para obtener más información sobre cómo funciona `CLAUDE.md`.

Para orientación específica de revisión que no desea aplicar a sesiones generales de Claude Code, use [`REVIEW.md`](#review-md) en su lugar.

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md` es un archivo en la raíz de su repositorio que anula cómo se comporta Code Review en su repositorio. Su contenido se inyecta en el prompt del sistema de cada agente en la canalización de revisión como el bloque de instrucción de prioridad más alta, tomando precedencia sobre la orientación de revisión predeterminada.

Porque se pega textualmente, `REVIEW.md` es instrucciones simples: la [sintaxis de importación `@`](/docs/es/memory#import-additional-files) no se expande, y los archivos referenciados no se leen en el prompt. Ponga las reglas que desea aplicar directamente en el archivo.

<h4 id="what-you-can-tune">
  Qué puede ajustar
</h4>

`REVIEW.md` es markdown de forma libre, por lo que cualquier cosa que pueda expresar como una instrucción de revisión está en el alcance. Los patrones a continuación tienen el mayor impacto en la práctica.

**Severidad**: redefina qué significa 🔴 Importante para su repositorio. La calibración predeterminada se dirige al código de producción; un repositorio de documentos, un repositorio de configuración, o un prototipo podría querer una definición mucho más estrecha. Indique explícitamente qué clases de hallazgo son Importantes y cuáles son Nit como máximo. También puede escalar en la otra dirección, por ejemplo tratando cualquier violación de `CLAUDE.md` como Importante en lugar del nit predeterminado.

**Volumen de nit**: limite cuántos comentarios 🟡 Nit publica una única revisión. La prosa y los archivos de configuración pueden pulirse para siempre. Un límite como "reportar como máximo cinco nits, mencionar el resto como un conteo en el resumen" mantiene las revisiones accionables.

**Reglas de omisión**: enumere rutas, patrones de rama y categorías de hallazgo donde Claude no debe publicar hallazgos. Los candidatos comunes son código generado, archivos de bloqueo, dependencias vendidas y ramas creadas por máquinas, junto con cualquier cosa que su CI ya aplique como linting o verificación ortográfica. Para rutas que justifiquen alguna revisión pero no escrutinio completo, establezca una barra más alta en lugar de omitir completamente: "en `scripts/`, solo reportar si está cerca de cierto y es severo."

**Verificaciones específicas del repositorio**: agregue reglas que desea marcar en cada PR, como "las nuevas rutas de API deben tener una prueba de integración." Porque `REVIEW.md` se inyecta como prioridad más alta, estas se aterrizan más confiablemente que las mismas reglas en un `CLAUDE.md` largo.

**Barra de verificación**: requiera evidencia antes de que se publique una clase de hallazgo. Por ejemplo, "las afirmaciones de comportamiento necesitan una cita `file:line` en la fuente, no una inferencia de nombres" reduce falsos positivos que de otro modo costarían al autor un viaje de ida y vuelta.

**Convergencia de re-revisión**: dígale a Claude cómo comportarse cuando un PR ya ha sido revisado. Una regla como "después de la primera revisión, suprima nits nuevos y publique hallazgos Importantes solo" detiene una corrección de una línea de alcanzar la ronda siete solo por estilo.

**Forma de resumen**: pida que el cuerpo de revisión se abra con un conteo de una línea como `2 factual, 4 style`, y que comience con "no hay problemas factuales" cuando ese sea el caso. El autor quiere saber la forma del trabajo antes de los detalles.

<h4 id="example">
  Ejemplo
</h4>

Este `REVIEW.md` recalibra la severidad para un servicio backend, limita nits, omite archivos generados y agrega verificaciones específicas del repositorio.

```markdown theme={null}
# Instrucciones de revisión

## Qué significa Importante aquí

Reserve Importante para hallazgos que romperían el comportamiento, filtrarían datos,
o bloquearían un retroceso: lógica incorrecta, consultas de base de datos sin alcance, PII
en registros o mensajes de error, y migraciones que no son compatibles hacia atrás.
El estilo, nombres y sugerencias de refactorización son Nit como máximo.

## Limitar los nits

Reportar como máximo cinco Nits por revisión. Si encontró más, diga "más N
elementos similares" en el resumen en lugar de publicarlos en línea. Si
todo lo que encontró es un Nit, comience el resumen con "Sin problemas bloqueantes."

## No reportar

- Cualquier cosa que CI ya aplique: lint, formato, errores de tipo
- Archivos generados bajo `src/gen/` y cualquier archivo `*.lock`
- Código solo de prueba que intencionalmente viola reglas de producción

## Siempre verificar

- Las nuevas rutas de API tienen una prueba de integración
- Las líneas de registro no incluyen direcciones de correo electrónico, IDs de usuario o cuerpos de solicitud
- Las consultas de base de datos están limitadas al inquilino del llamador
```

<h4 id="keep-it-focused">
  Mantenerlo enfocado
</h4>

La longitud tiene un costo: un `REVIEW.md` largo diluye las reglas que más importan. Manténgalo en instrucciones que cambien el comportamiento de revisión, y deje el contexto general del proyecto en `CLAUDE.md`.

<h2 id="view-usage">
  Ver uso
</h2>

Vaya a [claude.ai/analytics/code-review](https://claude.ai/analytics/code-review) para ver la actividad de Code Review en toda su organización. El panel muestra:

| Sección              | Lo que muestra                                                                                                  |
| :------------------- | :-------------------------------------------------------------------------------------------------------------- |
| PRs reviewed         | Conteo diario de solicitudes de extracción revisadas durante el rango de tiempo seleccionado                    |
| Cost weekly          | Gasto semanal en Code Review                                                                                    |
| Feedback             | Conteo de comentarios de revisión que se resolvieron automáticamente porque un desarrollador abordó el problema |
| Repository breakdown | Conteos por repositorio de PR revisados y comentarios resueltos                                                 |

La tabla de repositorios en la configuración de administrador también muestra el costo promedio por revisión para cada repositorio. Las cifras de costo del panel son estimaciones para monitorear la actividad; para gasto preciso en factura, consulte su factura de Anthropic.

<h2 id="pricing">
  Precios
</h2>

Code Review se factura según el uso de tokens. Cada revisión promedia \$15-25 en costo, escalando con el tamaño del PR, la complejidad de la base de código y cuántos problemas requieren verificación. El uso de Code Review se factura por separado a través de [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) y no cuenta contra el uso incluido de su plan.

El disparador de revisión que elija afecta el costo total:

* **Once after PR creation**: se ejecuta una vez por PR
* **After every push**: se ejecuta en cada push, multiplicando el costo por el número de push
* **Manual**: sin revisiones hasta que alguien comente `@claude review` en un PR

En cualquier modo, comentar `@claude review` [opta el PR en revisiones activadas por push](#manually-trigger-reviews), por lo que se acumula costo adicional por push después de ese comentario. Para ejecutar una única revisión sin suscribirse a push futuros, comente `@claude review once` en su lugar.

Los costos aparecen en su factura de Anthropic independientemente de si su organización usa Amazon Bedrock o Google Cloud's Agent Platform para otras características de Claude Code. Para establecer un límite de gasto mensual para Code Review, vaya a [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage) y configure el límite para el servicio Claude Code Review.

Monitoree el gasto a través del gráfico de costo semanal en [analytics](#view-usage) o la columna de costo promedio por repositorio en la configuración de administrador.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Las ejecuciones de revisión son de mejor esfuerzo. Una ejecución fallida nunca bloquea su PR, pero tampoco se reintenta por sí sola. Esta sección cubre cómo recuperarse de una ejecución fallida y dónde buscar cuando la ejecución de verificación reporta problemas que no puede encontrar.

<h3 id="retrigger-a-failed-or-timed-out-review">
  Reactivar una revisión fallida o agotada por tiempo
</h3>

Cuando la infraestructura de revisión golpea un error interno o excede su límite de tiempo, la ejecución de verificación se completa con un título de **Code review encountered an error** o **Code review timed out**. La conclusión sigue siendo neutral, por lo que nada bloquea su fusión, pero no se publican hallazgos.

Para ejecutar la revisión nuevamente, comente `@claude review once` en el PR. Esto inicia una revisión nueva sin suscribir el PR a push futuros. Si el PR ya está suscrito a revisiones activadas por push, hacer push de un nuevo commit también inicia una nueva revisión.

El botón **Re-run** en la pestaña Checks de GitHub no reactiva Code Review. Use el comando de comentario o un nuevo push en su lugar.

<h3 id="review-didn’t-run-and-the-pr-shows-a-spend-cap-message">
  La revisión no se ejecutó y el PR muestra un mensaje de límite de gasto
</h3>

Cuando se alcanza el límite de gasto mensual de su organización, Code Review publica un único comentario en el PR explicando que la revisión fue omitida. Las revisiones se reanudan automáticamente al inicio del próximo período de facturación, o inmediatamente cuando un administrador aumenta el límite en [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage).

<h3 id="find-issues-that-aren’t-showing-as-inline-comments">
  Encontrar problemas que no se muestran como comentarios en línea
</h3>

Si el título de la ejecución de verificación dice que se encontraron problemas pero no ve comentarios de revisión en línea en el diff, busque en estas otras ubicaciones donde se muestran los hallazgos:

* **Check run Details**: haga clic en **Details** junto a la verificación Claude Code Review en la pestaña Checks. La tabla de severidad enumera cada hallazgo con su archivo, línea y resumen independientemente de si el comentario en línea fue aceptado.
* **Files changed annotations**: abra la pestaña **Files changed** en el PR. Los hallazgos se representan como anotaciones adjuntas directamente a las líneas de diff, separadas de los comentarios de revisión.
* **Review body**: si hizo push al PR mientras se ejecutaba una revisión, algunos hallazgos pueden hacer referencia a líneas que ya no existen en el diff actual. Esos aparecen bajo un encabezado **Additional findings** en el texto del cuerpo de revisión en lugar de como comentarios en línea.

<h2 id="review-a-diff-locally">
  Revisar un diff localmente
</h2>

El comando [`/code-review`](/docs/es/commands) revisa un diff en su terminal sin instalar la aplicación de GitHub. Ejecutelo en cualquier sesión de Claude Code: reporta errores de corrección y reutilización, simplificación y limpiezas de eficiencia. De forma predeterminada, la revisión local cubre los commits de su rama por delante de su rama ascendente más cualquier cambio sin confirmar en el árbol de trabajo. Pase `--comment` para publicar hallazgos como comentarios de PR en línea, o `--fix` para aplicar los hallazgos a su árbol de trabajo después de la revisión.

Los [niveles de esfuerzo](/docs/es/model-config#adjust-effort-level) más bajos devuelven menos hallazgos con mayor confianza, mientras que `high` hasta `max` dan cobertura más amplia y pueden incluir hallazgos inciertos. Sin un argumento de esfuerzo, la revisión utiliza el esfuerzo actual de la sesión. Para revisar algo diferente del diff predeterminado, pase un objetivo: una ruta de archivo, un número de PR, un nombre de rama o un rango de ref como `main...my-feature`. La forma de rango de ref revisa el diff confirmado que una solicitud de extracción de `my-feature` a `main` contendría, independientemente de cómo esté configurada la rama ascendente de la rama.

`/code-review ultra --fix` ejecuta la [ultrareview](/docs/es/ultrareview) más profunda en la nube, luego aplica sus hallazgos a su árbol de trabajo cuando regresan a su sesión. Ultrareview utiliza su propio alcance: su rama actual contra la rama predeterminada del repositorio, más cualquier cambio sin confirmar y preparado en el árbol de trabajo.

El comando se llamaba `/simplify` antes de v2.1.147, cuando aplicaba correcciones de forma predeterminada. A partir de v2.1.154, `/simplify` ejecuta una revisión separada de solo limpieza que aplica correcciones sin buscar errores. Si escribió scripts con `/simplify` para búsqueda de errores, cambie a `/code-review --fix`, que no ha cambiado.

<h2 id="related-resources">
  Recursos relacionados
</h2>

Code Review está diseñado para funcionar junto con el resto de Claude Code. Si desea ejecutar revisiones localmente antes de abrir un PR, necesita una configuración autohospedada, o desea profundizar en cómo `CLAUDE.md` forma el comportamiento de Claude en todas las herramientas, estas páginas son buenos siguientes pasos:

* [Commands](/docs/es/commands): ejecute `/code-review` en una sesión local de Claude Code para verificar un diff antes de hacer push
* [GitHub Actions](/docs/es/github-actions): ejecute Claude en sus propios flujos de trabajo de GitHub Actions para automatización personalizada más allá de la revisión de código
* [GitLab CI/CD](/docs/es/gitlab-ci-cd): integración de Claude autohospedada para canalizaciones de GitLab
* [Memory](/docs/es/memory): cómo funcionan los archivos `CLAUDE.md` en Claude Code
* [Analytics](/docs/es/analytics): rastrear el uso de Claude Code más allá de la revisión de código
