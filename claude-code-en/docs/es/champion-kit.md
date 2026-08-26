> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kit de campeón

> Un manual para ingenieros que defienden Claude Code internamente: qué compartir, cómo responder preguntas y cómo aumentar la adopción en su equipo.

Esta página es para ingenieros individuales que ya están usando Claude Code y quieren ayudar a su equipo a adoptarlo. Cubre qué compartir, cómo responder las preguntas que recibirá, un manual de treinta días y respuestas a preocupaciones comunes.

La adopción de una herramienta para desarrolladores rara vez ocurre debido a un anuncio de lanzamiento. Ocurre porque alguien en el equipo comienza a usar la herramienta bien, habla sobre ella abiertamente y facilita que otros la sigan. El trabajo que realiza como campeón tiene un efecto desproporcionado: cada ejemplo que comparte acorta la curva de aprendizaje para los ingenieros que vienen después de usted, y cada pregunta que responde en público convierte la experiencia de una persona en algo en lo que todo el equipo puede construir. Está actuando como un multiplicador para su equipo, no como un servicio de ayuda, y esta guía está estructurada para mantener el rol sostenible en esos términos.

<h2 id="the-champion-role">
  El rol de campeón
</h2>

El rol consiste en tres comportamientos que se refuerzan mutuamente.

| Comportamiento                           | Cómo se ve en la práctica                                                                                                                                                                                                 | Por qué importa                                                                                                                                                                                                           |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Comparta lo que descubre                 | Publique los prompts, capturas de pantalla y pequeñas victorias de su propio trabajo en los lugares donde su equipo ya lee, como un canal de ingeniería, un hilo de standup o una descripción de solicitud de extracción. | Los ejemplos extraídos de su propio código base son más persuasivos que cualquier documentación externa, porque los colegas pueden ver exactamente cómo se aplica la herramienta a los problemas que comparten con usted. |
| Sea la persona a quien la gente pregunta | Cuando un colega pregunta cómo logró algo, responda con el prompt real que utilizó para que puedan aplicarlo directamente a su propia tarea.                                                                              | Un ejemplo concreto y ejecutable elimina la brecha entre la curiosidad y un primer uso exitoso, que es donde se estancan la mayoría de los esfuerzos de adopción.                                                         |
| Amplíe el círculo                        | Establezca un pequeño número de hábitos recurrentes y ligeros, como un canal dedicado o un hilo semanal, para que el impulso continúe incluso cuando su atención esté en otro lugar.                                      | La adopción que depende de una sola persona es frágil. La adopción que es llevada por hábitos compartidos continúa componiéndose por sí sola.                                                                             |

La mayoría de esto se ajusta naturalmente dentro del trabajo que ya está realizando. La diferencia es una pequeña cantidad de intención adicional sobre dónde se publican sus descubrimientos y cómo viajan sus respuestas.

<h3 id="what-this-should-cost-you">
  Cuánto debería costarle esto
</h3>

Establezca expectativas con usted mismo y con su líder. Las actividades a continuación están diseñadas para caber dentro de una semana laboral normal, y el rol debe seguir siendo un multiplicador de su trabajo existente en lugar de una responsabilidad de soporte adicional.

| Actividad                                  | Tiempo por semana          | Orientación                                                                                                                              |
| ------------------------------------------ | -------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| Publicar victorias y prompts               | Aproximadamente 15 minutos | Capture estos en el momento con una captura de pantalla y una o dos oraciones; evite convertirlos en escritos formales.                  |
| Responder preguntas en un canal compartido | Aproximadamente 20 minutos | Responda públicamente una vez, luego vincule a esa respuesta cuando la pregunta se repita.                                               |
| Alojar un hilo semanal de demostración     | Aproximadamente 5 minutos  | Usted publica el prompt de apertura; el equipo proporciona el contenido.                                                                 |
| Emparejamiento opcional o tutoriales       | 0 a 30 minutos             | Reserve esto para colegas que estén genuinamente bloqueados, y ofrezca el enlace [Quickstart](/docs/es/quickstart) antes de programar tiempo. |

