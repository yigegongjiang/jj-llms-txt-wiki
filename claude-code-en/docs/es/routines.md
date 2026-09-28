> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Automatizar el trabajo con rutinas

> Ponga Claude Code en piloto automático. Defina rutinas que se ejecuten en un horario, se activen en llamadas API o reaccionen a eventos de GitHub desde la infraestructura en la nube.

<Note>
  Las rutinas están en vista previa de investigación. El comportamiento, los límites y la superficie de la API pueden cambiar.
</Note>

Una rutina es una configuración guardada de Claude Code: un prompt, uno o más repositorios y un conjunto de [conectores](/docs/es/mcp), empaquetados una vez y ejecutados automáticamente. Las rutinas se ejecutan en la infraestructura en la nube administrada por Anthropic, o en el [entorno autohospedado](/docs/es/self-hosted-environments) de su organización cuando se enrutan allí, por lo que siguen funcionando cuando su portátil está cerrado.

Cada rutina puede tener uno o más disparadores adjuntos:

* **Programada**: se ejecuta en una cadencia recurrente como cada hora, cada noche o semanalmente, o una sola vez en un momento futuro específico
* **API**: se activa bajo demanda enviando un POST HTTP a un punto final por rutina con un token de portador
* **GitHub**: se ejecuta automáticamente en respuesta a eventos del repositorio como solicitudes de extracción o lanzamientos

Una única rutina puede combinar disparadores. Por ejemplo, una rutina de revisión de PR puede ejecutarse cada noche, activarse desde un script de implementación y también reaccionar a cada nuevo PR.

