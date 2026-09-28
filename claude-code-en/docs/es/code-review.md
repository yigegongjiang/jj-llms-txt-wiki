> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review

> Configure revisiones automatizadas de PR que detecten errores lógicos, vulnerabilidades de seguridad y regresiones mediante análisis multiagente de su base de código completa

<Note>
  Code Review está en vista previa de investigación, disponible para suscripciones de [Team y Enterprise](https://claude.ai/admin-settings/claude-code). No está disponible para organizaciones con [Zero Data Retention](/docs/es/zero-data-retention) habilitado. En otros planes, aún puede [revisar un diff localmente](#review-a-diff-locally) con el comando `/code-review`.
</Note>

Code Review analiza sus solicitudes de extracción de GitHub y publica hallazgos como comentarios en línea en las líneas de código donde encontró problemas. Una flota de agentes especializados examina los cambios de código en el contexto de su base de código completa, buscando errores lógicos, vulnerabilidades de seguridad, casos límite rotos y regresiones sutiles.

Los hallazgos se etiquetan por severidad y no aprueban ni bloquean su PR, por lo que los flujos de trabajo de revisión existentes permanecen intactos. Puede ajustar lo que Claude marca agregando un archivo `CLAUDE.md` o `REVIEW.md` a su repositorio.

Para ejecutar Claude en su propia infraestructura de CI en lugar de este servicio administrado, consulte [GitHub Actions](/docs/es/github-actions) o [GitLab CI/CD](/docs/es/gitlab-ci-cd). Para repositorios en una instancia de GitHub autohospedada, consulte [GitHub Enterprise Server](/docs/es/github-enterprise-server).

Esta página cubre:

* [Cómo funcionan las revisiones](#how-reviews-work)
* [Configuración](#set-up-code-review)
* [Disparar revisiones manualmente](#manually-trigger-reviews) con `@claude review` y `@claude review always`
* [Personalizar revisiones](#customize-reviews) con `CLAUDE.md` y `REVIEW.md`
* [Precios](#pricing)
* [Solución de problemas](#troubleshooting) ejecuciones fallidas y comentarios faltantes
* [Revisar un diff localmente](#review-a-diff-locally) con el comando `/code-review`

<h2 id="how-reviews-work">
  Cómo funcionan las revisiones
</h2>

Una vez que un administrador [habilita Code Review](#set-up-code-review) para su organización, las revisiones se activan cuando se abre un PR, en cada push, o cuando se solicita manualmente, según el comportamiento configurado del repositorio. Comentar `@claude review` [inicia una revisión en un PR](#manually-trigger-reviews) en cualquier modo.

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

Responder a un comentario en línea no solicita a Claude que responda o actualice el PR. Para actuar sobre un hallazgo, corrija el código y haga push. Si el PR está suscrito a revisiones activadas por push, la siguiente ejecución resuelve el hilo cuando se corrige el problema. Para solicitar una revisión nueva sin hacer push, comente `@claude review` como un [comentario de PR de nivel superior](#manually-trigger-reviews).

Para descartar un hallazgo sin un cambio de código, resuelva su hilo; responder no lo descarta.

<h3 id="check-run-output">
  Salida de ejecución de verificación
</h3>

Más allá de los comentarios de revisión en línea, cada revisión completa la ejecución de verificación **Claude Code Review** que aparece junto a sus verificaciones de CI. Expanda su enlace **Details** para ver un resumen de cada hallazgo en un solo lugar, ordenado por severidad:

| Severidad     | Archivo:Línea             | Problema                                                                                                |
| ------------- | ------------------------- | ------------------------------------------------------------------------------------------------------- |
| 🔴 Importante | `src/auth/session.ts:142` | La actualización de token corre una carrera con el cierre de sesión, dejando sesiones obsoletas activas |
| 🟡 Nit        | `src/auth/session.ts:88`  | `parseExpiry` devuelve silenciosamente 0 en entrada malformada                                          |

Cada hallazgo también aparece como una anotación en la pestaña **Files changed**, marcado directamente en las líneas de diff relevantes. Los hallazgos importantes se representan con un marcador rojo, los nits con una advertencia amarilla y los errores preexistentes con un aviso gris. Las anotaciones y la tabla de severidad se escriben en la ejecución de verificación independientemente de los comentarios de revisión en línea, por lo que permanecen disponibles incluso si GitHub rechaza un comentario en línea en una línea que se movió.

La ejecución de verificación siempre se completa con una conclusión neutral para que nunca bloquee la fusión a través de reglas de protección de rama. Si desea bloquear fusiones en hallazgos de Code Review, lea el desglose de severidad de la salida de ejecución de verificación en su propio CI. La última línea del texto de Details es un comentario legible por máquina que su flujo de trabajo puede analizar con `gh` y jq. Para encontrar el ID de ejecución de verificación, enumere las ejecuciones de verificación del commit con `gh api repos/OWNER/REPO/commits/<commit-sha>/check-runs --jq '.check_runs[] | {id, name}'` y tome el `id` de la ejecución **Claude Code Review**. Reemplace `OWNER`, `REPO` y `CHECK_RUN_ID` con el propietario del repositorio, el nombre del repositorio y ese ID:

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
    Siga las indicaciones para instalar la Claude GitHub App: seleccione la organización de GitHub que posee los repositorios que desea revisar, elija a qué repositorios puede acceder la aplicación y apruebe los permisos solicitados.

    Para revisar una solicitud de extracción, Claude lee el contenido de su repositorio a través del acceso de lectura de la aplicación y publica comentarios y la [ejecución de verificación](#check-run-output) a través de su acceso de escritura a solicitudes de extracción y verificaciones. Durante la instalación, otorga un conjunto de permisos más amplio compartido por otras características de Claude, como [GitHub Actions](/docs/es/github-actions); consulte [Permisos de GitHub App](/docs/es/github-actions#github-app-permissions) para obtener la lista completa.
  </Step>

  <Step title="Seleccionar repositorios">
    Elija qué repositorios habilitar para Code Review. Si no ve un repositorio, asegúrese de haber dado a la Claude GitHub App acceso a él durante la instalación. Puede agregar más repositorios más adelante.
  </Step>

  <Step title="Establecer disparadores de revisión por repositorio">
    Después de que se complete la configuración, la sección Code Review muestra sus repositorios en una tabla. Para cada repositorio, use el menú desplegable **Review Behavior** para elegir cuándo se ejecutan las revisiones:

    * **Once after PR creation**: la revisión se ejecuta una vez cuando se abre un PR o se marca como listo para revisión
    * **After every push**: la revisión se ejecuta en cada push a la rama del PR, detectando nuevos problemas a medida que el PR evoluciona y resolviendo automáticamente los hilos cuando corrige problemas marcados
    * **Manual**: abrir o hacer push a un PR no inicia una revisión; comente [`@claude review`](#manually-trigger-reviews) para solicitar una, o `@claude review always` para también suscribir el PR a revisiones en push posteriores

    Sea cual sea la opción que elija, Claude revisa una [solicitud de extracción de una bifurcación](#review-pull-requests-from-forks) solo cuando alguien comenta `@claude review` en ella.

    Revisar en cada push ejecuta la mayoría de revisiones y cuesta más. El modo manual es útil para repositorios de alto tráfico donde desea optar por revisión en PR específicos, o para comenzar a revisar sus PR solo cuando estén listos.
  </Step>
</Steps>

La tabla de repositorios también muestra el costo promedio por revisión para cada repositorio basado en la actividad reciente. Use el menú de acciones de fila para activar o desactivar Code Review por repositorio, o para eliminar un repositorio por completo.

Para verificar la configuración, abra un PR de prueba. Si eligió un disparador automático, aparece una ejecución de verificación llamada **Claude Code Review** dentro de unos minutos. Si eligió Manual, comente `@claude review` en el PR para iniciar la primera revisión. Si no aparece ninguna ejecución de verificación, confirme que el repositorio esté listado en su configuración de administrador y que la Claude GitHub App tenga acceso a él.

<h2 id="manually-trigger-reviews">
  Disparar revisiones manualmente
</h2>

Los comandos de comentario inician una revisión bajo demanda. Funcionan independientemente del disparador configurado del repositorio, por lo que puede usarlos para optar por PR específicos en revisión en modo Manual o para obtener una re-revisión inmediata en otros modos.

| Comando                 | Lo que hace                                                                      |
| :---------------------- | :------------------------------------------------------------------------------- |
| `@claude review`        | Inicia una única revisión sin suscribir el PR a futuras inserciones              |
| `@claude review always` | Inicia una revisión y suscribe el PR a revisiones activadas por push en adelante |
| `@claude review once`   | Lo mismo que `@claude review`: inicia una única revisión sin suscribir           |

Use `@claude review always` cuando desee que cada inserción posterior al PR inicie una revisión nueva, como en un PR de alta prioridad en un repositorio configurado en modo Manual. Dado que el comando simple no suscribe el PR, puede solicitar una segunda opinión única sin cambiar si las inserciones posteriores activan revisiones.

<Note>
  Antes de una actualización de julio de 2026, `@claude review` suscribía el PR a revisiones activadas por push. Si dependía de ese comportamiento, comente `@claude review always` en su lugar. `@claude review once` sigue funcionando y se comporta igual que el comando simple.
</Note>

Para que cualquiera de estos comandos active una revisión:

* Publíquelo como un comentario de PR de nivel superior, no un comentario en línea en una línea de diff
* Ponga el comando al inicio del comentario, con `once` o `always` en la misma línea que el resto del comando
* Debe tener permiso de escritura, mantenimiento o administrador en el repositorio
* El PR debe estar abierto

Si el repositorio pertenece a una organización y su membresía en esa organización es privada, que es el valor predeterminado de GitHub, GitHub no lo identifica ante Claude como miembro. Claude aún puede reaccionar a su comentario con 👀, pero no inicia una revisión a menos que haya sido agregado al repositorio directamente como colaborador, incluso cuando un equipo o los permisos base de la organización le otorguen acceso de escritura. Para solucionar esto, [haga pública su membresía en la organización](https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-personal-account-on-github/managing-your-membership-in-organizations/publicizing-or-hiding-organization-membership) o pida a un administrador del repositorio que lo agregue al repositorio como colaborador.

A diferencia de los disparadores automáticos, los disparadores manuales se ejecutan en PR de borrador, ya que una solicitud explícita señala que desea la revisión ahora independientemente del estado de borrador.

Si una revisión ya se está ejecutando en ese PR, la solicitud se pone en cola hasta que se complete la revisión en progreso. Puede monitorear el progreso a través de la ejecución de verificación en el PR.

<h3 id="review-pull-requests-from-forks">
  Revisar solicitudes de extracción desde bifurcaciones
</h3>

Claude no revisa una solicitud de extracción desde una bifurcación automáticamente, independientemente de la configuración de **Comportamiento de revisión** del repositorio. Para iniciar una, comente `@claude review` en la solicitud de extracción. Los [requisitos para comandos de comentario](#manually-trigger-reviews) aún se aplican, y el acceso de escritura que necesita es al repositorio base, no a la bifurcación.

Para obtener otra revisión de una solicitud de extracción de bifurcación, publique un nuevo comentario `@claude review`. `@claude review always` también funciona, pero no suscribe la solicitud de extracción a revisiones en inserciones posteriores. Nada más que un comando de comentario inicia una revisión en una solicitud de extracción de bifurcación:

* Hacer clic en **Re-ejecutar** en la ejecución de verificación no inicia una revisión
* Insertar nuevos commits no inicia una revisión, incluso en un repositorio configurado en **Después de cada push**

<h2 id="customize-reviews">
  Personalizar revisiones
</h2>

Code Review lee dos archivos de su repositorio para guiar lo que marca. Difieren en cuán fuertemente influyen en la revisión:

* **`CLAUDE.md`**: instrucciones de proyecto compartidas que Claude Code utiliza para todas las tareas, no solo revisiones. Code Review lo lee como contexto de proyecto e marca las violaciones recién introducidas como nits.
* **`REVIEW.md`**: instrucciones solo de revisión, proporcionadas a los agentes que encuentran y verifican hallazgos y consultadas por los agentes que clasifican y reportan hallazgos. Úselo para indicar qué desea que se marque su equipo, con qué severidad y cómo se reportan los hallazgos.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review lee sus archivos `CLAUDE.md` del repositorio y trata las violaciones recién introducidas como hallazgos de [nivel nit](#severity-levels). Esto funciona bidireccionalamente: si su PR cambia el código de una manera que hace que una declaración `CLAUDE.md` esté desactualizada, Claude marca que los documentos necesitan actualización también.

Claude lee archivos `CLAUDE.md` en cada nivel de su jerarquía de directorios, por lo que las reglas en el `CLAUDE.md` de un subdirectorio se aplican solo a archivos bajo esa ruta. Consulte la [documentación de memoria](/docs/es/memory) para obtener más información sobre cómo funciona `CLAUDE.md`.

Para orientación específica de revisión que no desea aplicar a sesiones generales de Claude Code, use [`REVIEW.md`](#review-md) en su lugar.

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md` es un archivo en la raíz de su repositorio que personaliza Code Review para su repositorio. Los agentes en la canalización de revisión que encuentran y verifican hallazgos reciben su contenido como las instrucciones de revisión de su repositorio, junto con la orientación de revisión predeterminada de Code Review, y los agentes que clasifican y reportan hallazgos la consultan antes de establecer la severidad y escribir la revisión.

Ponga las reglas que desea aplicar directamente en `REVIEW.md`.

<h4 id="what-you-can-tune">
  Qué puede ajustar
</h4>

`REVIEW.md` es markdown de forma libre, por lo que cualquier cosa que pueda expresar como una instrucción de revisión está en el alcance. Los patrones a continuación tienen el mayor impacto en la práctica.

**Severidad**: redefina qué significa 🔴 Importante para su repositorio. La calibración predeterminada se dirige al código de producción; un repositorio de documentos, un repositorio de configuración, o un prototipo podría querer una definición mucho más estrecha. Indique explícitamente qué clases de hallazgo son Importantes y cuáles son Nit como máximo. También puede escalar en la otra dirección, por ejemplo tratando cualquier violación de `CLAUDE.md` como Importante en lugar del nit predeterminado.

**Volumen de nit**: limite cuántos comentarios 🟡 Nit publica una única revisión. La prosa y los archivos de configuración pueden pulirse para siempre. Un límite como "reportar como máximo cinco nits, mencionar el resto como un conteo en el resumen" mantiene las revisiones accionables.

**Reglas de omisión**: enumere rutas, patrones de rama y categorías de hallazgo donde Claude no debe publicar hallazgos. Los candidatos comunes son código generado, archivos de bloqueo, dependencias vendidas y ramas creadas por máquinas, junto con cualquier cosa que su CI ya aplique como linting o verificación ortográfica. Para rutas que justifiquen alguna revisión pero no escrutinio completo, establezca una barra más alta en lugar de omitir completamente: "en `scripts/`, solo reportar si está cerca de cierto y es severo."

**Verificaciones específicas del repositorio**: agregue reglas que desea marcar en cada PR, como "las nuevas rutas de API deben tener una prueba de integración." Porque `REVIEW.md` llega a cada agente de hallazgo y verificación directamente, estos se aterrizan más confiablemente que las mismas reglas en un `CLAUDE.md` largo.

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

Las cifras de costo del panel son estimaciones para monitorear la actividad. Para gasto preciso en factura, consulte su factura de Anthropic.

<h2 id="pricing">
  Precios
</h2>

Code Review se factura según el uso de tokens. Cada revisión promedia \$15-25 en costo, escalando con el tamaño del PR, la complejidad de la base de código y cuántos problemas requieren verificación. El uso de Code Review se factura por separado a través de [créditos de uso](https://support.claude.com/es/articles/12429409-extra-usage-for-paid-claude-plans) y no cuenta contra el uso incluido de su plan.

El disparador de revisión que elija afecta el costo total:

* **Once after PR creation**: se ejecuta una vez por PR
* **After every push**: se ejecuta en cada push, multiplicando el costo por el número de push
* **Manual**: sin revisiones en PR abiertos o push, por lo que el costo se acumula solo de las revisiones que alguien solicita

En modo Once after PR creation o Manual, comentar `@claude review always` [opta el PR en revisiones activadas por push](#manually-trigger-reviews), por lo que se acumula costo adicional por push después de ese comentario. En modo After every push, los push ya activan revisiones, por lo que la suscripción no cambia el costo por push. Comentar `@claude review` ejecuta una única revisión sin suscribirse a push futuros. Claude revisa una [solicitud de extracción desde un fork](#review-pull-requests-from-forks) solo cuando alguien comenta `@claude review`, por lo que una solicitud de extracción de fork nunca acumula costo por push en ningún modo.

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

Para ejecutar la revisión nuevamente, comente `@claude review` en el PR. Esto inicia una revisión nueva sin suscribir el PR a push futuros. Si el PR no es [de una bifurcación](#review-pull-requests-from-forks), puede hacer clic en **Re-run** en la verificación **Claude Code Review** en la pestaña Checks de GitHub. Una reejecución también inicia una revisión nueva sin suscribir el PR.

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

El comando [`/code-review`](/docs/es/commands) revisa un diff en su terminal sin instalar la aplicación de GitHub. Reporta errores de corrección y reutilización, simplificación y limpiezas de eficiencia.

`/review` es un alias de `/code-review`; antes de v2.1.223, era un comando separado que ejecutaba una revisión de una sola pasada, de solo lectura, de una solicitud de extracción de GitHub.

<Steps>
  <Step title="Ejecutar /code-review">
    Desde la sesión en la que está trabajando, ejecute el comando:

    ```text theme={null}
    /code-review
    ```

    Revisa los commits de su rama por delante de su rama ascendente más cualquier cambio sin confirmar, por lo que necesita trabajo en la rama o en el árbol de trabajo para tener algo que reportar. Para revisar algo diferente, pase un objetivo: una ruta de archivo, un número de PR, un nombre de rama o un rango de ref como `main...my-feature`.

    También puede agregar indicadores:

    * `--fix`: aplica los hallazgos a su árbol de trabajo después de la revisión
    * `--comment`: publica los hallazgos como comentarios en línea en una solicitud de extracción de GitHub, o como una nota única en una solicitud de fusión de GitLab
    * `--post`: en una revisión en la nube `ultra` de una solicitud de extracción de `github.com`, preselecciona publicar los hallazgos terminados en el PR en el diálogo de lanzamiento; consulte [Publicar hallazgos en la solicitud de extracción](/docs/es/ultrareview#post-findings-to-the-pull-request). Requiere Claude Code v2.1.227 o posterior

    Cuando pasa `--comment` para una solicitud de fusión de GitLab, Claude Code publica los hallazgos a través de la CLI `glab` de GitLab. Requiere Claude Code v2.1.257 o posterior. Cuando `glab` no está instalado, Claude imprime los hallazgos en la terminal en su lugar.

    Pase la solicitud de fusión como su URL o una referencia `!123`. Claude Code trata un número desnudo o un nombre de rama como una solicitud de fusión solo cuando el checkout de origen está en `gitlab.com`. En una instancia de GitLab autogestionada, pase la URL o el formulario `!123`.
  </Step>

  <Step title="Continuar trabajando">
    La revisión se ejecuta como un [subagente](/docs/es/sub-agents) de fondo con su propia ventana de contexto, por lo que no llena su conversación. Los hallazgos llegan a su conversación cuando se completa la revisión.
  </Step>

  <Step title="Actuar sobre los hallazgos">
    Pida a Claude que corrija lo que encontró la revisión. Si pasó `--fix` o `--comment`, la revisión ya ha aplicado o publicado sus hallazgos.
  </Step>
</Steps>

Claude reporta los hallazgos como texto en la respuesta en ambas ejecuciones, incluso cuando una aplicación host solicita una lista de hallazgos:

* En una sesión de terminal, donde `/code-review` ejecuta la revisión como un [subagente bifurcado](/docs/es/skills#run-skills-in-a-subagent)
* En una ejecución `-p` con salida de texto o JSON

En una aplicación host que solicita la lista de hallazgos, como la [aplicación de escritorio](/docs/es/desktop), Claude reporta los hallazgos de la revisión a través de la herramienta [`ReportFindings`](/docs/es/tools-reference). Claude Code renderiza el informe como una lista de hallazgos, y cada entrada muestra la ubicación del archivo, un resumen de una oración y una etiqueta de categoría como `correctness` cuando el hallazgo lleva una. Una solicitud de host se aplica en cada nivel de esfuerzo y requiere Claude Code v2.1.218 o posterior.

Cuando Claude corrige hallazgos reportados más adelante en la sesión, los reporta nuevamente, y Claude Code marca cada hallazgo en la lista de hallazgos actualizada como corregido, omitido o sin cambios necesarios.

<h3 id="what-the-review-reads-and-edits">
  Qué lee y edita la revisión
</h3>

La revisión sigue su `CLAUDE.md` como cualquier sesión de Claude Code, pero no lee [`REVIEW.md`](#review-md). Una revisión de fondo aplica sus ediciones `--fix` fuera de los [puntos de control](/docs/es/checkpointing#subagent-edits-not-restored) de su sesión, por lo que `/rewind` no las deshace; use git para revertirlas. Cuando la revisión [se ejecuta en primer plano](#run-in-the-foreground), edita su árbol de trabajo durante su propio turno, por lo que `/rewind` restaura sus ediciones como de costumbre.

<h3 id="tune-effort-and-arguments">
  Ajustar esfuerzo y argumentos
</h3>

Pase un [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) para intercambiar cobertura por confianza. En `low` y `medium`, la revisión reporta solo los hallazgos en los que tiene más confianza, por lo que ve menos falsos positivos; `high` hasta `max` amplían la cobertura y pueden incluir hallazgos en los que la revisión tiene menos certeza.

Cuando no escribe un nivel, la revisión reutiliza el último nivel de `low` a `max` que escribió, incluso en una sesión anterior, y Claude Code muestra un aviso como `Reusing high effort, the level you typed last time`. Escriba un nivel, como `/code-review high`, para cambiar lo que reutilizan las ejecuciones posteriores; un nivel que pase en una ejecución `-p` no interactiva no lo actualiza. `ultra` ni actualiza ni utiliza el nivel recordado. Si nunca ha escrito un nivel, la revisión utiliza el esfuerzo actual de la sesión. Antes de v2.1.223, un `/code-review` sin un nivel siempre utilizaba el esfuerzo actual de la sesión.

Después del nivel de esfuerzo y los indicadores, Claude Code lee el resto de la línea de una de dos formas:

* **Sin `ultra`**: todo lo que queda es el objetivo de la revisión, incluso cuando comienza con otro nombre de comando. `/code-review /fix-issue 123` revisa con `/fix-issue 123` como texto objetivo en lugar de cargar `/fix-issue` como una [skill apilada](/docs/es/skills#pass-arguments-to-skills) separada. Antes de v2.1.218, un comando apilado después de `/code-review` se expandía como su propia skill.
* **Con `ultra`**: Claude Code lee una sola palabra como una rama base o número de PR, y convierte texto más largo que no nombra una rama o PR en [una nota adjunta a la revisión](/docs/es/ultrareview#pass-a-request-in-plain-words). `/code-review ultra check my auth changes` revisa su rama actual, y Claude relaciona los hallazgos con su nota.

<h3 id="run-in-the-foreground">
  Ejecutar en primer plano
</h3>

La revisión se ejecuta en segundo plano de forma predeterminada; antes de v2.1.218, se ejecutaba dentro de su conversación. Se ejecuta en primer plano en su lugar en casos como estos:

* Ejecuta `/code-review` nuevamente mientras una revisión anterior aún está en progreso
* Lo ejecuta en modo no interactivo, con la bandera `-p` o el Agent SDK; Claude Code espera la revisión e incluye los hallazgos en la respuesta, excepto para `ultra`, que [lanza la revisión en la nube sin esperar](#escalate-to-ultrareview)
* Establece [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/es/env-vars) en `1`, lo que también desactiva todas las demás características de tareas de fondo

<h3 id="let-claude-start-the-review">
  Dejar que Claude inicie la revisión
</h3>

Claude puede iniciar `/code-review` por su cuenta. Pídale que revise sus cambios en lenguaje natural y puede ejecutar la skill sin que escriba el comando, y una [tarea programada](/docs/es/scheduled-tasks) con `/code-review` como su indicación ejecuta la revisión.

Una tarea programada nunca lanza la [revisión en la nube](#escalate-to-ultrareview), por lo que programe `/code-review` sin el argumento `ultra`.

Para evitar que Claude y las tareas programadas inicien la revisión mientras mantiene `/code-review` disponible para que escriba, agregue una entrada [`skillOverrides`](/docs/es/skills#override-skill-visibility-from-settings) a un [archivo de configuración](/docs/es/settings#where-settings-live) como `~/.claude/settings.json`:

```json theme={null}
{
  "skillOverrides": {
    "code-review": "user-invocable-only"
  }
}
```

Antes de v2.1.246, Claude iniciaba `/code-review` por su cuenta solo donde una bandera de característica obtenida de Anthropic la activaba. En [sesiones que no obtienen banderas de características](/docs/es/env-vars#features-that-need-feature-flag-fetching), `/code-review` se ejecutaba solo cuando la escribía, y una `/code-review` programada llegaba a Claude como texto sin formato.

<h3 id="escalate-to-ultrareview">
  Escalar a ultrareview
</h3>

`/code-review ultra --fix` ejecuta la [ultrareview](/docs/es/ultrareview) más profunda en la nube, luego aplica sus hallazgos a su árbol de trabajo cuando regresan a su sesión.

Ultrareview utiliza su propio alcance: su rama actual contra la rama predeterminada del repositorio, más cambios sin confirmar y preparados en el árbol de trabajo. Para cambios sin confirmar en archivos nombrados como credenciales o claves, como archivos `.env` y `*.tfvars`, Claude Code sigue las reglas para [cargar un repositorio local en una sesión en la nube](/docs/es/claude-code-on-the-web#send-local-repositories-without-github). Pase un nombre de rama, como `/code-review ultra develop`, para comparar contra una base diferente.

Cuando el objetivo es una solicitud de extracción de `github.com`, puede hacer que Claude [publique los hallazgos terminados en el PR](/docs/es/ultrareview#post-findings-to-the-pull-request) como un comentario desde su cuenta de GitHub. Requiere Claude Code v2.1.227 o posterior.

<Note>
  Ultrareview requiere autenticación con una cuenta de claude.ai y no está disponible en Amazon Bedrock, Google Cloud's Agent Platform, o Microsoft Foundry, ni para organizaciones con Zero Data Retention habilitado. Cuando ultrareview no está disponible, `/code-review ultra` ejecuta una revisión local en su sesión en su lugar.
</Note>

Para iniciar una revisión en la nube desde un script o CI, ejecute `claude -p '/code-review ultra'`. Claude Code lanza la revisión e imprime un enlace para rastrearla. Requiere Claude Code v2.1.218 o posterior.

Cuando la revisión facturía [créditos de uso](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans), Claude Code se detiene antes de lanzar, porque la confirmación de facturación necesita una sesión interactiva. Ejecute el [subcomando `claude ultrareview`](/docs/es/ultrareview#run-ultrareview-non-interactively) en su lugar; al ejecutarlo, usted consiente el cargo.

El comando se llamaba `/simplify` antes de v2.1.147, cuando aplicaba correcciones de forma predeterminada. `/simplify` ejecuta una revisión separada de solo limpieza que aplica correcciones sin buscar errores. Si escribió scripts con `/simplify` para búsqueda de errores, cambie a `/code-review --fix`.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Commands](/docs/es/commands): ejecute `/code-review` en una sesión local de Claude Code para verificar un diff antes de hacer push
* [GitHub Actions](/docs/es/github-actions): ejecute Claude en sus propios flujos de trabajo de GitHub Actions para automatización personalizada más allá de la revisión de código
* [GitLab CI/CD](/docs/es/gitlab-ci-cd): integración de Claude autohospedada para canalizaciones de GitLab
* [Memory](/docs/es/memory): cómo funcionan los archivos `CLAUDE.md` en Claude Code
* [Analytics](/docs/es/analytics): rastrear el uso de Claude Code más allá de la revisión de código
* [How Anthropic secures its AI-native software development lifecycle](https://claude.com/blog/how-anthropic-secures-its-ai-native-software-development-lifecycle): cómo la revisión automatizada se ajusta como una capa del proceso de desarrollo seguro de Anthropic