<h2 id="share-what-you-discover">
  Comparta lo que descubre
</h2>

Su propia experiencia es el material más persuasivo que sus colegas encontrarán, porque es específico de la base de código, los flujos de trabajo y los problemas que todos comparten. La documentación le dice a la gente qué es posible; sus publicaciones les muestran qué está funcionando realmente en su entorno.

<h3 id="what-is-worth-sharing">
  Qué vale la pena compartir
</h3>

Las publicaciones más útiles describen una técnica que un colega puede reutilizar mañana en lugar de un resultado que ya está completo. Las técnicas se componen a medida que se propagan a través de un equipo; las actualizaciones de estado no.

Ejemplos de técnicas reutilizables:

* "Aprendí que @-mencionar un directorio funciona. Lo señalé a `@src/components/` y pregunté cuáles carecían de pruebas, lo que reveló dos que había pasado por alto."
* "Plan mode (`Shift+Tab`) muestra exactamente qué archivos se tocarán antes de cualquier edición, por lo que estoy cómodo usándolo en código compartido."
* "Configuré un hook Stop para recibir una notificación de escritorio cuando se completa una tarea larga. La configuración está en el hilo."
* "Ejecutar `/init` genera un `CLAUDE.md` desde el repositorio para que el asistente deje de preguntar sobre nuestras convenciones."

<h3 id="where-to-share-it">
  Dónde compartirlo
</h3>

Publique donde su equipo ya lee. El objetivo es colocar ejemplos en el camino del trabajo normal en lugar de crear un destino.

| Ubicación                                       | Mejor para                                                             | Formato recomendado                                                                           |
| ----------------------------------------------- | ---------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| Un canal `#claude-code` o de ingeniería general | Descubrimientos, prompts y momentos "hoy aprendí"                      | Una captura de pantalla acompañada de una o dos oraciones de contexto                         |
| Descripciones de solicitudes de extracción      | Demostrar el enfoque en código real que los revisores ya están leyendo | Una sola línea como "Claude y yo hicimos esta refactorización; feliz de explicar el enfoque." |
| Standups o actualizaciones escritas semanales   | Normalizar el uso con líderes y gerentes de nivel superior             | Una oración describiendo un resultado concreto                                                |
| Wiki del equipo o documentación interna         | Patrones duraderos, skills personalizados y ejemplos de `CLAUDE.md`    | Una página corta, vinculada desde el tema del canal para que permanezca detectable            |

<h3 id="the-format-that-works">
  El formato que funciona
</h3>

Una captura de pantalla acompañada de una sola línea de contexto, o una breve descripción antes y después, es generalmente el nivel de detalle correcto. Mantenga cada publicación lo suficientemente corta para que alguien que se desplace rápidamente aún absorba el punto. Una escritura larga tiende a guardarse para más tarde y olvidarse, mientras que una publicación corta con una captura de pantalla tiende a copiarse e intentarse.

Las publicaciones de ejemplo a continuación ilustran el tono y la longitud; adáptelas en lugar de copiarlas textualmente.

```text theme={null}
Aprendí hoy que @-mencionar un directorio funciona. Lo señalé a
@src/components/ y pregunté qué componentes carecían de pruebas, y
reveló dos que había olvidado.
```

```text theme={null}
Configuré un hook Stop para recibir una notificación de escritorio cuando
se completa una tarea larga. Comencé una refactorización, me alejé y fui
notificado cuando terminó. La configuración está en el hilo.
```

```text theme={null}
Plan mode es la razón por la que estoy cómodo usando esto en código que
importa. Presione Shift+Tab hasta que vea "plan"; establece exactamente
qué archivos tiene la intención de tocar antes de cambiar nada.
```

<h2 id="be-the-person-people-ask">
  Sea la persona a quien la gente pregunta
</h2>

Una vez que haya compartido algunos ejemplos, las preguntas seguirán. Aquí es donde el rol de campeón tiene el mayor apalancamiento, porque una buena respuesta a una persona frecuentemente desbloquea a varios otros que están viendo el mismo canal.