Las rutinas están disponibles en planes Pro, Max, Team y Enterprise. Créelas y adminístrelas en [claude.ai/code/routines](https://claude.ai/code/routines), o desde la CLI con `/schedule`.

Los propietarios de Team y Enterprise pueden desactivar las rutinas para todos los miembros con el botón de alternancia Routines en [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). Cuando se desactivan, las rutinas existentes dejan de ejecutarse y los miembros no pueden crear nuevas.

Esta página cubre la creación de una rutina, la configuración de cada tipo de disparador, la administración de ejecuciones y cómo se aplican los límites de uso.

<h2 id="example-use-cases">
  Casos de uso de ejemplo
</h2>

Cada ejemplo empareja un tipo de disparador con el tipo de trabajo para el que las rutinas son adecuadas: desatendido, repetible y vinculado a un resultado claro.

**Mantenimiento del trabajo pendiente.** Un disparador de horario se ejecuta cada noche de la semana contra su rastreador de problemas a través de un conector. La rutina lee los problemas abiertos desde la última ejecución, aplica etiquetas, asigna propietarios según el área de código referenciada y publica un resumen en Slack para que el equipo comience el día con una cola organizada.

**Triaje de alertas.** Su herramienta de monitoreo llama al punto final de la API de la rutina cuando se cruza un umbral de error, pasando el cuerpo de la alerta como `text`. La rutina extrae el seguimiento de pila, lo correlaciona con commits recientes en el repositorio y abre una solicitud de extracción en borrador con una corrección propuesta y un enlace de vuelta a la alerta. El personal de guardia revisa el PR en lugar de comenzar desde una terminal en blanco.

**Revisión de código personalizada.** Un disparador de GitHub se ejecuta en `pull_request.opened`. La rutina aplica la lista de verificación de revisión de su equipo, deja comentarios en línea para problemas de seguridad, rendimiento y estilo, y agrega un comentario de resumen para que los revisores humanos puedan enfocarse en el diseño en lugar de verificaciones mecánicas.

**Verificación de implementación.** Su canalización de CD llama al punto final de la API de la rutina después de cada implementación en producción. La rutina ejecuta verificaciones de humo contra la nueva compilación, escanea registros de errores en busca de regresiones y publica un sí o no al canal de lanzamiento antes de que se cierre la ventana de implementación.

**Desfase de documentación.** Un disparador de horario se ejecuta semanalmente. La rutina escanea los PR fusionados desde la última ejecución, marca la documentación que hace referencia a API modificadas y abre PR de actualización contra el repositorio de documentación para que un editor revise.

**Puerto de biblioteca.** Un disparador de GitHub se ejecuta en `pull_request.closed` filtrado a PR fusionados en un repositorio de SDK. La rutina porta el cambio a un SDK paralelo en otro idioma y abre un PR coincidente, manteniendo las dos bibliotecas sincronizadas sin que un humano reimplemente cada cambio.

<h2 id="create-a-routine">
  Crear una rutina
</h2>

Cree una rutina desde la web en [claude.ai/code/routines](https://claude.ai/code/routines), desde la aplicación de escritorio o desde la CLI. Las tres superficies escriben en la misma cuenta en la nube, por lo que una rutina que cree en una aparece en las otras inmediatamente. En la aplicación de escritorio, en la pestaña **Code**, haga clic en **Routines** en la barra lateral o en el menú **More** de la barra lateral, luego en **New routine**, y elija **Cloud**; elegir **Local** en su lugar crea una [tarea programada de escritorio](/docs/es/desktop-scheduled-tasks), que se ejecuta en su máquina en lugar de en la nube.

El formulario de creación configura el prompt de la rutina, repositorios, entorno, conectores y disparadores.

Las rutinas se ejecutan de forma autónoma como sesiones completas de Claude Code en la nube: no hay selector de modo de permiso, y la sesión ejecuta comandos de shell, usa [skills](/docs/es/skills) comprometidas con el repositorio clonado y llama a cualquier conector que incluya, todo sin detenerse para obtener aprobación aparte de algunas acciones de [artefactos](/docs/es/artifacts).

Lo que una rutina puede alcanzar está determinado por los repositorios que seleccione, el [acceso a la red del entorno](/docs/es/cloud-environments) y variables, y los conectores que incluya. Delimite cada uno de esos a lo que la rutina realmente necesita.

Cuando el cronograma de la rutina o **Run now** inicia una ejecución, Claude republica un artefacto existente sin preguntar solo cuando se cumplen todas estas condiciones:

* Puede editar el artefacto y pertenece a su propia organización
* El artefacto no se comparte públicamente, y no se comparte con personas específicas u su organización con la última versión elegida como la versión que ven los espectadores
* La publicación lleva solo la página, sin archivos de apoyo ni nada más agregado, y no fuerza una versión más nueva
* La página no contiene una concesión que vaya más allá de la página, como [llamadas de conectores](/docs/es/artifacts#pull-live-data-with-mcp-connectors)

En todos los demás casos, incluida la publicación de un nuevo artefacto, Claude pregunta primero. Cuando el trabajo de una rutina es mantener una página actual, déle un artefacto que ya haya publicado.

Las rutinas pertenecen a su cuenta individual de claude.ai. No se comparten con compañeros de equipo y cuentan contra la asignación diaria de ejecuciones de su cuenta. Cualquier cosa que una rutina haga a través de su identidad de GitHub conectada o conectores aparece como usted: los commits y las solicitudes de extracción llevan su usuario de GitHub, y los mensajes de Slack, tickets de Linear u otras acciones de conectores utilizan sus cuentas vinculadas para esos servicios.

<h3 id="create-from-the-web">
  Crear desde la web
</h3>

<Steps>
  <Step title="Abrir el formulario de creación">
    Visite [claude.ai/code/routines](https://claude.ai/code/routines) y haga clic en **New routine**.
  </Step>

  <Step title="Nombrar la rutina y escribir el prompt">
    Dé a la rutina un nombre descriptivo y escriba el prompt que Claude ejecuta cada vez. El prompt es la parte más importante: la rutina se ejecuta de forma autónoma, por lo que el prompt debe ser autónomo y explícito sobre qué hacer y qué significa el éxito.

    Cuando se activa un disparador, la sesión recibe el prompt guardado de la rutina como su tarea asignada y lo ejecuta, en lugar de tratarlo como contenido no confiable que llegó a mitad de la conversación. El disparador solo atestigua que el prompt fue almacenado con anticipación por una sesión autorizada en su cuenta, por lo que el prompt activado no es entrada de usuario en vivo y no puede actuar como aprobación o consentimiento para acciones durante la ejecución. El contenido que la sesión obtiene durante la ejecución mantiene su manejo normal. Antes de v2.1.213, la sesión recibía el mismo prompt enmarcado como una notificación de fondo no confiable y podía negarse a actuar sobre él.

    La entrada del prompt incluye un selector de modelo. Claude utiliza el modelo seleccionado en cada ejecución.
  </Step>

  <Step title="Seleccionar repositorios">
    Agregue uno o más repositorios de GitHub para que Claude trabaje. Cada repositorio se clona al inicio de una ejecución, comenzando desde la rama predeterminada. Claude crea ramas con prefijo `claude/` para sus cambios.
  </Step>

  <Step title="Seleccionar un entorno">
    Elija un [entorno en la nube](/docs/es/cloud-environments) para la rutina. Los entornos controlan a qué tiene acceso la sesión en la nube:

    * **Network access**: establezca el nivel de acceso a Internet disponible durante cada ejecución
    * **Environment variables**: proporcione valores que Claude pueda usar durante cada ejecución. Son [visibles para cualquiera que use el entorno](/docs/es/cloud-environments#what-carries-over-from-your-setup), por lo que en planes Pro y Max, almacene claves para las API que Claude llama durante una ejecución como [credenciales de API](/docs/es/cloud-environments#add-api-credentials) en su lugar. Esa sección también enumera las solicitudes que nunca obtienen una credencial
    * **Setup script**: instale dependencias y herramientas que la rutina necesita. El resultado se [almacena en caché](/docs/es/cloud-environments#environment-caching), por lo que el script no se vuelve a ejecutar en cada sesión

    Se proporciona un entorno **Default** con acceso a la red **Trusted**, que permite solo el [conjunto predeterminado](/docs/es/cloud-environments#default-allowed-domains) de registros de paquetes, API de proveedores de nube, registros de contenedores y dominios de desarrollo comunes a través de la red de la sesión. Los conectores que agregue a la rutina alcanzan sus servicios a través de los servidores de Anthropic, por lo que no necesitan cambios en la lista de permitidos. Si su rutina necesita alcanzar sus propios servicios directamente o un dominio fuera de esa lista, edite el [acceso a la red](/docs/es/cloud-environments#network-access) del entorno antes de ejecutar. Para usar un entorno separado, [cree uno](/docs/es/cloud-environments#configure-your-environment) primero.
  </Step>

  <Step title="Seleccionar un disparador">
    En **Select a trigger**, elija cómo comienza la rutina. Puede elegir un tipo de disparador o combinar varios.

    <Tabs>
      <Tab title="Schedule">
        Elija una frecuencia preestablecida para una ejecución recurrente, o programe una ejecución única en una marca de tiempo específica. Consulte [Add a schedule trigger](#add-a-schedule-trigger) para el manejo de zonas horarias, escalonamiento, intervalos cron personalizados y ejecuciones únicas.
      </Tab>

      <Tab title="GitHub event">
        Seleccione el repositorio, el evento al que reaccionar y filtros opcionales. Consulte [Add a GitHub trigger](#add-a-github-trigger) para la lista completa de eventos admitidos y campos de filtro.
      </Tab>

      <Tab title="API">
        Seleccione **API** aquí, luego guarde la rutina. La URL y el token se generan después de que se guarda la rutina, ya que dependen del ID de la rutina. Consulte [Add an API trigger](#add-an-api-trigger) para copiar la URL y generar un token.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Revisar conectores">
    En **Connectors** en la parte inferior del formulario, todos sus [conectores MCP](/docs/es/mcp) conectados se incluyen de forma predeterminada. Elimine cualquiera que la rutina no necesite: Claude puede usar todas las herramientas de un conector incluido, incluidas las escrituras, sin pedir permiso durante una ejecución.
  </Step>

  <Step title="Crear la rutina">
    Haga clic en **Create**. La rutina aparece en la lista y se ejecuta la próxima vez que uno de sus disparadores coincida. Para iniciar una ejecución inmediatamente, haga clic en **Run now** en la página de detalles de la rutina.

    Cada ejecución crea una nueva sesión junto con sus otras sesiones, donde puede ver qué hizo Claude, revisar cambios y crear una solicitud de extracción.
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  Crear desde la CLI
</h3>

Ejecute `/schedule` en cualquier sesión para crear una rutina programada conversacionalmente. También puede pasar una descripción directamente, para una rutina recurrente como `/schedule daily PR review at 9am` o una única como `/schedule clean up feature flag in one week`. Claude recorre la misma información que recopila el formulario web, luego guarda la rutina en su cuenta. El comando también está disponible bajo el alias `/routines`.

Una ejecución exitosa se parece a una conversación: Claude hace preguntas de seguimiento sobre el cronograma, repositorios y prompt antes de guardar. Si Claude en su lugar responde que necesita autenticarse o que no puede conectarse a su cuenta remota de claude.ai, no se creó ninguna rutina; consulte [Troubleshooting](#troubleshooting).

`/schedule` en la CLI crea rutinas programadas. Para agregar un disparador de API, edite la rutina en la web en [claude.ai/code/routines](https://claude.ai/code/routines). Puede agregar un [disparador de GitHub](#add-a-github-trigger) desde la web o desde la CLI. La ruta de CLI requiere Claude Code v2.1.225 o posterior.

Una rutina sin disparador de cronograma, como una iniciada solo por llamadas de API o eventos de GitHub, no tiene tiempo de próxima ejecución, y la CLI no muestra ninguno cuando Claude la guarda o actualiza. Antes de v2.1.211, la CLI reportaba un tiempo de próxima ejecución en el año 1 para estas rutinas.

<h2 id="configure-triggers">
  Configurar disparadores
</h2>

Una rutina comienza cuando uno de sus disparadores coincide. Puede adjuntar cualquier combinación de disparadores de horario, API y GitHub a la misma rutina, y agregarlos o quitarlos en cualquier momento desde la sección **Select a trigger** del formulario de edición de la rutina.

<h3 id="add-a-schedule-trigger">
  Agregar un disparador de horario
</h3>

Un disparador de horario ejecuta la rutina en una cadencia recurrente, o una sola vez en un momento futuro específico. Elija una frecuencia preestablecida en la sección **Select a trigger**: cada hora, diaria, días de semana o semanal. Los tiempos se ingresan en su zona local y se convierten automáticamente, por lo que la rutina se ejecuta a esa hora de reloj de pared independientemente de dónde se encuentre la infraestructura en la nube.

Las ejecuciones pueden comenzar unos minutos después de la hora programada debido al escalonamiento. El desplazamiento es consistente para cada rutina.

Para un intervalo personalizado como cada dos horas o el primero de cada mes, elija el preestablecido más cercano en el formulario, luego ejecute `/schedule update` en la CLI para establecer una expresión cron específica. El intervalo mínimo es una hora; las expresiones que se ejecutan con más frecuencia se rechazan.

<h4 id="schedule-a-one-off-run">
  Programar una ejecución única
</h4>

Una programación única dispara la rutina una sola vez en una marca de tiempo específica. Úsela para recordarse más adelante en la semana, para abrir un PR de limpieza después de que finalice un despliegue, o para iniciar una tarea de seguimiento cuando llega un cambio ascendente. Después de que se dispara la rutina, se desactiva automáticamente y la interfaz de usuario web la marca como **Ran**. Para ejecutarla nuevamente, edite la rutina y establezca una nueva hora única.

Cree una ejecución única desde la CLI describiendo la hora en lenguaje natural. Claude resuelve la frase contra la hora actual y confirma la marca de tiempo absoluta antes de guardar.

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

La misma conversión de zona local a UTC que las programaciones recurrentes se aplica a las marcas de tiempo únicas.

Las ejecuciones únicas no cuentan contra el límite diario de ejecuciones de rutina. Consulte [Uso y límites](#usage-and-limits) para obtener detalles.

<h3 id="add-an-api-trigger">
  Agregar un disparador de API
</h3>

Un disparador de API proporciona a una rutina un punto final HTTP dedicado. POSTear al punto final con el token de portador de la rutina inicia una nueva sesión y devuelve una URL de sesión. Úselo para conectar Claude Code en sistemas de alertas, canalizaciones de implementación, herramientas internas o en cualquier lugar donde pueda hacer una solicitud HTTP autenticada.

Los disparadores de API se agregan a una rutina existente desde la web. La CLI actualmente no puede crear ni revocar tokens.

<Steps>
  <Step title="Abrir la rutina para editar">
    Vaya a [claude.ai/code/routines](https://claude.ai/code/routines), haga clic en la rutina que desea activar a través de API, luego abra el menú junto al nombre de la rutina y seleccione **Edit**.
  </Step>

  <Step title="Agregar un disparador de API">
    Desplácese hasta la sección **Select a trigger** debajo del cuadro **Instructions**, haga clic en **Add another trigger** y elija **API**.
  </Step>

  <Step title="Copiar la URL y generar un token">
    El modal muestra la URL para esta rutina junto con un comando curl de ejemplo. Copie la URL, luego haga clic en **Generate token** y copie el token inmediatamente. El token se muestra una vez y no se puede recuperar más tarde, así que guárdelo en un lugar seguro como el almacén de secretos de su herramienta de alertas.
  </Step>

  <Step title="Llamar al punto final">
    Envíe el token en el encabezado `Authorization: Bearer` cuando POST a la URL. La sección [Activar una rutina](#trigger-a-routine) a continuación muestra un ejemplo completo.
  </Step>
</Steps>

Cada rutina tiene su propio token, limitado a activar solo esa rutina. Para rotarlo o revocarlo, vuelva al mismo modal y haga clic en **Regenerate** o **Revoke**.

<h4 id="trigger-a-routine">
  Activar una rutina
</h4>

Envíe una solicitud POST al punto final `/fire` con el token de portador en el encabezado `Authorization`. El cuerpo de la solicitud acepta un campo `text` opcional para contexto específico de la ejecución, como un cuerpo de alerta o un registro fallido, pasado a la rutina junto con su prompt guardado. El valor es texto de forma libre y no se analiza: si envía JSON u otra carga útil estructurada, la rutina la recibe como una cadena literal.

El valor `text` no llega a la rutina como un mensaje desnudo. Llega envuelto en un bloque `<routine-fire-payload>` que lo etiqueta como datos no confiables y le dice a Claude que no siga instrucciones dentro de él a menos que el prompt de la rutina lo diga. El mismo envolvimiento se aplica al texto suministrado con **Run now** en la interfaz de usuario web.

Esto significa que el prompt guardado de una rutina debe optar por actuar sobre el texto de fuego: escriba el prompt para hacer referencia explícita a la carga útil, por ejemplo "Investigue la alerta descrita en el bloque routine-fire-payload", o la rutina trata el texto como contexto inerte. Cualquiera que tenga el token de portador puede enviar `text`, por lo que el envolvimiento hace que el texto de fuego de un token filtrado llegue etiquetado como datos no confiables en lugar de como instrucciones directas a su rutina.

El ejemplo a continuación activa una rutina desde un shell. El ID de rutina y el token mostrados son marcadores de posición: reemplácelos con la URL y el token que copió al [agregar el disparador de API](#add-an-api-trigger), o la solicitud falla con un error de autenticación `401`:

```bash theme={null}
curl -X POST https://api.anthropic.com/v1/claude_code/routines/trig_01ABCDEFGHJKLMNOPQRSTUVW/fire \
  -H "Authorization: Bearer sk-ant-oat01-xxxxx" \
  -H "anthropic-beta: experimental-cc-routine-2026-04-01" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"text": "Sentry alert SEN-4521 fired in prod. Stack trace attached."}'
```

Una solicitud exitosa devuelve un cuerpo JSON con el nuevo ID de sesión y URL:

```json theme={null}
{
  "type": "routine_fire",
  "claude_code_session_id": "session_01HJKLMNOPQRSTUVWXYZ",
  "claude_code_session_url": "https://claude.ai/code/session_01HJKLMNOPQRSTUVWXYZ"
}
```

Abra la URL de sesión en un navegador para ver la ejecución en tiempo real, revisar cambios o continuar la conversación manualmente.

<Warning>
  El punto final `/fire` se envía bajo el encabezado beta `experimental-cc-routine-2026-04-01`. Las formas de solicitud y respuesta, los límites de velocidad y la semántica de tokens pueden cambiar mientras la característica está en vista previa de investigación. Los cambios importantes se envían detrás de nuevas versiones de encabezado beta con fecha, y las dos versiones de encabezado anteriores más recientes continúan funcionando para que los llamadores tengan tiempo de migrar.
</Warning>

<h4 id="api-reference">
  Referencia de API
</h4>

Para la referencia completa de la API, incluidas todas las respuestas de error, reglas de validación y límites de campo, consulte [Activar una rutina a través de API](https://platform.claude.com/docs/en/api/claude-code/routines-fire) en la documentación de la plataforma Claude.

El punto final `/fire` está disponible solo para usuarios de claude.ai y no es parte de la superficie de la API de Claude Platform.

<h3 id="add-a-github-trigger">
  Agregar un disparador de GitHub
</h3>

Un disparador de GitHub inicia una nueva sesión automáticamente cuando ocurre un evento coincidente en un repositorio conectado. Claude Code no reutiliza sesiones entre eventos, por lo que dos actualizaciones de PR producen dos sesiones independientes.

<Note>
  Durante la vista previa de investigación, los eventos de webhook de GitHub están sujetos a límites por hora por rutina y por cuenta. Los eventos más allá del límite se descartan hasta que se reinicia la ventana. Vea sus límites actuales en [claude.ai/code/routines](https://claude.ai/code/routines).
</Note>

La aplicación Claude GitHub debe estar instalada en el repositorio al que desea suscribirse, sea cual sea la superficie desde la que configure el disparador.

* Configure disparadores de GitHub desde la interfaz de usuario web, que le solicita que instale la aplicación cuando falta. Siga los pasos a continuación para configurar uno en la web.
* Desde la CLI, instale la aplicación desde la [página de la aplicación GitHub](https://github.com/apps/claude) primero, luego pida a Claude que adjunte un disparador de GitHub a una rutina existente, por ejemplo `/schedule add a GitHub trigger to my nightly review for pull requests opened in acme/webapp`. La ruta de CLI requiere Claude Code v2.1.225 o posterior. Cuando Claude agrega el disparador, responde con un enlace a la rutina que dispara el disparador.

<Steps>
  <Step title="Abrir la rutina para editar">
    Vaya a [claude.ai/code/routines](https://claude.ai/code/routines), haga clic en la rutina, luego abra el menú junto al nombre de la rutina y seleccione **Edit**.
  </Step>

  <Step title="Agregar un disparador de evento de GitHub">
    Desplácese hasta la sección **Select a trigger**, haga clic en **Add another trigger** y elija **GitHub event**.

    <Note>
      Ejecutar `/web-setup` en la CLI otorga acceso al repositorio para clonar, pero no instala la aplicación Claude GitHub y no habilita la entrega de webhook.
    </Note>
  </Step>

  <Step title="Configurar el disparador">
    Seleccione el repositorio, elija un evento de la lista de [eventos admitidos](#supported-events) y opcionalmente agregue filtros. Guarde el disparador.
  </Step>
</Steps>

<h4 id="supported-events">
  Eventos admitidos
</h4>

Los disparadores de GitHub pueden suscribirse a cualquiera de las siguientes categorías de eventos. Dentro de cada categoría, puede elegir una acción específica, como `pull_request.opened`, o reaccionar a todas las acciones en la categoría.

| Evento                  | Se activa cuando                                                               |
| :---------------------- | :----------------------------------------------------------------------------- |
| Solicitud de extracción | Se abre, cierra, asigna, etiqueta, sincroniza o actualiza de otra manera un PR |
| Lanzamiento             | Se crea, publica, edita o elimina un lanzamiento                               |

<h4 id="filter-pull-requests">
  Filtrar solicitudes de extracción
</h4>

Use filtros para reducir qué solicitudes de extracción inician una nueva sesión. Todas las condiciones de filtro deben coincidir para que la rutina se active. Los campos de filtro disponibles son:

| Filtro         | Coincide                                     |
| :------------- | :------------------------------------------- |
| Autor          | Nombre de usuario de GitHub del autor del PR |
| Título         | Texto del título del PR                      |
| Cuerpo         | Texto de descripción del PR                  |
| Rama base      | Rama a la que se dirige el PR                |
| Rama principal | Rama de la que proviene el PR                |
| Etiquetas      | Etiquetas aplicadas al PR                    |
| Es borrador    | Si el PR está en estado de borrador          |
| Está fusionado | Si el PR ha sido fusionado                   |

Cada filtro empareja un campo con un operador: es igual a, contiene, comienza con, es uno de, no es uno de o coincide con regex.

El operador `matches regex` prueba el valor de campo completo, no una subcadena dentro de él. Para coincidir con cualquier título que contenga `hotfix`, escriba `.*hotfix.*`. Sin el `.*` circundante, el filtro coincide solo con un título que es exactamente `hotfix` sin nada antes o después. Para coincidencia de subcadena literal sin sintaxis regex, use el operador `contains` en su lugar.

Algunos ejemplos de combinaciones de filtros:

* **Revisión del módulo de autenticación**: rama base `main`, rama principal contiene `auth-provider`. Envía cualquier PR que toque autenticación a un revisor enfocado.
* **Solo listo para revisión**: es borrador es `false`. Omite borradores para que la rutina solo se ejecute cuando el PR esté listo para revisión.
* **Retroportación controlada por etiqueta**: las etiquetas incluyen `needs-backport`. Activa una rutina de puerto a otra rama solo cuando un mantenedor etiqueta el PR.

<h2 id="manage-routines">
  Administrar rutinas
</h2>

Haga clic en una rutina en la lista para abrir su página de detalles. La página de detalles muestra los repositorios de la rutina, conectores, prompt, horario, tokens de API, disparadores de GitHub y una lista de ejecuciones anteriores.

<h3 id="view-and-interact-with-runs">
  Ver e interactuar con ejecuciones
</h3>

Haga clic en cualquier ejecución para abrirla como una sesión completa. Desde allí puede ver qué hizo Claude, revisar cambios, crear una solicitud de extracción o continuar la conversación. Cada sesión de ejecución funciona como cualquier otra sesión: use el menú desplegable junto al título de la sesión para renombrar, archivar o eliminar.

<Note>
  Un estado verde en la lista de ejecuciones significa que la sesión se inició y se cerró sin un error de infraestructura. No significa que la tarea en su prompt haya tenido éxito. Abra la ejecución para leer la transcripción y confirmar qué hizo realmente Claude. Las solicitudes de red bloqueadas, las herramientas de conectores faltantes y los fallos a nivel de tarea aparecen allí en lugar de en el indicador de estado.
</Note>

<h3 id="edit-and-control-routines">
  Editar y controlar rutinas
</h3>

Desde la página de detalles de la rutina puede:

* Haga clic en **Run now** para iniciar una ejecución inmediatamente sin esperar la próxima hora programada. Opcionalmente puede proporcionar texto específico de la ejecución, que llega a la rutina de la misma manera que el campo `text` del disparador de API.
* Use el botón de alternancia en la parte superior de la página para pausar o reanudar el horario. Las rutinas pausadas mantienen su configuración pero no se ejecutan hasta que las vuelva a habilitar.
* Abra el menú junto al nombre de la rutina y seleccione **Edit** para cambiar el nombre, prompt, repositorios, entorno, conectores o cualquiera de los disparadores de la rutina. La sección **Select a trigger** es donde agrega o elimina horarios, tokens de API y disparadores de eventos de GitHub.
* Abra el mismo menú y seleccione **Delete** para eliminar la rutina.

<h3 id="manage-routines-from-the-cli">
  Administrar rutinas desde la CLI
</h3>

La CLI admite la administración de rutinas existentes. Ejecute `/schedule list` para ver todas las rutinas, `/schedule update` para cambiar una, o `/schedule run` para activarla inmediatamente.

También puede preguntar sobre el historial de ejecuciones de una rutina, por ejemplo `/schedule why did my nightly review do nothing this morning?`. Claude enumera las ejecuciones recientes de la rutina con su estado y un enlace para [abrir cada ejecución en la web](#view-and-interact-with-runs), y lee el registro de una ejecución para explicar qué sucedió, incluidos errores de herramientas, denegaciones de permisos y el resultado final. Requiere Claude Code v2.1.227 o posterior.

<h3 id="repositories-and-branch-permissions">
  Repositorios y permisos de rama
</h3>

Las rutinas necesitan acceso a GitHub para clonar repositorios. Cuando crea una rutina desde la CLI con `/schedule`, Claude verifica si su cuenta tiene acceso a GitHub para el repositorio desde el que la ejecutó y, si no es así, agrega una nota de configuración indicando cómo otorgarlo. Consulte [Opciones de autenticación de GitHub](/docs/es/claude-code-on-the-web#github-authentication-options) para las dos formas de otorgar acceso.

Si su conexión a GitHub falta o ha expirado cuando una ejecución está programada, la rutina omite ejecuciones hasta que se reconecte, hasta 72 horas. Reconecte GitHub dentro de esa ventana y la rutina se reanuda automáticamente. Después de 72 horas sin una conexión, la rutina se desactiva y usted la vuelve a activar después de reconectar GitHub.

Cada repositorio que agregue se clona en cada ejecución. Claude comienza desde la rama predeterminada del repositorio a menos que su prompt especifique lo contrario.

Claude inserta su trabajo en ramas con prefijo `claude/`, que siempre se aceptan. Cuando su prompt dirige a Claude a insertar en otra rama, Claude Code verifica la inserción primero y la rechaza si alguno de los siguientes es verdadero:

* La rama está protegida en GitHub
* Alguien más tiene una solicitud de extracción abierta desde esa rama
* La rama contiene confirmaciones creadas por alguien que no es usted

<h3 id="connectors">
  Conectores
</h3>

Las rutinas pueden usar sus conectores MCP conectados para leer y escribir en servicios externos durante cada ejecución. Por ejemplo, una rutina que clasifica solicitudes de soporte podría leer de un canal de Slack y crear problemas en Linear.

Los conectores son las [integraciones de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) en su cuenta. Los servidores MCP que agregó localmente en la CLI con `claude mcp add` se almacenan en su máquina en lugar de su cuenta de claude.ai, por lo que no aparecen en la lista de conectores. Para usar uno de esos servidores en una rutina, agréguelo como conector en [claude.ai/customize/connectors](https://claude.ai/customize/connectors). Para una rutina con un repositorio, puede declararlo en un [`.mcp.json`](/docs/es/mcp#project-scope) confirmado para que sea parte del repositorio clonado.

Cuando crea una rutina, todos sus conectores actualmente conectados se incluyen de forma predeterminada. Elimine cualquiera que no sea necesario para limitar a qué herramientas tiene acceso Claude durante la ejecución. También puede agregar conectores directamente desde el formulario de rutina.

Para administrar o agregar conectores fuera del formulario de rutina, visite [claude.ai/customize/connectors](https://claude.ai/customize/connectors) o use `/schedule update` en la CLI.

<h3 id="environments-and-network-access">
  Entornos y acceso a la red
</h3>

Cada rutina usa un [entorno en la nube](/docs/es/cloud-environments) que controla el acceso a la red, variables de entorno y scripts de configuración. La rutina hereda la política de red del entorno en cada ejecución.

El entorno **Default** utiliza acceso a la red **Trusted**, que permite solo la [lista de permitidos predeterminada](/docs/es/cloud-environments#default-allowed-domains) a través de la red de la sesión. Las solicitudes en esa ruta a hosts fuera de la lista de permitidos fallan con `403` y `x-deny-reason: host_not_allowed`. El tráfico del conector MCP se enruta a través de los servidores de Anthropic en lugar de esa ruta, por lo que los conectores que agregue a la rutina funcionan sin agregar sus hosts a **Allowed domains**. Elimine cualquier conector que no necesite en [Conectores](#connectors).

Para permitir dominios adicionales en uno de sus propios entornos, siga estos pasos. Un [entorno compartido por la organización](/docs/es/cloud-environments#organization-shared-environments) se abre como solo lectura aquí, por lo que un Propietario cambia su acceso a la red desde la página **Cloud environments** en [configuración de administración](https://claude.ai/admin-settings) en su lugar.

<Steps>
  <Step title="Abra la rutina para editar">
    En la página de detalles de la rutina, abra el menú junto al nombre de la rutina y seleccione **Edit**.
  </Step>

  <Step title="Abra el selector de entorno">
    Debajo del cuadro **Instructions**, seleccione el icono de nube que muestra el nombre de su entorno, como **Default**.
  </Step>

  <Step title="Abra la configuración del entorno">
    Pase el cursor sobre el entorno en la lista y haga clic en el icono de configuración que aparece a la derecha.
  </Step>

  <Step title="Cambie el nivel de acceso a la red">
    En el diálogo **Update cloud environment**, cambie **Network access** a **Custom** e ingrese sus dominios en **Allowed domains**. Marque **Also include default list of common package managers** para mantener la [lista de permitidos predeterminada](/docs/es/cloud-environments#default-allowed-domains) junto con sus dominios personalizados. Seleccione **Full** en su lugar para acceso sin restricciones.
  </Step>

  <Step title="Guardar">
    Haga clic en **Save changes**. La nueva política se aplica desde la próxima ejecución.
  </Step>
</Steps>

Consulte [Network access](/docs/es/cloud-environments#network-access) para obtener detalles sobre los niveles de acceso y la lista de permitidos predeterminada.

<h2 id="usage-and-limits">
  Uso y límites
</h2>

Las rutinas reducen el uso de suscripción de la misma manera que lo hacen las sesiones interactivas. Además de los límites de suscripción estándar, las rutinas tienen un límite diario de cuántas ejecuciones pueden comenzar por cuenta. Vea su consumo actual y ejecuciones diarias de rutina restantes en [claude.ai/code/routines](https://claude.ai/code/routines) o [claude.ai/settings/usage](https://claude.ai/settings/usage).

Cuando una rutina alcanza el límite diario o el límite de uso de su suscripción, las organizaciones con créditos de uso habilitados pueden continuar ejecutando rutinas en exceso medido. Sin créditos de uso, las ejecuciones adicionales se rechazan hasta que se reinicia la ventana. Habilite los créditos de uso en [claude.ai/settings/usage](https://claude.ai/settings/usage). En los planes Team y Enterprise, un administrador los habilita para la organización en [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage).

Las ejecuciones únicas no cuentan contra el límite diario de rutina. Reducen su uso de suscripción regular como cualquier otra sesión.

Mientras su suscripción está pausada, sus rutinas se ponen en espera y no se ejecutan. Una vez que su suscripción esté activa nuevamente, actívelas de nuevo.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

<h3 id="schedule-returns-unknown-command">
  `/schedule` devuelve "Unknown command"
</h3>

La CLI oculta `/schedule` cuando uno de sus requisitos no se cumple: el menú de comandos muestra `No commands match "/schedule"` mientras escribe. Enviarlo devuelve `Unknown command: /schedule`, excepto en los casos siguientes que indican una respuesta diferente.

La causa suele ser una de las siguientes:

* Está autenticado con una clave de API de Console, un [perfil de Anthropic o credencial de federación](/docs/es/authentication#anthropic-profiles-and-federation-credentials), o un proveedor de nube como Amazon Bedrock, Google Cloud's Agent Platform o Microsoft Foundry. `/schedule` requiere un inicio de sesión de suscripción de claude.ai. Con una clave de API de Console o un perfil, y con la obtención de indicadores de características habilitada, enviar `/schedule` muestra en su lugar `/schedule is available with Claude for Enterprise — ask your admin about migrating from API-key access`. Con un inicio de sesión de proveedor de nube, aún ve `Unknown command: /schedule`. Si `ANTHROPIC_API_KEY` o `ANTHROPIC_AUTH_TOKEN` está configurado en su shell, o `apiKeyHelper` está configurado en `settings.json`, elimínelo primero, ya que estos tienen prioridad sobre un inicio de sesión de claude.ai. Un perfil o credencial de federación también tiene prioridad, así que desactívelo también
* Está completamente desconectado, sin clave de API ni otra credencial. Con la obtención de indicadores de características habilitada, enviar `/schedule` muestra `/schedule requires a claude.ai subscription. Run /login to sign in with your claude.ai account.` Antes de v2.1.268, una sesión desconectada mostraba el mismo mensaje de Claude for Enterprise que una clave de API de Console
* Se encuentra dentro de una sesión de nube, donde enviar `/schedule` responde que el comando no está disponible en ese entorno. Administre rutinas desde la [interfaz web](https://claude.ai/code/routines) en su lugar
* La política de su organización desactiva [Claude Code en la web](/docs/es/claude-code-on-the-web), en la que se ejecutan las rutinas. En este caso, enviar `/schedule` responde [`Cloud sessions are disabled by your organization's policy`](/docs/es/errors#cloud-sessions-are-disabled-by-your-organizations-policy) en su lugar. Antes de v2.1.268, devolvía `Unknown command: /schedule`
* Un propietario [desactivó las rutinas](#routines-are-disabled-by-your-organizations-policy) para su organización de Team o Enterprise. Antes de v2.1.227, el comando aún aparecía en este caso, y claude.ai rechazaba la rutina cuando Claude intentaba crearla o ejecutarla

A menos que la política de su organización desactive las rutinas o Claude Code en la web, puede crear y administrar rutinas en [claude.ai/code/routines](https://claude.ai/code/routines) independientemente de cómo esté configurada la CLI.

<h3 id="routines-are-disabled-by-your-organizations-policy">
  "Routines are disabled by your organization's policy"
</h3>

Un propietario en su organización de Team o Enterprise probablemente ha desactivado el botón de alternancia **Routines** en [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code). En Claude Code v2.1.227 o posterior, el mismo botón de alternancia también oculta `/schedule` en la CLI. Esta es una configuración de organización del lado del servidor, por lo que no se puede anular desde su configuración local. Pida a un propietario que habilite las rutinas para su organización.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [`/loop` e programación en sesión](/docs/es/scheduled-tasks): programe tareas locales dentro de una sesión de CLI abierta
* [Tareas programadas de escritorio](/docs/es/desktop-scheduled-tasks): tareas programadas locales que se ejecutan en su máquina con acceso a archivos locales
* [Entornos en la nube](/docs/es/cloud-environments): configure el acceso a la red, las variables de entorno y los scripts de configuración para sesiones en la nube
* [Proyectos](/docs/es/claude-projects): trabajo en curso que Claude coordina en sesiones en la nube paralelas; las rutinas creadas a partir de un proyecto aparecen en su pestaña **Routines**
* [Conectores MCP](/docs/es/mcp): conecte servicios externos como Slack, Linear y Google Drive
* [GitHub Actions](/docs/es/github-actions): ejecute Claude en su canalización de CI en eventos del repositorio