<h3 id="answer-with-a-prompt-rather-than-an-explanation">
  Responda con un prompt en lugar de una explicación
</h3>

Cuando un colega pregunta cómo logró algo, la respuesta más útil es el prompt que realmente utilizó. Aprenderán más ejecutando ese prompt contra su propio problema que de cualquier descripción que pudiera escribir, y les da algo en lo que pueden actuar inmediatamente.

```text theme={null}
Colega: ¿Cómo lograste encontrar esa condición de carrera?

Campeón: Pregunté, "La prueba en @tests/scheduler.test.ts es inestable,
descubre por qué," y rastreó dos promesas sin unir en el programador.
Intenta la misma redacción en tu prueba.
```

<h3 id="point-at-the-feature-rather-than-the-documentation">
  Señale la característica en lugar de la documentación
</h3>

Una respuesta como "Intenta plan mode, presiona `Shift+Tab` hasta que lo veas" es más útil en el momento que un enlace a la documentación. Si la persona necesita más profundidad más tarde, la encontrará por su cuenta; ahora necesita la única cosa que los desbloquea.

<h3 id="questions-you-are-likely-to-hear">
  Preguntas que probablemente escuchará
</h3>

| Pregunta                                               | Respuesta sugerida                                                                                                                                                                                                                                 | Recurso de seguimiento                                  |
| ------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| "¿En qué debería intentarlo primero?"                  | Recomiende una tarea real pero contenida, idealmente un error o tarea que la persona ha estado posponiendo porque es tediosa en lugar de difícil.                                                                                                  | [Common workflows](/docs/es/common-workflows)                |
| "¿Cómo confío en que toque mi código?"                 | Introduzca plan mode: presionar `Shift+Tab` lo cicla, Claude propone exactamente qué tiene la intención de cambiar, y nada se modifica hasta que el usuario aprueba.                                                                               | [Permissions](/docs/es/permissions)                          |
| "¿Vale la pena el esfuerzo de configuración?"          | La instalación toma aproximadamente dos minutos, se ejecuta en la terminal y no requiere extensión IDE. Ejecutar `/init` una vez es suficiente para comenzar a trabajar.                                                                           | [Quickstart](/docs/es/quickstart)                            |
| "Produjo un resultado incorrecto."                     | Anímelos a proporcionar el fallo a Claude. Pegar el mensaje de error o la prueba fallida es mucho más efectivo que reformular la solicitud original.                                                                                               | [Common workflows](/docs/es/common-workflows)                |
| "No entiende las convenciones de nuestro código base." | Sugiera ejecutar `/init` para generar un archivo `CLAUDE.md`, luego agregue las convenciones del equipo, comandos de prueba y cualquier directorio que deba evitarse.                                                                              | [Memory](/docs/es/memory)                                    |
| "¿Es esto solo autocompletado?"                        | Ofrezca una breve demostración en la que Claude explique un archivo desconocido, rastree un error entre servicios o redacte un plan de migración. Estas tareas requieren razonamiento en todo el repositorio en lugar de completar una sola línea. | Una demostración en vivo de dos minutos                 |
| "¿Qué hay sobre seguridad y manejo de datos?"          | Remita esta pregunta a su administrador. La política de implementación y manejo de datos de su organización ya está configurada, y los campeones no deben improvisar esta respuesta.                                                               | [Security](/docs/es/security) · [Data usage](/docs/es/data-usage) |

<h2 id="grow-the-circle">
  Amplíe el círculo
</h2>

El objetivo no es construir un programa o poseer un lanzamiento. Es establecer un pequeño número de hábitos ligeros que permitan que el impulso continúe después de que haya dejado de impulsarlo activamente. Cuando las preguntas en el canal están siendo respondidas por personas que no sean usted, el rol ha cumplido su función.

<h3 id="patterns-that-tend-to-work">
  Patrones que tienden a funcionar
</h3>

| Patrón                                               | Cómo ejecutarlo                                                                                                                                                                                                                                                                                 | Esfuerzo requerido                                             |
| ---------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| Un canal dedicado                                    | Cree un canal `#claude-code` (o un hilo recurrente en uno existente), fije el enlace [Quickstart](/docs/es/quickstart) y un ejemplo fuerte, y responda preguntas públicamente para que cada respuesta beneficie a todos los que están viendo.                                                        | Aproximadamente cinco minutos para configurar, luego ambiental |
| Un hilo semanal de demostración                      | Cada viernes, publique "¿Con qué te ayudó Claude esta semana?" No se requiere preparación, diapositivas o reunión; capturas de pantalla y descripciones cortas son suficientes.                                                                                                                 | Aproximadamente dos minutos por semana                         |
| Comparta un skill personalizado                      | Publique su archivo `.claude/skills/<name>/SKILL.md` más útil, por ejemplo un skill `/ship` que ejecuta pruebas y lint antes de confirmar, con una descripción de una línea. Debido a que los skills son Markdown simple, los colegas pueden adoptarlos inmediatamente.                         | Aproximadamente cinco minutos por skill                        |
| Genere una guía de configuración desde su propio uso | Ejecute `/team-onboarding` en un proyecto en el que haya pasado tiempo real. Claude escanea sus sesiones recientes, comandos y servidores MCP, luego produce una guía que un nuevo compañero de equipo puede pegar como su primer mensaje para reproducir su configuración. Fíjela en el canal. | Aproximadamente dos minutos                                    |
| Emparéjese en una primera tarea                      | Ofrezca una única sesión de emparejamiento de quince minutos a cualquiera que esté comenzando. Un resultado exitoso en su propio código es más persuasivo que cualquier presentación.                                                                                                           | Aproximadamente quince minutos por persona                     |
| Identifique el próximo campeón                       | El colega que le hace más preguntas generalmente está listo para asumir este rol. Reenvíele esta página y divida las responsabilidades del canal entre ustedes.                                                                                                                                 | Negligible                                                     |

<h3 id="thirty-day-playbook">
  Manual de treinta días
</h3>

Si un plan flexible es útil, la secuencia a continuación refleja lo que tiende a funcionar en la mayoría de los equipos. Ajuste libremente para adaptarse a su contexto.

<Steps>
  <Step title="Semana 1: Sembrar el canal">
    Cree el canal, fije el [Quickstart](/docs/es/quickstart) y publique dos o tres de sus propios ejemplos con los prompts incluidos.

    **Señal de que está funcionando:** algunos colegas reaccionan o responden, y al menos una pregunta se hace en el canal.
  </Step>

  <Step title="Semana 2: Comience el ritmo">
    Comience el hilo semanal de demostración, responda cada pregunta públicamente y comparta un skill personalizado o un fragmento de `CLAUDE.md`.

    **Señal de que está funcionando:** alguien que no sea usted publica un ejemplo de su propio trabajo.
  </Step>

  <Step title="Semana 3: Emparéjese y consolide">
    Ofrezca dos o tres sesiones cortas de emparejamiento y consolide las preguntas y respuestas más comunes en un mensaje de preguntas frecuentes fijado.

    **Señal de que está funcionando:** ve uso repetido, con los mismos colegas regresando en lugar de intentar una vez y detenerse.
  </Step>

  <Step title="Semana 4: Entregue">
    Identifique un segundo campeón y comparta un breve resumen de qué está funcionando y qué no con su líder o administrador.

    **Señal de que está funcionando:** las preguntas en el canal están siendo respondidas por personas que no sean usted.
  </Step>
</Steps>

<h3 id="when-someone-wants-to-go-deeper">
  Cuando alguien quiere profundizar
</h3>

Usted es la introducción cálida en lugar del programa de incorporación. Cuando un colega se mueve más allá de "¿debería intentar esto?" hacia "¿cómo me vuelvo efectivo con esto?", señálelos a las páginas [Quickstart](/docs/es/quickstart) y [Common workflows](/docs/es/common-workflows). Contienen secciones cortas que cubren las características que son genuinamente útiles pero difíciles de descubrir por su cuenta.

<h2 id="respond-to-common-concerns">
  Responda a preocupaciones comunes
</h2>

El escepticismo saludable es esperado; los ingenieros deben ser cautelosos con las herramientas que tocan su código. La respuesta más efectiva rara vez es argumentar el caso general. En su lugar, reconozca la preocupación, ofrezca un breve replanteamiento y proponga una demostración concreta en el código de la persona. La mayoría de las preocupaciones se resuelven con una sola experiencia exitosa.

| Preocupación                                            | Respuesta sugerida                                                                                                                                                                                                                            | Evidencia a ofrecer                                                |
| ------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| "Soy más rápido sin esto."                              | Eso probablemente es cierto para el código que la persona escribe rutinariamente. Sugiera intentarlo en el trabajo que tienden a evitar: archivos heredados, servicios desconocidos o andamiaje de pruebas, donde el apalancamiento es mayor. | Cronometrar una tarea tediosa de ambas formas y comparar.          |
| "No confío en que la IA toque el código de producción." | Acepte que ningún cambio debe aterrizar sin ser leído. Plan mode combinado con revisión de diff normal significa que nada se aplica que el ingeniero no haya inspeccionado, el mismo estándar que cualquier solicitud de extracción.          | Demuestre plan mode en un archivo real.                            |
| "Hará que los ingenieros junior sean más débiles."      | Usado bien, es un explicador efectivo. Anime a los ingenieros junior a pedirle a Claude que explique un archivo y sus sitios de llamada antes de pedirle que cambie nada.                                                                     | Ejecute "Explicar @file y dónde se llama desde" juntos.            |
| "Lo intenté una vez y alucinó."                         | Esto suele ser un problema de contexto en lugar de un problema de modelo. @-mencionar los archivos relevantes, ejecutar `/init` y proporcionar la salida de error real generalmente lo resuelve.                                              | Vuelva a ejecutar su prompt original con el contexto `@` adecuado. |
| "No tenemos tiempo para aprender otra herramienta."     | Claude Code es un comando de terminal en lugar de una plataforma. Si no devuelve valor dentro de la primera sesión, es razonable dejarlo de lado.                                                                                             | Una instalación de dos minutos seguida de un error real.           |

<h2 id="quick-reference-sheet">
  Hoja de referencia rápida
</h2>

Las técnicas a continuación son las que más confiablemente mueven a alguien de una primera prueba al uso diario. Fije esta tabla en un canal o compártala por su cuenta.

| Técnica                                    | Cómo aplicarla                                                                                                                                                                    |
| ------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Proporcione el contexto correcto           | Use referencias `@file` o `@directory/`, o pegue la salida de error o registro directamente. Proporcionar contexto relevante es más efectivo que un prompt elaborado.             |
| Revise el plan antes de la edición         | Presione `Shift+Tab` para entrar en plan mode. Claude describirá los cambios previstos para su aprobación antes de ejecutarlos.                                                   |
| Enseñe su repositorio                      | Ejecute `/init` para generar un archivo `CLAUDE.md`, luego agregue sus convenciones, comandos de prueba y cualquier directorio que no deba modificarse. Ver [Memory](/docs/es/memory). |
| Reutilice un flujo de trabajo              | Guarde un archivo `SKILL.md` en `.claude/skills/<name>/` para crear un skill `/name` que todo el equipo pueda usar. Ver [Skills](/docs/es/skills).                                     |
| Manténgase informado durante tareas largas | Configure un hook Stop para recibir una notificación de escritorio cuando se completa una tarea de larga duración. Ver [Hooks](/docs/es/hooks-guide).                                  |
| Recuperarse de un resultado incorrecto     | En lugar de reformular la solicitud, pegue la prueba fallida o el seguimiento de pila a Claude y pídale que aborde esa falla específica.                                          |
| Mantenga las ediciones quirúrgicas         | Pida un diff, o especifique "solo cambie X." Claude respeta el alcance cuando el alcance se indica.                                                                               |

<Tip>
  Claude Code se actualiza frecuentemente. Verifique los detalles específicos de la versión contra la [página de inicio de documentación](/docs/es/overview) antes de distribuir este material internamente.
</Tip>
